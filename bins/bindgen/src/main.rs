use cainome::rs::abigen;

abigen!(
    AppchainContract,
    "./target/dev/piltover_appchain.contract_class.json",
    output_path("./piltover/src/bindgen.rs"),
    type_aliases {
        piltover::messaging::component::messaging_cpt::Event as MessagingEvent;
        piltover::state::component::state_cpt::Event as StateEvent;
        piltover::config::component::config_cpt::Event as ConfigEvent;
        openzeppelin_security::reentrancyguard::ReentrancyGuardComponent::Event as ReentrancyguardEvent;
        openzeppelin_upgrades::upgradeable::UpgradeableComponent::Event as UpgradeableEvent;
        openzeppelin_access::ownable::ownable::OwnableComponent::Event as OwnableEvent;
        piltover::appchain::appchain::Event as AppchainEvent;
    }
);

fn main() {}
