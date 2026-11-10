// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {AcoinBridge} from "../src/AcoinBridge.sol";

contract AcoinBridgeTest is Test {
    AcoinBridge bridge;

    // Hardhat test private keys
    uint256 constant PK1 = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
    uint256 constant PK2 = 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d;
    uint256 constant PK3 = 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a;

    address alice;
    address bob;
    address charlie;
    address user = address(0xA11CE);

    function setUp() public {
        alice = vm.addr(PK1);
        bob = vm.addr(PK2);
        charlie = vm.addr(PK3);

        bridge = new AcoinBridge(2); // threshold = 2
        bridge.addValidator(alice);
        bridge.addValidator(bob);
        bridge.addValidator(charlie);
        bridge.addToken("aETH", 18);
    }

    // ===== Helpers =====

    function _digest(address to, string memory symbol, uint256 amount, bytes32 ext) internal view returns (bytes32) {
        return keccak256(abi.encode(to, symbol, amount, ext, block.chainid));
    }

    function _sign(uint256 pk, bytes32 digest) internal pure returns (bytes memory) {
        bytes32 ethHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", digest));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, ethHash);
        return abi.encodePacked(r, s, v);
    }

    // ===== Constructor =====

    function test_constructor_sets_owner_and_threshold() public view {
        assertEq(bridge.owner(), address(this));
        assertEq(bridge.threshold(), 2);
    }

    // ===== Admin =====

    function test_add_and_remove_validator() public {
        address newV = address(0xBEEF);
        bridge.addValidator(newV);
        assertTrue(bridge.validators(newV));
        assertEq(bridge.validatorCount(), 4);

        bridge.removeValidator(newV);
        assertFalse(bridge.validators(newV));
        assertEq(bridge.validatorCount(), 3);
    }

    function test_only_owner_can_admin() public {
        vm.prank(user);
        vm.expectRevert(AcoinBridge.NotOwner.selector);
        bridge.addValidator(address(0xDEAD));
    }

    function test_set_threshold() public {
        bridge.setThreshold(3);
        assertEq(bridge.threshold(), 3);

        vm.expectRevert(AcoinBridge.InvalidThreshold.selector);
        bridge.setThreshold(0);

        vm.expectRevert(AcoinBridge.InvalidThreshold.selector);
        bridge.setThreshold(100);
    }

    // ===== Mint =====

    function test_mint_with_threshold_signatures() public {
        bytes32 extHash = keccak256("ethereum-tx-1");
        bytes32 digest = _digest(user, "aETH", 1 ether, extHash);

        bytes[] memory sigs = new bytes[](2);
        sigs[0] = _sign(PK1, digest);
        sigs[1] = _sign(PK2, digest);

        vm.expectEmit(true, false, false, true);
        emit AcoinBridge.Minted(user, "aETH", 1 ether, extHash);
        bridge.mint(user, "aETH", 1 ether, extHash, sigs);

        assertEq(bridge.totalSupply("aETH"), 1 ether);
        assertTrue(bridge.processedExternalTxs(extHash));
    }

    function test_mint_rejects_insufficient_signatures() public {
        bytes32 extHash = keccak256("ethereum-tx-2");
        bytes32 digest = _digest(user, "aETH", 1 ether, extHash);

        bytes[] memory sigs = new bytes[](1);
        sigs[0] = _sign(PK1, digest);

        vm.expectRevert(abi.encodeWithSelector(AcoinBridge.ThresholdNotMet.selector, 1, 2));
        bridge.mint(user, "aETH", 1 ether, extHash, sigs);
    }

    function test_mint_rejects_replay() public {
        bytes32 extHash = keccak256("ethereum-tx-3");
        bytes32 digest = _digest(user, "aETH", 1 ether, extHash);

        bytes[] memory sigs = new bytes[](2);
        sigs[0] = _sign(PK1, digest);
        sigs[1] = _sign(PK2, digest);

        bridge.mint(user, "aETH", 1 ether, extHash, sigs);

        vm.expectRevert(abi.encodeWithSelector(AcoinBridge.AlreadyProcessed.selector, extHash));
        bridge.mint(user, "aETH", 1 ether, extHash, sigs);
    }

    function test_mint_rejects_disabled_token() public {
        bridge.setTokenEnabled("aETH", false);
        bytes32 extHash = keccak256("ethereum-tx-4");
        bytes32 digest = _digest(user, "aETH", 1 ether, extHash);

        bytes[] memory sigs = new bytes[](2);
        sigs[0] = _sign(PK1, digest);
        sigs[1] = _sign(PK2, digest);

        vm.expectRevert(AcoinBridge.TokenNotEnabled.selector);
        bridge.mint(user, "aETH", 1 ether, extHash, sigs);
    }

    function test_mint_ignores_non_validator_signature() public {
        bytes32 extHash = keccak256("ethereum-tx-5");
        bytes32 digest = _digest(user, "aETH", 1 ether, extHash);

        bytes[] memory sigs = new bytes[](2);
        sigs[0] = _sign(PK1, digest);
        sigs[1] = _sign(0xBADBAD, digest); // unknown key

        vm.expectRevert(abi.encodeWithSelector(AcoinBridge.ThresholdNotMet.selector, 1, 2));
        bridge.mint(user, "aETH", 1 ether, extHash, sigs);
    }

    // ===== Burn =====

    function test_burn_success() public {
        // Сначала наминтив
        bytes32 extHash = keccak256("ethereum-tx-6");
        bytes32 digest = _digest(user, "aETH", 2 ether, extHash);
        bytes[] memory sigs = new bytes[](2);
        sigs[0] = _sign(PK1, digest);
        sigs[1] = _sign(PK2, digest);
        bridge.mint(user, "aETH", 2 ether, extHash, sigs);

        // Burn 1 ether
        bridge.burn("aETH", 1 ether);
        assertEq(bridge.totalSupply("aETH"), 1 ether);
    }

    function test_burn_rejects_insufficient_supply() public {
        vm.expectRevert(AcoinBridge.InsufficientBalance.selector);
        bridge.burn("aETH", 1 ether);
    }

    function test_burn_rejects_zero() public {
        vm.expectRevert(AcoinBridge.ZeroAmount.selector);
        bridge.burn("aETH", 0);
    }
}
