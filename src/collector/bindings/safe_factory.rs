pub use safe_factory::*;
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
pub mod safe_factory {
    #[rustfmt::skip]
    const __ABI: &str = "[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"contract GnosisSafeProxy\",\n        \"name\": \"proxy\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"singleton\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ProxyCreation\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_singleton\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"initializer\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"saltNonce\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"calculateCreateProxyWithNonceAddress\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract GnosisSafeProxy\",\n        \"name\": \"proxy\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"singleton\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"createProxy\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract GnosisSafeProxy\",\n        \"name\": \"proxy\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_singleton\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"initializer\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"saltNonce\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"contract IProxyCreationCallback\",\n        \"name\": \"callback\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"createProxyWithCallback\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract GnosisSafeProxy\",\n        \"name\": \"proxy\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_singleton\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"initializer\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"saltNonce\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"createProxyWithNonce\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract GnosisSafeProxy\",\n        \"name\": \"proxy\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"proxyCreationCode\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"proxyRuntimeCode\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  }\n]";
    ///The parsed JSON ABI of the contract.
    pub static SAFE_FACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct safe_factory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for safe_factory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for safe_factory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for safe_factory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for safe_factory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(safe_factory)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> safe_factory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SAFE_FACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `calculateCreateProxyWithNonceAddress` (0x2500510e) function
        pub fn calculate_create_proxy_with_nonce_address(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([37, 0, 81, 14], (singleton, initializer, salt_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxy` (0x61b69abd) function
        pub fn create_proxy(
            &self,
            singleton: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([97, 182, 154, 189], (singleton, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxyWithCallback` (0xd18af54d) function
        pub fn create_proxy_with_callback(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [209, 138, 245, 77],
                    (singleton, initializer, salt_nonce, callback),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxyWithNonce` (0x1688f0b9) function
        pub fn create_proxy_with_nonce(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 136, 240, 185], (singleton, initializer, salt_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxyCreationCode` (0x53e5d935) function
        pub fn proxy_creation_code(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([83, 229, 217, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxyRuntimeCode` (0xaddacc0f) function
        pub fn proxy_runtime_code(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([173, 218, 204, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ProxyCreation` event
        pub fn proxy_creation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProxyCreationFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProxyCreationFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for safe_factory<M> {
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
    #[ethevent(name = "ProxyCreation", abi = "ProxyCreation(address,address)")]
    pub struct ProxyCreationFilter {
        pub proxy: ::ethers::core::types::Address,
        pub singleton: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `calculateCreateProxyWithNonceAddress` function with signature `calculateCreateProxyWithNonceAddress(address,bytes,uint256)` and selector `0x2500510e`
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
    name = "calculateCreateProxyWithNonceAddress",
    abi = "calculateCreateProxyWithNonceAddress(address,bytes,uint256)"
    )]
    pub struct CalculateCreateProxyWithNonceAddressCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createProxy` function with signature `createProxy(address,bytes)` and selector `0x61b69abd`
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
    #[ethcall(name = "createProxy", abi = "createProxy(address,bytes)")]
    pub struct CreateProxyCall {
        pub singleton: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `createProxyWithCallback` function with signature `createProxyWithCallback(address,bytes,uint256,address)` and selector `0xd18af54d`
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
    name = "createProxyWithCallback",
    abi = "createProxyWithCallback(address,bytes,uint256,address)"
    )]
    pub struct CreateProxyWithCallbackCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createProxyWithNonce` function with signature `createProxyWithNonce(address,bytes,uint256)` and selector `0x1688f0b9`
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
    name = "createProxyWithNonce",
    abi = "createProxyWithNonce(address,bytes,uint256)"
    )]
    pub struct CreateProxyWithNonceCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `proxyCreationCode` function with signature `proxyCreationCode()` and selector `0x53e5d935`
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
    #[ethcall(name = "proxyCreationCode", abi = "proxyCreationCode()")]
    pub struct ProxyCreationCodeCall;
    ///Container type for all input parameters for the `proxyRuntimeCode` function with signature `proxyRuntimeCode()` and selector `0xaddacc0f`
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
    #[ethcall(name = "proxyRuntimeCode", abi = "proxyRuntimeCode()")]
    pub struct ProxyRuntimeCodeCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum safe_factoryCalls {
        CalculateCreateProxyWithNonceAddress(CalculateCreateProxyWithNonceAddressCall),
        CreateProxy(CreateProxyCall),
        CreateProxyWithCallback(CreateProxyWithCallbackCall),
        CreateProxyWithNonce(CreateProxyWithNonceCall),
        ProxyCreationCode(ProxyCreationCodeCall),
        ProxyRuntimeCode(ProxyRuntimeCodeCall),
    }
    impl ::ethers::core::abi::AbiDecode for safe_factoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CalculateCreateProxyWithNonceAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateCreateProxyWithNonceAddress(decoded));
            }
            if let Ok(decoded)
                = <CreateProxyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateProxy(decoded));
            }
            if let Ok(decoded)
                = <CreateProxyWithCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateProxyWithCallback(decoded));
            }
            if let Ok(decoded)
                = <CreateProxyWithNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateProxyWithNonce(decoded));
            }
            if let Ok(decoded)
                = <ProxyCreationCodeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxyCreationCode(decoded));
            }
            if let Ok(decoded)
                = <ProxyRuntimeCodeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxyRuntimeCode(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for safe_factoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CalculateCreateProxyWithNonceAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxyWithCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxyWithNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxyCreationCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxyRuntimeCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for safe_factoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculateCreateProxyWithNonceAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateProxy(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateProxyWithCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateProxyWithNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProxyCreationCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxyRuntimeCode(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CalculateCreateProxyWithNonceAddressCall>
    for safe_factoryCalls {
        fn from(value: CalculateCreateProxyWithNonceAddressCall) -> Self {
            Self::CalculateCreateProxyWithNonceAddress(value)
        }
    }
    impl ::core::convert::From<CreateProxyCall> for safe_factoryCalls {
        fn from(value: CreateProxyCall) -> Self {
            Self::CreateProxy(value)
        }
    }
    impl ::core::convert::From<CreateProxyWithCallbackCall> for safe_factoryCalls {
        fn from(value: CreateProxyWithCallbackCall) -> Self {
            Self::CreateProxyWithCallback(value)
        }
    }
    impl ::core::convert::From<CreateProxyWithNonceCall> for safe_factoryCalls {
        fn from(value: CreateProxyWithNonceCall) -> Self {
            Self::CreateProxyWithNonce(value)
        }
    }
    impl ::core::convert::From<ProxyCreationCodeCall> for safe_factoryCalls {
        fn from(value: ProxyCreationCodeCall) -> Self {
            Self::ProxyCreationCode(value)
        }
    }
    impl ::core::convert::From<ProxyRuntimeCodeCall> for safe_factoryCalls {
        fn from(value: ProxyRuntimeCodeCall) -> Self {
            Self::ProxyRuntimeCode(value)
        }
    }
    ///Container type for all return fields from the `calculateCreateProxyWithNonceAddress` function with signature `calculateCreateProxyWithNonceAddress(address,bytes,uint256)` and selector `0x2500510e`
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
    pub struct CalculateCreateProxyWithNonceAddressReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxy` function with signature `createProxy(address,bytes)` and selector `0x61b69abd`
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
    pub struct CreateProxyReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxyWithCallback` function with signature `createProxyWithCallback(address,bytes,uint256,address)` and selector `0xd18af54d`
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
    pub struct CreateProxyWithCallbackReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxyWithNonce` function with signature `createProxyWithNonce(address,bytes,uint256)` and selector `0x1688f0b9`
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
    pub struct CreateProxyWithNonceReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `proxyCreationCode` function with signature `proxyCreationCode()` and selector `0x53e5d935`
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
    pub struct ProxyCreationCodeReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `proxyRuntimeCode` function with signature `proxyRuntimeCode()` and selector `0xaddacc0f`
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
    pub struct ProxyRuntimeCodeReturn(pub ::ethers::core::types::Bytes);
}
