// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {AICID} from "../src/AICID.sol";

contract AICIDTest is Test {
    AICID id;

    address alice = address(0xA11CE);
    address bob   = address(0xB0B);

    event NameRegistered(string name, address indexed owner, uint64 expiresAt);
    event NameRenewed(string name, uint64 expiresAt);
    event NameTransferred(string name, address indexed from, address indexed to);

    function setUp() public {
        id = new AICID();
    }

    // --- register ---

    function test_register_success() public {
        vm.expectEmit(true, false, false, true);
        emit NameRegistered("alice", alice, uint64(block.timestamp) + 365 days);

        id.register("alice", alice);

        assertEq(id.resolve("alice"), alice);
        assertEq(id.reverse(alice), keccak256(bytes("alice")));
        assertEq(id.reverseResolve(alice), "alice");
        assertFalse(id.isAvailable("alice"));
    }

    function test_register_reverts_if_taken() public {
        id.register("alice", alice);
        vm.expectRevert(abi.encodeWithSelector(AICID.NameTaken.selector, "alice"));
        id.register("alice", bob);
    }

    function test_register_reverts_on_invalid_name() public {
        vm.expectRevert(abi.encodeWithSelector(AICID.InvalidName.selector, "Alice"));
        id.register("Alice", alice);

        vm.expectRevert(abi.encodeWithSelector(AICID.InvalidName.selector, ""));
        id.register("", alice);

        vm.expectRevert(abi.encodeWithSelector(AICID.InvalidName.selector, "with space"));
        id.register("with space", alice);
    }

    function test_register_reverts_on_zero_address() public {
        vm.expectRevert(AICID.ZeroAddress.selector);
        id.register("alice", address(0));
    }

    // --- resolve ---

    function test_resolve_unknown_returns_zero() public view {
        assertEq(id.resolve("ghost"), address(0));
    }

    // --- renew ---

    function test_renew_extends_expiry() public {
        id.register("alice", alice);
        (, , uint64 expBefore) = id.records(keccak256(bytes("alice")));

        vm.warp(block.timestamp + 100 days);

        vm.expectEmit(false, false, false, true);
        emit NameRenewed("alice", uint64(block.timestamp) + 365 days);

        vm.prank(alice);
        id.renew("alice");

        (, , uint64 expAfter) = id.records(keccak256(bytes("alice")));
        assertGt(expAfter, expBefore);
        assertEq(expAfter, uint64(block.timestamp) + 365 days);
    }

    function test_renew_reverts_if_not_owner() public {
        id.register("alice", alice);
        vm.prank(bob);
        vm.expectRevert(abi.encodeWithSelector(AICID.NotOwner.selector, "alice", bob));
        id.renew("alice");
    }

    // --- expiry ---

    function test_expired_name_becomes_available() public {
        id.register("alice", alice);
        assertFalse(id.isAvailable("alice"));

        vm.warp(block.timestamp + 366 days);
        assertTrue(id.isAvailable("alice"));
        assertEq(id.resolve("alice"), address(0));
    }

    function test_expired_name_can_be_re_registered_by_anyone() public {
        id.register("alice", alice);
        vm.warp(block.timestamp + 366 days);

        id.register("alice", bob);
        assertEq(id.resolve("alice"), bob);
    }

    // --- transfer ---

    function test_transfer_success_updates_reverse() public {
        id.register("alice", alice);

        vm.expectEmit(true, true, false, true);
        emit NameTransferred("alice", alice, bob);

        vm.prank(alice);
        id.transferName("alice", bob);

        assertEq(id.resolve("alice"), bob);
        assertEq(id.reverse(alice), bytes32(0));
        assertEq(id.reverse(bob), keccak256(bytes("alice")));
        assertEq(id.reverseResolve(bob), "alice");
    }

    function test_transfer_reverts_if_not_owner() public {
        id.register("alice", alice);
        vm.prank(bob);
        vm.expectRevert(abi.encodeWithSelector(AICID.NotOwner.selector, "alice", bob));
        id.transferName("alice", bob);
    }

    function test_transfer_reverts_to_zero_address() public {
        id.register("alice", alice);
        vm.prank(alice);
        vm.expectRevert(AICID.ZeroAddress.selector);
        id.transferName("alice", address(0));
    }

    // --- reverseResolve ---

    function test_reverse_resolve_reverts_for_unknown_address() public {
        vm.expectRevert(abi.encodeWithSelector(AICID.NameNotFound.selector, "(reverse)"));
        id.reverseResolve(bob);
    }

    // --- validation ---

    function test_is_valid_name() public view {
        assertTrue(id.isValidName("a"));
        assertTrue(id.isValidName("alice"));
        assertTrue(id.isValidName("alice-01"));
        assertTrue(id.isValidName("a_b-c_9"));
        assertTrue(id.isValidName("12345678901234567890123456789012")); // 32 chars

        assertFalse(id.isValidName(""));
        assertFalse(id.isValidName("Alice"));        // uppercase
        assertFalse(id.isValidName("with space"));
        assertFalse(id.isValidName("dot.name"));
        assertFalse(id.isValidName("123456789012345678901234567890123")); // 33 chars
    }
}
