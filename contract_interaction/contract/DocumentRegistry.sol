pragma solidity ^0.5.6;


/**
*  @dev Smart Contract resposible to notarize documents on the Ethereum Blockchain
*/
contract DocumentRegistry {

    struct Document {
        address signer; // Notary
        uint date; // Date of notarization
        string hash; // Document Hash
    }

    /**
     *  @dev Storage space used to record all documents notarized with metadata
     */
    mapping(bytes32 => Document) registry;

    /**
     *  @dev Notarize a document identified by the hash of the document hash, the sender and date in the registry
     *  @dev Emit an event Notarized in case of success
     *  @param _documentHash Document hash
     */
    function notarizeDocument(string calldata _documentHash) external returns (bool) {

        bytes32 id = keccak256(abi.encodePacked(_documentHash));

        //Check this document has not already been notarized
        require(registry[id].signer == address(0));

        registry[id].signer = msg.sender;
        registry[id].date = now;
        registry[id].hash = _documentHash;

        emit Notarized(msg.sender, _documentHash);

        return true;
    }

    /**
     *  @dev Verify a document identified by its has was noterized in the registry previsouly.
     *  @param _documentHash Document hash
     *  @return bool if document was noterized previsouly in the registry
     */
    function isNotarized(string calldata _documentHash) external view returns (bool) {
        return registry[keccak256(abi.encodePacked(_documentHash))].signer != address(0);
    }

    /**
     *  @dev Definition of the event triggered when a document is successfully notarized in the registry
     */
    event Notarized(address indexed _signer, string _documentHash);
}