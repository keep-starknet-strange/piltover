// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface IFactRegistry {
    function isValid(bytes32 fact) external view returns (bool);
}

interface IStarknetMessaging {
    function sendMessageToL2(
        uint256 toAddress,
        uint256 selector,
        uint256[] calldata payload
    ) external payable returns (bytes32 msgHash, uint256 nonce);
}

contract PrivilyFactRelay {
    IFactRegistry public immutable gpsVerifier;
    IStarknetMessaging public immutable starknetCore;
    uint256 public immutable l2Receiver;
    uint256 public immutable l2Selector;

    mapping(uint256 updateId => bytes32 sharpFact) public relayedSharpFacts;

    event FactRelayed(
        uint256 indexed updateId,
        bytes32 indexed sharpFact,
        bytes32 factProgramHash,
        bytes32 stateTransitionFact,
        bytes32 messageHash,
        uint256 nonce
    );

    constructor(
        address gpsVerifier_,
        address starknetCore_,
        uint256 l2Receiver_,
        uint256 l2Selector_
    ) {
        gpsVerifier = IFactRegistry(gpsVerifier_);
        starknetCore = IStarknetMessaging(starknetCore_);
        l2Receiver = l2Receiver_;
        l2Selector = l2Selector_;
    }

    function relayAttestedFact(
        uint256 updateId,
        bytes32 factProgramHash,
        bytes32 stateTransitionFact
    ) external payable returns (bytes32 msgHash, uint256 nonce) {
        bytes32 sharpFact = computeSharpFact(factProgramHash, stateTransitionFact);
        require(gpsVerifier.isValid(sharpFact), "SHARP_FACT_NOT_REGISTERED");

        uint256[] memory payload = new uint256[](7);
        payload[0] = updateId;
        (payload[1], payload[2]) = splitU256(uint256(factProgramHash));
        (payload[3], payload[4]) = splitU256(uint256(stateTransitionFact));
        (payload[5], payload[6]) = splitU256(uint256(sharpFact));

        relayedSharpFacts[updateId] = sharpFact;
        (msgHash, nonce) = starknetCore.sendMessageToL2{value: msg.value}(
            l2Receiver,
            l2Selector,
            payload
        );

        emit FactRelayed(
            updateId,
            sharpFact,
            factProgramHash,
            stateTransitionFact,
            msgHash,
            nonce
        );
    }

    function computeSharpFact(
        bytes32 factProgramHash,
        bytes32 stateTransitionFact
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(factProgramHash, stateTransitionFact));
    }

    function splitU256(uint256 value) public pure returns (uint256 low, uint256 high) {
        low = value & type(uint128).max;
        high = value >> 128;
    }
}
