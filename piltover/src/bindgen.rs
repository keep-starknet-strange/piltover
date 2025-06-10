#[derive()]
pub struct AppchainContract<A: starknet::accounts::ConnectedAccount + Sync> {
    pub address: starknet::core::types::Felt,
    pub account: A,
    pub block_id: starknet::core::types::BlockId,
}
impl<A: starknet::accounts::ConnectedAccount + Sync> AppchainContract<A> {
    pub fn new(address: starknet::core::types::Felt, account: A) -> Self {
        Self {
            address,
            account,
            block_id: starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending),
        }
    }
    pub fn set_contract_address(&mut self, address: starknet::core::types::Felt) {
        self.address = address;
    }
    pub fn provider(&self) -> &A::Provider {
        self.account.provider()
    }
    pub fn set_block(&mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive()]
pub struct AppchainContractReader<P: starknet::providers::Provider + Sync> {
    pub address: starknet::core::types::Felt,
    pub provider: P,
    pub block_id: starknet::core::types::BlockId,
}
impl<P: starknet::providers::Provider + Sync> AppchainContractReader<P> {
    pub fn new(address: starknet::core::types::Felt, provider: P) -> Self {
        Self {
            address,
            provider,
            block_id: starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending),
        }
    }
    pub fn set_contract_address(&mut self, address: starknet::core::types::Felt) {
        self.address = address;
    }
    pub fn provider(&self) -> &P {
        &self.provider
    }
    pub fn set_block(&mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive()]
pub struct LogStateTransitionFact {
    pub state_transition_fact: cainome::cairo_serde::U256,
}
impl cainome::cairo_serde::CairoSerde for LogStateTransitionFact {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.state_transition_fact);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(
            &__rust.state_transition_fact,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let state_transition_fact =
            cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&state_transition_fact);
        Ok(LogStateTransitionFact {
            state_transition_fact,
        })
    }
}
impl LogStateTransitionFact {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("LogStateTransitionFact").unwrap()
    }
    pub fn event_name() -> &'static str {
        "LogStateTransitionFact"
    }
}
#[derive()]
pub struct LogStateUpdate {
    pub state_root: starknet::core::types::Felt,
    pub block_number: starknet::core::types::Felt,
    pub block_hash: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for LogStateUpdate {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.state_root);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.block_number);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.block_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.state_root,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.block_number,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.block_hash,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let state_root = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&state_root);
        let block_number = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&block_number);
        let block_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&block_hash);
        Ok(LogStateUpdate {
            state_root,
            block_number,
            block_hash,
        })
    }
}
impl LogStateUpdate {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("LogStateUpdate").unwrap()
    }
    pub fn event_name() -> &'static str {
        "LogStateUpdate"
    }
}
#[derive()]
pub struct MessageCanceled {
    pub message_hash: starknet::core::types::Felt,
    pub from: cainome::cairo_serde::ContractAddress,
    pub to: cainome::cairo_serde::ContractAddress,
    pub selector: starknet::core::types::Felt,
    pub payload: Vec<starknet::core::types::Felt>,
    pub nonce: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for MessageCanceled {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.message_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.from);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.to);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.payload);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.nonce);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.message_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.from,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.to,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.payload,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.nonce));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let message_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
        let from = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
        let to = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let payload = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
        let nonce = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
        Ok(MessageCanceled {
            message_hash,
            from,
            to,
            selector,
            payload,
            nonce,
        })
    }
}
impl MessageCanceled {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("MessageCanceled").unwrap()
    }
    pub fn event_name() -> &'static str {
        "MessageCanceled"
    }
}
#[derive()]
pub struct MessageCancellationStarted {
    pub message_hash: starknet::core::types::Felt,
    pub from: cainome::cairo_serde::ContractAddress,
    pub to: cainome::cairo_serde::ContractAddress,
    pub selector: starknet::core::types::Felt,
    pub payload: Vec<starknet::core::types::Felt>,
    pub nonce: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for MessageCancellationStarted {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.message_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.from);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.to);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.payload);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.nonce);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.message_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.from,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.to,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.payload,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.nonce));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let message_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
        let from = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
        let to = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let payload = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
        let nonce = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
        Ok(MessageCancellationStarted {
            message_hash,
            from,
            to,
            selector,
            payload,
            nonce,
        })
    }
}
impl MessageCancellationStarted {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("MessageCancellationStarted").unwrap()
    }
    pub fn event_name() -> &'static str {
        "MessageCancellationStarted"
    }
}
#[derive()]
pub struct MessageConsumed {
    pub message_hash: starknet::core::types::Felt,
    pub from: cainome::cairo_serde::ContractAddress,
    pub to: cainome::cairo_serde::ContractAddress,
    pub payload: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for MessageConsumed {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.message_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.from);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.to);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.payload);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.message_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.from,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.to,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.payload,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let message_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
        let from = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
        let to = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
        let payload = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
        Ok(MessageConsumed {
            message_hash,
            from,
            to,
            payload,
        })
    }
}
impl MessageConsumed {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("MessageConsumed").unwrap()
    }
    pub fn event_name() -> &'static str {
        "MessageConsumed"
    }
}
#[derive()]
pub struct MessageSent {
    pub message_hash: starknet::core::types::Felt,
    pub from: cainome::cairo_serde::ContractAddress,
    pub to: cainome::cairo_serde::ContractAddress,
    pub selector: starknet::core::types::Felt,
    pub nonce: starknet::core::types::Felt,
    pub payload: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for MessageSent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.message_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.from);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.to);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.nonce);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.payload);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.message_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.from,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.to,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.nonce));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.payload,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let message_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
        let from = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
        let to = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let nonce = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
        let payload = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
        Ok(MessageSent {
            message_hash,
            from,
            to,
            selector,
            nonce,
            payload,
        })
    }
}
impl MessageSent {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("MessageSent").unwrap()
    }
    pub fn event_name() -> &'static str {
        "MessageSent"
    }
}
#[derive()]
pub struct MessageToAppchainSealed {
    pub message_hash: starknet::core::types::Felt,
    pub from: cainome::cairo_serde::ContractAddress,
    pub to: cainome::cairo_serde::ContractAddress,
    pub selector: starknet::core::types::Felt,
    pub nonce: starknet::core::types::Felt,
    pub payload: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for MessageToAppchainSealed {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.message_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.from);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.to);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.nonce);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.payload);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.message_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.from,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.to,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.nonce));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.payload,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let message_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
        let from = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
        let to = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let nonce = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
        let payload = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
        Ok(MessageToAppchainSealed {
            message_hash,
            from,
            to,
            selector,
            nonce,
            payload,
        })
    }
}
impl MessageToAppchainSealed {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("MessageToAppchainSealed").unwrap()
    }
    pub fn event_name() -> &'static str {
        "MessageToAppchainSealed"
    }
}
#[derive()]
pub struct MessageToStarknetReceived {
    pub message_hash: starknet::core::types::Felt,
    pub from: cainome::cairo_serde::ContractAddress,
    pub to: cainome::cairo_serde::ContractAddress,
    pub payload: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for MessageToStarknetReceived {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.message_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.from);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.to);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.payload);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.message_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.from,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.to,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.payload,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let message_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
        let from = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
        let to = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
        let payload = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
        Ok(MessageToStarknetReceived {
            message_hash,
            from,
            to,
            payload,
        })
    }
}
impl MessageToStarknetReceived {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("MessageToStarknetReceived").unwrap()
    }
    pub fn event_name() -> &'static str {
        "MessageToStarknetReceived"
    }
}
#[derive()]
pub struct OwnershipTransferStarted {
    pub previous_owner: cainome::cairo_serde::ContractAddress,
    pub new_owner: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for OwnershipTransferStarted {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.previous_owner);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.new_owner);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.previous_owner,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.new_owner,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let previous_owner =
            cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
        let new_owner =
            cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
        Ok(OwnershipTransferStarted {
            previous_owner,
            new_owner,
        })
    }
}
impl OwnershipTransferStarted {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("OwnershipTransferStarted").unwrap()
    }
    pub fn event_name() -> &'static str {
        "OwnershipTransferStarted"
    }
}
#[derive()]
pub struct OwnershipTransferred {
    pub previous_owner: cainome::cairo_serde::ContractAddress,
    pub new_owner: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for OwnershipTransferred {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.previous_owner);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.new_owner);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.previous_owner,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.new_owner,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let previous_owner =
            cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
        let new_owner =
            cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
        Ok(OwnershipTransferred {
            previous_owner,
            new_owner,
        })
    }
}
impl OwnershipTransferred {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("OwnershipTransferred").unwrap()
    }
    pub fn event_name() -> &'static str {
        "OwnershipTransferred"
    }
}
#[derive()]
pub struct ProgramInfo {
    pub bootloader_program_hash: starknet::core::types::Felt,
    pub snos_config_hash: starknet::core::types::Felt,
    pub snos_program_hash: starknet::core::types::Felt,
    pub layout_bridge_program_hash: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for ProgramInfo {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            starknet::core::types::Felt::cairo_serialized_size(&__rust.bootloader_program_hash);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.snos_config_hash);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.snos_program_hash);
        __size +=
            starknet::core::types::Felt::cairo_serialized_size(&__rust.layout_bridge_program_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.bootloader_program_hash,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.snos_config_hash,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.snos_program_hash,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.layout_bridge_program_hash,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let bootloader_program_hash =
            starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&bootloader_program_hash);
        let snos_config_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&snos_config_hash);
        let snos_program_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&snos_program_hash);
        let layout_bridge_program_hash =
            starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&layout_bridge_program_hash);
        Ok(ProgramInfo {
            bootloader_program_hash,
            snos_config_hash,
            snos_program_hash,
            layout_bridge_program_hash,
        })
    }
}
#[derive()]
pub struct ProgramInfoChanged {
    pub changed_by: cainome::cairo_serde::ContractAddress,
    pub old_program_info: ProgramInfo,
    pub new_program_info: ProgramInfo,
}
impl cainome::cairo_serde::CairoSerde for ProgramInfoChanged {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.changed_by);
        __size += ProgramInfo::cairo_serialized_size(&__rust.old_program_info);
        __size += ProgramInfo::cairo_serialized_size(&__rust.new_program_info);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.changed_by,
        ));
        __out.extend(ProgramInfo::cairo_serialize(&__rust.old_program_info));
        __out.extend(ProgramInfo::cairo_serialize(&__rust.new_program_info));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let changed_by =
            cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&changed_by);
        let old_program_info = ProgramInfo::cairo_deserialize(__felts, __offset)?;
        __offset += ProgramInfo::cairo_serialized_size(&old_program_info);
        let new_program_info = ProgramInfo::cairo_deserialize(__felts, __offset)?;
        __offset += ProgramInfo::cairo_serialized_size(&new_program_info);
        Ok(ProgramInfoChanged {
            changed_by,
            old_program_info,
            new_program_info,
        })
    }
}
impl ProgramInfoChanged {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("ProgramInfoChanged").unwrap()
    }
    pub fn event_name() -> &'static str {
        "ProgramInfoChanged"
    }
}
#[derive()]
pub struct Upgraded {
    pub class_hash: cainome::cairo_serde::ClassHash,
}
impl cainome::cairo_serde::CairoSerde for Upgraded {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            &__rust.class_hash,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        Ok(Upgraded { class_hash })
    }
}
impl Upgraded {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("Upgraded").unwrap()
    }
    pub fn event_name() -> &'static str {
        "Upgraded"
    }
}
#[derive()]
pub enum AppchainEvent {
    OwnableEvent(OwnableEvent),
    UpgradeableEvent(UpgradeableEvent),
    ConfigEvent(ConfigEvent),
    MessagingEvent(MessagingEvent),
    ReentrancyGuardEvent(ReentrancyguardEvent),
    StateEvent(StateEvent),
    LogStateUpdate(LogStateUpdate),
    LogStateTransitionFact(LogStateTransitionFact),
}
impl cainome::cairo_serde::CairoSerde for AppchainEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            AppchainEvent::OwnableEvent(val) => OwnableEvent::cairo_serialized_size(val) + 1,
            AppchainEvent::UpgradeableEvent(val) => {
                UpgradeableEvent::cairo_serialized_size(val) + 1
            }
            AppchainEvent::ConfigEvent(val) => ConfigEvent::cairo_serialized_size(val) + 1,
            AppchainEvent::MessagingEvent(val) => MessagingEvent::cairo_serialized_size(val) + 1,
            AppchainEvent::ReentrancyGuardEvent(val) => {
                ReentrancyguardEvent::cairo_serialized_size(val) + 1
            }
            AppchainEvent::StateEvent(val) => StateEvent::cairo_serialized_size(val) + 1,
            AppchainEvent::LogStateUpdate(val) => LogStateUpdate::cairo_serialized_size(val) + 1,
            AppchainEvent::LogStateTransitionFact(val) => {
                LogStateTransitionFact::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            AppchainEvent::OwnableEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(OwnableEvent::cairo_serialize(val));
                temp
            }
            AppchainEvent::UpgradeableEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(UpgradeableEvent::cairo_serialize(val));
                temp
            }
            AppchainEvent::ConfigEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(ConfigEvent::cairo_serialize(val));
                temp
            }
            AppchainEvent::MessagingEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(MessagingEvent::cairo_serialize(val));
                temp
            }
            AppchainEvent::ReentrancyGuardEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&4usize));
                temp.extend(ReentrancyguardEvent::cairo_serialize(val));
                temp
            }
            AppchainEvent::StateEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&5usize));
                temp.extend(StateEvent::cairo_serialize(val));
                temp
            }
            AppchainEvent::LogStateUpdate(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&6usize));
                temp.extend(LogStateUpdate::cairo_serialize(val));
                temp
            }
            AppchainEvent::LogStateTransitionFact(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&7usize));
                temp.extend(LogStateTransitionFact::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(AppchainEvent::OwnableEvent(
                OwnableEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            1usize => Ok(AppchainEvent::UpgradeableEvent(
                UpgradeableEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            2usize => Ok(AppchainEvent::ConfigEvent(ConfigEvent::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            3usize => Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            4usize => Ok(AppchainEvent::ReentrancyGuardEvent(
                ReentrancyguardEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            5usize => Ok(AppchainEvent::StateEvent(StateEvent::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            6usize => Ok(AppchainEvent::LogStateUpdate(
                LogStateUpdate::cairo_deserialize(__felts, __offset + 1)?,
            )),
            7usize => Ok(AppchainEvent::LogStateTransitionFact(
                LogStateTransitionFact::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "AppchainEvent"
                )))
            }
        }
    }
}
impl TryFrom<&starknet::core::types::EmittedEvent> for AppchainEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferred")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnershipTransferred"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "previous_owner", "OwnershipTransferred", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_owner", "OwnershipTransferred", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
            return Ok(AppchainEvent::OwnableEvent(
                OwnableEvent::OwnershipTransferred(OwnershipTransferred {
                    previous_owner,
                    new_owner,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferStarted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnershipTransferStarted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "previous_owner", "OwnershipTransferStarted", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_owner", "OwnershipTransferStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
            return Ok(AppchainEvent::OwnableEvent(
                OwnableEvent::OwnershipTransferStarted(OwnershipTransferStarted {
                    previous_owner,
                    new_owner,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("Upgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "Upgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "Upgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(AppchainEvent::UpgradeableEvent(UpgradeableEvent::Upgraded(
                Upgraded { class_hash },
            )));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ProgramInfoChanged")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ProgramInfoChanged"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let changed_by = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "changed_by", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&changed_by);
            let old_program_info = match ProgramInfo::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "old_program_info", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset += ProgramInfo::cairo_serialized_size(&old_program_info);
            let new_program_info = match ProgramInfo::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_program_info", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset += ProgramInfo::cairo_serialized_size(&new_program_info);
            return Ok(AppchainEvent::ConfigEvent(ConfigEvent::ProgramInfoChanged(
                ProgramInfoChanged {
                    changed_by,
                    old_program_info,
                    new_program_info,
                },
            )));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageSent")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageSent"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageSent", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageSent", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageSent", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageSent", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageSent", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageSent", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(AppchainEvent::MessagingEvent(MessagingEvent::MessageSent(
                MessageSent {
                    message_hash,
                    from,
                    to,
                    selector,
                    nonce,
                    payload,
                },
            )));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageConsumed")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageConsumed"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageConsumed", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageConsumed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageConsumed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageConsumed", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageConsumed(MessageConsumed {
                    message_hash,
                    from,
                    to,
                    payload,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageCancellationStarted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageCancellationStarted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageCancellationStarted", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageCancellationStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageCancellationStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageCancellationStarted", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageCancellationStarted", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageCancellationStarted", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageCancellationStarted(MessageCancellationStarted {
                    message_hash,
                    from,
                    to,
                    selector,
                    payload,
                    nonce,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageCanceled")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageCanceled"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageCanceled", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageCanceled", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageCanceled", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageCanceled", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageCanceled", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageCanceled", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageCanceled(MessageCanceled {
                    message_hash,
                    from,
                    to,
                    selector,
                    payload,
                    nonce,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageToStarknetReceived")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageToStarknetReceived"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageToStarknetReceived", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageToStarknetReceived", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageToStarknetReceived", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageToStarknetReceived", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageToStarknetReceived(MessageToStarknetReceived {
                    message_hash,
                    from,
                    to,
                    payload,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageToAppchainSealed")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageToAppchainSealed"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageToAppchainSealed", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageToAppchainSealed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageToAppchainSealed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageToAppchainSealed", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageToAppchainSealed", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageToAppchainSealed", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageToAppchainSealed(MessageToAppchainSealed {
                    message_hash,
                    from,
                    to,
                    selector,
                    nonce,
                    payload,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("LogStateUpdate")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "LogStateUpdate"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let state_root =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "state_root", "LogStateUpdate", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&state_root);
            let block_number =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "block_number", "LogStateUpdate", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&block_number);
            let block_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "block_hash", "LogStateUpdate", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&block_hash);
            return Ok(AppchainEvent::LogStateUpdate(LogStateUpdate {
                state_root,
                block_number,
                block_hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("LogStateTransitionFact")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "LogStateTransitionFact"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let state_transition_fact =
                match cainome::cairo_serde::U256::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "state_transition_fact", "LogStateTransitionFact", e
                        ))
                    }
                };
            data_offset +=
                cainome::cairo_serde::U256::cairo_serialized_size(&state_transition_fact);
            return Ok(AppchainEvent::LogStateTransitionFact(
                LogStateTransitionFact {
                    state_transition_fact,
                },
            ));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl TryFrom<&starknet::core::types::Event> for AppchainEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::Event) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferred")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnershipTransferred"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "previous_owner", "OwnershipTransferred", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_owner", "OwnershipTransferred", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
            return Ok(AppchainEvent::OwnableEvent(
                OwnableEvent::OwnershipTransferred(OwnershipTransferred {
                    previous_owner,
                    new_owner,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferStarted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnershipTransferStarted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "previous_owner", "OwnershipTransferStarted", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_owner", "OwnershipTransferStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
            return Ok(AppchainEvent::OwnableEvent(
                OwnableEvent::OwnershipTransferStarted(OwnershipTransferStarted {
                    previous_owner,
                    new_owner,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("Upgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "Upgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "Upgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(AppchainEvent::UpgradeableEvent(UpgradeableEvent::Upgraded(
                Upgraded { class_hash },
            )));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ProgramInfoChanged")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ProgramInfoChanged"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let changed_by = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "changed_by", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&changed_by);
            let old_program_info = match ProgramInfo::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "old_program_info", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset += ProgramInfo::cairo_serialized_size(&old_program_info);
            let new_program_info = match ProgramInfo::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_program_info", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset += ProgramInfo::cairo_serialized_size(&new_program_info);
            return Ok(AppchainEvent::ConfigEvent(ConfigEvent::ProgramInfoChanged(
                ProgramInfoChanged {
                    changed_by,
                    old_program_info,
                    new_program_info,
                },
            )));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageSent")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageSent"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageSent", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageSent", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageSent", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageSent", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageSent", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageSent", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(AppchainEvent::MessagingEvent(MessagingEvent::MessageSent(
                MessageSent {
                    message_hash,
                    from,
                    to,
                    selector,
                    nonce,
                    payload,
                },
            )));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageConsumed")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageConsumed"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageConsumed", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageConsumed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageConsumed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageConsumed", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageConsumed(MessageConsumed {
                    message_hash,
                    from,
                    to,
                    payload,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageCancellationStarted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageCancellationStarted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageCancellationStarted", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageCancellationStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageCancellationStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageCancellationStarted", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageCancellationStarted", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageCancellationStarted", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageCancellationStarted(MessageCancellationStarted {
                    message_hash,
                    from,
                    to,
                    selector,
                    payload,
                    nonce,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageCanceled")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageCanceled"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageCanceled", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageCanceled", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageCanceled", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageCanceled", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageCanceled", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageCanceled", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageCanceled(MessageCanceled {
                    message_hash,
                    from,
                    to,
                    selector,
                    payload,
                    nonce,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageToStarknetReceived")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageToStarknetReceived"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageToStarknetReceived", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageToStarknetReceived", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageToStarknetReceived", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageToStarknetReceived", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageToStarknetReceived(MessageToStarknetReceived {
                    message_hash,
                    from,
                    to,
                    payload,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageToAppchainSealed")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageToAppchainSealed"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageToAppchainSealed", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageToAppchainSealed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageToAppchainSealed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageToAppchainSealed", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageToAppchainSealed", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageToAppchainSealed", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(AppchainEvent::MessagingEvent(
                MessagingEvent::MessageToAppchainSealed(MessageToAppchainSealed {
                    message_hash,
                    from,
                    to,
                    selector,
                    nonce,
                    payload,
                }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("LogStateUpdate")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "LogStateUpdate"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let state_root =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "state_root", "LogStateUpdate", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&state_root);
            let block_number =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "block_number", "LogStateUpdate", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&block_number);
            let block_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "block_hash", "LogStateUpdate", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&block_hash);
            return Ok(AppchainEvent::LogStateUpdate(LogStateUpdate {
                state_root,
                block_number,
                block_hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("LogStateTransitionFact")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "LogStateTransitionFact"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let state_transition_fact =
                match cainome::cairo_serde::U256::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "state_transition_fact", "LogStateTransitionFact", e
                        ))
                    }
                };
            data_offset +=
                cainome::cairo_serde::U256::cairo_serialized_size(&state_transition_fact);
            return Ok(AppchainEvent::LogStateTransitionFact(
                LogStateTransitionFact {
                    state_transition_fact,
                },
            ));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive()]
pub enum ConfigEvent {
    ProgramInfoChanged(ProgramInfoChanged),
}
impl cainome::cairo_serde::CairoSerde for ConfigEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            ConfigEvent::ProgramInfoChanged(val) => {
                ProgramInfoChanged::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            ConfigEvent::ProgramInfoChanged(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(ProgramInfoChanged::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(ConfigEvent::ProgramInfoChanged(
                ProgramInfoChanged::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "ConfigEvent"
                )))
            }
        }
    }
}
impl TryFrom<&starknet::core::types::EmittedEvent> for ConfigEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ProgramInfoChanged")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ProgramInfoChanged"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let changed_by = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "changed_by", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&changed_by);
            let old_program_info = match ProgramInfo::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "old_program_info", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset += ProgramInfo::cairo_serialized_size(&old_program_info);
            let new_program_info = match ProgramInfo::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_program_info", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset += ProgramInfo::cairo_serialized_size(&new_program_info);
            return Ok(ConfigEvent::ProgramInfoChanged(ProgramInfoChanged {
                changed_by,
                old_program_info,
                new_program_info,
            }));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl TryFrom<&starknet::core::types::Event> for ConfigEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::Event) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ProgramInfoChanged")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ProgramInfoChanged"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let changed_by = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "changed_by", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&changed_by);
            let old_program_info = match ProgramInfo::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "old_program_info", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset += ProgramInfo::cairo_serialized_size(&old_program_info);
            let new_program_info = match ProgramInfo::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_program_info", "ProgramInfoChanged", e
                    ))
                }
            };
            data_offset += ProgramInfo::cairo_serialized_size(&new_program_info);
            return Ok(ConfigEvent::ProgramInfoChanged(ProgramInfoChanged {
                changed_by,
                old_program_info,
                new_program_info,
            }));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive()]
pub enum MessageToAppchainStatus {
    NotSent,
    Sealed,
    Cancelled,
    Pending(starknet::core::types::Felt),
    Cancelling,
}
impl cainome::cairo_serde::CairoSerde for MessageToAppchainStatus {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            MessageToAppchainStatus::NotSent => 1,
            MessageToAppchainStatus::Sealed => 1,
            MessageToAppchainStatus::Cancelled => 1,
            MessageToAppchainStatus::Pending(val) => {
                starknet::core::types::Felt::cairo_serialized_size(val) + 1
            }
            MessageToAppchainStatus::Cancelling => 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            MessageToAppchainStatus::NotSent => usize::cairo_serialize(&0usize),
            MessageToAppchainStatus::Sealed => usize::cairo_serialize(&1usize),
            MessageToAppchainStatus::Cancelled => usize::cairo_serialize(&2usize),
            MessageToAppchainStatus::Pending(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(starknet::core::types::Felt::cairo_serialize(val));
                temp
            }
            MessageToAppchainStatus::Cancelling => usize::cairo_serialize(&4usize),
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(MessageToAppchainStatus::NotSent),
            1usize => Ok(MessageToAppchainStatus::Sealed),
            2usize => Ok(MessageToAppchainStatus::Cancelled),
            3usize => Ok(MessageToAppchainStatus::Pending(
                starknet::core::types::Felt::cairo_deserialize(__felts, __offset + 1)?,
            )),
            4usize => Ok(MessageToAppchainStatus::Cancelling),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "MessageToAppchainStatus"
                )))
            }
        }
    }
}
#[derive()]
pub enum MessageToStarknetStatus {
    NothingToConsume,
    ReadyToConsume(starknet::core::types::Felt),
}
impl cainome::cairo_serde::CairoSerde for MessageToStarknetStatus {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            MessageToStarknetStatus::NothingToConsume => 1,
            MessageToStarknetStatus::ReadyToConsume(val) => {
                starknet::core::types::Felt::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            MessageToStarknetStatus::NothingToConsume => usize::cairo_serialize(&0usize),
            MessageToStarknetStatus::ReadyToConsume(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(starknet::core::types::Felt::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(MessageToStarknetStatus::NothingToConsume),
            1usize => Ok(MessageToStarknetStatus::ReadyToConsume(
                starknet::core::types::Felt::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "MessageToStarknetStatus"
                )))
            }
        }
    }
}
#[derive()]
pub enum MessagingEvent {
    MessageSent(MessageSent),
    MessageConsumed(MessageConsumed),
    MessageCancellationStarted(MessageCancellationStarted),
    MessageCanceled(MessageCanceled),
    MessageToStarknetReceived(MessageToStarknetReceived),
    MessageToAppchainSealed(MessageToAppchainSealed),
}
impl cainome::cairo_serde::CairoSerde for MessagingEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            MessagingEvent::MessageSent(val) => MessageSent::cairo_serialized_size(val) + 1,
            MessagingEvent::MessageConsumed(val) => MessageConsumed::cairo_serialized_size(val) + 1,
            MessagingEvent::MessageCancellationStarted(val) => {
                MessageCancellationStarted::cairo_serialized_size(val) + 1
            }
            MessagingEvent::MessageCanceled(val) => MessageCanceled::cairo_serialized_size(val) + 1,
            MessagingEvent::MessageToStarknetReceived(val) => {
                MessageToStarknetReceived::cairo_serialized_size(val) + 1
            }
            MessagingEvent::MessageToAppchainSealed(val) => {
                MessageToAppchainSealed::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            MessagingEvent::MessageSent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(MessageSent::cairo_serialize(val));
                temp
            }
            MessagingEvent::MessageConsumed(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(MessageConsumed::cairo_serialize(val));
                temp
            }
            MessagingEvent::MessageCancellationStarted(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(MessageCancellationStarted::cairo_serialize(val));
                temp
            }
            MessagingEvent::MessageCanceled(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(MessageCanceled::cairo_serialize(val));
                temp
            }
            MessagingEvent::MessageToStarknetReceived(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&4usize));
                temp.extend(MessageToStarknetReceived::cairo_serialize(val));
                temp
            }
            MessagingEvent::MessageToAppchainSealed(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&5usize));
                temp.extend(MessageToAppchainSealed::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(MessagingEvent::MessageSent(MessageSent::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            1usize => Ok(MessagingEvent::MessageConsumed(
                MessageConsumed::cairo_deserialize(__felts, __offset + 1)?,
            )),
            2usize => Ok(MessagingEvent::MessageCancellationStarted(
                MessageCancellationStarted::cairo_deserialize(__felts, __offset + 1)?,
            )),
            3usize => Ok(MessagingEvent::MessageCanceled(
                MessageCanceled::cairo_deserialize(__felts, __offset + 1)?,
            )),
            4usize => Ok(MessagingEvent::MessageToStarknetReceived(
                MessageToStarknetReceived::cairo_deserialize(__felts, __offset + 1)?,
            )),
            5usize => Ok(MessagingEvent::MessageToAppchainSealed(
                MessageToAppchainSealed::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "MessagingEvent"
                )))
            }
        }
    }
}
impl TryFrom<&starknet::core::types::EmittedEvent> for MessagingEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageSent")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageSent"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageSent", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageSent", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageSent", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageSent", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageSent", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageSent", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(MessagingEvent::MessageSent(MessageSent {
                message_hash,
                from,
                to,
                selector,
                nonce,
                payload,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageConsumed")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageConsumed"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageConsumed", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageConsumed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageConsumed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageConsumed", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(MessagingEvent::MessageConsumed(MessageConsumed {
                message_hash,
                from,
                to,
                payload,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageCancellationStarted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageCancellationStarted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageCancellationStarted", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageCancellationStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageCancellationStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageCancellationStarted", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageCancellationStarted", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageCancellationStarted", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            return Ok(MessagingEvent::MessageCancellationStarted(
                MessageCancellationStarted {
                    message_hash,
                    from,
                    to,
                    selector,
                    payload,
                    nonce,
                },
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageCanceled")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageCanceled"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageCanceled", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageCanceled", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageCanceled", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageCanceled", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageCanceled", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageCanceled", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            return Ok(MessagingEvent::MessageCanceled(MessageCanceled {
                message_hash,
                from,
                to,
                selector,
                payload,
                nonce,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageToStarknetReceived")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageToStarknetReceived"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageToStarknetReceived", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageToStarknetReceived", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageToStarknetReceived", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageToStarknetReceived", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(MessagingEvent::MessageToStarknetReceived(
                MessageToStarknetReceived {
                    message_hash,
                    from,
                    to,
                    payload,
                },
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageToAppchainSealed")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageToAppchainSealed"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageToAppchainSealed", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageToAppchainSealed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageToAppchainSealed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageToAppchainSealed", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageToAppchainSealed", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageToAppchainSealed", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(MessagingEvent::MessageToAppchainSealed(
                MessageToAppchainSealed {
                    message_hash,
                    from,
                    to,
                    selector,
                    nonce,
                    payload,
                },
            ));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl TryFrom<&starknet::core::types::Event> for MessagingEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::Event) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageSent")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageSent"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageSent", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageSent", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageSent", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageSent", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageSent", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageSent", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(MessagingEvent::MessageSent(MessageSent {
                message_hash,
                from,
                to,
                selector,
                nonce,
                payload,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageConsumed")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageConsumed"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageConsumed", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageConsumed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageConsumed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageConsumed", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(MessagingEvent::MessageConsumed(MessageConsumed {
                message_hash,
                from,
                to,
                payload,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageCancellationStarted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageCancellationStarted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageCancellationStarted", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageCancellationStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageCancellationStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageCancellationStarted", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageCancellationStarted", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageCancellationStarted", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            return Ok(MessagingEvent::MessageCancellationStarted(
                MessageCancellationStarted {
                    message_hash,
                    from,
                    to,
                    selector,
                    payload,
                    nonce,
                },
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageCanceled")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageCanceled"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageCanceled", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageCanceled", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageCanceled", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageCanceled", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageCanceled", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageCanceled", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            return Ok(MessagingEvent::MessageCanceled(MessageCanceled {
                message_hash,
                from,
                to,
                selector,
                payload,
                nonce,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageToStarknetReceived")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageToStarknetReceived"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageToStarknetReceived", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageToStarknetReceived", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageToStarknetReceived", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageToStarknetReceived", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(MessagingEvent::MessageToStarknetReceived(
                MessageToStarknetReceived {
                    message_hash,
                    from,
                    to,
                    payload,
                },
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MessageToAppchainSealed")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MessageToAppchainSealed"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let message_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "message_hash", "MessageToAppchainSealed", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&message_hash);
            let from = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "from", "MessageToAppchainSealed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&from);
            let to = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "to", "MessageToAppchainSealed", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "MessageToAppchainSealed", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let nonce =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "nonce", "MessageToAppchainSealed", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
            let payload = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "payload", "MessageToAppchainSealed", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&payload);
            return Ok(MessagingEvent::MessageToAppchainSealed(
                MessageToAppchainSealed {
                    message_hash,
                    from,
                    to,
                    selector,
                    nonce,
                    payload,
                },
            ));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive()]
pub enum OwnableEvent {
    OwnershipTransferred(OwnershipTransferred),
    OwnershipTransferStarted(OwnershipTransferStarted),
}
impl cainome::cairo_serde::CairoSerde for OwnableEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            OwnableEvent::OwnershipTransferred(val) => {
                OwnershipTransferred::cairo_serialized_size(val) + 1
            }
            OwnableEvent::OwnershipTransferStarted(val) => {
                OwnershipTransferStarted::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            OwnableEvent::OwnershipTransferred(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(OwnershipTransferred::cairo_serialize(val));
                temp
            }
            OwnableEvent::OwnershipTransferStarted(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(OwnershipTransferStarted::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(OwnableEvent::OwnershipTransferred(
                OwnershipTransferred::cairo_deserialize(__felts, __offset + 1)?,
            )),
            1usize => Ok(OwnableEvent::OwnershipTransferStarted(
                OwnershipTransferStarted::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "OwnableEvent"
                )))
            }
        }
    }
}
impl TryFrom<&starknet::core::types::EmittedEvent> for OwnableEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferred")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnershipTransferred"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "previous_owner", "OwnershipTransferred", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_owner", "OwnershipTransferred", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
            return Ok(OwnableEvent::OwnershipTransferred(OwnershipTransferred {
                previous_owner,
                new_owner,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferStarted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnershipTransferStarted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "previous_owner", "OwnershipTransferStarted", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_owner", "OwnershipTransferStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
            return Ok(OwnableEvent::OwnershipTransferStarted(
                OwnershipTransferStarted {
                    previous_owner,
                    new_owner,
                },
            ));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl TryFrom<&starknet::core::types::Event> for OwnableEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::Event) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferred")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnershipTransferred"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "previous_owner", "OwnershipTransferred", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_owner", "OwnershipTransferred", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
            return Ok(OwnableEvent::OwnershipTransferred(OwnershipTransferred {
                previous_owner,
                new_owner,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferStarted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnershipTransferStarted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "previous_owner", "OwnershipTransferStarted", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&previous_owner);
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_owner", "OwnershipTransferStarted", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
            return Ok(OwnableEvent::OwnershipTransferStarted(
                OwnershipTransferStarted {
                    previous_owner,
                    new_owner,
                },
            ));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive()]
pub enum ReentrancyguardEvent {}
impl cainome::cairo_serde::CairoSerde for ReentrancyguardEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "ReentrancyguardEvent"
                )))
            }
        }
    }
}
impl TryFrom<&starknet::core::types::EmittedEvent> for ReentrancyguardEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl TryFrom<&starknet::core::types::Event> for ReentrancyguardEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::Event) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive()]
pub enum StateEvent {}
impl cainome::cairo_serde::CairoSerde for StateEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "StateEvent"
                )))
            }
        }
    }
}
impl TryFrom<&starknet::core::types::EmittedEvent> for StateEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl TryFrom<&starknet::core::types::Event> for StateEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::Event) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive()]
pub enum UpgradeableEvent {
    Upgraded(Upgraded),
}
impl cainome::cairo_serde::CairoSerde for UpgradeableEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            UpgradeableEvent::Upgraded(val) => Upgraded::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            UpgradeableEvent::Upgraded(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(Upgraded::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(UpgradeableEvent::Upgraded(Upgraded::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "UpgradeableEvent"
                )))
            }
        }
    }
}
impl TryFrom<&starknet::core::types::EmittedEvent> for UpgradeableEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("Upgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "Upgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "Upgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(UpgradeableEvent::Upgraded(Upgraded { class_hash }));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl TryFrom<&starknet::core::types::Event> for UpgradeableEvent {
    type Error = String;
    fn try_from(event: &starknet::core::types::Event) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("Upgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "Upgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "Upgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(UpgradeableEvent::Upgraded(Upgraded { class_hash }));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl<A: starknet::accounts::ConnectedAccount + Sync> AppchainContract<A> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn appchain_to_sn_messages(
        &self,
        message_hash: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, MessageToStarknetStatus> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(message_hash));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("appchain_to_sn_messages"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_facts_registry(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, cainome::cairo_serde::ContractAddress> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_facts_registry"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_program_info(&self) -> cainome::cairo_serde::call::FCall<A::Provider, ProgramInfo> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_program_info"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_state(
        &self,
    ) -> cainome::cairo_serde::call::FCall<
        A::Provider,
        (
            starknet::core::types::Felt,
            starknet::core::types::Felt,
            starknet::core::types::Felt,
        ),
    > {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_state"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_operator(
        &self,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_operator"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn owner(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, cainome::cairo_serde::ContractAddress> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn pending_owner(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, cainome::cairo_serde::ContractAddress> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("pending_owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn sn_to_appchain_messages(
        &self,
        message_hash: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, MessageToAppchainStatus> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(message_hash));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("sn_to_appchain_messages"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn accept_ownership_getcall(&self) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("accept_ownership"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn accept_ownership(&self) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("accept_ownership"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn cancel_message_getcall(
        &self,
        to_address: &cainome::cairo_serde::ContractAddress,
        selector: &starknet::core::types::Felt,
        payload: &Vec<starknet::core::types::Felt>,
        nonce: &starknet::core::types::Felt,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            to_address,
        ));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(payload));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(nonce));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("cancel_message"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn cancel_message(
        &self,
        to_address: &cainome::cairo_serde::ContractAddress,
        selector: &starknet::core::types::Felt,
        payload: &Vec<starknet::core::types::Felt>,
        nonce: &starknet::core::types::Felt,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            to_address,
        ));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(payload));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(nonce));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("cancel_message"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn consume_message_from_appchain_getcall(
        &self,
        from_address: &cainome::cairo_serde::ContractAddress,
        payload: &Vec<starknet::core::types::Felt>,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            from_address,
        ));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(payload));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("consume_message_from_appchain"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn consume_message_from_appchain(
        &self,
        from_address: &cainome::cairo_serde::ContractAddress,
        payload: &Vec<starknet::core::types::Felt>,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            from_address,
        ));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(payload));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("consume_message_from_appchain"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_operator_getcall(
        &self,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_operator"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_operator(
        &self,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_operator"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn renounce_ownership_getcall(&self) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("renounce_ownership"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn renounce_ownership(&self) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("renounce_ownership"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn send_message_to_appchain_getcall(
        &self,
        to_address: &cainome::cairo_serde::ContractAddress,
        selector: &starknet::core::types::Felt,
        payload: &Vec<starknet::core::types::Felt>,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            to_address,
        ));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(payload));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("send_message_to_appchain"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn send_message_to_appchain(
        &self,
        to_address: &cainome::cairo_serde::ContractAddress,
        selector: &starknet::core::types::Felt,
        payload: &Vec<starknet::core::types::Felt>,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            to_address,
        ));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(payload));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("send_message_to_appchain"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_facts_registry_getcall(
        &self,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_facts_registry"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_facts_registry(
        &self,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_facts_registry"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_program_info_getcall(
        &self,
        program_info: &ProgramInfo,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(ProgramInfo::cairo_serialize(program_info));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_program_info"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_program_info(
        &self,
        program_info: &ProgramInfo,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(ProgramInfo::cairo_serialize(program_info));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_program_info"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn start_message_cancellation_getcall(
        &self,
        to_address: &cainome::cairo_serde::ContractAddress,
        selector: &starknet::core::types::Felt,
        payload: &Vec<starknet::core::types::Felt>,
        nonce: &starknet::core::types::Felt,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            to_address,
        ));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(payload));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(nonce));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("start_message_cancellation"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn start_message_cancellation(
        &self,
        to_address: &cainome::cairo_serde::ContractAddress,
        selector: &starknet::core::types::Felt,
        payload: &Vec<starknet::core::types::Felt>,
        nonce: &starknet::core::types::Felt,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            to_address,
        ));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(payload));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(nonce));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("start_message_cancellation"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn transfer_ownership_getcall(
        &self,
        new_owner: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            new_owner,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("transfer_ownership"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn transfer_ownership(
        &self,
        new_owner: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            new_owner,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("transfer_ownership"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn unregister_operator_getcall(
        &self,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("unregister_operator"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn unregister_operator(
        &self,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("unregister_operator"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn update_state_getcall(
        &self,
        snos_output: &Vec<starknet::core::types::Felt>,
        layout_bridge_output: &Vec<starknet::core::types::Felt>,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            snos_output,
        ));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            layout_bridge_output,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("update_state"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn update_state(
        &self,
        snos_output: &Vec<starknet::core::types::Felt>,
        layout_bridge_output: &Vec<starknet::core::types::Felt>,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            snos_output,
        ));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            layout_bridge_output,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("update_state"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade_getcall(
        &self,
        new_class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            new_class_hash,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade(
        &self,
        new_class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            new_class_hash,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
}
impl<P: starknet::providers::Provider + Sync> AppchainContractReader<P> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn appchain_to_sn_messages(
        &self,
        message_hash: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<P, MessageToStarknetStatus> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(message_hash));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("appchain_to_sn_messages"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_facts_registry(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::ContractAddress> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_facts_registry"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_program_info(&self) -> cainome::cairo_serde::call::FCall<P, ProgramInfo> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_program_info"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_state(
        &self,
    ) -> cainome::cairo_serde::call::FCall<
        P,
        (
            starknet::core::types::Felt,
            starknet::core::types::Felt,
            starknet::core::types::Felt,
        ),
    > {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_state"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_operator(
        &self,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_operator"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn owner(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::ContractAddress> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn pending_owner(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::ContractAddress> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("pending_owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn sn_to_appchain_messages(
        &self,
        message_hash: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<P, MessageToAppchainStatus> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(message_hash));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("sn_to_appchain_messages"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
}
