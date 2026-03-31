// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/**
 * @dev x402PaymentGateway enables AI agents to pay for game actions via x402 protocol.
 * Supports per-request micropayments in USDC or ORT tokens.
 */
contract x402PaymentGateway is Ownable, ReentrancyGuard {
    struct PaymentTerms {
        uint256 maxAmount;
        address payTo;
        address asset;
        string network;
        uint256 expiresAt;
        bytes32 nonce;
    }

    mapping(bytes32 => bool) public usedNonces;
    mapping(address => uint256) public paidAmounts;
    
    address public paymentRecipient;
    address public USDC;
    address public ORT;
    
    uint256 public constant GAME_ACTION_COST = 0.01e6; // $0.01 in USDC
    
    event PaymentReceived(
        address indexed payer,
        uint256 amount,
        address asset,
        bytes32 paymentId
    );
    
    event PaymentVerified(
        bytes32 paymentId,
        address payer,
        uint256 amount
    );

    constructor(address _USDC, address _ORT) {
        USDC = _USDC;
        ORT = _ORT;
        paymentRecipient = msg.sender;
    }

    function getPaymentTerms() public pure returns (PaymentTerms memory) {
        return PaymentTerms({
            maxAmount: GAME_ACTION_COST,
            payTo: address(0),
            asset: address(0),
            network: "solana",
            expiresAt: block.timestamp + 5 minutes,
            nonce: bytes32(0)
        });
    }

    function verifyAndProcessPayment(
        bytes32 paymentId,
        uint256 amount,
        address asset,
        address payer,
        bytes calldata signature
    ) public nonReentrant returns (bool) {
        require(!usedNonces[paymentId], "Nonce already used");
        require(amount <= GAME_ACTION_COST, "Amount exceeds max");
        require(asset == USDC || asset == ORT, "Unsupported asset");
        
        usedNonces[paymentId] = true;
        
        if (asset == USDC) {
            require(
                IERC20(USDC).transferFrom(payer, paymentRecipient, amount),
                "USDC transfer failed"
            );
        } else {
            require(
                IERC20(ORT).transferFrom(payer, paymentRecipient, amount),
                "ORT transfer failed"
            );
        }
        
        paidAmounts[payer] += amount;
        
        emit PaymentVerified(paymentId, payer, amount);
        
        return true;
    }

    function getPaymentManifest(address payer) public view returns (string memory) {
        return string(
            abi.encodePacked(
                '{"maxAmountRequired":"',
                Strings.toString(GAME_ACTION_COST / 1e6),
                '","payTo":"',
                Strings.toHexString(uint256(uint160(paymentRecipient)), 20),
                '","asset":"',
                Strings.toHexString(uint256(uint160(USDC)), 20),
                '","network":"solana"}'
            )
        );
    }

    function setPaymentRecipient(address _recipient) public onlyOwner {
        paymentRecipient = _recipient;
    }

    function withdrawFunds() public onlyOwner {
        payable(paymentRecipient).transfer(address(this).balance);
    }
}

library Strings {
    function toString(uint256 value) internal pure returns (string memory) {
        if (value == 0) return "0";
        uint256 temp = value;
        uint256 digits;
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer = new bytes(digits);
        while (value != 0) {
            digits--;
            buffer[digits] = bytes1(uint8(48 + (value % 10)));
            value /= 10;
        }
        return string(buffer);
    }

    function toHexString(uint256 value, uint256 length) internal pure returns (string memory) {
        bytes memory buffer = new bytes(2 * length + 2);
        buffer[0] = "0";
        buffer[1] = "x";
        for (uint256 i = 0; i < length; i++) {
            buffer[2 * length + 1 - 2 * (i + 1)] = bytes1(hexValues[uint8((value / (16 ** (2 * i))) % 16)]);
            buffer[2 * length + 2 - 2 * (i + 1)] = bytes1(hexValues[uint8((value / (16 ** (2 * i + 1))) % 16)]);
        }
        return string(buffer);
    }

    function toHexString(uint256 value) internal pure returns (string memory) {
        return toHexString(value, (value == 0 ? 0 : ((value - 1) / 0x1000000000000000000 / 0x10 + 1)));
    }

    bytes16 private constant hexValues = "0123456789abcdef";
}
