use crate::config::{Client, Config};
use ibc::ics02_client::client_def::AnyClientState;
use ibc::ics02_client::client_type::ClientType;
use ibc::ics02_client::context::ClientKeeper;
use ibc::ics07_tendermint::client_state::ClientState;
use ibc::ics24_host::identifier::ClientId;
use ibc::Height;
use std::str::FromStr;
use tendermint;
use tendermint::consensus::Params;
use tendermint::trust_threshold::TrustThresholdFraction;

pub fn init<T: ClientKeeper>(keeper: &mut T, config: &Config) {
    for client in &config.clients {
        add_client(keeper, client, &config.chain_id);
    }
}

fn add_client<T: ClientKeeper>(keeper: &mut T, client: &Client, chain_id: &str) {
    let client_id =
        ClientId::from_str(&client.id).expect(&format!("Invalid client id: {}", &client.id));
    let client_state = new_client_state(chain_id);
    keeper
        .store_client_state(client_id.clone(), client_state)
        .unwrap();
    keeper
        .store_client_type(client_id, ClientType::Tendermint)
        .unwrap();
}

fn new_client_state(chain_id: &str) -> AnyClientState {
    let duration = std::time::Duration::new(60, 0);
    let height = Height::new(1, 1);
    let client_state = ClientState {
        chain_id: String::from(chain_id),
        trusting_period: duration.clone(),
        trust_level: TrustThresholdFraction::new(1, 3).unwrap(),
        unbonding_period: duration.clone(),
        max_clock_drift: duration,
        frozen_height: height.clone(),
        latest_height: height,
        upgrade_path: String::from("path"),
        allow_update_after_expiry: false,
        allow_update_after_misbehaviour: false,
        consensus_params: Params {
            version: None,
            block: tendermint::block::Size {
                max_bytes: 10000,
                max_gas: 10000,
            },
            evidence: tendermint::evidence::Params {
                max_num: 10000,
                max_age_duration: tendermint::evidence::Duration(std::time::Duration::from_secs(
                    3600,
                )),
                max_age_num_blocks: 10000,
            },
            validator: tendermint::consensus::params::ValidatorParams {
                pub_key_types: vec![],
            },
        },
    };
    AnyClientState::Tendermint(client_state)
}
