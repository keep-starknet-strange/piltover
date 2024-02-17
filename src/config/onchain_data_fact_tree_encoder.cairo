mod OnchainDataFactTreeEncoder {
    use alexandria_bytes::Bytes;
    use alexandria_bytes::BytesTrait;

    #[derive(Copy, Drop, Serde)]
    struct data_availability_fact {
        onchain_data_hash: felt252,
        onchain_data_size: u256,
    }

    //Encodes a GPS fact Merkle tree where the root has two children.
    //The left child contains the data we care about and the right child contains on-chain data for the fact.
    fn encode_fact_with_onchain_data(
        program_output: Span<felt252>, fact_data: data_availability_fact
    ) -> u256 {
        //   The state transition fact is computed as a Merkle tree.
        //   The root has two children.
        //   The left child is a leaf that includes the main part - the information regarding
        //   the state transition required by this contract.
        //   The right child contains the onchain-data which shouldn't be accessed by this
        //   We are only given its hash and length

        // Compute the hash without the two additional fields.

        let main_public_input_len: u256 = program_output.len().into();
        let main_public_input_hash: u256 = hash_main_public_input(program_output);

        //Compute the hash of the fact Merkle tree.

        let mut bytes = BytesTrait::new_empty();
        bytes.append_u256(main_public_input_len);
        bytes.append_u256(main_public_input_hash);
        bytes.append_felt252(fact_data.onchain_data_hash);
        bytes.append_u256(main_public_input_len + fact_data.onchain_data_size);
        let hash_result: u256 = bytes.sha256();

        // Add one to the hash to indicate it represents an inner node, rather than a leaf.

        return hash_result + 1;
    }

    //Hashes the main public input.
    fn hash_main_public_input(program_output: Span<felt252>) -> u256 {
        let mut bytes = BytesTrait::new_empty();
        let mut i = 0;
        loop {
            if (i == program_output.len()) {
                break;
            }
            bytes.append_felt252(*program_output.at(i));
            i += 1;
        };

        bytes.sha256()
    }
}
