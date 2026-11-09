// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {AICID} from "../src/AICID.sol";

contract AICIDTest is Test {
    AICID id;

    address alice = address(0xA11CE);
    address bob   = address(0xB0B);
    address treasury = address(0xF);

    uint256 constant FEE = 10 ether;
    uint256 constant RENEW_FEE = 5 ether;

    function setUp() public {
        id = new AICID();
        vm.deal(alice, 100 ether);
        vm.deal(bob, 100 ether);
    }

    function test_register_success_with_fee() public {
        uint256 treasuryBefore = treasury.balance;

        vm.prank(alice);
        id.register{value: FEE}("alice", alice);

        assertEq(id.resolve("alice"), alice);
        assertEq(id.reverseResolve(alice), "alice");
        assertEq(treasury.balance, treasuryBefore + FEE);
        assertFalse(id.isAvailable("alice"));
    }

    function test_register_rejects_insufficient_fee() public {
        vm.prank(alice);
        vm.expectRevert(abi.encodeWithSelector(AICID.InsufficientFee.selector, 5 ether, FEE));
        id.register{value: 5 ether}("alice", alice);
    }


    function test_register_reverts_if_taken() public {
        vm.prank(alice);
        id.register{value: FEE}("alice", alice);

        vm.prank(bob);
        vm.expectRevert(abi.encodeWithSelector(AICID.NameTaken.selector, "alice"));
        id.register{value: FEE}("alice", bob);
    }

    function test_register_reverts_on_invalid_name() public {
        vm.startPrank(alice);
        vm.expectRevert(abi.encodeWithSelector(AICID.InvalidName.selector, "Alice"));
        id.register{value: FEE}("Alice", alice);
        vm.expectRevert(abi.encodeWithSelector(AICID.InvalidName.selector, ""));
        id.register{value: FEE}("", alice);
        vm.stopPrank();
    }

    function test_renew_extends_expiry_with_fee() public {
        vm.prank(alice);
        id.register{value: FEE}("alice", alice);

        (, , uint64 expBefore) = id.records(keccak256(bytes("alice")));
        vm.warp(block.timestamp + 100 days);

        uint256 treasuryBefore = treasury.balance;
        vm.prank(alice);
        id.renew{value: RENEW_FEE}("alice");

        (, , uint64 expAfter) = id.records(keccak256(bytes("alice")));
        assertGt(expAfter, expBefore);
        assertEq(treasury.balance, treasuryBefore + RENEW_FEE);
    }

    function test_renew_rejects_insufficient_fee() public {
        vm.prank(alice);
        id.register{value: FEE}("alice", alice);

        vm.prank(alice);
        vm.expectRevert(abi.encodeWithSelector(AICID.InsufficientFee.selector, 1 ether, RENEW_FEE));
        id.renew{value: 1 ether}("alice");
    }

    function test_expired_name_becomes_available() public {
        vm.prank(alice);
        id.register{value: FEE}("alice", alice);

        vm.warp(block.timestamp + 366 days);
        assertTrue(id.isAvailable("alice"));
        assertEq(id.resolve("alice"), address(0));
    }

    function test_transfer_is_free() public {
        vm.prank(alice);
        id.register{value: FEE}("alice", alice);

        uint256 aliceBefore = alice.balance;
        vm.prank(alice);
        id.transferName("alice", bob);

        // transfer -- бесплатный, баланс не изменился
        assertEq(alice.balance, aliceBefore);
        assertEq(id.resolve("alice"), bob);
    }

    function test_treasury_accumulates_fees() public {
        uint256 start = treasury.balance;

        vm.prank(alice); id.register{value: FEE}("alice", alice);
        vm.prank(bob);   id.register{value: FEE}("bob", bob);
        vm.prank(alice); id.register{value: FEE}("charlie", alice);
        vm.prank(bob);   id.register{value: FEE}("dave", bob);

        // 4 регистрации x 10 AIC = 40 AIC в казне
        assertEq(treasury.balance, start + 4 * FEE);
    }

    function test_is_valid_name() public view {
        assertTrue(id.isValidName("alice"));
        assertTrue(id.isValidName("a_b-c"));
        assertFalse(id.isValidName(""));
        assertFalse(id.isValidName("Alice"));
        assertFalse(id.isValidName("with space"));
    }

    function test_constants() public view {
        assertEq(id.REGISTRATION_FEE(), 10 ether);
        assertEq(id.RENEWAL_FEE(), 5 ether);
    }
}

