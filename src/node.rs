use crate::store::{InMemoryStore, Storage};
use ibc::ics02_client::client_def::{AnyClientState, AnyConsensusState};
use ibc::ics02_client::client_type::ClientType;
use ibc::ics02_client::context::{ClientKeeper, ClientReader};
use ibc::ics02_client::error::Error as ClientError;
use ibc::ics24_host::identifier::ClientId;
use ibc::Height;
use prost_types::Any;
use std::convert::TryFrom;
use std::str::FromStr;

// Define path in private & provable stores.
const CONSENSUS_STATE_URL: &'static str = "/ibc.lightclients.tendermint.v1.ConsensusState";
const CLIENT_STATE_URL: &'static str = "/ibc.lightclients.tendermint.v1.ClientState";

pub struct Node<S: Storage> {
    private_store: S,
    provable_store: S,
    chain: Vec<bool>, // TODO: use light blocks.
}

impl Node<InMemoryStore> {
    pub fn new() -> Self {
        Node {
            private_store: InMemoryStore::new(),
            provable_store: InMemoryStore::new(),
            chain: vec![],
        }
    }
}

impl<S: Storage> Node<S> {
    pub fn get_store(&self) -> &S {
        &self.provable_store
    }
}

impl<S: Storage> ClientReader for Node<S> {
    fn client_type(&self, client_id: &ClientId) -> Option<ClientType> {
        let path = format!("clients/{}/clientType", client_id.as_str());
        let client_type = self.provable_store.get(0, path.as_bytes())?;
        let client_type = String::from_utf8(client_type.to_vec());
        match client_type {
            Err(_) => None,
            Ok(client_type) => ClientType::from_str(&client_type).ok(),
        }
    }

    fn client_state(&self, client_id: &ClientId) -> Option<AnyClientState> {
        let path = format!("clients/{}/clientState", client_id.as_str());
        let value = self.private_store.get(0, path.as_bytes())?.to_owned();
        let client_state = Any {
            type_url: String::from(CLIENT_STATE_URL),
            value,
        };
        AnyClientState::try_from(client_state).ok()
    }

    fn consensus_state(&self, client_id: &ClientId, height: Height) -> Option<AnyConsensusState> {
        let path = format!(
            "clients/{}/consensusState/{}",
            client_id.as_str(),
            height.to_string()
        );
        let value = self.provable_store.get(0, path.as_bytes())?.to_owned();
        let consensus_state = Any {
            type_url: String::from(CONSENSUS_STATE_URL),
            value,
        };
        AnyConsensusState::try_from(consensus_state).ok()
    }
}

impl<S: Storage> ClientKeeper for Node<S> {
    fn store_client_type(
        &mut self,
        client_id: ClientId,
        client_type: ClientType,
    ) -> Result<(), ClientError> {
        let path = format!("clients/{}/clientType", client_id.as_str());
        self.provable_store.set(
            0,
            path.as_bytes().to_owned(),
            client_type.as_string().as_bytes().to_owned(),
        );
        Ok(())
    }

    fn store_client_state(
        &mut self,
        client_id: ClientId,
        client_state: AnyClientState,
    ) -> Result<(), ClientError> {
        let path = format!("clients/{}/clientState", client_id.as_str());
        let data: Any = client_state.into();
        self.private_store.set(0, path.into_bytes(), data.value);
        Ok(())
    }

    fn store_consensus_state(
        &mut self,
        client_id: ClientId,
        height: Height,
        consensus_state: AnyConsensusState,
    ) -> Result<(), ClientError> {
        let path = format!(
            "clients/{}/consensusState/{}",
            client_id.to_string(),
            height.to_string()
        );
        let data: Any = consensus_state.into();
        self.provable_store.set(0, path.into_bytes(), data.value);
        Ok(())
    }
}
