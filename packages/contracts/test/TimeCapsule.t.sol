// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {TimeCapsule} from "../src/TimeCapsule.sol";

contract TimeCapsuleTest is Test {
    TimeCapsule tc;

    address alice = address(0xA11CE);
    address bob   = address(0xB0B);
    address eve   = address(0xE7E);

    event CapsuleCreated(uint256 indexed id, address indexed sender, address indexed recipient, uint256 amount, uint64 unlockAt);
    event CapsuleToppedUp(uint256 indexed id, uint256 amount);
    event CapsuleClaimed(uint256 indexed id, address indexed recipient, uint256 amount);
    event CapsuleCancelled(uint256 indexed id, address indexed sender, uint256 refund);

    function setUp() public {
        tc = new TimeCapsule();
        vm.deal(alice, 100 ether);
        vm.deal(bob, 100 ether);
    }

    // --- create ---

    function test_create_success() public {
        uint64 unlock = uint64(block.timestamp + 7 days);
        vm.expectEmit(true, true, true, true);
        emit CapsuleCreated(1, alice, bob, 1 ether, unlock);

        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, unlock);

        assertEq(id, 1);
        (address s, address r, uint64 u, uint256 bal, bool claimed, bool cancelled) = tc.capsules(1);
        assertEq(s, alice);
        assertEq(r, bob);
        assertEq(u, unlock);
        assertEq(bal, 1 ether);
        assertFalse(claimed);
        assertFalse(cancelled);
    }

    function test_create_increments_id() public {
        vm.startPrank(alice);
        uint256 a = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 days));
        uint256 b = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 2 days));
        vm.stopPrank();
        assertEq(a, 1);
        assertEq(b, 2);
        assertEq(tc.nextId(), 3);
    }

    function test_create_rejects_zero_recipient() public {
        vm.prank(alice);
        vm.expectRevert(TimeCapsule.InvalidRecipient.selector);
        tc.create{value: 1 ether}(address(0), uint64(block.timestamp + 1 days));
    }

    function test_create_rejects_zero_amount() public {
        vm.prank(alice);
        vm.expectRevert(TimeCapsule.ZeroAmount.selector);
        tc.create{value: 0}(bob, uint64(block.timestamp + 1 days));
    }

    function test_create_rejects_too_soon() public {
        vm.prank(alice);
        vm.expectRevert(TimeCapsule.InvalidUnlockAt.selector);
        tc.create{value: 1 ether}(bob, uint64(block.timestamp + 30 minutes));
    }

    function test_create_rejects_too_far() public {
        vm.prank(alice);
        vm.expectRevert(TimeCapsule.InvalidUnlockAt.selector);
        tc.create{value: 1 ether}(bob, uint64(block.timestamp + 101 * 365 days));
    }

    // --- topUp ---

    function test_topup_success() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 days));

        vm.expectEmit(true, false, false, true);
        emit CapsuleToppedUp(id, 2 ether);
        vm.prank(alice);
        tc.topUp{value: 2 ether}(id);

        (, , , uint256 bal, , ) = tc.capsules(id);
        assertEq(bal, 3 ether);
    }

    function test_topup_rejects_non_sender() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 days));

        vm.prank(bob);
        vm.expectRevert(TimeCapsule.NotSender.selector);
        tc.topUp{value: 1 ether}(id);
    }

    function test_topup_rejects_zero() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 days));
        vm.prank(alice);
        vm.expectRevert(TimeCapsule.ZeroAmount.selector);
        tc.topUp{value: 0}(id);
    }

    function test_topup_rejects_after_claim() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 hours));
        vm.warp(block.timestamp + 1 hours + 1);
        vm.prank(bob);
        tc.claim(id);

        vm.prank(alice);
        vm.expectRevert(TimeCapsule.AlreadyClaimed.selector);
        tc.topUp{value: 1 ether}(id);
    }

    // --- claim ---

    function test_claim_reverts_before_unlock() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 days));

        vm.prank(bob);
        vm.expectRevert(TimeCapsule.NotYetUnlocked.selector);
        tc.claim(id);
    }

    function test_claim_success() public {
        uint64 unlock = uint64(block.timestamp + 1 days);
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, unlock);

        vm.warp(unlock + 1);

        uint256 before = bob.balance;
        vm.expectEmit(true, true, false, true);
        emit CapsuleClaimed(id, bob, 1 ether);
        vm.prank(bob);
        tc.claim(id);

        assertEq(bob.balance, before + 1 ether);
        (, , , uint256 bal, bool claimed, ) = tc.capsules(id);
        assertEq(bal, 0);
        assertTrue(claimed);
    }

    function test_claim_reverts_if_not_recipient() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 hours));
        vm.warp(block.timestamp + 1 hours + 1);

        vm.prank(eve);
        vm.expectRevert(TimeCapsule.NotRecipient.selector);
        tc.claim(id);
    }

    function test_claim_reverts_twice() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 hours));
        vm.warp(block.timestamp + 1 hours + 1);

        vm.prank(bob);
        tc.claim(id);
        vm.prank(bob);
        vm.expectRevert(TimeCapsule.AlreadyClaimed.selector);
        tc.claim(id);
    }

    // --- cancel ---

    function test_cancel_success_before_unlock() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 days));

        uint256 before = alice.balance;
        vm.expectEmit(true, true, false, true);
        emit CapsuleCancelled(id, alice, 1 ether);
        vm.prank(alice);
        tc.cancel(id);

        assertEq(alice.balance, before + 1 ether);
        (, , , uint256 bal, , bool cancelled) = tc.capsules(id);
        assertEq(bal, 0);
        assertTrue(cancelled);
    }

    function test_cancel_reverts_after_unlock() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 hours));
        vm.warp(block.timestamp + 1 hours);

        vm.prank(alice);
        vm.expectRevert(TimeCapsule.NotYetUnlocked.selector);
        tc.cancel(id);
    }

    function test_cancel_reverts_if_not_sender() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 hours));

        vm.prank(bob);
        vm.expectRevert(TimeCapsule.NotSender.selector);
        tc.cancel(id);
    }

    // --- timeUntilUnlock ---

    function test_time_until_unlock() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 days));
        assertEq(tc.timeUntilUnlock(id), 1 days);

        vm.warp(block.timestamp + 12 hours);
        assertEq(tc.timeUntilUnlock(id), 12 hours);

        vm.warp(block.timestamp + 13 hours);
        assertEq(tc.timeUntilUnlock(id), 0);
    }

    function test_time_until_unlock_after_claim_is_zero() public {
        vm.prank(alice);
        uint256 id = tc.create{value: 1 ether}(bob, uint64(block.timestamp + 1 hours));
        vm.warp(block.timestamp + 1 hours + 1);
        vm.prank(bob);
        tc.claim(id);
        assertEq(tc.timeUntilUnlock(id), 0);
    }

    function test_time_until_unknown_id_is_zero() public view {
        assertEq(tc.timeUntilUnlock(999), 0);
    }
}
