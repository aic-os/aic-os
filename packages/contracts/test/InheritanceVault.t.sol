// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {InheritanceVault} from "../src/InheritanceVault.sol";

contract InheritanceVaultTest is Test {
    InheritanceVault vault;

    address owner       = address(0xA11CE);
    address beneficiary = address(0xB0B);
    address stranger    = address(0xBAD);

    uint64 constant ONE_YEAR = 365 days;

    event Deposited(address indexed owner, address indexed beneficiary, uint256 amount, uint64 inactivityPeriod);
    event Pinged(address indexed owner, uint64 lastActivity);
    event Withdrawn(address indexed owner, uint256 amount);
    event Claimed(address indexed owner, address indexed beneficiary, uint256 amount);
    event BeneficiaryChanged(address indexed owner, address indexed oldBeneficiary, address indexed newBeneficiary);

    function setUp() public {
        vault = new InheritanceVault();
        vm.deal(owner, 100 ether);
    }

    // --- deposit ---

    function test_deposit_success() public {
        vm.expectEmit(true, true, false, true);
        emit Deposited(owner, beneficiary, 1 ether, ONE_YEAR);

        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);

        (address b, uint64 last, uint64 period, uint256 bal, bool active) = vault.vaults(owner);
        assertEq(b, beneficiary);
        assertEq(last, uint64(block.timestamp));
        assertEq(period, ONE_YEAR);
        assertEq(bal, 1 ether);
        assertTrue(active);
        assertEq(address(vault).balance, 1 ether);
    }

    function test_deposit_topup() public {
        vm.startPrank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);
        vault.deposit{value: 2 ether}(beneficiary, ONE_YEAR);
        vm.stopPrank();

        (, , , uint256 bal, ) = vault.vaults(owner);
        assertEq(bal, 3 ether);
    }

    function test_deposit_rejects_zero_value() public {
        vm.prank(owner);
        vm.expectRevert(InheritanceVault.ZeroAmount.selector);
        vault.deposit{value: 0}(beneficiary, ONE_YEAR);
    }

    function test_deposit_rejects_zero_beneficiary() public {
        vm.prank(owner);
        vm.expectRevert(InheritanceVault.InvalidBeneficiary.selector);
        vault.deposit{value: 1 ether}(address(0), ONE_YEAR);
    }

    function test_deposit_rejects_short_period() public {
        vm.prank(owner);
        vm.expectRevert(InheritanceVault.InvalidInactivityPeriod.selector);
        vault.deposit{value: 1 ether}(beneficiary, 29 days);
    }

    function test_deposit_rejects_long_period() public {
        vm.prank(owner);
        vm.expectRevert(InheritanceVault.InvalidInactivityPeriod.selector);
        vault.deposit{value: 1 ether}(beneficiary, 10 * 365 days + 1);
    }

    // --- ping ---

    function test_ping_updates_last_activity() public {
        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);

        uint64 firstTs = uint64(block.timestamp);
        vm.warp(firstTs + 100 days);

        vm.prank(owner);
        vault.ping();

        (, uint64 last, , , ) = vault.vaults(owner);
        assertEq(last, firstTs + 100 days);
    }

    function test_ping_reverts_without_vault() public {
        vm.prank(stranger);
        vm.expectRevert(InheritanceVault.NoVault.selector);
        vault.ping();
    }

    // --- withdraw ---

    function test_withdraw_returns_funds() public {
        vm.prank(owner);
        vault.deposit{value: 2 ether}(beneficiary, ONE_YEAR);

        uint256 before = owner.balance;
        vm.prank(owner);
        vault.withdraw();

        assertEq(owner.balance, before + 2 ether);
        assertEq(address(vault).balance, 0);
        (, , , uint256 bal, bool active) = vault.vaults(owner);
        assertEq(bal, 0);
        assertFalse(active);
    }

    function test_withdraw_reverts_without_vault() public {
        vm.prank(stranger);
        vm.expectRevert(InheritanceVault.NoVault.selector);
        vault.withdraw();
    }

    // --- claim ---

    function test_claim_reverts_before_period() public {
        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);

        vm.warp(block.timestamp + ONE_YEAR - 1);
        vm.prank(beneficiary);
        vm.expectRevert(InheritanceVault.StillActive.selector);
        vault.claim(owner);
    }

    function test_claim_success_after_period() public {
        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);

        vm.warp(block.timestamp + ONE_YEAR + 1);

        uint256 before = beneficiary.balance;
        vm.expectEmit(true, true, false, true);
        emit Claimed(owner, beneficiary, 1 ether);
        vm.prank(beneficiary);
        vault.claim(owner);

        assertEq(beneficiary.balance, before + 1 ether);
        assertEq(address(vault).balance, 0);
        (, , , uint256 bal, bool active) = vault.vaults(owner);
        assertEq(bal, 0);
        assertFalse(active);
    }

    function test_claim_reverts_if_not_beneficiary() public {
        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);
        vm.warp(block.timestamp + ONE_YEAR + 1);

        vm.prank(stranger);
        vm.expectRevert(InheritanceVault.NotBeneficiary.selector);
        vault.claim(owner);
    }

    function test_claim_twice_reverts() public {
        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);
        vm.warp(block.timestamp + ONE_YEAR + 1);

        vm.prank(beneficiary);
        vault.claim(owner);

        vm.prank(beneficiary);
        vm.expectRevert(InheritanceVault.NoVault.selector);
        vault.claim(owner);
    }

    function test_ping_after_claim_reverts() public {
        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);
        vm.warp(block.timestamp + ONE_YEAR + 1);

        vm.prank(beneficiary);
        vault.claim(owner);

        vm.prank(owner);
        vm.expectRevert(InheritanceVault.NoVault.selector);
        vault.ping();
    }

    // --- changeBeneficiary ---

    function test_change_beneficiary() public {
        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);

        vm.warp(block.timestamp + 100 days);

        vm.expectEmit(true, true, true, true);
        emit BeneficiaryChanged(owner, beneficiary, stranger);
        vm.prank(owner);
        vault.changeBeneficiary(stranger);

        (address b, uint64 last, , , ) = vault.vaults(owner);
        assertEq(b, stranger);
        // lastActivity reset
        assertGt(last, uint64(block.timestamp - 1));
    }

    function test_change_beneficiary_rejects_zero() public {
        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);
        vm.prank(owner);
        vm.expectRevert(InheritanceVault.InvalidBeneficiary.selector);
        vault.changeBeneficiary(address(0));
    }

    // --- timeUntilClaim ---

    function test_time_until_claim() public {
        vm.prank(owner);
        vault.deposit{value: 1 ether}(beneficiary, ONE_YEAR);
        assertEq(vault.timeUntilClaim(owner), ONE_YEAR);

        vm.warp(block.timestamp + 100 days);
        assertEq(vault.timeUntilClaim(owner), ONE_YEAR - 100 days);

        vm.warp(block.timestamp + ONE_YEAR);
        assertEq(vault.timeUntilClaim(owner), 0);
    }

    function test_time_until_claim_returns_zero_for_inactive() public view {
        assertEq(vault.timeUntilClaim(stranger), 0);
    }
}
