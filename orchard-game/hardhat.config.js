import "@nomicfoundation/hardhat-toolbox";

/** @type import("hardhat/config").HardhatUserConfig */
export default {
  solidity: "0.8.24",
  networks: {
    hardhat: {
      chainId: 1337
    },
    localhost: {
      url: "http://127.0.0.1:8545",
      chainId: 1337
    },
    sepolia: {
      url: process.env.SEPOLIA_RPC_URL || "",
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      chainId: 11155111,
      confirmations: 2
    },
    mainnet: {
      url: process.env.MAINNET_RPC_URL || "",
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      chainId: 1,
      confirmations: 2
    }
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY || ""
  }
};
