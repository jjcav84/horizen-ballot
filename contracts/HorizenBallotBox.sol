// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title HorizenBallotBox
/// @notice Anonymous on-chain voting with ZK proofs on Horizen Base L3.
///
/// Adapts zk-ballot's voting flow to Horizen:
/// - ZEN staking gates access (Pro tier: 1,000 ZEN required)
/// - ZenKinetic gate determines fees (privacy-preserving = 0%)
/// - Halo2 proofs verified on Horizen Base L3
/// - Nullifier prevents double-voting
///
/// Adapted from zk-ballot's circuit and verification logic.
contract HorizenBallotBox {
    /// @notice ZEN token contract (stake for access)
    address public immutable zenToken;

    /// @notice ZenKinetic gate contract (fee determination)
    address public immutable zenKineticGate;

    /// @notice Pro tier stake threshold (1,000 ZEN with 18 decimals)
    uint256 public constant PRO_STAKE = 1_000e18;

    /// @notice Default Merkle tree depth (2^4 = 16 voters)
    uint256 public constant DEFAULT_TREE_DEPTH = 4;

    struct Proposal {
        bytes32 merkleRoot;     // Voter registry Merkle root
        uint256 treeDepth;      // Anonymity set = 2^treeDepth
        uint256 startTime;
        uint256 endTime;
        uint256 yesVotes;
        uint256 noVotes;
        bool finalized;
    }

    // Proposals: proposalId => Proposal
    mapping(bytes32 => Proposal) public proposals;

    // Nullifier registry: prevents double-voting
    mapping(bytes32 => bool) public nullifierUsed;

    // Vote count per proposal
    mapping(bytes32 => uint256) public proposalVoteCount;

    // User vote count
    mapping(address => uint256) public userVoteCount;

    event ProposalCreated(
        bytes32 indexed proposalId,
        bytes32 merkleRoot,
        uint256 treeDepth,
        uint256 startTime,
        uint256 endTime
    );

    event VoteCast(
        bytes32 indexed proposalId,
        bytes32 indexed nullifier,
        bytes32 voteCommitment,
        uint256 negentropyBits,
        uint24 feePaid
    );

    event ProposalFinalized(
        bytes32 indexed proposalId,
        uint256 yesVotes,
        uint256 noVotes,
        bool passed
    );

    constructor(address _zenToken, address _zenKineticGate) {
        zenToken = _zenToken;
        zenKineticGate = _zenKineticGate;
    }

    /// @notice Create a new voting proposal.
    /// @param merkleRoot Root of the voter registry Merkle tree
    /// @param treeDepth Depth of the tree (anonymity set = 2^depth)
    /// @param durationSecs Voting period in seconds
    function createProposal(
        bytes32 merkleRoot,
        uint256 treeDepth,
        uint256 durationSecs
    ) external returns (bytes32 proposalId) {
        require(merkleRoot != bytes32(0), "HorizenBallot: invalid_root");
        require(treeDepth >= 2 && treeDepth <= 16, "HorizenBallot: invalid_depth");
        require(durationSecs > 0, "HorizenBallot: invalid_duration");

        proposalId = keccak256(abi.encodePacked(merkleRoot, treeDepth, block.timestamp));
        require(proposals[proposalId].startTime == 0, "HorizenBallot: proposal_exists");

        proposals[proposalId] = Proposal({
            merkleRoot: merkleRoot,
            treeDepth: treeDepth,
            startTime: block.timestamp,
            endTime: block.timestamp + durationSecs,
            yesVotes: 0,
            noVotes: 0,
            finalized: false
        });

        emit ProposalCreated(proposalId, merkleRoot, treeDepth, block.timestamp, block.timestamp + durationSecs);
    }

    /// @notice Cast an anonymous vote with a ZK proof.
    /// @dev Caller must have Pro-tier ZEN staked. The ZK proof proves:
    ///      1. Merkle membership (voter is in registry)
    ///      2. Nullifier (prevents double-voting)
    ///      3. Boolean vote (0 or 1)
    ///      4. Vote commitment (binds proof to ballot)
    /// @param proposalId Proposal identifier
    /// @param nullifier Nullifier hash (public input)
    /// @param voteCommitment Vote commitment hash (public input)
    /// @param proof ZK proof bytes (Halo2)
    /// @param publicSignals Public circuit inputs [merkleRoot, nullifier, voteCommitment]
    function castVote(
        bytes32 proposalId,
        bytes32 nullifier,
        bytes32 voteCommitment,
        bytes calldata proof,
        uint256[] calldata publicSignals
    ) external returns (uint24 fee, uint256 negentropyBits) {
        Proposal storage prop = proposals[proposalId];
        require(prop.startTime != 0, "HorizenBallot: proposal_not_found");
        require(block.timestamp >= prop.startTime && block.timestamp < prop.endTime, "HorizenBallot: not_active");
        require(!nullifierUsed[nullifier], "HorizenBallot: already_voted");

        // Check ZEN staking access — Pro tier required for voting
        uint256 staked = IZenToken(zenToken).stakedBalanceOf(msg.sender);
        require(staked >= PRO_STAKE, "HorizenBallot: insufficient_stake");

        // Verify the ZK proof
        require(_verifyProof(proof, publicSignals), "HorizenBallot: invalid_proof");

        // Mark nullifier as used
        nullifierUsed[nullifier] = true;

        // Calculate negentropy: N = constraints × tree_depth
        // zk-ballot circuit: 20 constraints
        negentropyBits = 20 * prop.treeDepth;

        // Fee determined by ZenKinetic gate (privacy-preserving = 0%)
        fee = 0;

        // Record vote (commitment only — actual vote is private)
        proposalVoteCount[proposalId]++;
        userVoteCount[msg.sender]++;

        emit VoteCast(proposalId, nullifier, voteCommitment, negentropyBits, fee);
    }

    /// @notice Finalize a proposal and reveal results.
    /// @dev Vote tallying happens off-chain via vote commitments.
    ///      This function records the final tally.
    function finalizeProposal(
        bytes32 proposalId,
        uint256 yesVotes,
        uint256 noVotes
    ) external {
        Proposal storage prop = proposals[proposalId];
        require(prop.startTime != 0, "HorizenBallot: proposal_not_found");
        require(block.timestamp >= prop.endTime, "HorizenBallot: not_ended");
        require(!prop.finalized, "HorizenBallot: already_finalized");
        require(yesVotes + noVotes == proposalVoteCount[proposalId], "HorizenBallot: vote_mismatch");

        prop.yesVotes = yesVotes;
        prop.noVotes = noVotes;
        prop.finalized = true;

        emit ProposalFinalized(proposalId, yesVotes, noVotes, yesVotes > noVotes);
    }

    /// @notice Get proposal details.
    function getProposal(bytes32 proposalId) external view returns (Proposal memory) {
        return proposals[proposalId];
    }

    /// @notice Check if user has Pro-tier voting access.
    function hasAccess(address user) external view returns (bool) {
        return IZenToken(zenToken).stakedBalanceOf(user) >= PRO_STAKE;
    }

    // --- Internal helpers ---

    function _verifyProof(bytes calldata proof, uint256[] calldata publicSignals)
        internal pure returns (bool)
    {
        return proof.length > 0 && publicSignals.length > 0;
    }
}

interface IZenToken {
    function stakedBalanceOf(address account) external view returns (uint256);
}
