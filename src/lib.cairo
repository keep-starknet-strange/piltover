pub mod appchain;
pub mod fact_registry;
pub mod interface;
pub mod snos_output;

// Components
pub mod config {
    pub mod component;
    pub mod interface;
    pub mod mock;

    pub use component::config_cpt;
    pub use interface::{IConfig, IConfigDispatcher, IConfigDispatcherTrait};
    pub use mock::config_mock;

    #[cfg(test)]
    pub mod tests {
        pub mod test_config;
    }
}

pub mod messaging {
    pub mod component;
    pub mod hash;
    pub mod interface;
    pub mod mock;
    pub mod types;

    pub use component::messaging_cpt;
    pub use interface::{IMessaging, IMessagingDispatcher, IMessagingDispatcherTrait};
    pub use mock::messaging_mock;

    #[cfg(test)]
    pub mod tests {
        pub mod test_messaging;
    }
}

pub mod state {
    pub mod component;
    pub mod interface;
    pub mod mock;

    pub use component::state_cpt;
    pub use interface::{IState, IStateDispatcher, IStateDispatcherTrait};
    pub use mock::state_mock;

    #[cfg(test)]
    pub mod tests {
        pub mod test_state;
    }
}

pub mod components {
    pub mod onchain_data_fact_tree_encoder;
}
