#[derive(Copy, Drop, Serde)]
pub struct DataAvailabilityFact {
    pub onchain_data_hash: felt252,
    pub onchain_data_size: u256,
}

/// Encodes a GPS fact Merkle tree where the root has two children.
/// The left child contains the data we care about and the right child contains on-chain data for
/// the fact.
///
/// # Arguments
///
/// * `program_output` - The output of the program that is being proven.
/// * `fact_data` - The on-chain data for the fact.
///
/// # Returns
///
/// * The hash of the fact Merkle tree.
pub fn encode_fact_with_onchain_data(
    program_output: Span<felt252>, fact_data: DataAvailabilityFact,
) -> u256 {
    // The state transition fact is computed as a Merkle tree.
    // The root has two children.
    // The left child is a leaf that includes the main part - the information regarding
    // The state transition required by this contract.
    // The right child contains the onchain-data which shouldn't be accessed by this
    // We are only given its hash and length

    // Compute the hash without the two additional fields.

    let main_public_input_len: u256 = program_output.len().into();
    let main_public_input_hash: u256 = hash_main_public_input(program_output);

    // Compute the hash of the fact Merkle tree.

    let mut keccak_input: Array<u256> = ArrayTrait::new();
    keccak_input.append(main_public_input_hash);
    keccak_input.append(main_public_input_len);
    keccak_input.append(fact_data.onchain_data_hash.into());
    keccak_input.append(main_public_input_len + fact_data.onchain_data_size);

    let hash_result: u256 = core::keccak::keccak_u256s_le_inputs(keccak_input.span());

    // Add one to the hash to indicate it represents an inner node, rather than a leaf.

    return hash_result + 1;
}

/// Hashes the main public input.
///
/// # Arguments
///
/// * `program_output` - The output of the program that is being proven.
///
/// # Returns
///
/// * The hash of the main public input.
pub fn hash_main_public_input(program_output: Span<felt252>) -> u256 {
    let mut keccak_input: Array<u256> = ArrayTrait::new();
    let mut i = 0;
    loop {
        if (i == program_output.len()) {
            break;
        }
        keccak_input.append((*program_output.at(i)).into());
        i += 1;
    };

    core::keccak::keccak_u256s_le_inputs(keccak_input.span())
}
