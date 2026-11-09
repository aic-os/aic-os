// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

/// @title AICID -- human-readable names for AIC-OS
contract AICID {
    struct NameRecord {
        address owner;
        uint64  registeredAt;
        uint64  expiresAt;
    }

    uint64 public constant REGISTRATION_PERIOD = 365 days;

    mapping(bytes32 => NameRecord) public records;
    mapping(address => bytes32) public reverse;
    mapping(bytes32 => string) private _names;

    event NameRegistered(string name, address indexed owner, uint64 expiresAt);
    event NameRenewed(string name, uint64 expiresAt);
    event NameTransferred(string name, address indexed from, address indexed to);

    error NameTaken(string name);
    error NameNotFound(string name);
    error NameExpired(string name);
    error NotOwner(string name, address caller);
    error InvalidName(string name);
    error ZeroAddress();

    modifier onlyNameOwner(string calldata name) {
        bytes32 h = _hash(name);
        NameRecord memory r = records[h];
        if (r.owner == address(0)) revert NameNotFound(name);
        if (r.owner != msg.sender) revert NotOwner(name, msg.sender);
        if (r.expiresAt <= _now()) revert NameExpired(name);
        _;
    }

    function register(string calldata name, address owner) external {
        if (owner == address(0)) revert ZeroAddress();
        if (!isValidName(name)) revert InvalidName(name);

        bytes32 h = _hash(name);
        NameRecord storage r = records[h];

        if (r.owner != address(0) && r.expiresAt > _now()) {
            revert NameTaken(name);
        }

        uint64 nowTs = _now();
        uint64 expiresAt = nowTs + REGISTRATION_PERIOD;
        records[h] = NameRecord({
            owner: owner,
            registeredAt: nowTs,
            expiresAt: expiresAt
        });
        reverse[owner] = h;
        _names[h] = name;

        emit NameRegistered(name, owner, expiresAt);
    }

    function renew(string calldata name) external onlyNameOwner(name) {
        bytes32 h = _hash(name);
        NameRecord storage r = records[h];
        r.expiresAt = _now() + REGISTRATION_PERIOD;
        emit NameRenewed(name, r.expiresAt);
    }

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
            bool ok = (c >= 0x61 && c <= 0x7a) // a..z
                   || (c >= 0x30 && c <= 0x39) // 0..9
                   || c == 0x2d                // -
                   || c == 0x5f;               // _
            if (!ok) return false;
        }
        return true;
    }

    // --- internal ---

    /// Current block timestamp as uint64.
    /// Safe: block.timestamp currently ~1.7e9, uint64 max ~1.8e19 --
    /// no truncation for >500 million years.
    function _now() internal view returns (uint64) {
        // forge-lint: disable-next-line(unsafe-typecast)
        return uint64(block.timestamp);
    }

    function _hash(string calldata name) internal pure returns (bytes32) {
        return keccak256(bytes(name));
    }
}

