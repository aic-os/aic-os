// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

/// @title QuantumBounty -- public challenge for breaking crypto.
/// @notice Mode A: 1M AIC for breaking classic ECDSA (on-chain, ecrecover).
///         Mode B: 2M AIC for breaking PQ (ML-DSA-65) -- validated by multisig off-chain.
contract QuantumBounty {
    // ===== Common =====
    address public owner;
    address public treasury;
    uint256 public totalPrizePool;

    // ===== Mode A: Classic ECDSA (on-chain) =====
    /// Target address (derived from hidden private key).
    address public classicTarget;
    uint256 public classicReward;
    bool public classicSolved;
    address public classicSolver;
    bytes32 public classicMessageHash; // keccak256 of challenge message

    // ===== Mode B: PQ (off-chain multisig) =====
    /// PQ bounty: validators confirm PQ was broken off-chain, then call vote.
    uint256 public pqReward;
    bool public pqSolved;
    address public pqSolver;
    mapping(address => bool) public pqVotes;
    mapping(address => bool) public validators;
    uint256 public validatorCount;
    uint256 public pqVoteThreshold;
    uint256 public pqVoteCount;

    // ===== Events =====
    event ClassicChallengeSet(address target, uint256 reward, bytes32 messageHash);
    event ClassicSolved(address solver, uint256 reward);
    event PQVoteAdded(address validator, address claimedSolver, uint256 count);
    event PQSolved(address solver, uint256 reward);
    event ValidatorAdded(address v);
    event ValidatorRemoved(address v);
    event ThresholdChanged(uint256 threshold);
    event Funded(address from, uint256 amount);

    // ===== Errors =====
    error NotOwner();
    error NotValidator();
    error AlreadySolved();
    error NotSolved();
    error InvalidSignature();
    error InsufficientPool();
    error TransferFailed();
    error AlreadyVoted();
    error InvalidThreshold();

    modifier onlyOwner() {
        if (msg.sender != owner) revert NotOwner();
        _;
    }

    constructor(address _treasury) {
        owner = msg.sender;
        treasury = _treasury;
    }

    // ===== Admin =====

    function fund() external payable {
        totalPrizePool += msg.value;
        emit Funded(msg.sender, msg.value);
    }

    /// Set classic challenge. Owner must send 1M AIC (or more).
    function setClassicChallenge(address target, uint256 reward, string calldata message) external payable onlyOwner {
        require(!classicSolved, "already solved");
        classicTarget = target;
        classicReward = reward;
        classicMessageHash = keccak256(bytes(message));
        emit ClassicChallengeSet(target, reward, classicMessageHash);
    }

    function setPQReward(uint256 reward) external onlyOwner {
        pqReward = reward;
    }

    function addValidator(address v) external onlyOwner {
        if (validators[v]) return;
        validators[v] = true;
        validatorCount++;
        emit ValidatorAdded(v);
    }

    function removeValidator(address v) external onlyOwner {
        if (!validators[v]) return;
        validators[v] = false;
        validatorCount--;
        emit ValidatorRemoved(v);
    }

    function setPQThreshold(uint256 t) external onlyOwner {
        if (t == 0 || t > validatorCount) revert InvalidThreshold();
        pqVoteThreshold = t;
        emit ThresholdChanged(t);
    }

    // ===== Mode A: solve (on-chain) =====

    /// Caller provides a signature over classicMessageHash, signed by classicTarget's private key.
    /// If ecrecover matches classicTarget -- reward is released.
    function solveClassic(bytes calldata signature) external {
        if (classicSolved) revert AlreadySolved();
        if (classicTarget == address(0)) revert NotSolved();

        address recovered = _recover(classicMessageHash, signature);
        if (recovered != classicTarget) revert InvalidSignature();

        if (totalPrizePool < classicReward) revert InsufficientPool();

        classicSolved = true;
        classicSolver = msg.sender;
        totalPrizePool -= classicReward;

        (bool ok, ) = payable(msg.sender).call{value: classicReward}("");
        if (!ok) revert TransferFailed();

        emit ClassicSolved(msg.sender, classicReward);
    }

    // ===== Mode B: PQ (off-chain multisig) =====

    /// Validator confirms PQ crypto was broken (off-chain verification).
    function votePQSolved(address solver) external {
        if (!validators[msg.sender]) revert NotValidator();
        if (pqVotes[msg.sender]) revert AlreadyVoted();
        pqVotes[msg.sender] = true;
        pqVoteCount++;
        pqSolver = solver;
        emit PQVoteAdded(msg.sender, solver, pqVoteCount);
    }

    /// Anyone can trigger the PQ payout once threshold is reached.
    function claimPQ() external {
        if (pqSolved) revert AlreadySolved();
        if (pqVoteThreshold == 0 || pqVoteCount < pqVoteThreshold) revert NotSolved();
        if (totalPrizePool < pqReward) revert InsufficientPool();

        pqSolved = true;
        totalPrizePool -= pqReward;

        (bool ok, ) = payable(pqSolver).call{value: pqReward}("");
        if (!ok) revert TransferFailed();

        emit PQSolved(pqSolver, pqReward);
    }

    // ===== Internal =====

    function _recover(bytes32 messageHash, bytes calldata sig) internal pure returns (address) {
        if (sig.length != 65) return address(0);
        bytes32 ethHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash));
        bytes32 r = bytes32(sig[0:32]);
        bytes32 s = bytes32(sig[32:64]);
        uint8 v = uint8(sig[64]);
        if (v < 27) v += 27;
        return ecrecover(ethHash, v, r, s);
    }
}
