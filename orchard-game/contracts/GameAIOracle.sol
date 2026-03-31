// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev GameAIOracle uses multiple Citrea AI precompiles to make complex game decisions.
 * Coordinates MODEL_INFERENCE, PROOF_VERIFY, MODEL_METADATA, and MODEL_BENCHMARK.
 */
contract GameAIOracle is Ownable {
    // AI Precompile addresses
    address public constant MODEL_INFERENCE = address(0x0101);
    address public constant MODEL_METADATA = address(0x0103);
    address public constant PROOF_VERIFY = address(0x0104);
    address public constant MODEL_BENCHMARK = address(0x0105);

    // Oracle requests
    enum RequestType { SEED_GENERATION, STRATEGY_ANALYSIS, OPPONENT_MODELING, MARKET_PREDICTION }

    struct OracleRequest {
        bytes32 requestId;
        RequestType requestType;
        address requester;
        bytes32 modelId;
        bytes inputData;
        uint256 timestamp;
        bool fulfilled;
        bytes output;
    }

    mapping(bytes32 => OracleRequest) public requests;
    bytes32[] public requestIds;

    // Model registry for game-specific models
    struct GameModel {
        string name;
        string purpose;
        bytes32 modelId;
        uint256 performance;
        bool isActive;
    }

    mapping(bytes32 => GameModel) public gameModels;
    bytes32[] public registeredGameModels;

    // Subscription for AI services
    struct Subscription {
        address subscriber;
        uint256 tier; // 1: basic, 2: premium, 3: enterprise
        uint256 requestsRemaining;
        uint256 expiration;
    }

    mapping(address => Subscription) public subscriptions;

    // Events
    event RequestCreated(bytes32 indexed requestId, RequestType requestType, address requester);
    event RequestFulfilled(bytes32 indexed requestId, bytes output);
    event ModelRegistered(bytes32 indexed modelId, string name, string purpose);
    event SubscriptionCreated(address indexed subscriber, uint256 tier);
    event ModelBenchmarkUpdated(bytes32 indexed modelId, uint256 performance);

    constructor() {
        // Register default game models
        _registerDefaultModels();
    }

    /**
     * @dev Create an oracle request for AI processing
     */
    function createRequest(
        RequestType requestType,
        bytes32 modelId,
        bytes memory inputData
    ) public returns (bytes32) {
        // Check subscription
        Subscription storage sub = subscriptions[msg.sender];
        require(sub.requestsRemaining > 0, "No requests remaining");
        require(block.timestamp < sub.expiration, "Subscription expired");

        bytes32 requestId = keccak256(abi.encodePacked(
            msg.sender,
            modelId,
            block.timestamp,
            inputData
        ));

        requests[requestId] = OracleRequest({
            requestId: requestId,
            requestType: requestType,
            requester: msg.sender,
            modelId: modelId,
            inputData: inputData,
            timestamp: block.timestamp,
            fulfilled: false,
            output: bytes("")
        });

        requestIds.push(requestId);
        sub.requestsRemaining--;

        emit RequestCreated(requestId, requestType, msg.sender);

        // Process immediately
        _processRequest(requestId);

        return requestId;
    }

    /**
     * @dev Process oracle request using AI precompile
     */
    function _processRequest(bytes32 requestId) internal {
        OracleRequest storage request = requests[requestId];

        // Call MODEL_INFERENCE
        bytes memory payload = abi.encodePacked(
            request.modelId,
            bytes20(request.requester),
            request.inputData
        );

        (bool ok, bytes memory output) = MODEL_INFERENCE.call(payload);

        if (ok && output.length > 0) {
            request.output = output;
            request.fulfilled = true;

            emit RequestFulfilled(requestId, output);
        }
    }

    /**
     * @dev Verify proof for a request
     */
    function verifyRequestProof(
        bytes32 requestId,
        bytes32 commitment,
        bytes32 response,
        bytes memory statement
    ) public view returns (bool) {
        OracleRequest storage request = requests[requestId];
        require(request.fulfilled, "Request not fulfilled");

        bytes memory proofPayload = abi.encodePacked(
            request.modelId,
            commitment,
            response,
            statement
        );

        (bool ok, bytes memory result) = PROOF_VERIFY.staticcall(proofPayload);

        return ok && result.length == 1 && result[0] == 0x01;
    }

    /**
     * @dev Get model metadata
     */
    function getModelInfo(bytes32 modelId) public view returns (
        string memory name,
        string memory purpose,
        uint256 performance,
        bool isActive
    ) {
        GameModel storage model = gameModels[modelId];
        return (
            model.name,
            model.purpose,
            model.performance,
            model.isActive
        );
    }

    /**
     * @dev Register a game-specific model
     */
    function registerGameModel(
        string memory name,
        string memory purpose,
        bytes32 modelId
    ) public onlyOwner {
        gameModels[modelId] = GameModel({
            name: name,
            purpose: purpose,
            modelId: modelId,
            performance: 0,
            isActive: true
        });

        registeredGameModels.push(modelId);

        emit ModelRegistered(modelId, name, purpose);
    }

    /**
     * @dev Benchmark a model's performance
     */
    function benchmarkModel(bytes32 modelId) public returns (uint256) {
        (bool ok, bytes memory data) = MODEL_BENCHMARK.staticcall(abi.encodePacked(modelId));

        uint256 performance = 50;
        if (ok && data.length > 0) {
            // Parse performance from benchmark output
            performance = parsePerformance(data);
        }

        gameModels[modelId].performance = performance;

        emit ModelBenchmarkUpdated(modelId, performance);

        return performance;
    }

    /**
     * @dev Create subscription
     */
    function createSubscription(uint256 tier, uint256 requestCount) public payable {
        require(tier >= 1 && tier <= 3, "Invalid tier");
        require(requestCount > 0, "Invalid count");

        // Payment would be handled here
        subscriptions[msg.sender] = Subscription({
            subscriber: msg.sender,
            tier: tier,
            requestsRemaining: requestCount,
            expiration: block.timestamp + 30 days
        });

        emit SubscriptionCreated(msg.sender, tier);
    }

    /**
     * @dev Get request output
     */
    function getRequestOutput(bytes32 requestId) public view returns (bytes memory) {
        return requests[requestId].output;
    }

    /**
     * @dev Check if request is fulfilled
     */
    function isRequestFulfilled(bytes32 requestId) public view returns (bool) {
        return requests[requestId].fulfilled;
    }

    /**
     * @dev Get subscription info
     */
    function getSubscription(address subscriber) public view returns (
        uint256 tier,
        uint256 requestsRemaining,
        uint256 expiration
    ) {
        Subscription storage sub = subscriptions[subscriber];
        return (sub.tier, sub.requestsRemaining, sub.expiration);
    }

    /**
     * @dev Generate seed idea using AI
     */
    function generateSeedIdea(
        bytes32 modelId,
        bytes memory context
    ) public returns (string memory) {
        bytes32 requestId = createRequest(
            RequestType.SEED_GENERATION,
            modelId,
            context
        );

        return string(requests[requestId].output);
    }

    /**
     * @dev Analyze strategy
     */
    function analyzeStrategy(
        bytes32 modelId,
        bytes memory gameState
    ) public returns (bytes memory) {
        bytes32 requestId = createRequest(
            RequestType.STRATEGY_ANALYSIS,
            modelId,
            gameState
        );

        return requests[requestId].output;
    }

    /**
     * @dev Model opponent
     */
    function modelOpponent(
        bytes32 modelId,
        address opponent
    ) public returns (bytes memory) {
        bytes memory inputData = abi.encodePacked(opponent, block.timestamp);
        
        bytes32 requestId = createRequest(
            RequestType.OPPONENT_MODELING,
            modelId,
            inputData
        );

        return requests[requestId].output;
    }

    /**
     * @dev Predict market/growth
     */
    function predictGrowth(
        bytes32 modelId,
        bytes memory historicalData
    ) public returns (uint256) {
        bytes32 requestId = createRequest(
            RequestType.MARKET_PREDICTION,
            modelId,
            historicalData
        );

        bytes memory output = requests[requestId].output;
        
        if (output.length >= 32) {
            return uint256(bytes32(output)) % 101;
        }
        
        return 50;
    }

    /**
     * @dev Register default models
     */
    function _registerDefaultModels() internal {
        bytes32 seedGenModel = keccak256("seed-generator-v1");
        gameModels[seedGenModel] = GameModel({
            name: "Seed Generator",
            purpose: "Generate creative seed ideas",
            modelId: seedGenModel,
            performance: 75,
            isActive: true
        });
        registeredGameModels.push(seedGenModel);

        bytes32 strategyModel = keccak256("strategy-analyzer-v1");
        gameModels[strategyModel] = GameModel({
            name: "Strategy Analyzer",
            purpose: "Analyze game strategy and suggest moves",
            modelId: strategyModel,
            performance: 80,
            isActive: true
        });
        registeredGameModels.push(strategyModel);

        bytes32 opponentModel = keccak256("opponent-modeler-v1");
        gameModels[opponentModel] = GameModel({
            name: "Opponent Modeler",
            purpose: "Model opponent playstyle and predict moves",
            modelId: opponentModel,
            performance: 70,
            isActive: true
        });
        registeredGameModels.push(opponentModel);
    }

    function parsePerformance(bytes memory data) internal pure returns (uint256) {
        if (data.length >= 32) {
            return uint256(bytes32(data)) % 101;
        }
        return uint256(uint8(data[0])) % 101;
    }

    receive() external payable {}
}
