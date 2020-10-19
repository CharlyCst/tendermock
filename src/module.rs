use ibc::ics02_client::client_def::{AnyClientState, AnyConsensusState};
use ibc::ics02_client::client_type::ClientType;
use ibc::ics02_client::context::{ClientKeeper, ClientReader};
use ibc::ics02_client::error::Error as ClientError;
use ibc::ics03_connection::connection::ConnectionEnd;
use ibc::ics03_connection::context::{ConnectionKeeper, ConnectionReader};
use ibc::ics03_connection::error::Error as ConnectionError;
use ibc::ics23_commitment::commitment::CommitmentPrefix;
use ibc::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_proto::ibc::core::connection::v1::ConnectionEnd as RawConnectionEnd;
use prost::Message;

use ibc::Height;
use prost_types::Any;
use std::convert::TryFrom;
use std::str::FromStr;

use crate::store::Storage;

const CONSENSUS_STATE_URL: &'static str = "/ibc.lightclients.tendermint.v1.ConsensusState";
const CLIENT_STATE_URL: &'static str = "/ibc.lightclients.tendermint.v1.ClientState";

impl ConnectionReader for dyn Storage {
    fn connection_end(&self, conn_id: &ConnectionId) -> Option<&ConnectionEnd> {
        unimplemented!()
    }

    fn client_state(&self, client_id: &ClientId) -> Option<AnyClientState> {
        let path = format!("clients/{}/clientState", client_id.as_str());
        let value = self.get(0, path.as_bytes())?.to_owned();
        let client_state = Any {
            type_url: String::from(CLIENT_STATE_URL),
            value,
        };
        AnyClientState::try_from(client_state).ok()
    }

    fn host_current_height(&self) -> Height {
        unimplemented!()
    }

    fn chain_consensus_states_history_size(&self) -> usize {
        unimplemented!()
    }

    fn commitment_prefix(&self) -> CommitmentPrefix {
        unimplemented!()
    }

    fn client_consensus_state(
        &self,
        client_id: &ClientId,
        height: Height,
    ) -> Option<AnyConsensusState> {
        unimplemented!()
    }

    fn host_consensus_state(&self, height: Height) -> Option<AnyConsensusState> {
        unimplemented!()
    }

    fn get_compatible_versions(&self) -> Vec<String> {
        unimplemented!()
    }

    fn pick_version(&self, counterparty_candidate_versions: Vec<String>) -> String {
        unimplemented!()
    }
}

impl ConnectionKeeper for dyn Storage {
    fn store_connection(
        &mut self,
        connection_id: &ConnectionId,
        connection_end: &ConnectionEnd,
    ) -> Result<(), ConnectionError> {
        let mut buffer = Vec::new();
        let path = format!("connections/{}", connection_id.as_str());
        let raw: RawConnectionEnd = connection_end.to_owned().into();
        raw.encode(&mut buffer).unwrap();
        self.set(0, path.into_bytes(), buffer);
        Ok(())
    }

    // TODO: store a vec of connection_id indead of a single connection.
    fn store_connection_to_client(
        &mut self,
        connection_id: &ConnectionId,
        client_id: &ClientId,
    ) -> Result<(), ConnectionError> {
        let path = format!("clients/{}/connections", client_id.as_str());
        self.set(0, path.into_bytes(), connection_id.as_bytes().to_owned());
        Ok(())
    }
}

impl ClientReader for dyn Storage {
    fn client_type(&self, client_id: &ClientId) -> Option<ClientType> {
        let path = format!("clients/{}/clientType", client_id.as_str());
        let client_type = self.get(0, path.as_bytes())?;
        let client_type = String::from_utf8(client_type.to_vec());
        match client_type {
            Err(_) => None,
            Ok(client_type) => ClientType::from_str(&client_type).ok(),
        }
    }

    fn client_state(&self, client_id: &ClientId) -> Option<AnyClientState> {
        let path = format!("clients/{}/clientState", client_id.as_str());
        let value = self.get(0, path.as_bytes())?.to_owned();
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
        let value = self.get(0, path.as_bytes())?.to_owned();
        let consensus_state = Any {
            type_url: String::from(CONSENSUS_STATE_URL),
            value,
        };
        AnyConsensusState::try_from(consensus_state).ok()
    }
}

impl ClientKeeper for dyn Storage {
    fn store_client_type(
        &mut self,
        client_id: ClientId,
        client_type: ClientType,
    ) -> Result<(), ClientError> {
        let path = format!("clients/{}/clientType", client_id.as_str());
        self.set(
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
        self.set(0, path.into_bytes(), data.value);
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
        self.set(0, path.into_bytes(), data.value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::InMemoryStore;
    use ibc::ics02_client::client_type::ClientType;
    use ibc::ics07_tendermint::client_state::ClientState;
    use ibc::ics07_tendermint::consensus_state::ConsensusState;
    use ibc::ics23_commitment::commitment::CommitmentRoot;
    use ibc::ics24_host::identifier::ClientId;
    use ibc::Height;
    use std::convert::TryInto;

    #[test]
    fn client() {
        let mut store: Box<dyn Storage> = Box::new(InMemoryStore::new());
        let height = Height {
            version_number: 0,
            version_height: 0,
        };
        let client_id = ClientId::from_str("UncleScrooge").unwrap();
        let client_state = dummy_client_state();
        let consensus_state = dummy_consensus_state();

        // ClientType
        store
            .store_client_type(client_id.clone(), ClientType::Tendermint)
            .unwrap();
        let client_type = store.client_type(&client_id).unwrap();
        assert_eq!(client_type, ClientType::Tendermint);
        // ClientState
        store
            .store_client_state(client_id.clone(), client_state.clone())
            .unwrap();
        let retrieved_client =
            <dyn Storage as ClientReader>::client_state(&*store, &client_id).unwrap();
        assert_eq!(client_state, retrieved_client);
        // ConsensusState
        store
            .store_consensus_state(client_id.clone(), height.clone(), consensus_state.clone())
            .unwrap();
        let retrieved_consensus = store.consensus_state(&client_id, height).unwrap();
        assert_eq!(consensus_state, retrieved_consensus);
    }

    fn dummy_consensus_state() -> AnyConsensusState {
        let root = CommitmentRoot::from_bytes(b"root");
        let tm_consensus_state = ConsensusState {
            timestamp: std::time::SystemTime::now().into(),
            next_validators_hash: vec![14; tendermint::hash::SHA256_HASH_SIZE]
                .try_into()
                .unwrap(),
            root,
        };
        AnyConsensusState::Tendermint(tm_consensus_state)
    }

    fn dummy_client_state() -> AnyClientState {
        let duration = std::time::Duration::new(60, 0);
        let height = Height {
            version_height: 0,
            version_number: 0,
        };
        let client_state = ClientState {
            chain_id: String::from("test_chain"),
            trusting_period: duration.clone(),
            unbonding_period: duration.clone(),
            max_clock_drift: duration,
            frozen_height: height.clone(),
            latest_height: height,
            upgrade_path: String::from("path"),
            allow_update_after_expiry: false,
            allow_update_after_misbehaviour: false,
        };
        AnyClientState::Tendermint(client_state)
    }
}
