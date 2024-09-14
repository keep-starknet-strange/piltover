mod appchain;
mod interface;
mod mocks;
mod snos_output;

// Components
pub mod config {
    pub mod component;
    pub mod interface;
    pub mod mock;

    pub use component::config_cpt;
    pub use interface::{IConfig, IConfigDispatcher, IConfigDispatcherTrait};
    pub use mock::config_mock;

    #[cfg(test)]
    mod tests {
        mod test_config;
    }
}

pub mod messaging {
    pub mod component;
    pub mod hash;
    pub mod interface;
    pub mod mock;
    pub mod output_process;
    pub mod types;

    pub use component::messaging_cpt;
    pub use interface::{IMessaging, IMessagingDispatcher, IMessagingDispatcherTrait};
    pub use mock::messaging_mock;

    #[cfg(test)]
    mod tests {
        mod test_messaging;
    }
}

mod state {
    mod component;
    mod interface;
    mod mock;

    use component::state_cpt;
    use interface::{IState, IStateDispatcher, IStateDispatcherTrait};
    use mock::state_mock;

    #[cfg(test)]
    mod tests {
        mod test_state;
    }
}
pub mod components {
    pub mod onchain_data_fact_tree_encoder;
}
