// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

/// @title AcoinBridge -- wrap external assets (aETH, aBTC, aUSDT) into AIC Chain.
/// @notice MVP: multisig validators (threshold signatures over external tx hash).
///         Next step: DKG + actual Ethereum bridge.
contract AcoinBridge {
    struct Token {
        string  symbol;
        uint8   decimals;
        bool    enabled;
        uint256 totalMinted;
    }

    address public owner;
    uint8   public threshold;
    address[] public validatorList;
    mapping(address => bool) public validators;

    mapping(string => Token) public tokens;
    mapping(string => uint256) public totalSupply;

    /// externalTxHash => already processed (replay protection)
    mapping(bytes32 => bool) public processedExternalTxs;

    event ValidatorAdded(address indexed v);
    event ValidatorRemoved(address indexed v);
    event ThresholdChanged(uint8 newThreshold);
    event TokenAdded(string symbol, uint8 decimals);
    event TokenToggled(string symbol, bool enabled);
    event Minted(address indexed to, string symbol, uint256 amount, bytes32 externalTxHash);
    event Burned(address indexed from, string symbol, uint256 amount);
    event OwnershipTransferred(address indexed from, address indexed to);

    error NotOwner();
    error NotValidator();
    error TokenNotEnabled();
    error AlreadyProcessed(bytes32 externalTxHash);
    error ThresholdNotMet(uint256 got, uint8 required);
    error InvalidThreshold();
    error InvalidValidator();
    error ZeroAmount();
    error InsufficientBalance();

    modifier onlyOwner() {
        if (msg.sender != owner) revert NotOwner();
        _;
    }

    constructor(uint8 _threshold) {
        owner = msg.sender;
        threshold = _threshold;
        emit OwnershipTransferred(address(0), msg.sender);
    }

    // ============ Admin ============

    function addValidator(address v) external onlyOwner {
        if (v == address(0)) revert InvalidValidator();
        if (validators[v]) return;
        validators[v] = true;
        validatorList.push(v);
        emit ValidatorAdded(v);
    }

    function removeValidator(address v) external onlyOwner {
        if (!validators[v]) revert InvalidValidator();
        validators[v] = false;
        // compact array
        for (uint256 i = 0; i < validatorList.length; i++) {
            if (validatorList[i] == v) {
                validatorList[i] = validatorList[validatorList.length - 1];
                validatorList.pop();
                break;
            }
        }
        emit ValidatorRemoved(v);
    }

    function setThreshold(uint8 t) external onlyOwner {
        if (t == 0 || t > validatorList.length) revert InvalidThreshold();
        threshold = t;
        emit ThresholdChanged(t);
    }

    function addToken(string calldata symbol, uint8 decimals) external onlyOwner {
        tokens[symbol] = Token(symbol, decimals, true, 0);
        emit TokenAdded(symbol, decimals);
    }

    function setTokenEnabled(string calldata symbol, bool enabled) external onlyOwner {
        tokens[symbol].enabled = enabled;
        emit TokenToggled(symbol, enabled);
    }

    function transferOwnership(address newOwner) external onlyOwner {
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }

    // ============ Mint (validators multisig) ============

    /// @notice Mint aCOIN token to user. Requires threshold signatures from validators.
    /// @param to           recipient address
    /// @param symbol       token symbol (aETH, aBTC, ...)
    /// @param amount       amount in token's smallest unit
    /// @param externalTxHash  hash of external tx (for replay protection)
    /// @param signatures   ECDSA signatures by validators over keccak256(abi.encode(to, symbol, amount, externalTxHash, chainId))
    function mint(
        address to,
        string calldata symbol,
        uint256 amount,
        bytes32 externalTxHash,
        bytes[] calldata signatures
    ) external {
        if (amount == 0) revert ZeroAmount();
        if (!tokens[symbol].enabled) revert TokenNotEnabled();
        if (processedExternalTxs[externalTxHash]) revert AlreadyProcessed(externalTxHash);

        bytes32 digest = _mintDigest(to, symbol, amount, externalTxHash);
        uint256 ok = _countValidSignatures(digest, signatures);
        if (ok < threshold) revert ThresholdNotMet(ok, threshold);

        processedExternalTxs[externalTxHash] = true;
        tokens[symbol].totalMinted += amount;
        totalSupply[symbol] += amount;

        emit Minted(to, symbol, amount, externalTxHash);
    }

    // ============ Burn (user wants to withdraw) ============

    /// @notice Burn aCOIN token to withdraw the underlying external asset.
    function burn(string calldata symbol, uint256 amount) external {
        if (amount == 0) revert ZeroAmount();
        if (totalSupply[symbol] < amount) revert InsufficientBalance();
        totalSupply[symbol] -= amount;
        emit Burned(msg.sender, symbol, amount);
    }

    // ============ Internal ============

    function _mintDigest(
        address to,
        string calldata symbol,
        uint256 amount,
        bytes32 externalTxHash
    ) internal view returns (bytes32) {
        return keccak256(abi.encode(to, symbol, amount, externalTxHash, block.chainid));
    }

    function _countValidSignatures(
        bytes32 digest,
        bytes[] calldata signatures
    ) internal view returns (uint256 count) {
        bytes32 ethHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", digest));
        address last = address(0);
        for (uint256 i = 0; i < signatures.length; i++) {
            address signer = _recover(ethHash, signatures[i]);
            if (signer == address(0) || signer == last) continue;
            if (validators[signer]) {
                count++;
                last = signer;
            }
        }
    }

    function _recover(bytes32 hash, bytes calldata sig) internal pure returns (address) {
        if (sig.length != 65) return address(0);
        bytes32 r = bytes32(sig[0:32]);
        bytes32 s = bytes32(sig[32:64]);
        uint8 v = uint8(sig[64]);
        if (v < 27) v += 27;
        return ecrecover(hash, v, r, s);
    }

    function validatorCount() external view returns (uint256) {
        return validatorList.length;
    }
}
