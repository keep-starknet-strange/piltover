use starknet::ContractAddress;

pub const OPERATOR: ContractAddress = 'OPERATOR'.try_into().unwrap();
pub const OTHER: ContractAddress = 'OTHER'.try_into().unwrap();
pub const OWNER: ContractAddress = 'OWNER'.try_into().unwrap();
pub const RECIPIENT: ContractAddress = 'RECIPIENT'.try_into().unwrap();
pub const SPENDER: ContractAddress = 'SPENDER'.try_into().unwrap();
pub const ZERO: ContractAddress = 0.try_into().unwrap();
pub const NEW_OWNER: ContractAddress = 'NEW_OWNER'.try_into().unwrap();
