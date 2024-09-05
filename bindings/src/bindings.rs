#[derive(Debug)] pub struct AppChain < A : starknet :: accounts ::
ConnectedAccount + Sync >
{ pub address : starknet :: core :: types :: FieldElement, pub account : A, }
impl < A : starknet :: accounts :: ConnectedAccount + Sync > AppChain < A >
{
    pub fn
    new(address : starknet :: core :: types :: FieldElement, account : A) ->
    Self { Self { address, account } } pub fn
    set_contract_address(mut self, address : starknet :: core :: types ::
    FieldElement) { self.address = address; } pub fn provider(& self) -> & A
    :: Provider { self.account.provider() }
} #[derive(Debug)] pub struct AppChainReader < P : starknet :: providers ::
Provider + Sync >
{
    pub address : starknet :: core :: types :: FieldElement, pub provider : P,
    pub block_id : starknet :: core :: types :: BlockId,
} impl < P : starknet :: providers :: Provider + Sync > AppChainReader < P >
{
    pub fn
    new(address : starknet :: core :: types :: FieldElement, provider : P,) ->
    Self
    {
        Self
        {
            address, provider, block_id : starknet :: core :: types :: BlockId
            :: Tag(starknet :: core :: types :: BlockTag :: Pending)
        }
    } pub fn
    set_contract_address(mut self, address : starknet :: core :: types ::
    FieldElement) { self.address = address; } pub fn provider(& self) -> & P
    { & self.provider } pub fn
    with_block(self, block_id : starknet :: core :: types :: BlockId) -> Self
    { Self { block_id, .. self } }
} #[derive(Debug, PartialEq, Clone)] pub struct U256
{ pub low : u128, pub high : u128 } impl cainome :: cairo_serde :: CairoSerde
for U256
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += u128 ::
        cairo_serialized_size(& __rust.low); __size += u128 ::
        cairo_serialized_size(& __rust.high); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! []; __out.extend(u128 :: cairo_serialize(& __rust.low));
        __out.extend(u128 :: cairo_serialize(& __rust.high)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let low = u128 ::
        cairo_deserialize(__felts, __offset) ? ; __offset += u128 ::
        cairo_serialized_size(& low); let high = u128 ::
        cairo_deserialize(__felts, __offset) ? ; __offset += u128 ::
        cairo_serialized_size(& high); Ok(U256 { low, high })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct MessageToStarknetReceived
{
    pub message_hash : starknet :: core :: types :: FieldElement, pub from :
    cainome :: cairo_serde :: ContractAddress, pub to : cainome :: cairo_serde
    :: ContractAddress, pub payload : Vec < starknet :: core :: types ::
    FieldElement >
} impl cainome :: cairo_serde :: CairoSerde for MessageToStarknetReceived
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += starknet :: core :: types ::
        FieldElement :: cairo_serialized_size(& __rust.message_hash); __size
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.from); __size += cainome :: cairo_serde
        :: ContractAddress :: cairo_serialized_size(& __rust.to); __size +=
        Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialized_size(& __rust.payload); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.message_hash));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.from));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.to));
        __out.extend(Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialize(& __rust.payload)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message_hash = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& message_hash); let from = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& from); let to = cainome :: cairo_serde ::
        ContractAddress :: cairo_deserialize(__felts, __offset) ? ; __offset
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& to); let payload = Vec :: < starknet :: core
        :: types :: FieldElement > :: cairo_deserialize(__felts, __offset) ? ;
        __offset += Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialized_size(& payload);
        Ok(MessageToStarknetReceived { message_hash, from, to, payload })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct ProgramInfoChanged
{
    pub changed_by : cainome :: cairo_serde :: ContractAddress, pub
    old_program_hash : starknet :: core :: types :: FieldElement, pub
    new_program_hash : starknet :: core :: types :: FieldElement, pub
    old_config_hash : starknet :: core :: types :: FieldElement, pub
    new_config_hash : starknet :: core :: types :: FieldElement
} impl cainome :: cairo_serde :: CairoSerde for ProgramInfoChanged
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde ::
        ContractAddress :: cairo_serialized_size(& __rust.changed_by); __size
        += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.old_program_hash); __size += starknet
        :: core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.new_program_hash); __size += starknet
        :: core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.old_config_hash); __size += starknet ::
        core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.new_config_hash); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.changed_by));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.old_program_hash));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.new_program_hash));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.old_config_hash));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.new_config_hash)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let changed_by = cainome :: cairo_serde
        :: ContractAddress :: cairo_deserialize(__felts, __offset) ? ;
        __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& changed_by); let old_program_hash = starknet
        :: core :: types :: FieldElement ::
        cairo_deserialize(__felts, __offset) ? ; __offset += starknet :: core
        :: types :: FieldElement :: cairo_serialized_size(& old_program_hash);
        let new_program_hash = starknet :: core :: types :: FieldElement ::
        cairo_deserialize(__felts, __offset) ? ; __offset += starknet :: core
        :: types :: FieldElement :: cairo_serialized_size(& new_program_hash);
        let old_config_hash = starknet :: core :: types :: FieldElement ::
        cairo_deserialize(__felts, __offset) ? ; __offset += starknet :: core
        :: types :: FieldElement :: cairo_serialized_size(& old_config_hash);
        let new_config_hash = starknet :: core :: types :: FieldElement ::
        cairo_deserialize(__felts, __offset) ? ; __offset += starknet :: core
        :: types :: FieldElement :: cairo_serialized_size(& new_config_hash);
        Ok(ProgramInfoChanged
        {
            changed_by, old_program_hash, new_program_hash, old_config_hash,
            new_config_hash
        })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct MessageCanceled
{
    pub message_hash : starknet :: core :: types :: FieldElement, pub from :
    cainome :: cairo_serde :: ContractAddress, pub to : cainome :: cairo_serde
    :: ContractAddress, pub selector : starknet :: core :: types ::
    FieldElement, pub payload : Vec < starknet :: core :: types ::
    FieldElement > , pub nonce : starknet :: core :: types :: FieldElement
} impl cainome :: cairo_serde :: CairoSerde for MessageCanceled
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += starknet :: core :: types ::
        FieldElement :: cairo_serialized_size(& __rust.message_hash); __size
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.from); __size += cainome :: cairo_serde
        :: ContractAddress :: cairo_serialized_size(& __rust.to); __size +=
        starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.selector); __size += Vec :: < starknet
        :: core :: types :: FieldElement > ::
        cairo_serialized_size(& __rust.payload); __size += starknet :: core ::
        types :: FieldElement :: cairo_serialized_size(& __rust.nonce); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.message_hash));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.from));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.to));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.selector));
        __out.extend(Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialize(& __rust.payload));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.nonce)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message_hash = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& message_hash); let from = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& from); let to = cainome :: cairo_serde ::
        ContractAddress :: cairo_deserialize(__felts, __offset) ? ; __offset
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& to); let selector = starknet :: core :: types
        :: FieldElement :: cairo_deserialize(__felts, __offset) ? ; __offset
        += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& selector); let payload = Vec :: < starknet ::
        core :: types :: FieldElement > ::
        cairo_deserialize(__felts, __offset) ? ; __offset += Vec :: < starknet
        :: core :: types :: FieldElement > ::
        cairo_serialized_size(& payload); let nonce = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& nonce);
        Ok(MessageCanceled
        { message_hash, from, to, selector, payload, nonce })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct OwnershipTransferred
{
    pub previous_owner : cainome :: cairo_serde :: ContractAddress, pub
    new_owner : cainome :: cairo_serde :: ContractAddress
} impl cainome :: cairo_serde :: CairoSerde for OwnershipTransferred
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde ::
        ContractAddress :: cairo_serialized_size(& __rust.previous_owner);
        __size += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.new_owner); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.previous_owner));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.new_owner)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let previous_owner = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& previous_owner); let new_owner = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& new_owner);
        Ok(OwnershipTransferred { previous_owner, new_owner })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct MessageCancellationStarted
{
    pub message_hash : starknet :: core :: types :: FieldElement, pub from :
    cainome :: cairo_serde :: ContractAddress, pub to : cainome :: cairo_serde
    :: ContractAddress, pub selector : starknet :: core :: types ::
    FieldElement, pub payload : Vec < starknet :: core :: types ::
    FieldElement > , pub nonce : starknet :: core :: types :: FieldElement
} impl cainome :: cairo_serde :: CairoSerde for MessageCancellationStarted
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += starknet :: core :: types ::
        FieldElement :: cairo_serialized_size(& __rust.message_hash); __size
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.from); __size += cainome :: cairo_serde
        :: ContractAddress :: cairo_serialized_size(& __rust.to); __size +=
        starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.selector); __size += Vec :: < starknet
        :: core :: types :: FieldElement > ::
        cairo_serialized_size(& __rust.payload); __size += starknet :: core ::
        types :: FieldElement :: cairo_serialized_size(& __rust.nonce); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.message_hash));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.from));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.to));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.selector));
        __out.extend(Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialize(& __rust.payload));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.nonce)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message_hash = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& message_hash); let from = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& from); let to = cainome :: cairo_serde ::
        ContractAddress :: cairo_deserialize(__felts, __offset) ? ; __offset
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& to); let selector = starknet :: core :: types
        :: FieldElement :: cairo_deserialize(__felts, __offset) ? ; __offset
        += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& selector); let payload = Vec :: < starknet ::
        core :: types :: FieldElement > ::
        cairo_deserialize(__felts, __offset) ? ; __offset += Vec :: < starknet
        :: core :: types :: FieldElement > ::
        cairo_serialized_size(& payload); let nonce = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& nonce);
        Ok(MessageCancellationStarted
        { message_hash, from, to, selector, payload, nonce })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct LogStateTransitionFact
{ pub state_transition_fact : U256 } impl cainome :: cairo_serde :: CairoSerde
for LogStateTransitionFact
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += U256 ::
        cairo_serialized_size(& __rust.state_transition_fact); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(U256 :: cairo_serialize(& __rust.state_transition_fact));
        __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let state_transition_fact = U256 ::
        cairo_deserialize(__felts, __offset) ? ; __offset += U256 ::
        cairo_serialized_size(& state_transition_fact);
        Ok(LogStateTransitionFact { state_transition_fact })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct MessageConsumed
{
    pub message_hash : starknet :: core :: types :: FieldElement, pub from :
    cainome :: cairo_serde :: ContractAddress, pub to : cainome :: cairo_serde
    :: ContractAddress, pub payload : Vec < starknet :: core :: types ::
    FieldElement >
} impl cainome :: cairo_serde :: CairoSerde for MessageConsumed
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += starknet :: core :: types ::
        FieldElement :: cairo_serialized_size(& __rust.message_hash); __size
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.from); __size += cainome :: cairo_serde
        :: ContractAddress :: cairo_serialized_size(& __rust.to); __size +=
        Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialized_size(& __rust.payload); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.message_hash));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.from));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.to));
        __out.extend(Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialize(& __rust.payload)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message_hash = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& message_hash); let from = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& from); let to = cainome :: cairo_serde ::
        ContractAddress :: cairo_deserialize(__felts, __offset) ? ; __offset
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& to); let payload = Vec :: < starknet :: core
        :: types :: FieldElement > :: cairo_deserialize(__felts, __offset) ? ;
        __offset += Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialized_size(& payload);
        Ok(MessageConsumed { message_hash, from, to, payload })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct LogStateUpdate
{
    pub state_root : starknet :: core :: types :: FieldElement, pub
    block_number : starknet :: core :: types :: FieldElement, pub block_hash :
    starknet :: core :: types :: FieldElement
} impl cainome :: cairo_serde :: CairoSerde for LogStateUpdate
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += starknet :: core :: types ::
        FieldElement :: cairo_serialized_size(& __rust.state_root); __size +=
        starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.block_number); __size += starknet ::
        core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.block_hash); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.state_root));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.block_number));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.block_hash)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let state_root = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& state_root); let block_number = starknet ::
        core :: types :: FieldElement :: cairo_deserialize(__felts, __offset)
        ? ; __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& block_number); let block_hash = starknet ::
        core :: types :: FieldElement :: cairo_deserialize(__felts, __offset)
        ? ; __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& block_hash);
        Ok(LogStateUpdate { state_root, block_number, block_hash })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct MessageSent
{
    pub message_hash : starknet :: core :: types :: FieldElement, pub from :
    cainome :: cairo_serde :: ContractAddress, pub to : cainome :: cairo_serde
    :: ContractAddress, pub selector : starknet :: core :: types ::
    FieldElement, pub nonce : starknet :: core :: types :: FieldElement, pub
    payload : Vec < starknet :: core :: types :: FieldElement >
} impl cainome :: cairo_serde :: CairoSerde for MessageSent
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += starknet :: core :: types ::
        FieldElement :: cairo_serialized_size(& __rust.message_hash); __size
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.from); __size += cainome :: cairo_serde
        :: ContractAddress :: cairo_serialized_size(& __rust.to); __size +=
        starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.selector); __size += starknet :: core
        :: types :: FieldElement :: cairo_serialized_size(& __rust.nonce);
        __size += Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialized_size(& __rust.payload); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.message_hash));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.from));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.to));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.selector));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.nonce));
        __out.extend(Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialize(& __rust.payload)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message_hash = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& message_hash); let from = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& from); let to = cainome :: cairo_serde ::
        ContractAddress :: cairo_deserialize(__felts, __offset) ? ; __offset
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& to); let selector = starknet :: core :: types
        :: FieldElement :: cairo_deserialize(__felts, __offset) ? ; __offset
        += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& selector); let nonce = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& nonce); let payload = Vec :: < starknet ::
        core :: types :: FieldElement > ::
        cairo_deserialize(__felts, __offset) ? ; __offset += Vec :: < starknet
        :: core :: types :: FieldElement > ::
        cairo_serialized_size(& payload);
        Ok(MessageSent { message_hash, from, to, selector, nonce, payload })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct MessageToAppchainSealed
{
    pub message_hash : starknet :: core :: types :: FieldElement, pub from :
    cainome :: cairo_serde :: ContractAddress, pub to : cainome :: cairo_serde
    :: ContractAddress, pub selector : starknet :: core :: types ::
    FieldElement, pub nonce : starknet :: core :: types :: FieldElement, pub
    payload : Vec < starknet :: core :: types :: FieldElement >
} impl cainome :: cairo_serde :: CairoSerde for MessageToAppchainSealed
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += starknet :: core :: types ::
        FieldElement :: cairo_serialized_size(& __rust.message_hash); __size
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.from); __size += cainome :: cairo_serde
        :: ContractAddress :: cairo_serialized_size(& __rust.to); __size +=
        starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.selector); __size += starknet :: core
        :: types :: FieldElement :: cairo_serialized_size(& __rust.nonce);
        __size += Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialized_size(& __rust.payload); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.message_hash));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.from));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.to));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.selector));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.nonce));
        __out.extend(Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialize(& __rust.payload)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message_hash = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& message_hash); let from = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& from); let to = cainome :: cairo_serde ::
        ContractAddress :: cairo_deserialize(__felts, __offset) ? ; __offset
        += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& to); let selector = starknet :: core :: types
        :: FieldElement :: cairo_deserialize(__felts, __offset) ? ; __offset
        += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& selector); let nonce = starknet :: core ::
        types :: FieldElement :: cairo_deserialize(__felts, __offset) ? ;
        __offset += starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& nonce); let payload = Vec :: < starknet ::
        core :: types :: FieldElement > ::
        cairo_deserialize(__felts, __offset) ? ; __offset += Vec :: < starknet
        :: core :: types :: FieldElement > ::
        cairo_serialized_size(& payload);
        Ok(MessageToAppchainSealed
        { message_hash, from, to, selector, nonce, payload })
    }
} #[derive(Debug, PartialEq, Clone)] pub struct OwnershipTransferStarted
{
    pub previous_owner : cainome :: cairo_serde :: ContractAddress, pub
    new_owner : cainome :: cairo_serde :: ContractAddress
} impl cainome :: cairo_serde :: CairoSerde for OwnershipTransferStarted
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde ::
        ContractAddress :: cairo_serialized_size(& __rust.previous_owner);
        __size += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.new_owner); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.previous_owner));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.new_owner)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let previous_owner = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& previous_owner); let new_owner = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& new_owner);
        Ok(OwnershipTransferStarted { previous_owner, new_owner })
    }
} #[derive(Debug, PartialEq, Clone)] pub enum Event
{
    MessageSent(MessageSent), MessageConsumed(MessageConsumed),
    MessageCancellationStarted(MessageCancellationStarted),
    MessageCanceled(MessageCanceled),
    MessageToStarknetReceived(MessageToStarknetReceived),
    MessageToAppchainSealed(MessageToAppchainSealed)
} impl cainome :: cairo_serde :: CairoSerde for Event
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        match __rust
        {
            Event :: MessageSent(val) => MessageSent ::
            cairo_serialized_size(val) + 1, Event :: MessageConsumed(val) =>
            MessageConsumed :: cairo_serialized_size(val) + 1, Event ::
            MessageCancellationStarted(val) => MessageCancellationStarted ::
            cairo_serialized_size(val) + 1, Event :: MessageCanceled(val) =>
            MessageCanceled :: cairo_serialized_size(val) + 1, Event ::
            MessageToStarknetReceived(val) => MessageToStarknetReceived ::
            cairo_serialized_size(val) + 1, Event ::
            MessageToAppchainSealed(val) => MessageToAppchainSealed ::
            cairo_serialized_size(val) + 1, _ => 0
        }
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        match __rust
        {
            Event :: MessageSent(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 0usize));
                temp.extend(MessageSent :: cairo_serialize(val)); temp
            }, Event :: MessageConsumed(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 1usize));
                temp.extend(MessageConsumed :: cairo_serialize(val)); temp
            }, Event :: MessageCancellationStarted(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 2usize));
                temp.extend(MessageCancellationStarted ::
                cairo_serialize(val)); temp
            }, Event :: MessageCanceled(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 3usize));
                temp.extend(MessageCanceled :: cairo_serialize(val)); temp
            }, Event :: MessageToStarknetReceived(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 4usize));
                temp.extend(MessageToStarknetReceived ::
                cairo_serialize(val)); temp
            }, Event :: MessageToAppchainSealed(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 5usize));
                temp.extend(MessageToAppchainSealed :: cairo_serialize(val));
                temp
            }, _ => vec! []
        }
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            0usize =>
            Ok(Event ::
            MessageSent(MessageSent ::
            cairo_deserialize(__felts, __offset + 1) ?)), 1usize =>
            Ok(Event ::
            MessageConsumed(MessageConsumed ::
            cairo_deserialize(__felts, __offset + 1) ?)), 2usize =>
            Ok(Event ::
            MessageCancellationStarted(MessageCancellationStarted ::
            cairo_deserialize(__felts, __offset + 1) ?)), 3usize =>
            Ok(Event ::
            MessageCanceled(MessageCanceled ::
            cairo_deserialize(__felts, __offset + 1) ?)), 4usize =>
            Ok(Event ::
            MessageToStarknetReceived(MessageToStarknetReceived ::
            cairo_deserialize(__felts, __offset + 1) ?)), 5usize =>
            Ok(Event ::
            MessageToAppchainSealed(MessageToAppchainSealed ::
            cairo_deserialize(__felts, __offset + 1) ?)), _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format! ("Index not handle for enum {}", "Event")))
        }
    }
} impl TryFrom < starknet :: core :: types :: EmittedEvent > for Event
{
    type Error = String; fn
    try_from(event : starknet :: core :: types :: EmittedEvent) -> Result <
    Self, Self :: Error >
    {
        use cainome :: cairo_serde :: CairoSerde; if event.keys.is_empty()
        { return Err("Event has no key".to_string()); } let selector =
        event.keys [0]; if selector == starknet :: core :: utils ::
        get_selector_from_name("MessageSent").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "MessageSent"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageSent", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageSent", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageSent", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let selector = match starknet :: core
            :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "selector",
                "MessageSent", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& selector); let nonce = match starknet ::
            core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "nonce",
                "MessageSent", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& nonce); let payload = match Vec :: <
            starknet :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageSent", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); return
            Ok(Event ::
            MessageSent(MessageSent
            { message_hash, from, to, selector, nonce, payload }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageConsumed").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "MessageConsumed"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageConsumed", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageConsumed", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageConsumed", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let payload = match Vec :: < starknet
            :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageConsumed", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); return
            Ok(Event ::
            MessageConsumed(MessageConsumed
            { message_hash, from, to, payload }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageCancellationStarted").unwrap_or_else(|
        _ | panic! ("Invalid selector for {}", "MessageCancellationStarted"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageCancellationStarted", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageCancellationStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageCancellationStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let selector = match starknet :: core
            :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "selector",
                "MessageCancellationStarted", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& selector); let payload = match Vec :: <
            starknet :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageCancellationStarted", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); let nonce =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "nonce",
                "MessageCancellationStarted", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& nonce); return
            Ok(Event ::
            MessageCancellationStarted(MessageCancellationStarted
            { message_hash, from, to, selector, payload, nonce }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageCanceled").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "MessageCanceled"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageCanceled", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageCanceled", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageCanceled", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let selector = match starknet :: core
            :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "selector",
                "MessageCanceled", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& selector); let payload = match Vec :: <
            starknet :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageCanceled", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); let nonce =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "nonce",
                "MessageCanceled", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& nonce); return
            Ok(Event ::
            MessageCanceled(MessageCanceled
            { message_hash, from, to, selector, payload, nonce }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageToStarknetReceived").unwrap_or_else(| _
        | panic! ("Invalid selector for {}", "MessageToStarknetReceived"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageToStarknetReceived", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageToStarknetReceived", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageToStarknetReceived", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let payload = match Vec :: < starknet
            :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageToStarknetReceived", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); return
            Ok(Event ::
            MessageToStarknetReceived(MessageToStarknetReceived
            { message_hash, from, to, payload }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageToAppchainSealed").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "MessageToAppchainSealed"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageToAppchainSealed", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageToAppchainSealed", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageToAppchainSealed", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let selector = match starknet :: core
            :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "selector",
                "MessageToAppchainSealed", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& selector); let nonce = match starknet ::
            core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "nonce",
                "MessageToAppchainSealed", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& nonce); let payload = match Vec :: <
            starknet :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageToAppchainSealed", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); return
            Ok(Event ::
            MessageToAppchainSealed(MessageToAppchainSealed
            { message_hash, from, to, selector, nonce, payload }))
        };
        Err(format! ("Could not match any event from keys {:?}", event.keys))
    }
} #[derive(Debug, PartialEq, Clone)] pub enum Event
{
    OwnableEvent(Event), ConfigEvent(Event), MessagingEvent(Event),
    ReentrancyGuardEvent(Event), StateEvent(Event),
    LogStateUpdate(LogStateUpdate),
    LogStateTransitionFact(LogStateTransitionFact)
} impl cainome :: cairo_serde :: CairoSerde for Event
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        match __rust
        {
            Event :: OwnableEvent(val) => Event :: cairo_serialized_size(val)
            + 1, Event :: ConfigEvent(val) => Event ::
            cairo_serialized_size(val) + 1, Event :: MessagingEvent(val) =>
            Event :: cairo_serialized_size(val) + 1, Event ::
            ReentrancyGuardEvent(val) => Event :: cairo_serialized_size(val) +
            1, Event :: StateEvent(val) => Event :: cairo_serialized_size(val)
            + 1, Event :: LogStateUpdate(val) => LogStateUpdate ::
            cairo_serialized_size(val) + 1, Event ::
            LogStateTransitionFact(val) => LogStateTransitionFact ::
            cairo_serialized_size(val) + 1, _ => 0
        }
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        match __rust
        {
            Event :: OwnableEvent(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 0usize));
                temp.extend(Event :: cairo_serialize(val)); temp
            }, Event :: ConfigEvent(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 1usize));
                temp.extend(Event :: cairo_serialize(val)); temp
            }, Event :: MessagingEvent(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 2usize));
                temp.extend(Event :: cairo_serialize(val)); temp
            }, Event :: ReentrancyGuardEvent(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 3usize));
                temp.extend(Event :: cairo_serialize(val)); temp
            }, Event :: StateEvent(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 4usize));
                temp.extend(Event :: cairo_serialize(val)); temp
            }, Event :: LogStateUpdate(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 5usize));
                temp.extend(LogStateUpdate :: cairo_serialize(val)); temp
            }, Event :: LogStateTransitionFact(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 6usize));
                temp.extend(LogStateTransitionFact :: cairo_serialize(val));
                temp
            }, _ => vec! []
        }
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            0usize =>
            Ok(Event ::
            OwnableEvent(Event :: cairo_deserialize(__felts, __offset + 1)
            ?)), 1usize =>
            Ok(Event ::
            ConfigEvent(Event :: cairo_deserialize(__felts, __offset + 1) ?)),
            2usize =>
            Ok(Event ::
            MessagingEvent(Event :: cairo_deserialize(__felts, __offset + 1)
            ?)), 3usize =>
            Ok(Event ::
            ReentrancyGuardEvent(Event ::
            cairo_deserialize(__felts, __offset + 1) ?)), 4usize =>
            Ok(Event ::
            StateEvent(Event :: cairo_deserialize(__felts, __offset + 1) ?)),
            5usize =>
            Ok(Event ::
            LogStateUpdate(LogStateUpdate ::
            cairo_deserialize(__felts, __offset + 1) ?)), 6usize =>
            Ok(Event ::
            LogStateTransitionFact(LogStateTransitionFact ::
            cairo_deserialize(__felts, __offset + 1) ?)), _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format! ("Index not handle for enum {}", "Event")))
        }
    }
} impl TryFrom < starknet :: core :: types :: EmittedEvent > for Event
{
    type Error = String; fn
    try_from(event : starknet :: core :: types :: EmittedEvent) -> Result <
    Self, Self :: Error >
    {
        use cainome :: cairo_serde :: CairoSerde; if event.keys.is_empty()
        { return Err("Event has no key".to_string()); } let selector =
        event.keys [0]; if selector == starknet :: core :: utils ::
        get_selector_from_name("OwnershipTransferred").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "OwnershipTransferred"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            previous_owner = match cainome :: cairo_serde :: ContractAddress
            :: cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "previous_owner", "OwnershipTransferred", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& previous_owner); let new_owner = match
            cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "new_owner",
                "OwnershipTransferred", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& new_owner); return
            Ok(Event ::
            OwnableEvent(Event ::
            OwnershipTransferred(OwnershipTransferred
            { previous_owner, new_owner })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("OwnershipTransferStarted").unwrap_or_else(| _
        | panic! ("Invalid selector for {}", "OwnershipTransferStarted"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            previous_owner = match cainome :: cairo_serde :: ContractAddress
            :: cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "previous_owner", "OwnershipTransferStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& previous_owner); let new_owner = match
            cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "new_owner",
                "OwnershipTransferStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& new_owner); return
            Ok(Event ::
            OwnableEvent(Event ::
            OwnershipTransferStarted(OwnershipTransferStarted
            { previous_owner, new_owner })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("ProgramInfoChanged").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "ProgramInfoChanged"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            changed_by = match cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "changed_by",
                "ProgramInfoChanged", e)),
            }; data_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& changed_by); let old_program_hash = match
            starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "old_program_hash", "ProgramInfoChanged", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& old_program_hash); let new_program_hash =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "new_program_hash", "ProgramInfoChanged", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& new_program_hash); let old_config_hash =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "old_config_hash", "ProgramInfoChanged", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& old_config_hash); let new_config_hash =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "new_config_hash", "ProgramInfoChanged", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& new_config_hash); return
            Ok(Event ::
            ConfigEvent(Event ::
            ProgramInfoChanged(ProgramInfoChanged
            {
                changed_by, old_program_hash, new_program_hash,
                old_config_hash, new_config_hash
            })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageSent").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "MessageSent"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageSent", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageSent", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageSent", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let selector = match starknet :: core
            :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "selector",
                "MessageSent", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& selector); let nonce = match starknet ::
            core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "nonce",
                "MessageSent", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& nonce); let payload = match Vec :: <
            starknet :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageSent", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); return
            Ok(Event ::
            MessagingEvent(Event ::
            MessageSent(MessageSent
            { message_hash, from, to, selector, nonce, payload })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageConsumed").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "MessageConsumed"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageConsumed", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageConsumed", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageConsumed", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let payload = match Vec :: < starknet
            :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageConsumed", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); return
            Ok(Event ::
            MessagingEvent(Event ::
            MessageConsumed(MessageConsumed
            { message_hash, from, to, payload })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageCancellationStarted").unwrap_or_else(|
        _ | panic! ("Invalid selector for {}", "MessageCancellationStarted"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageCancellationStarted", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageCancellationStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageCancellationStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let selector = match starknet :: core
            :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "selector",
                "MessageCancellationStarted", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& selector); let payload = match Vec :: <
            starknet :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageCancellationStarted", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); let nonce =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "nonce",
                "MessageCancellationStarted", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& nonce); return
            Ok(Event ::
            MessagingEvent(Event ::
            MessageCancellationStarted(MessageCancellationStarted
            { message_hash, from, to, selector, payload, nonce })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageCanceled").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "MessageCanceled"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageCanceled", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageCanceled", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageCanceled", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let selector = match starknet :: core
            :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "selector",
                "MessageCanceled", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& selector); let payload = match Vec :: <
            starknet :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageCanceled", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); let nonce =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "nonce",
                "MessageCanceled", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& nonce); return
            Ok(Event ::
            MessagingEvent(Event ::
            MessageCanceled(MessageCanceled
            { message_hash, from, to, selector, payload, nonce })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageToStarknetReceived").unwrap_or_else(| _
        | panic! ("Invalid selector for {}", "MessageToStarknetReceived"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageToStarknetReceived", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageToStarknetReceived", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageToStarknetReceived", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let payload = match Vec :: < starknet
            :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageToStarknetReceived", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); return
            Ok(Event ::
            MessagingEvent(Event ::
            MessageToStarknetReceived(MessageToStarknetReceived
            { message_hash, from, to, payload })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MessageToAppchainSealed").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "MessageToAppchainSealed"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            message_hash = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "message_hash", "MessageToAppchainSealed", e)),
            }; key_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& message_hash); let from = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "from",
                "MessageToAppchainSealed", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& from); let to = match cainome ::
            cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "to",
                "MessageToAppchainSealed", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& to); let selector = match starknet :: core
            :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "selector",
                "MessageToAppchainSealed", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& selector); let nonce = match starknet ::
            core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "nonce",
                "MessageToAppchainSealed", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& nonce); let payload = match Vec :: <
            starknet :: core :: types :: FieldElement > ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "payload",
                "MessageToAppchainSealed", e)),
            }; data_offset += Vec :: < starknet :: core :: types ::
            FieldElement > :: cairo_serialized_size(& payload); return
            Ok(Event ::
            MessagingEvent(Event ::
            MessageToAppchainSealed(MessageToAppchainSealed
            { message_hash, from, to, selector, nonce, payload })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("LogStateUpdate").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "LogStateUpdate"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            state_root = match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "state_root",
                "LogStateUpdate", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& state_root); let block_number = match
            starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "block_number", "LogStateUpdate", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& block_number); let block_hash = match
            starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "block_hash",
                "LogStateUpdate", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& block_hash); return
            Ok(Event ::
            LogStateUpdate(LogStateUpdate
            { state_root, block_number, block_hash }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("LogStateTransitionFact").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "LogStateTransitionFact"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            state_transition_fact = match U256 ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "state_transition_fact", "LogStateTransitionFact", e)),
            }; data_offset += U256 ::
            cairo_serialized_size(& state_transition_fact); return
            Ok(Event ::
            LogStateTransitionFact(LogStateTransitionFact
            { state_transition_fact }))
        };
        Err(format! ("Could not match any event from keys {:?}", event.keys))
    }
} #[derive(Debug, PartialEq, Clone)] pub enum MessageToStarknetStatus
{
    NothingToConsume,
    ReadyToConsume(starknet :: core :: types :: FieldElement)
} impl cainome :: cairo_serde :: CairoSerde for MessageToStarknetStatus
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        match __rust
        {
            MessageToStarknetStatus :: NothingToConsume => 1,
            MessageToStarknetStatus :: ReadyToConsume(val) => starknet :: core
            :: types :: FieldElement :: cairo_serialized_size(val) + 1, _ => 0
        }
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        match __rust
        {
            MessageToStarknetStatus :: NothingToConsume => usize ::
            cairo_serialize(& 0usize), MessageToStarknetStatus ::
            ReadyToConsume(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 1usize));
                temp.extend(starknet :: core :: types :: FieldElement ::
                cairo_serialize(val)); temp
            }, _ => vec! []
        }
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            0usize => Ok(MessageToStarknetStatus :: NothingToConsume), 1usize
            =>
            Ok(MessageToStarknetStatus ::
            ReadyToConsume(starknet :: core :: types :: FieldElement ::
            cairo_deserialize(__felts, __offset + 1) ?)), _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format!
            ("Index not handle for enum {}", "MessageToStarknetStatus")))
        }
    }
} #[derive(Debug, PartialEq, Clone)] pub enum Event
{ ProgramInfoChanged(ProgramInfoChanged) } impl cainome :: cairo_serde ::
CairoSerde for Event
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        match __rust
        {
            Event :: ProgramInfoChanged(val) => ProgramInfoChanged ::
            cairo_serialized_size(val) + 1, _ => 0
        }
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        match __rust
        {
            Event :: ProgramInfoChanged(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 0usize));
                temp.extend(ProgramInfoChanged :: cairo_serialize(val)); temp
            }, _ => vec! []
        }
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            0usize =>
            Ok(Event ::
            ProgramInfoChanged(ProgramInfoChanged ::
            cairo_deserialize(__felts, __offset + 1) ?)), _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format! ("Index not handle for enum {}", "Event")))
        }
    }
} impl TryFrom < starknet :: core :: types :: EmittedEvent > for Event
{
    type Error = String; fn
    try_from(event : starknet :: core :: types :: EmittedEvent) -> Result <
    Self, Self :: Error >
    {
        use cainome :: cairo_serde :: CairoSerde; if event.keys.is_empty()
        { return Err("Event has no key".to_string()); } let selector =
        event.keys [0]; if selector == starknet :: core :: utils ::
        get_selector_from_name("ProgramInfoChanged").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "ProgramInfoChanged"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            changed_by = match cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "changed_by",
                "ProgramInfoChanged", e)),
            }; data_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& changed_by); let old_program_hash = match
            starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "old_program_hash", "ProgramInfoChanged", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& old_program_hash); let new_program_hash =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "new_program_hash", "ProgramInfoChanged", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& new_program_hash); let old_config_hash =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "old_config_hash", "ProgramInfoChanged", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& old_config_hash); let new_config_hash =
            match starknet :: core :: types :: FieldElement ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "new_config_hash", "ProgramInfoChanged", e)),
            }; data_offset += starknet :: core :: types :: FieldElement ::
            cairo_serialized_size(& new_config_hash); return
            Ok(Event ::
            ProgramInfoChanged(ProgramInfoChanged
            {
                changed_by, old_program_hash, new_program_hash,
                old_config_hash, new_config_hash
            }))
        };
        Err(format! ("Could not match any event from keys {:?}", event.keys))
    }
} #[derive(Debug, PartialEq, Clone)] pub enum Event {} impl cainome ::
cairo_serde :: CairoSerde for Event
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    { match __rust { _ => 0 } } fn
    cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet :: core ::
    types :: FieldElement > { match __rust { _ => vec! [] } } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format! ("Index not handle for enum {}", "Event")))
        }
    }
} impl TryFrom < starknet :: core :: types :: EmittedEvent > for Event
{
    type Error = String; fn
    try_from(event : starknet :: core :: types :: EmittedEvent) -> Result <
    Self, Self :: Error >
    {
        use cainome :: cairo_serde :: CairoSerde; if event.keys.is_empty()
        { return Err("Event has no key".to_string()); }
        Err(format! ("Could not match any event from keys {:?}", event.keys))
    }
} #[derive(Debug, PartialEq, Clone)] pub enum Event
{
    OwnershipTransferred(OwnershipTransferred),
    OwnershipTransferStarted(OwnershipTransferStarted)
} impl cainome :: cairo_serde :: CairoSerde for Event
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        match __rust
        {
            Event :: OwnershipTransferred(val) => OwnershipTransferred ::
            cairo_serialized_size(val) + 1, Event ::
            OwnershipTransferStarted(val) => OwnershipTransferStarted ::
            cairo_serialized_size(val) + 1, _ => 0
        }
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        match __rust
        {
            Event :: OwnershipTransferred(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 0usize));
                temp.extend(OwnershipTransferred :: cairo_serialize(val));
                temp
            }, Event :: OwnershipTransferStarted(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 1usize));
                temp.extend(OwnershipTransferStarted :: cairo_serialize(val));
                temp
            }, _ => vec! []
        }
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            0usize =>
            Ok(Event ::
            OwnershipTransferred(OwnershipTransferred ::
            cairo_deserialize(__felts, __offset + 1) ?)), 1usize =>
            Ok(Event ::
            OwnershipTransferStarted(OwnershipTransferStarted ::
            cairo_deserialize(__felts, __offset + 1) ?)), _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format! ("Index not handle for enum {}", "Event")))
        }
    }
} impl TryFrom < starknet :: core :: types :: EmittedEvent > for Event
{
    type Error = String; fn
    try_from(event : starknet :: core :: types :: EmittedEvent) -> Result <
    Self, Self :: Error >
    {
        use cainome :: cairo_serde :: CairoSerde; if event.keys.is_empty()
        { return Err("Event has no key".to_string()); } let selector =
        event.keys [0]; if selector == starknet :: core :: utils ::
        get_selector_from_name("OwnershipTransferred").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "OwnershipTransferred"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            previous_owner = match cainome :: cairo_serde :: ContractAddress
            :: cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "previous_owner", "OwnershipTransferred", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& previous_owner); let new_owner = match
            cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "new_owner",
                "OwnershipTransferred", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& new_owner); return
            Ok(Event ::
            OwnershipTransferred(OwnershipTransferred
            { previous_owner, new_owner }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("OwnershipTransferStarted").unwrap_or_else(| _
        | panic! ("Invalid selector for {}", "OwnershipTransferStarted"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            previous_owner = match cainome :: cairo_serde :: ContractAddress
            :: cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "previous_owner", "OwnershipTransferStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& previous_owner); let new_owner = match
            cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "new_owner",
                "OwnershipTransferStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& new_owner); return
            Ok(Event ::
            OwnershipTransferStarted(OwnershipTransferStarted
            { previous_owner, new_owner }))
        };
        Err(format! ("Could not match any event from keys {:?}", event.keys))
    }
} #[derive(Debug, PartialEq, Clone)] pub enum Event {} impl cainome ::
cairo_serde :: CairoSerde for Event
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    { match __rust { _ => 0 } } fn
    cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet :: core ::
    types :: FieldElement > { match __rust { _ => vec! [] } } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format! ("Index not handle for enum {}", "Event")))
        }
    }
} impl TryFrom < starknet :: core :: types :: EmittedEvent > for Event
{
    type Error = String; fn
    try_from(event : starknet :: core :: types :: EmittedEvent) -> Result <
    Self, Self :: Error >
    {
        use cainome :: cairo_serde :: CairoSerde; if event.keys.is_empty()
        { return Err("Event has no key".to_string()); }
        Err(format! ("Could not match any event from keys {:?}", event.keys))
    }
} #[derive(Debug, PartialEq, Clone)] pub enum MessageToAppchainStatus
{ SealedOrNotSent, Pending(starknet :: core :: types :: FieldElement) } impl
cainome :: cairo_serde :: CairoSerde for MessageToAppchainStatus
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        match __rust
        {
            MessageToAppchainStatus :: SealedOrNotSent => 1,
            MessageToAppchainStatus :: Pending(val) => starknet :: core ::
            types :: FieldElement :: cairo_serialized_size(val) + 1, _ => 0
        }
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        match __rust
        {
            MessageToAppchainStatus :: SealedOrNotSent => usize ::
            cairo_serialize(& 0usize), MessageToAppchainStatus :: Pending(val)
            =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 1usize));
                temp.extend(starknet :: core :: types :: FieldElement ::
                cairo_serialize(val)); temp
            }, _ => vec! []
        }
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            0usize => Ok(MessageToAppchainStatus :: SealedOrNotSent), 1usize
            =>
            Ok(MessageToAppchainStatus ::
            Pending(starknet :: core :: types :: FieldElement ::
            cairo_deserialize(__felts, __offset + 1) ?)), _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format!
            ("Index not handle for enum {}", "MessageToAppchainStatus")))
        }
    }
} impl < A : starknet :: accounts :: ConnectedAccount + Sync > AppChain < A >
{
    #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub fn
    get_facts_registry(& self,) -> cainome :: cairo_serde :: call :: FCall < A
    :: Provider, cainome :: cairo_serde :: ContractAddress >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("get_facts_registry"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    sn_to_appchain_messages(& self, message_hash : & starknet :: core :: types
    :: FieldElement) -> cainome :: cairo_serde :: call :: FCall < A ::
    Provider, MessageToAppchainStatus >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(message_hash)); let __call = starknet :: core :: types
        :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("sn_to_appchain_messages"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    appchain_to_sn_messages(& self, message_hash : & starknet :: core :: types
    :: FieldElement) -> cainome :: cairo_serde :: call :: FCall < A ::
    Provider, MessageToStarknetStatus >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(message_hash)); let __call = starknet :: core :: types
        :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("appchain_to_sn_messages"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn get_state(& self,) -> cainome :: cairo_serde :: call :: FCall < A ::
    Provider,
    (starknet :: core :: types :: FieldElement, starknet :: core :: types ::
    FieldElement, starknet :: core :: types :: FieldElement) >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("get_state"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    is_operator(& self, address : & cainome :: cairo_serde :: ContractAddress)
    -> cainome :: cairo_serde :: call :: FCall < A :: Provider, bool >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(address)); let __call = starknet :: core :: types ::
        FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("is_operator"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn get_program_info(& self,) -> cainome :: cairo_serde :: call :: FCall <
    A :: Provider,
    (starknet :: core :: types :: FieldElement, starknet :: core :: types ::
    FieldElement) >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("get_program_info"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    update_state_getcall(& self, program_output : & Vec :: < starknet :: core
    :: types :: FieldElement > , onchain_data_hash : & starknet :: core ::
    types :: FieldElement, onchain_data_size : & U256) -> starknet :: accounts
    :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(program_output));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(onchain_data_hash));
        __calldata.extend(U256 :: cairo_serialize(onchain_data_size));
        starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("update_state"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    update_state(& self, program_output : & Vec :: < starknet :: core :: types
    :: FieldElement > , onchain_data_hash : & starknet :: core :: types ::
    FieldElement, onchain_data_size : & U256) -> starknet :: accounts ::
    Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(program_output));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(onchain_data_hash));
        __calldata.extend(U256 :: cairo_serialize(onchain_data_size)); let
        __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("update_state"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    set_program_info_getcall(& self, program_hash : & starknet :: core ::
    types :: FieldElement, config_hash : & starknet :: core :: types ::
    FieldElement) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(program_hash));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(config_hash)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("set_program_info"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    set_program_info(& self, program_hash : & starknet :: core :: types ::
    FieldElement, config_hash : & starknet :: core :: types :: FieldElement)
    -> starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(program_hash));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(config_hash)); let __call = starknet :: accounts ::
        Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("set_program_info"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    unregister_operator_getcall(& self, address : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(address)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("unregister_operator"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    unregister_operator(& self, address : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(address)); let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("unregister_operator"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    update_getcall(& self, program_output : & Vec :: < starknet :: core ::
    types :: FieldElement >) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(program_output)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("update"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    update(& self, program_output : & Vec :: < starknet :: core :: types ::
    FieldElement >) -> starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(program_output)); let __call = starknet :: accounts
        :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("update"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    send_message_to_appchain_getcall(& self, to_address : & cainome ::
    cairo_serde :: ContractAddress, selector : & starknet :: core :: types ::
    FieldElement, payload : & Vec :: < starknet :: core :: types ::
    FieldElement >) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(to_address));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(selector));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(payload)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("send_message_to_appchain"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    send_message_to_appchain(& self, to_address : & cainome :: cairo_serde ::
    ContractAddress, selector : & starknet :: core :: types :: FieldElement,
    payload : & Vec :: < starknet :: core :: types :: FieldElement >) ->
    starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(to_address));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(selector));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(payload)); let __call = starknet :: accounts ::
        Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("send_message_to_appchain"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    consume_message_from_appchain_getcall(& self, from_address : & cainome ::
    cairo_serde :: ContractAddress, payload : & Vec :: < starknet :: core ::
    types :: FieldElement >) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(from_address));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(payload)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("consume_message_from_appchain"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    consume_message_from_appchain(& self, from_address : & cainome ::
    cairo_serde :: ContractAddress, payload : & Vec :: < starknet :: core ::
    types :: FieldElement >) -> starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(from_address));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(payload)); let __call = starknet :: accounts ::
        Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("consume_message_from_appchain"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    cancel_message_getcall(& self, to_address : & cainome :: cairo_serde ::
    ContractAddress, selector : & starknet :: core :: types :: FieldElement,
    payload : & Vec :: < starknet :: core :: types :: FieldElement > , nonce :
    & starknet :: core :: types :: FieldElement) -> starknet :: accounts ::
    Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(to_address));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(selector));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(payload));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(nonce)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("cancel_message"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    cancel_message(& self, to_address : & cainome :: cairo_serde ::
    ContractAddress, selector : & starknet :: core :: types :: FieldElement,
    payload : & Vec :: < starknet :: core :: types :: FieldElement > , nonce :
    & starknet :: core :: types :: FieldElement) -> starknet :: accounts ::
    Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(to_address));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(selector));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(payload));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(nonce)); let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("cancel_message"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    register_operator_getcall(& self, address : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(address)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("register_operator"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    register_operator(& self, address : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(address)); let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("register_operator"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    start_message_cancellation_getcall(& self, to_address : & cainome ::
    cairo_serde :: ContractAddress, selector : & starknet :: core :: types ::
    FieldElement, payload : & Vec :: < starknet :: core :: types ::
    FieldElement > , nonce : & starknet :: core :: types :: FieldElement) ->
    starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(to_address));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(selector));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(payload));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(nonce)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("start_message_cancellation"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    start_message_cancellation(& self, to_address : & cainome :: cairo_serde
    :: ContractAddress, selector : & starknet :: core :: types ::
    FieldElement, payload : & Vec :: < starknet :: core :: types ::
    FieldElement > , nonce : & starknet :: core :: types :: FieldElement) ->
    starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(to_address));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(selector));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(payload));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(nonce)); let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("start_message_cancellation"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    set_facts_registry_getcall(& self, address : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(address)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("set_facts_registry"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    set_facts_registry(& self, address : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(address)); let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("set_facts_registry"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    }
} impl < P : starknet :: providers :: Provider + Sync > AppChainReader < P >
{
    #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub fn
    get_facts_registry(& self,) -> cainome :: cairo_serde :: call :: FCall <
    P, cainome :: cairo_serde :: ContractAddress >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("get_facts_registry"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    sn_to_appchain_messages(& self, message_hash : & starknet :: core :: types
    :: FieldElement) -> cainome :: cairo_serde :: call :: FCall < P,
    MessageToAppchainStatus >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(message_hash)); let __call = starknet :: core :: types
        :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("sn_to_appchain_messages"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    appchain_to_sn_messages(& self, message_hash : & starknet :: core :: types
    :: FieldElement) -> cainome :: cairo_serde :: call :: FCall < P,
    MessageToStarknetStatus >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(message_hash)); let __call = starknet :: core :: types
        :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("appchain_to_sn_messages"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn get_state(& self,) -> cainome :: cairo_serde :: call :: FCall < P,
    (starknet :: core :: types :: FieldElement, starknet :: core :: types ::
    FieldElement, starknet :: core :: types :: FieldElement) >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("get_state"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    is_operator(& self, address : & cainome :: cairo_serde :: ContractAddress)
    -> cainome :: cairo_serde :: call :: FCall < P, bool >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(address)); let __call = starknet :: core :: types ::
        FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("is_operator"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn get_program_info(& self,) -> cainome :: cairo_serde :: call :: FCall <
    P,
    (starknet :: core :: types :: FieldElement, starknet :: core :: types ::
    FieldElement) >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("get_program_info"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    }
}