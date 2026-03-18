// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./GradientRoles.sol";

/// @title PoolClaimToken — ERC-1155 representing pool withdrawal rights
/// @notice Each token ID corresponds to a pool ID. Tokens are non-transferable in v1.
///         Claims can only be minted by the CLAIM_MINTER role and burned by the ESCROW_OPERATOR role.
contract PoolClaimToken is ERC1155, AccessControl, Pausable, ReentrancyGuard {
    // --- Events ---
    event ClaimMinted(address indexed to, uint256 indexed poolId, uint256 amount);
    event ClaimBurned(address indexed from, uint256 indexed poolId, uint256 amount);
    event TransferPolicyUpdated(uint256 indexed poolId, uint8 policy);

    // --- State ---
    // Transfer policy per pool: 0 = blocked (default), 1 = allowlist
    mapping(uint256 => uint8) private _transferPolicy;
    // Allowlist per pool (only used when policy = 1)
    mapping(uint256 => mapping(address => bool)) private _transferAllowlist;
    // Track total supply per pool for invariant checking
    mapping(uint256 => uint256) private _totalSupply;

    constructor(address admin) ERC1155("") {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(GradientRoles.CLAIM_MINTER, admin);
        _grantRole(GradientRoles.ESCROW_OPERATOR, admin);
        _grantRole(GradientRoles.POOL_ADMIN, admin);
        _grantRole(GradientRoles.EMERGENCY_ADMIN, admin);
    }

    /// @notice Mint claim tokens for a contributor
    /// @param to The contributor receiving the claim
    /// @param poolId The pool this claim is for (used as ERC-1155 token ID)
    /// @param qty Number of claims to mint
    function mintClaim(
        address to,
        uint256 poolId,
        uint256 qty
    ) external onlyRole(GradientRoles.CLAIM_MINTER) whenNotPaused nonReentrant {
        // Checks
        require(to != address(0), "Invalid recipient");
        require(poolId > 0, "Invalid pool ID");
        require(qty > 0, "Quantity must be > 0");

        // Effects
        _totalSupply[poolId] += qty;

        // Interactions
        _mint(to, poolId, qty, "");

        emit ClaimMinted(to, poolId, qty);
    }

    /// @notice Burn claim tokens when a reservation is finalized
    /// @param from The claim holder
    /// @param poolId The pool ID
    /// @param qty Number of claims to burn
    function burnClaim(
        address from,
        uint256 poolId,
        uint256 qty
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused nonReentrant {
        // Checks
        require(balanceOf(from, poolId) >= qty, "Insufficient claim balance");

        // Effects
        _totalSupply[poolId] -= qty;

        // Interactions
        _burn(from, poolId, qty);

        emit ClaimBurned(from, poolId, qty);
    }

    /// @notice Set transfer policy for a pool
    /// @param poolId The pool to configure
    /// @param policy 0 = blocked (default), 1 = allowlist
    function setTransferPolicy(
        uint256 poolId,
        uint8 policy
    ) external onlyRole(GradientRoles.POOL_ADMIN) {
        require(policy <= 1, "Invalid policy");

        _transferPolicy[poolId] = policy;

        emit TransferPolicyUpdated(poolId, policy);
    }

    /// @notice Add or remove an address from a pool's transfer allowlist
    /// @param poolId The pool
    /// @param account The address to allow/disallow
    /// @param allowed Whether the address is allowed
    function setAllowlistEntry(
        uint256 poolId,
        address account,
        bool allowed
    ) external onlyRole(GradientRoles.POOL_ADMIN) {
        _transferAllowlist[poolId][account] = allowed;
    }

    /// @notice Emergency pause
    function emergencyPause() external onlyRole(GradientRoles.EMERGENCY_ADMIN) {
        _pause();
    }

    /// @notice Emergency unpause
    function emergencyUnpause() external onlyRole(GradientRoles.EMERGENCY_ADMIN) {
        _unpause();
    }

    // --- View Functions ---

    function claimBalance(address account, uint256 poolId) external view returns (uint256) {
        return balanceOf(account, poolId);
    }

    function totalSupplyForPool(uint256 poolId) external view returns (uint256) {
        return _totalSupply[poolId];
    }

    function getTransferPolicy(uint256 poolId) external view returns (uint8) {
        return _transferPolicy[poolId];
    }

    function isAllowlisted(uint256 poolId, address account) external view returns (bool) {
        return _transferAllowlist[poolId][account];
    }

    // --- Transfer Restrictions ---

    function _update(
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory values
    ) internal virtual override {
        // Allow minting (from == 0) and burning (to == 0)
        if (from != address(0) && to != address(0)) {
            // This is a transfer — check policy for each token ID
            for (uint256 i = 0; i < ids.length; i++) {
                uint8 policy = _transferPolicy[ids[i]];
                if (policy == 0) {
                    revert("PoolClaimToken: transfers blocked for this pool");
                } else if (policy == 1) {
                    require(
                        _transferAllowlist[ids[i]][from] || _transferAllowlist[ids[i]][to],
                        "PoolClaimToken: not on allowlist"
                    );
                }
            }
        }
        super._update(from, to, ids, values);
    }

    // --- Interface Support ---

    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override(ERC1155, AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
