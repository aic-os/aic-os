// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

/// @title TimeCapsule -- lock ETH until a specific timestamp.
/// @notice Anyone can create a capsule to themselves or others.
///         Recipient can withdraw after `unlockAt`. Sender can cancel before unlock.
contract TimeCapsule {
    struct Capsule {
        address sender;
        address recipient;
        uint64  unlockAt;
        uint256 balance;
        bool    claimed;
        bool    cancelled;
    }

    uint64 public constant MIN_LOCK = 1 hours;
    uint64 public constant MAX_LOCK = 100 * 365 days;

    uint256 public nextId = 1;
    mapping(uint256 => Capsule) public capsules;

    event CapsuleCreated(uint256 indexed id, address indexed sender, address indexed recipient, uint256 amount, uint64 unlockAt);
    event CapsuleToppedUp(uint256 indexed id, uint256 amount);
    event CapsuleClaimed(uint256 indexed id, address indexed recipient, uint256 amount);
    event CapsuleCancelled(uint256 indexed id, address indexed sender, uint256 refund);

    error InvalidRecipient();
    error InvalidUnlockAt();
    error ZeroAmount();
    error NotFound();
    error AlreadyClaimed();
    error AlreadyCancelled();
    error NotYetUnlocked();
    error NotRecipient();
    error NotSender();
    error TransferFailed();

    /// Create a new capsule. `unlockAt` must be in [now+MIN_LOCK, now+MAX_LOCK].
    function create(address recipient, uint64 unlockAt) external payable returns (uint256 id) {
        if (recipient == address(0)) revert InvalidRecipient();
        if (msg.value == 0) revert ZeroAmount();
        uint64 nowTs = _now();
        if (unlockAt < nowTs + MIN_LOCK || unlockAt > nowTs + MAX_LOCK) {
            revert InvalidUnlockAt();
        }

        id = nextId++;
        capsules[id] = Capsule({
            sender: msg.sender,
            recipient: recipient,
            unlockAt: unlockAt,
            balance: msg.value,
            claimed: false,
            cancelled: false
        });
        emit CapsuleCreated(id, msg.sender, recipient, msg.value, unlockAt);
    }

    /// Sender may top-up an active capsule.
    function topUp(uint256 id) external payable {
        Capsule storage c = _active(id);
        if (msg.sender != c.sender) revert NotSender();
        if (msg.value == 0) revert ZeroAmount();
        c.balance += msg.value;
        emit CapsuleToppedUp(id, msg.value);
    }

    /// Recipient claims after unlockAt.
    function claim(uint256 id) external {
        Capsule storage c = capsules[id];
        if (c.recipient == address(0)) revert NotFound();
        if (c.claimed) revert AlreadyClaimed();
        if (c.cancelled) revert AlreadyCancelled();
        if (msg.sender != c.recipient) revert NotRecipient();
        if (_now() < c.unlockAt) revert NotYetUnlocked();

        uint256 amount = c.balance;
        c.balance = 0;
        c.claimed = true;
        _send(c.recipient, amount);
        emit CapsuleClaimed(id, c.recipient, amount);
    }

    /// Sender may cancel before unlockAt and get a refund.
    function cancel(uint256 id) external {
        Capsule storage c = capsules[id];
        if (c.sender == address(0)) revert NotFound();
        if (c.claimed) revert AlreadyClaimed();
        if (c.cancelled) revert AlreadyCancelled();
        if (msg.sender != c.sender) revert NotSender();
        if (_now() >= c.unlockAt) revert NotYetUnlocked(); // too late, recipient can claim

        uint256 amount = c.balance;
        c.balance = 0;
        c.cancelled = true;
        _send(c.sender, amount);
        emit CapsuleCancelled(id, c.sender, amount);
    }

    /// Seconds until a capsule can be claimed (0 if already claimable).
    function timeUntilUnlock(uint256 id) external view returns (uint64) {
        Capsule memory c = capsules[id];
        if (c.recipient == address(0)) return 0;
        if (c.claimed || c.cancelled) return 0;
        uint64 nowTs = _now();
        if (nowTs >= c.unlockAt) return 0;
        return c.unlockAt - nowTs;
    }

    // --- internal ---

    function _active(uint256 id) internal view returns (Capsule storage c) {
        c = capsules[id];
        if (c.sender == address(0)) revert NotFound();
        if (c.claimed) revert AlreadyClaimed();
        if (c.cancelled) revert AlreadyCancelled();
    }

    function _send(address to, uint256 amount) internal {
        (bool ok, ) = payable(to).call{value: amount}("");
        if (!ok) revert TransferFailed();
    }

    function _now() internal view returns (uint64) {
        // forge-lint: disable-next-line(unsafe-typecast)
        return uint64(block.timestamp);
    }
}
