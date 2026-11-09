// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

/// @title InheritanceVault -- dead man's switch for ETH.
/// @notice Owner deposits ETH, names a beneficiary, and pings periodically.
///         If the owner does not ping within `inactivityPeriod`, the beneficiary
///         can claim the entire vault.
contract InheritanceVault {
    struct Vault {
        address beneficiary;
        uint64  lastActivity;
        uint64  inactivityPeriod;
        uint256 balance;
        bool    active;
    }

    uint64 public constant MIN_INACTIVITY = 30 days;
    uint64 public constant MAX_INACTIVITY = 10 * 365 days;

    /// owner => vault
    mapping(address => Vault) public vaults;

    event Deposited(address indexed owner, address indexed beneficiary, uint256 amount, uint64 inactivityPeriod);
    event Pinged(address indexed owner, uint64 lastActivity);
    event Withdrawn(address indexed owner, uint256 amount);
    event Claimed(address indexed owner, address indexed beneficiary, uint256 amount);
    event BeneficiaryChanged(address indexed owner, address indexed oldBeneficiary, address indexed newBeneficiary);

    error NoVault();
    error NotBeneficiary();
    error StillActive();
    error InvalidBeneficiary();
    error InvalidInactivityPeriod();
    error ZeroAmount();
    error TransferFailed();

    /// Create or top-up a vault. Owner may call multiple times; beneficiary and
    /// period are overwritten with the latest values.
    function deposit(address beneficiary, uint64 inactivityPeriod) external payable {
        if (beneficiary == address(0)) revert InvalidBeneficiary();
        if (inactivityPeriod < MIN_INACTIVITY || inactivityPeriod > MAX_INACTIVITY) {
            revert InvalidInactivityPeriod();
        }
        if (msg.value == 0) revert ZeroAmount();

        Vault storage v = vaults[msg.sender];
        v.beneficiary = beneficiary;
        v.lastActivity = _now();
        v.inactivityPeriod = inactivityPeriod;
        v.balance += msg.value;
        v.active = true;

        emit Deposited(msg.sender, beneficiary, msg.value, inactivityPeriod);
    }

    /// Owner signal "I am alive". Resets the inactivity timer.
    function ping() external {
        Vault storage v = vaults[msg.sender];
        if (!v.active) revert NoVault();
        v.lastActivity = _now();
        emit Pinged(msg.sender, v.lastActivity);
    }

    /// Owner changes the beneficiary and resets the timer.
    function changeBeneficiary(address newBeneficiary) external {
        if (newBeneficiary == address(0)) revert InvalidBeneficiary();
        Vault storage v = vaults[msg.sender];
        if (!v.active) revert NoVault();
        address old = v.beneficiary;
        v.beneficiary = newBeneficiary;
        v.lastActivity = _now();
        emit BeneficiaryChanged(msg.sender, old, newBeneficiary);
    }

    /// Owner withdraws everything and closes the vault.
    function withdraw() external {
        Vault storage v = vaults[msg.sender];
        if (!v.active) revert NoVault();
        uint256 amount = v.balance;
        v.balance = 0;
        v.active = false;
        _send(msg.sender, amount);
        emit Withdrawn(msg.sender, amount);
    }

    /// Beneficiary claims the vault after the inactivity period has elapsed.
    function claim(address owner) external {
        Vault storage v = vaults[owner];
        if (!v.active) revert NoVault();
        if (v.beneficiary != msg.sender) revert NotBeneficiary();
        if (_now() < v.lastActivity + v.inactivityPeriod) revert StillActive();

        uint256 amount = v.balance;
        v.balance = 0;
        v.active = false;
        _send(msg.sender, amount);
        emit Claimed(owner, msg.sender, amount);
    }

    /// Seconds remaining until beneficiary can claim (0 if already claimable).
    function timeUntilClaim(address owner) external view returns (uint64) {
        Vault memory v = vaults[owner];
        if (!v.active) return 0;
        uint64 claimAt = v.lastActivity + v.inactivityPeriod;
        uint64 nowTs = _now();
        if (nowTs >= claimAt) return 0;
        return claimAt - nowTs;
    }

    // --- internal ---

    function _send(address to, uint256 amount) internal {
        (bool ok, ) = payable(to).call{value: amount}("");
        if (!ok) revert TransferFailed();
    }

    /// Safe: block.timestamp currently ~1.7e9, uint64 max ~1.8e19.
    function _now() internal view returns (uint64) {
        // forge-lint: disable-next-line(unsafe-typecast)
        return uint64(block.timestamp);
    }
}
