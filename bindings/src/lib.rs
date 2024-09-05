use cainome::rs::abigen;

abigen!(
	AppChain,
	"../target/dev/piltover_appchain.contract_class.json",
	output_path("src/bindings.rs"),
	derives(Debug, PartialEq, Eq, Clone, Copy)
);
