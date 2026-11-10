// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {QuantumBounty} from "../src/QuantumBounty.sol";

contract QuantumBountyTest is Test {
    QuantumBounty bounty;

    uint256 constant HIDDEN_PK = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    address hiddenAddr;

    // Validators for PQ voting (Hardhat keys)
    uint256 constant V1_PK = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
    uint256 constant V2_PK = 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d;
    uint256 constant V3_PK = 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a;

    address v1;
    address v2;
    address v3;
    address treasury = address(0xF00D);
    address solver = address(0xA11CE);

    uint256 constant CLASSIC_REWARD = 1_000_000 ether;
    uint256 constant PQ_REWARD = 2_000_000 ether;

    function setUp() public {
        hiddenAddr = vm.addr(HIDDEN_PK);
        v1 = vm.addr(V1_PK);
        v2 = vm.addr(V2_PK);
        v3 = vm.addr(V3_PK);

        bounty = new QuantumBounty(treasury);

        // Fund pool: 3M AIC
        vm.deal(address(this), 3_000_000 ether);
        bounty.fund{value: 3_000_000 ether}();

        // Setup classic challenge
        bounty.setClassicChallenge(hiddenAddr, CLASSIC_REWARD, "Break me if you can, quantum!");

        // Setup PQ validators (2 of 3)
        bounty.addValidator(v1);
        bounty.addValidator(v2);
        bounty.addValidator(v3);
        bounty.setPQThreshold(2);
        bounty.setPQReward(PQ_REWARD);
    }

    // ===== Helpers =====

    function _signClassic(string memory message) internal view returns (bytes memory) {
        bytes32 msgHash = keccak256(bytes(message));
        bytes32 ethHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", msgHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(HIDDEN_PK, ethHash);
        return abi.encodePacked(r, s, v);
    }

    // ===== Constructor & admin =====

    function test_constructor() public view {
        assertEq(bounty.owner(), address(this));
        assertEq(bounty.treasury(), treasury);
    }

    function test_fund() public view {
        assertEq(bounty.totalPrizePool(), 3_000_000 ether);
    }

    function test_only_owner_admin() public {
        vm.prank(solver);
        vm.expectRevert(QuantumBounty.NotOwner.selector);
        bounty.setPQReward(1 ether);
    }

    // ===== Mode A: Classic =====

    function test_classic_solved_with_correct_signature() public {
        bytes memory sig = _signClassic("Break me if you can, quantum!");

        uint256 before = solver.balance;
        vm.prank(solver);
        bounty.solveClassic(sig);

        assertEq(solver.balance, before + CLASSIC_REWARD);
        assertTrue(bounty.classicSolved());
        assertEq(bounty.classicSolver(), solver);
        assertEq(bounty.totalPrizePool(), 2_000_000 ether);
    }

    function test_classic_rejects_wrong_signature() public {
        // Подписываем другим ключом
        bytes32 msgHash = keccak256(bytes("Break me if you can, quantum!"));
        bytes32 ethHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", msgHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(V1_PK, ethHash);
        bytes memory wrongSig = abi.encodePacked(r, s, v);

        vm.expectRevert(QuantumBounty.InvalidSignature.selector);
        bounty.solveClassic(wrongSig);
    }

    function test_classic_rejects_double_solve() public {
        bytes memory sig = _signClassic("Break me if you can, quantum!");

        // solver -- EOA, может принимать ETH
        vm.prank(solver);
        bounty.solveClassic(sig);

        vm.expectRevert(QuantumBounty.AlreadySolved.selector);
        vm.prank(solver);
        bounty.solveClassic(sig);
    }

    function test_classic_rejects_bad_sig_length() public {
        bytes memory badSig = new bytes(10);
        vm.expectRevert(QuantumBounty.InvalidSignature.selector);
        bounty.solveClassic(badSig);
    }

    // ===== Mode B: PQ =====

    function test_pq_solved_with_threshold_votes() public {
        // v1 и v2 голосуют
        vm.prank(v1);
        bounty.votePQSolved(solver);
        assertEq(bounty.pqVoteCount(), 1);

        vm.prank(v2);
        bounty.votePQSolved(solver);
        assertEq(bounty.pqVoteCount(), 2);

        // claim
        uint256 before = solver.balance;
        bounty.claimPQ();
        assertEq(solver.balance, before + PQ_REWARD);
        assertTrue(bounty.pqSolved());
        assertEq(bounty.totalPrizePool(), 1_000_000 ether);
    }

    function test_pq_rejects_non_validator_vote() public {
        vm.prank(solver);
        vm.expectRevert(QuantumBounty.NotValidator.selector);
        bounty.votePQSolved(solver);
    }

    function test_pq_rejects_double_vote() public {
        vm.prank(v1);
        bounty.votePQSolved(solver);
        vm.prank(v1);
        vm.expectRevert(QuantumBounty.AlreadyVoted.selector);
        bounty.votePQSolved(solver);
    }

    function test_pq_rejects_claim_before_threshold() public {
        vm.prank(v1);
        bounty.votePQSolved(solver);

        vm.expectRevert(QuantumBounty.NotSolved.selector);
        bounty.claimPQ();
    }

    function test_pq_rejects_double_claim() public {
        vm.prank(v1);
        bounty.votePQSolved(solver);
        vm.prank(v2);
        bounty.votePQSolved(solver);
        bounty.claimPQ();

        vm.expectRevert(QuantumBounty.AlreadySolved.selector);
        bounty.claimPQ();
    }

    // ===== Edge: insufficient pool =====

    function test_classic_rejects_insufficient_pool() public {
        // Уменьшаем пул до 0
        // Админ контракта = this. Выведем всё через fund нельзя, поэтому новый контракт
        QuantumBounty small = new QuantumBounty(treasury);
        vm.deal(address(this), 100 ether);
        small.fund{value: 100 ether}();
        small.setClassicChallenge(hiddenAddr, 1_000_000 ether, "test");

        bytes32 msgHash = keccak256(bytes("test"));
        bytes32 ethHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", msgHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(HIDDEN_PK, ethHash);
        bytes memory sig = abi.encodePacked(r, s, v);

        vm.expectRevert(QuantumBounty.InsufficientPool.selector);
        small.solveClassic(sig);
    }
}

