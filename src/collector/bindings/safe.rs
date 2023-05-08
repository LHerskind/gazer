pub use safe::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
clippy::enum_variant_names,
clippy::too_many_arguments,
clippy::upper_case_acronyms,
clippy::type_complexity,
dead_code,
non_camel_case_types,
)]
pub mod safe {
    #[rustfmt::skip]
    const __ABI: &str = "[\n  {\n    \"inputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"AddedOwner\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"approvedHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ApproveHash\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"handler\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ChangedFallbackHandler\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"guard\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ChangedGuard\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"threshold\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ChangedThreshold\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"module\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"DisabledModule\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"module\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"EnabledModule\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"txHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"payment\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ExecutionFailure\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"module\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ExecutionFromModuleFailure\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"module\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ExecutionFromModuleSuccess\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"txHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"payment\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ExecutionSuccess\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RemovedOwner\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"SafeReceived\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"initiator\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address[]\",\n        \"name\": \"owners\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"threshold\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"initializer\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"fallbackHandler\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"SafeSetup\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"msgHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"SignMsg\",\n    \"type\": \"event\"\n  },\n  {\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"fallback\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"VERSION\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_threshold\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"addOwnerWithThreshold\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"hashToApprove\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"approveHash\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"approvedHashes\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_threshold\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"changeThreshold\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"dataHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"signatures\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"requiredSignatures\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"checkNSignatures\",\n    \"outputs\": [],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"dataHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"signatures\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"checkSignatures\",\n    \"outputs\": [],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"prevModule\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"module\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"disableModule\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"domainSeparator\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"module\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"enableModule\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"enum Enum.Operation\",\n        \"name\": \"operation\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"safeTxGas\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"baseGas\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"gasPrice\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"gasToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"refundReceiver\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_nonce\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"encodeTransactionData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"enum Enum.Operation\",\n        \"name\": \"operation\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"safeTxGas\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"baseGas\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"gasPrice\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"gasToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"refundReceiver\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"signatures\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"execTransaction\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"success\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"enum Enum.Operation\",\n        \"name\": \"operation\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"execTransactionFromModule\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"success\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"enum Enum.Operation\",\n        \"name\": \"operation\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"execTransactionFromModuleReturnData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"success\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"returnData\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getChainId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"start\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"pageSize\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getModulesPaginated\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"array\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"next\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getOwners\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"offset\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"length\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getStorageAt\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getThreshold\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"enum Enum.Operation\",\n        \"name\": \"operation\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"safeTxGas\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"baseGas\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"gasPrice\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"gasToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"refundReceiver\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_nonce\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getTransactionHash\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"module\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isModuleEnabled\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isOwner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"nonce\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"prevOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_threshold\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"removeOwner\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"enum Enum.Operation\",\n        \"name\": \"operation\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"requiredTxGas\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"handler\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setFallbackHandler\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"guard\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setGuard\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"_owners\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_threshold\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"fallbackHandler\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"paymentToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"payment\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"paymentReceiver\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setup\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"signedMessages\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"targetContract\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"calldataPayload\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"simulateAndRevert\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"prevOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"oldOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"swapOwner\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"stateMutability\": \"payable\",\n    \"type\": \"receive\"\n  }\n]";
    ///The parsed JSON ABI of the contract.
    pub static SAFE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct safe<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for safe<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for safe<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for safe<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for safe<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(safe)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> safe<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SAFE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addOwnerWithThreshold` (0x0d582f13) function
        pub fn add_owner_with_threshold(
            &self,
            owner: ::ethers::core::types::Address,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 88, 47, 19], (owner, threshold))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveHash` (0xd4d9bdcd) function
        pub fn approve_hash(
            &self,
            hash_to_approve: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 217, 189, 205], hash_to_approve)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approvedHashes` (0x7d832974) function
        pub fn approved_hashes(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([125, 131, 41, 116], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeThreshold` (0x694e80c3) function
        pub fn change_threshold(
            &self,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 78, 128, 195], threshold)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkNSignatures` (0x12fb68e0) function
        pub fn check_n_signatures(
            &self,
            data_hash: [u8; 32],
            data: ::ethers::core::types::Bytes,
            signatures: ::ethers::core::types::Bytes,
            required_signatures: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [18, 251, 104, 224],
                    (data_hash, data, signatures, required_signatures),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures` (0x934f3a11) function
        pub fn check_signatures(
            &self,
            data_hash: [u8; 32],
            data: ::ethers::core::types::Bytes,
            signatures: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 79, 58, 17], (data_hash, data, signatures))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableModule` (0xe009cfde) function
        pub fn disable_module(
            &self,
            prev_module: ::ethers::core::types::Address,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 9, 207, 222], (prev_module, module))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `domainSeparator` (0xf698da25) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableModule` (0x610b5925) function
        pub fn enable_module(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 11, 89, 37], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeTransactionData` (0xe86637db) function
        pub fn encode_transaction_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [232, 102, 55, 219],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransaction` (0x6a761202) function
        pub fn exec_transaction(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            signatures: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [106, 118, 18, 2],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        signatures,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransactionFromModule` (0x468721a7) function
        pub fn exec_transaction_from_module(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([70, 135, 33, 167], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransactionFromModuleReturnData` (0x5229073f) function
        pub fn exec_transaction_from_module_return_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([82, 41, 7, 63], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getModulesPaginated` (0xcc2f8452) function
        pub fn get_modules_paginated(
            &self,
            start: ::ethers::core::types::Address,
            page_size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([204, 47, 132, 82], (start, page_size))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOwners` (0xa0e67e2b) function
        pub fn get_owners(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([160, 230, 126, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStorageAt` (0x5624b25b) function
        pub fn get_storage_at(
            &self,
            offset: ::ethers::core::types::U256,
            length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([86, 36, 178, 91], (offset, length))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getThreshold` (0xe75235b8) function
        pub fn get_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 82, 53, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTransactionHash` (0xd8d11f78) function
        pub fn get_transaction_hash(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [216, 209, 31, 120],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isModuleEnabled` (0x2d9ad53d) function
        pub fn is_module_enabled(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([45, 154, 213, 61], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOwner` (0x2f54bf6e) function
        pub fn is_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 84, 191, 110], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0xaffed0e0) function
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeOwner` (0xf8dc5dd9) function
        pub fn remove_owner(
            &self,
            prev_owner: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 220, 93, 217], (prev_owner, owner, threshold))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requiredTxGas` (0xc4ca3a9c) function
        pub fn required_tx_gas(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 202, 58, 156], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFallbackHandler` (0xf08a0323) function
        pub fn set_fallback_handler(
            &self,
            handler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 138, 3, 35], handler)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGuard` (0xe19a9dd9) function
        pub fn set_guard(
            &self,
            guard: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 154, 157, 217], guard)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setup` (0xb63e800d) function
        pub fn setup(
            &self,
            owners: ::std::vec::Vec<::ethers::core::types::Address>,
            threshold: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            fallback_handler: ::ethers::core::types::Address,
            payment_token: ::ethers::core::types::Address,
            payment: ::ethers::core::types::U256,
            payment_receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [182, 62, 128, 13],
                    (
                        owners,
                        threshold,
                        to,
                        data,
                        fallback_handler,
                        payment_token,
                        payment,
                        payment_receiver,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedMessages` (0x5ae6bd37) function
        pub fn signed_messages(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 230, 189, 55], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateAndRevert` (0xb4faba09) function
        pub fn simulate_and_revert(
            &self,
            target_contract: ::ethers::core::types::Address,
            calldata_payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 250, 186, 9], (target_contract, calldata_payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapOwner` (0xe318b52b) function
        pub fn swap_owner(
            &self,
            prev_owner: ::ethers::core::types::Address,
            old_owner: ::ethers::core::types::Address,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 24, 181, 43], (prev_owner, old_owner, new_owner))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddedOwner` event
        pub fn added_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddedOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApproveHash` event
        pub fn approve_hash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApproveHashFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedFallbackHandler` event
        pub fn changed_fallback_handler_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedFallbackHandlerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedGuard` event
        pub fn changed_guard_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedGuardFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedThreshold` event
        pub fn changed_threshold_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedThresholdFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DisabledModule` event
        pub fn disabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DisabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnabledModule` event
        pub fn enabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFailure` event
        pub fn execution_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleFailure` event
        pub fn execution_from_module_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleSuccess` event
        pub fn execution_from_module_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionSuccess` event
        pub fn execution_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemovedOwner` event
        pub fn removed_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemovedOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SafeReceived` event
        pub fn safe_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeReceivedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SafeSetup` event
        pub fn safe_setup_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeSetupFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SignMsg` event
        pub fn sign_msg_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignMsgFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, safeEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for safe<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "AddedOwner", abi = "AddedOwner(address)")]
    pub struct AddedOwnerFilter {
        pub owner: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "ApproveHash", abi = "ApproveHash(bytes32,address)")]
    pub struct ApproveHashFilter {
        #[ethevent(indexed)]
        pub approved_hash: [u8; 32],
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "ChangedFallbackHandler", abi = "ChangedFallbackHandler(address)")]
    pub struct ChangedFallbackHandlerFilter {
        pub handler: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "ChangedGuard", abi = "ChangedGuard(address)")]
    pub struct ChangedGuardFilter {
        pub guard: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "ChangedThreshold", abi = "ChangedThreshold(uint256)")]
    pub struct ChangedThresholdFilter {
        pub threshold: ::ethers::core::types::U256,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "DisabledModule", abi = "DisabledModule(address)")]
    pub struct DisabledModuleFilter {
        pub module: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "EnabledModule", abi = "EnabledModule(address)")]
    pub struct EnabledModuleFilter {
        pub module: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "ExecutionFailure", abi = "ExecutionFailure(bytes32,uint256)")]
    pub struct ExecutionFailureFilter {
        pub tx_hash: [u8; 32],
        pub payment: ::ethers::core::types::U256,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(
    name = "ExecutionFromModuleFailure",
    abi = "ExecutionFromModuleFailure(address)"
    )]
    pub struct ExecutionFromModuleFailureFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(
    name = "ExecutionFromModuleSuccess",
    abi = "ExecutionFromModuleSuccess(address)"
    )]
    pub struct ExecutionFromModuleSuccessFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "ExecutionSuccess", abi = "ExecutionSuccess(bytes32,uint256)")]
    pub struct ExecutionSuccessFilter {
        pub tx_hash: [u8; 32],
        pub payment: ::ethers::core::types::U256,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "RemovedOwner", abi = "RemovedOwner(address)")]
    pub struct RemovedOwnerFilter {
        pub owner: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "SafeReceived", abi = "SafeReceived(address,uint256)")]
    pub struct SafeReceivedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(
    name = "SafeSetup",
    abi = "SafeSetup(address,address[],uint256,address,address)"
    )]
    pub struct SafeSetupFilter {
        #[ethevent(indexed)]
        pub initiator: ::ethers::core::types::Address,
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub threshold: ::ethers::core::types::U256,
        pub initializer: ::ethers::core::types::Address,
        pub fallback_handler: ::ethers::core::types::Address,
    }
    #[derive(
    Clone,
    ::ethers::contract::EthEvent,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethevent(name = "SignMsg", abi = "SignMsg(bytes32)")]
    pub struct SignMsgFilter {
        #[ethevent(indexed)]
        pub msg_hash: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum safeEvents {
        AddedOwnerFilter(AddedOwnerFilter),
        ApproveHashFilter(ApproveHashFilter),
        ChangedFallbackHandlerFilter(ChangedFallbackHandlerFilter),
        ChangedGuardFilter(ChangedGuardFilter),
        ChangedThresholdFilter(ChangedThresholdFilter),
        DisabledModuleFilter(DisabledModuleFilter),
        EnabledModuleFilter(EnabledModuleFilter),
        ExecutionFailureFilter(ExecutionFailureFilter),
        ExecutionFromModuleFailureFilter(ExecutionFromModuleFailureFilter),
        ExecutionFromModuleSuccessFilter(ExecutionFromModuleSuccessFilter),
        ExecutionSuccessFilter(ExecutionSuccessFilter),
        RemovedOwnerFilter(RemovedOwnerFilter),
        SafeReceivedFilter(SafeReceivedFilter),
        SafeSetupFilter(SafeSetupFilter),
        SignMsgFilter(SignMsgFilter),
    }
    impl ::ethers::contract::EthLogDecode for safeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedOwnerFilter::decode_log(log) {
                return Ok(safeEvents::AddedOwnerFilter(decoded));
            }
            if let Ok(decoded) = ApproveHashFilter::decode_log(log) {
                return Ok(safeEvents::ApproveHashFilter(decoded));
            }
            if let Ok(decoded) = ChangedFallbackHandlerFilter::decode_log(log) {
                return Ok(safeEvents::ChangedFallbackHandlerFilter(decoded));
            }
            if let Ok(decoded) = ChangedGuardFilter::decode_log(log) {
                return Ok(safeEvents::ChangedGuardFilter(decoded));
            }
            if let Ok(decoded) = ChangedThresholdFilter::decode_log(log) {
                return Ok(safeEvents::ChangedThresholdFilter(decoded));
            }
            if let Ok(decoded) = DisabledModuleFilter::decode_log(log) {
                return Ok(safeEvents::DisabledModuleFilter(decoded));
            }
            if let Ok(decoded) = EnabledModuleFilter::decode_log(log) {
                return Ok(safeEvents::EnabledModuleFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFailureFilter::decode_log(log) {
                return Ok(safeEvents::ExecutionFailureFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFromModuleFailureFilter::decode_log(log) {
                return Ok(safeEvents::ExecutionFromModuleFailureFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFromModuleSuccessFilter::decode_log(log) {
                return Ok(safeEvents::ExecutionFromModuleSuccessFilter(decoded));
            }
            if let Ok(decoded) = ExecutionSuccessFilter::decode_log(log) {
                return Ok(safeEvents::ExecutionSuccessFilter(decoded));
            }
            if let Ok(decoded) = RemovedOwnerFilter::decode_log(log) {
                return Ok(safeEvents::RemovedOwnerFilter(decoded));
            }
            if let Ok(decoded) = SafeReceivedFilter::decode_log(log) {
                return Ok(safeEvents::SafeReceivedFilter(decoded));
            }
            if let Ok(decoded) = SafeSetupFilter::decode_log(log) {
                return Ok(safeEvents::SafeSetupFilter(decoded));
            }
            if let Ok(decoded) = SignMsgFilter::decode_log(log) {
                return Ok(safeEvents::SignMsgFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for safeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedOwnerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveHashFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangedFallbackHandlerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedGuardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedThresholdFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DisabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovedOwnerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeReceivedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeSetupFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignMsgFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddedOwnerFilter> for safeEvents {
        fn from(value: AddedOwnerFilter) -> Self {
            Self::AddedOwnerFilter(value)
        }
    }
    impl ::core::convert::From<ApproveHashFilter> for safeEvents {
        fn from(value: ApproveHashFilter) -> Self {
            Self::ApproveHashFilter(value)
        }
    }
    impl ::core::convert::From<ChangedFallbackHandlerFilter> for safeEvents {
        fn from(value: ChangedFallbackHandlerFilter) -> Self {
            Self::ChangedFallbackHandlerFilter(value)
        }
    }
    impl ::core::convert::From<ChangedGuardFilter> for safeEvents {
        fn from(value: ChangedGuardFilter) -> Self {
            Self::ChangedGuardFilter(value)
        }
    }
    impl ::core::convert::From<ChangedThresholdFilter> for safeEvents {
        fn from(value: ChangedThresholdFilter) -> Self {
            Self::ChangedThresholdFilter(value)
        }
    }
    impl ::core::convert::From<DisabledModuleFilter> for safeEvents {
        fn from(value: DisabledModuleFilter) -> Self {
            Self::DisabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<EnabledModuleFilter> for safeEvents {
        fn from(value: EnabledModuleFilter) -> Self {
            Self::EnabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFailureFilter> for safeEvents {
        fn from(value: ExecutionFailureFilter) -> Self {
            Self::ExecutionFailureFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleFailureFilter> for safeEvents {
        fn from(value: ExecutionFromModuleFailureFilter) -> Self {
            Self::ExecutionFromModuleFailureFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleSuccessFilter> for safeEvents {
        fn from(value: ExecutionFromModuleSuccessFilter) -> Self {
            Self::ExecutionFromModuleSuccessFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionSuccessFilter> for safeEvents {
        fn from(value: ExecutionSuccessFilter) -> Self {
            Self::ExecutionSuccessFilter(value)
        }
    }
    impl ::core::convert::From<RemovedOwnerFilter> for safeEvents {
        fn from(value: RemovedOwnerFilter) -> Self {
            Self::RemovedOwnerFilter(value)
        }
    }
    impl ::core::convert::From<SafeReceivedFilter> for safeEvents {
        fn from(value: SafeReceivedFilter) -> Self {
            Self::SafeReceivedFilter(value)
        }
    }
    impl ::core::convert::From<SafeSetupFilter> for safeEvents {
        fn from(value: SafeSetupFilter) -> Self {
            Self::SafeSetupFilter(value)
        }
    }
    impl ::core::convert::From<SignMsgFilter> for safeEvents {
        fn from(value: SignMsgFilter) -> Self {
            Self::SignMsgFilter(value)
        }
    }
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `addOwnerWithThreshold` function with signature `addOwnerWithThreshold(address,uint256)` and selector `0x0d582f13`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "addOwnerWithThreshold",
    abi = "addOwnerWithThreshold(address,uint256)"
    )]
    pub struct AddOwnerWithThresholdCall {
        pub owner: ::ethers::core::types::Address,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `approveHash` function with signature `approveHash(bytes32)` and selector `0xd4d9bdcd`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "approveHash", abi = "approveHash(bytes32)")]
    pub struct ApproveHashCall {
        pub hash_to_approve: [u8; 32],
    }
    ///Container type for all input parameters for the `approvedHashes` function with signature `approvedHashes(address,bytes32)` and selector `0x7d832974`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "approvedHashes", abi = "approvedHashes(address,bytes32)")]
    pub struct ApprovedHashesCall(pub ::ethers::core::types::Address, pub [u8; 32]);
    ///Container type for all input parameters for the `changeThreshold` function with signature `changeThreshold(uint256)` and selector `0x694e80c3`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "changeThreshold", abi = "changeThreshold(uint256)")]
    pub struct ChangeThresholdCall {
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkNSignatures` function with signature `checkNSignatures(bytes32,bytes,bytes,uint256)` and selector `0x12fb68e0`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "checkNSignatures",
    abi = "checkNSignatures(bytes32,bytes,bytes,uint256)"
    )]
    pub struct CheckNSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
        pub required_signatures: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,bytes)` and selector `0x934f3a11`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "checkSignatures", abi = "checkSignatures(bytes32,bytes,bytes)")]
    pub struct CheckSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `disableModule` function with signature `disableModule(address,address)` and selector `0xe009cfde`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "disableModule", abi = "disableModule(address,address)")]
    pub struct DisableModuleCall {
        pub prev_module: ::ethers::core::types::Address,
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `enableModule` function with signature `enableModule(address)` and selector `0x610b5925`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "enableModule", abi = "enableModule(address)")]
    pub struct EnableModuleCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `encodeTransactionData` function with signature `encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xe86637db`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "encodeTransactionData",
    abi = "encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)"
    )]
    pub struct EncodeTransactionDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `execTransaction` function with signature `execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)` and selector `0x6a761202`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "execTransaction",
    abi = "execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)"
    )]
    pub struct ExecTransactionCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub signatures: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "execTransactionFromModule",
    abi = "execTransactionFromModule(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "execTransactionFromModuleReturnData",
    abi = "execTransactionFromModuleReturnData(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleReturnDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "getModulesPaginated",
    abi = "getModulesPaginated(address,uint256)"
    )]
    pub struct GetModulesPaginatedCall {
        pub start: ::ethers::core::types::Address,
        pub page_size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getOwners` function with signature `getOwners()` and selector `0xa0e67e2b`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "getOwners", abi = "getOwners()")]
    pub struct GetOwnersCall;
    ///Container type for all input parameters for the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `0x5624b25b`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "getStorageAt", abi = "getStorageAt(uint256,uint256)")]
    pub struct GetStorageAtCall {
        pub offset: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getThreshold` function with signature `getThreshold()` and selector `0xe75235b8`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "getThreshold", abi = "getThreshold()")]
    pub struct GetThresholdCall;
    ///Container type for all input parameters for the `getTransactionHash` function with signature `getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xd8d11f78`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "getTransactionHash",
    abi = "getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)"
    )]
    pub struct GetTransactionHashCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "isModuleEnabled", abi = "isModuleEnabled(address)")]
    pub struct IsModuleEnabledCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isOwner` function with signature `isOwner(address)` and selector `0x2f54bf6e`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "isOwner", abi = "isOwner(address)")]
    pub struct IsOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    ///Container type for all input parameters for the `removeOwner` function with signature `removeOwner(address,address,uint256)` and selector `0xf8dc5dd9`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "removeOwner", abi = "removeOwner(address,address,uint256)")]
    pub struct RemoveOwnerCall {
        pub prev_owner: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requiredTxGas` function with signature `requiredTxGas(address,uint256,bytes,uint8)` and selector `0xc4ca3a9c`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "requiredTxGas",
    abi = "requiredTxGas(address,uint256,bytes,uint8)"
    )]
    pub struct RequiredTxGasCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `setFallbackHandler` function with signature `setFallbackHandler(address)` and selector `0xf08a0323`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "setFallbackHandler", abi = "setFallbackHandler(address)")]
    pub struct SetFallbackHandlerCall {
        pub handler: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGuard` function with signature `setGuard(address)` and selector `0xe19a9dd9`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "setGuard", abi = "setGuard(address)")]
    pub struct SetGuardCall {
        pub guard: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setup` function with signature `setup(address[],uint256,address,bytes,address,address,uint256,address)` and selector `0xb63e800d`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "setup",
    abi = "setup(address[],uint256,address,bytes,address,address,uint256,address)"
    )]
    pub struct SetupCall {
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub threshold: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub fallback_handler: ::ethers::core::types::Address,
        pub payment_token: ::ethers::core::types::Address,
        pub payment: ::ethers::core::types::U256,
        pub payment_receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `signedMessages` function with signature `signedMessages(bytes32)` and selector `0x5ae6bd37`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "signedMessages", abi = "signedMessages(bytes32)")]
    pub struct SignedMessagesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `simulateAndRevert` function with signature `simulateAndRevert(address,bytes)` and selector `0xb4faba09`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "simulateAndRevert", abi = "simulateAndRevert(address,bytes)")]
    pub struct SimulateAndRevertCall {
        pub target_contract: ::ethers::core::types::Address,
        pub calldata_payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `swapOwner` function with signature `swapOwner(address,address,address)` and selector `0xe318b52b`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(name = "swapOwner", abi = "swapOwner(address,address,address)")]
    pub struct SwapOwnerCall {
        pub prev_owner: ::ethers::core::types::Address,
        pub old_owner: ::ethers::core::types::Address,
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum safeCalls {
        Version(VersionCall),
        AddOwnerWithThreshold(AddOwnerWithThresholdCall),
        ApproveHash(ApproveHashCall),
        ApprovedHashes(ApprovedHashesCall),
        ChangeThreshold(ChangeThresholdCall),
        CheckNSignatures(CheckNSignaturesCall),
        CheckSignatures(CheckSignaturesCall),
        DisableModule(DisableModuleCall),
        DomainSeparator(DomainSeparatorCall),
        EnableModule(EnableModuleCall),
        EncodeTransactionData(EncodeTransactionDataCall),
        ExecTransaction(ExecTransactionCall),
        ExecTransactionFromModule(ExecTransactionFromModuleCall),
        ExecTransactionFromModuleReturnData(ExecTransactionFromModuleReturnDataCall),
        GetChainId(GetChainIdCall),
        GetModulesPaginated(GetModulesPaginatedCall),
        GetOwners(GetOwnersCall),
        GetStorageAt(GetStorageAtCall),
        GetThreshold(GetThresholdCall),
        GetTransactionHash(GetTransactionHashCall),
        IsModuleEnabled(IsModuleEnabledCall),
        IsOwner(IsOwnerCall),
        Nonce(NonceCall),
        RemoveOwner(RemoveOwnerCall),
        RequiredTxGas(RequiredTxGasCall),
        SetFallbackHandler(SetFallbackHandlerCall),
        SetGuard(SetGuardCall),
        Setup(SetupCall),
        SignedMessages(SignedMessagesCall),
        SimulateAndRevert(SimulateAndRevertCall),
        SwapOwner(SwapOwnerCall),
    }
    impl ::ethers::core::abi::AbiDecode for safeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded)
                = <AddOwnerWithThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddOwnerWithThreshold(decoded));
            }
            if let Ok(decoded)
                = <ApproveHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApproveHash(decoded));
            }
            if let Ok(decoded)
                = <ApprovedHashesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApprovedHashes(decoded));
            }
            if let Ok(decoded)
                = <ChangeThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChangeThreshold(decoded));
            }
            if let Ok(decoded)
                = <CheckNSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckNSignatures(decoded));
            }
            if let Ok(decoded)
                = <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded)
                = <DisableModuleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableModule(decoded));
            }
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <EnableModuleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableModule(decoded));
            }
            if let Ok(decoded)
                = <EncodeTransactionDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EncodeTransactionData(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecTransaction(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionFromModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecTransactionFromModule(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionFromModuleReturnDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecTransactionFromModuleReturnData(decoded));
            }
            if let Ok(decoded)
                = <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded)
                = <GetModulesPaginatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetModulesPaginated(decoded));
            }
            if let Ok(decoded)
                = <GetOwnersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOwners(decoded));
            }
            if let Ok(decoded)
                = <GetStorageAtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetStorageAt(decoded));
            }
            if let Ok(decoded)
                = <GetThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetThreshold(decoded));
            }
            if let Ok(decoded)
                = <GetTransactionHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTransactionHash(decoded));
            }
            if let Ok(decoded)
                = <IsModuleEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsModuleEnabled(decoded));
            }
            if let Ok(decoded)
                = <IsOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsOwner(decoded));
            }
            if let Ok(decoded)
                = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded)
                = <RemoveOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveOwner(decoded));
            }
            if let Ok(decoded)
                = <RequiredTxGasCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequiredTxGas(decoded));
            }
            if let Ok(decoded)
                = <SetFallbackHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFallbackHandler(decoded));
            }
            if let Ok(decoded)
                = <SetGuardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetGuard(decoded));
            }
            if let Ok(decoded)
                = <SetupCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Setup(decoded));
            }
            if let Ok(decoded)
                = <SignedMessagesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SignedMessages(decoded));
            }
            if let Ok(decoded)
                = <SimulateAndRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateAndRevert(decoded));
            }
            if let Ok(decoded)
                = <SwapOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for safeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddOwnerWithThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApprovedHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckNSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DisableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeTransactionData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetModulesPaginated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOwners(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStorageAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTransactionHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsModuleEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequiredTxGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFallbackHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGuard(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Setup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateAndRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for safeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOwnerWithThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApproveHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovedHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckNSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeTransactionData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecTransactionFromModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetModulesPaginated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOwners(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStorageAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTransactionHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsModuleEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequiredTxGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFallbackHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetGuard(element) => ::core::fmt::Display::fmt(element, f),
                Self::Setup(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateAndRevert(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapOwner(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<VersionCall> for safeCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<AddOwnerWithThresholdCall> for safeCalls {
        fn from(value: AddOwnerWithThresholdCall) -> Self {
            Self::AddOwnerWithThreshold(value)
        }
    }
    impl ::core::convert::From<ApproveHashCall> for safeCalls {
        fn from(value: ApproveHashCall) -> Self {
            Self::ApproveHash(value)
        }
    }
    impl ::core::convert::From<ApprovedHashesCall> for safeCalls {
        fn from(value: ApprovedHashesCall) -> Self {
            Self::ApprovedHashes(value)
        }
    }
    impl ::core::convert::From<ChangeThresholdCall> for safeCalls {
        fn from(value: ChangeThresholdCall) -> Self {
            Self::ChangeThreshold(value)
        }
    }
    impl ::core::convert::From<CheckNSignaturesCall> for safeCalls {
        fn from(value: CheckNSignaturesCall) -> Self {
            Self::CheckNSignatures(value)
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for safeCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<DisableModuleCall> for safeCalls {
        fn from(value: DisableModuleCall) -> Self {
            Self::DisableModule(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for safeCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<EnableModuleCall> for safeCalls {
        fn from(value: EnableModuleCall) -> Self {
            Self::EnableModule(value)
        }
    }
    impl ::core::convert::From<EncodeTransactionDataCall> for safeCalls {
        fn from(value: EncodeTransactionDataCall) -> Self {
            Self::EncodeTransactionData(value)
        }
    }
    impl ::core::convert::From<ExecTransactionCall> for safeCalls {
        fn from(value: ExecTransactionCall) -> Self {
            Self::ExecTransaction(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleCall> for safeCalls {
        fn from(value: ExecTransactionFromModuleCall) -> Self {
            Self::ExecTransactionFromModule(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleReturnDataCall> for safeCalls {
        fn from(value: ExecTransactionFromModuleReturnDataCall) -> Self {
            Self::ExecTransactionFromModuleReturnData(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for safeCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetModulesPaginatedCall> for safeCalls {
        fn from(value: GetModulesPaginatedCall) -> Self {
            Self::GetModulesPaginated(value)
        }
    }
    impl ::core::convert::From<GetOwnersCall> for safeCalls {
        fn from(value: GetOwnersCall) -> Self {
            Self::GetOwners(value)
        }
    }
    impl ::core::convert::From<GetStorageAtCall> for safeCalls {
        fn from(value: GetStorageAtCall) -> Self {
            Self::GetStorageAt(value)
        }
    }
    impl ::core::convert::From<GetThresholdCall> for safeCalls {
        fn from(value: GetThresholdCall) -> Self {
            Self::GetThreshold(value)
        }
    }
    impl ::core::convert::From<GetTransactionHashCall> for safeCalls {
        fn from(value: GetTransactionHashCall) -> Self {
            Self::GetTransactionHash(value)
        }
    }
    impl ::core::convert::From<IsModuleEnabledCall> for safeCalls {
        fn from(value: IsModuleEnabledCall) -> Self {
            Self::IsModuleEnabled(value)
        }
    }
    impl ::core::convert::From<IsOwnerCall> for safeCalls {
        fn from(value: IsOwnerCall) -> Self {
            Self::IsOwner(value)
        }
    }
    impl ::core::convert::From<NonceCall> for safeCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<RemoveOwnerCall> for safeCalls {
        fn from(value: RemoveOwnerCall) -> Self {
            Self::RemoveOwner(value)
        }
    }
    impl ::core::convert::From<RequiredTxGasCall> for safeCalls {
        fn from(value: RequiredTxGasCall) -> Self {
            Self::RequiredTxGas(value)
        }
    }
    impl ::core::convert::From<SetFallbackHandlerCall> for safeCalls {
        fn from(value: SetFallbackHandlerCall) -> Self {
            Self::SetFallbackHandler(value)
        }
    }
    impl ::core::convert::From<SetGuardCall> for safeCalls {
        fn from(value: SetGuardCall) -> Self {
            Self::SetGuard(value)
        }
    }
    impl ::core::convert::From<SetupCall> for safeCalls {
        fn from(value: SetupCall) -> Self {
            Self::Setup(value)
        }
    }
    impl ::core::convert::From<SignedMessagesCall> for safeCalls {
        fn from(value: SignedMessagesCall) -> Self {
            Self::SignedMessages(value)
        }
    }
    impl ::core::convert::From<SimulateAndRevertCall> for safeCalls {
        fn from(value: SimulateAndRevertCall) -> Self {
            Self::SimulateAndRevert(value)
        }
    }
    impl ::core::convert::From<SwapOwnerCall> for safeCalls {
        fn from(value: SwapOwnerCall) -> Self {
            Self::SwapOwner(value)
        }
    }
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `approvedHashes` function with signature `approvedHashes(address,bytes32)` and selector `0x7d832974`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct ApprovedHashesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeTransactionData` function with signature `encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xe86637db`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct EncodeTransactionDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `execTransaction` function with signature `execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)` and selector `0x6a761202`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct ExecTransactionReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct ExecTransactionFromModuleReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct ExecTransactionFromModuleReturnDataReturn {
        pub success: bool,
        pub return_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct GetChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct GetModulesPaginatedReturn {
        pub array: ::std::vec::Vec<::ethers::core::types::Address>,
        pub next: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getOwners` function with signature `getOwners()` and selector `0xa0e67e2b`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct GetOwnersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `0x5624b25b`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct GetStorageAtReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getThreshold` function with signature `getThreshold()` and selector `0xe75235b8`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct GetThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTransactionHash` function with signature `getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xd8d11f78`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct GetTransactionHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct IsModuleEnabledReturn(pub bool);
    ///Container type for all return fields from the `isOwner` function with signature `isOwner(address)` and selector `0x2f54bf6e`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct IsOwnerReturn(pub bool);
    ///Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `requiredTxGas` function with signature `requiredTxGas(address,uint256,bytes,uint8)` and selector `0xc4ca3a9c`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct RequiredTxGasReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `signedMessages` function with signature `signedMessages(bytes32)` and selector `0x5ae6bd37`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct SignedMessagesReturn(pub ::ethers::core::types::U256);
}
