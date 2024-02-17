mod OnchainDataFactTreeEncoder {
    use alexandria_bytes::Bytes;
    use alexandria_bytes::BytesTrait;

    #[derive(Copy, Drop, Serde)]
    struct data_availability_fact {
        onchain_data_hash: felt252,
        onchain_data_size: u256,
    }
    fn encode_fact_with_onchain_data(
        program_output: Span<felt252>, fact_data: data_availability_fact
    ) -> u256 {
        let main_public_input_len: u256 = program_output.len().into();
        let main_public_input_hash: u256 = hash_main_public_input(program_output);

        //compute the hash

        let mut bytes = BytesTrait::new_empty();
        bytes.append_u256(main_public_input_len);
        bytes.append_u256(main_public_input_hash);
        bytes.append_felt252(fact_data.onchain_data_hash);
        bytes.append_u256(main_public_input_len + fact_data.onchain_data_size);
        let hash_result: u256 = bytes.sha256();
        return hash_result + 1;
    }
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
