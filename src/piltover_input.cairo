use bool::False;
use piltover::snos_output::{StarknetOsOutput, deserialize_os_output};

pub const HEADER_SIZE: usize = 8;

#[derive(Drop, Serde, Debug,)]
pub enum PiltoverInput {
    LayoutBridgeOutputNoDa: Span<felt252>,
    LayoutBridgeOutputWithDa: (Span<felt252>, DaLayerInfo),
}

pub trait PiltoverInputTrait {
    fn get_layout_bridge_output(self: @PiltoverInput) -> LayoutBridgeOutput;
    fn get_raw_output(self: @PiltoverInput) -> Span<felt252> {
        match self {
            PiltoverInput::LayoutBridgeOutputNoDa(lb_output) => *lb_output,
            PiltoverInput::LayoutBridgeOutputWithDa((lb_output, _)) => *lb_output,
        }
    }
}

impl PiltoverInputImpl of PiltoverInputTrait {
    fn get_layout_bridge_output(self: @PiltoverInput) -> LayoutBridgeOutput {
        match self {
            PiltoverInput::LayoutBridgeOutputNoDa(lb_output) => deserialize_layout_bridge_output(*lb_output),
            PiltoverInput::LayoutBridgeOutputWithDa((lb_output, _)) => deserialize_layout_bridge_output(*lb_output),
        }
    }
    fn get_raw_output(self: @PiltoverInput) -> Span<felt252> {
        match self {
            PiltoverInput::LayoutBridgeOutputNoDa(lb_output) => *lb_output,
            PiltoverInput::LayoutBridgeOutputWithDa((lb_output, _)) => *lb_output,
        }
    }
}

// This is only a placeholder for future use.
#[derive(Drop, Serde, Debug)]
pub struct DaLayerInfo {
    blob_size: u128,
}

#[derive(Drop, Serde, Debug)]
pub struct LayoutBridgeOutput {
    pub bootloader_task_count: felt252,
    pub output_length: felt252,
    pub layout_bridge_program_hash: felt252,
    pub bootloader_program_hash: felt252,
    pub snos_output_length: felt252,
    pub bootloader_output: SnosBootloaderOutput,
}


#[derive(Drop, Serde, Debug)]
pub struct SnosBootloaderOutput {
    pub bootloader_task_count: felt252,
    pub output_length: felt252,
    pub snos_program_hash: felt252,
    pub snos_output: StarknetOsOutput,
}

pub fn deserialize_layout_bridge_output(data: Span<felt252>) -> LayoutBridgeOutput {
    let snos_output: Array<felt252> = data
        .slice(HEADER_SIZE, data.len() - HEADER_SIZE)
        .into_iter()
        .map(|f| *f)
        .collect();
    let mut snos_output_iter = snos_output.span().into_iter();
    let output = deserialize_os_output(ref snos_output_iter, False);

    LayoutBridgeOutput {
        bootloader_task_count: *data.at(0),
        output_length: *data.at(1),
        layout_bridge_program_hash: *data.at(2),
        bootloader_program_hash: *data.at(3),
        snos_output_length: *data.at(4),
        bootloader_output: SnosBootloaderOutput {
            bootloader_task_count: *data.at(5),
            output_length: *data.at(6),
            snos_program_hash: *data.at(7),
            snos_output: output,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ELEMENT_COUNT: usize = 0x2F;
    #[test]
    fn test_piltover_input_serialization() {
        let mut felts = array![
            0x1, 0x36, 0x43c5c4cc37c4614d2cf3a833379052c3a38cd18d688b617e2c720e8f941cb8,
            0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07, 0x32, 0x1, 0x31,
            0x6228ae335151708cc9290c9940dc17f14f870056c1ae919af238e247d500ad4,
            0x529f8903315a861413a2ecfaa7dd168fe8d82b53d08eddccb7a0672bff7807,
            0x7c49388f0dc97b8d10a559485b30fa09d2db8de90751abf61e4ccdd3b241b19, 0x0, 0x1,
            0x7a2b7581f7b03af58d9f5b13e45db548fb99d9c3da7346ea02869917b3aa5f3,
            0x2a478f2f550ccde8aa9c05d3d7a0ade9f3a7d3757303153fab8d9235f6e6a2f, 0x0,
            0x2d807016250227f289fff622fd61e8f8363660fff5e5aa1c3c23c8a5031d042, 0x0, 0x0, 0x0, 0x0,
            0x3, 0x2, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x8f, 0x90,
            0x7b62949c85c6af8a50c11c22927f9302f7a2e40bc93b4c988415915b0f97f09, 0x0, 0x8f, 0x84, 0x1,
            0x2, 0x7dc7899aa655b0aae51eadff6d801a58e97dd99cf4666ee59e704249e51adf2,
            0x7dc7899aa655b0aae51eadff6d801a58e97dd99cf4666ee59e704249e51adf2, 0x0, 0x8d, 0x0, 0x0,
            0xa2475bc66197c751d854ea8c39c6ad9781eb284103bcd856b58e6b500078ac,
            0xa2475bc66197c751d854ea8c39c6ad9781eb284103bcd856b58e6b500078ac, 0x2, 0x8a,
            0x21e19e0c9bab2400000, 0x21d7db84cfec5370516, 0x8f, 0x0, 0x9c287cbbed08faea, 0x1,
            0x57994b6a75fad550ca18b41ee82e2110e158c59028c4478109a67965a0e5b1e, 0x0,
            0x68e080a6925a58dbd6e776ec2324a3fee67c5e09a9eeff6bd8aac94637806b6,
        ]
            .span();

        let _deserialized = deserialize_layout_bridge_output(felts);
    }
}
