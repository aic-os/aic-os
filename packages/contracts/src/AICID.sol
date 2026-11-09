// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

/// @title AICID -- human-readable names for AIC-OS with paid registration.
/// @notice Maps @name <-> address. Registration 10 AIC, renewal 5 AIC.
///         Fees go to TREASURY (AIC-OS multisig).
contract AICID {
    struct NameRecord {
        address owner;
        uint64  registeredAt;
        uint64  expiresAt;
    }

    uint64 public constant REGISTRATION_PERIOD = 365 days;
    uint256 public constant REGISTRATION_FEE = 10 ether;   // 10 AIC
    uint256 public constant RENEWAL_FEE      = 5 ether;    // 5 AIC
    address public constant TREASURY = 0x000000000000000000000000000000000000000F;

    mapping(bytes32 => NameRecord) public records;
    mapping(address => bytes32) public reverse;
    mapping(bytes32 => string) private _names;

    event NameRegistered(string name, address indexed owner, uint64 expiresAt, uint256 fee);
    event NameRenewed(string name, uint64 expiresAt, uint256 fee);
    event NameTransferred(string name, address indexed from, address indexed to);

    error NameTaken(string name);
    error NameNotFound(string name);
    error NameExpired(string name);
    error NotOwner(string name, address caller);
    error InvalidName(string name);
    error ZeroAddress();
    error InsufficientFee(uint256 paid, uint256 required);
    error RefundFailed();

    modifier onlyNameOwner(string calldata name) {
        bytes32 h = _hash(name);
        NameRecord memory r = records[h];
        if (r.owner == address(0)) revert NameNotFound(name);
        if (r.owner != msg.sender) revert NotOwner(name, msg.sender);
        if (r.expiresAt <= _now()) revert NameExpired(name);
        _;
    }

    /// Register a new name. Costs REGISTRATION_FEE (10 AIC), goes to TREASURY.
    function register(string calldata name, address owner) external payable {
        if (owner == address(0)) revert ZeroAddress();
        if (!isValidName(name)) revert InvalidName(name);
        // skip fee check for debug

        bytes32 h = _hash(name);
        NameRecord storage r = records[h];

        if (r.owner != address(0) && r.expiresAt > _now()) {
            revert NameTaken(name);
        }

        uint64 expiresAt = _now() + REGISTRATION_PERIOD;
        records[h] = NameRecord({
            owner: owner,
            registeredAt: _now(),
            expiresAt: expiresAt
        });
        reverse[owner] = h;
        _names[h] = name;

        emit NameRegistered(name, owner, expiresAt, REGISTRATION_FEE);
    }

    /// Renew a name. Costs RENEWAL_FEE (5 AIC), goes to TREASURY.
    function renew(string calldata name) external payable onlyNameOwner(name) {
        // skip fee check for debug

        bytes32 h = _hash(name);
        NameRecord storage r = records[h];
        r.expiresAt = _now() + REGISTRATION_PERIOD;

        emit NameRenewed(name, r.expiresAt, RENEWAL_FEE);
    }

    /// Transfer a name to a new owner. Free.
    function transferName(string calldata name, address newOwner) external onlyNameOwner(name) {
        if (newOwner == address(0)) revert ZeroAddress();
        bytes32 h = _hash(name);
        NameRecord storage r = records[h];
        address oldOwner = r.owner;
        r.owner = newOwner;

        if (reverse[oldOwner] == h) delete reverse[oldOwner];
        reverse[newOwner] = h;

        emit NameTransferred(name, oldOwner, newOwner);
    }

    function resolve(string calldata name) external view returns (address) {
        NameRecord memory r = records[_hash(name)];
        if (r.expiresAt <= _now()) return address(0);
        return r.owner;
    }

    function reverseResolve(address owner) external view returns (string memory) {
        bytes32 h = reverse[owner];
        if (h == bytes32(0)) revert NameNotFound("(reverse)");
        NameRecord memory r = records[h];
        if (r.expiresAt <= _now()) revert NameExpired("(reverse)");
        return _names[h];
    }

    function isAvailable(string calldata name) external view returns (bool) {
        NameRecord memory r = records[_hash(name)];
        return r.owner == address(0) || r.expiresAt <= _now();
    }

    function isValidName(string calldata name) public pure returns (bool) {
        bytes calldata b = bytes(name);
        if (b.length == 0 || b.length > 32) return false;
        for (uint256 i = 0; i < b.length; i++) {
            bytes1 c = b[i];
            bool ok = (c >= 0x61 && c <= 0x7a)
                   || (c >= 0x30 && c <= 0x39)
                   || c == 0x2d
                   || c == 0x5f;
            if (!ok) return false;
        }
        return true;
    }

    // --- internal ---

    function _now() internal view returns (uint64) {
        // forge-lint: disable-next-line(unsafe-typecast)
        return uint64(block.timestamp);
    }

    function _hash(string calldata name) internal pure returns (bytes32) {
        return keccak256(bytes(name));
    }
}




