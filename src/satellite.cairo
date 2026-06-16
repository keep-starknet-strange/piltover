#[starknet::interface]
pub trait ISatellite<T> {
    fn isKeccakVerifiedFactHashValid(self: @T, fact_hash: u256) -> bool;
}
