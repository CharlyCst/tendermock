use ibc::ics02_client::client_def::{AnyClientState, AnyConsensusState};
use ibc::ics02_client::client_type::ClientType;
use ibc::ics02_client::context::{ClientKeeper, ClientReader};
use ibc::ics02_client::error::Error;
use ibc::ics24_host::identifier::ClientId;
use ibc::Height;
use prost_types::Any;
use std::convert::TryFrom;
use std::str::FromStr;

use crate::store::Storage;

const CONSENSUS_STATE_URL: &'static str = "/ibc.lightclients.tendermint.v1.ConsensusState";

impl ClientReader for dyn Storage {
    fn client_type(&self, client_id: &ClientId) -> Option<ClientType> {
        let path = format!("clients/{}/clientType", client_id.as_str());
        let client_type = self.get(0, path.as_bytes())?;
        let client_type = String::from_utf8(client_type.to_vec());
        match client_type {
            Err(_) => None,
            Ok(client_type) => match ClientType::from_str(&client_type) {
                Err(_) => None,
                Ok(t) => Some(t),
            },
        }
    }

    fn client_state(&self, client_id: &ClientId) -> Option<AnyClientState> {
        let path = format!("clients/{}/clientState", client_id.as_str());
        let value = self.get(0, path.as_bytes())?.to_owned();
        let client_state = Any {
            type_url: String::from(""),
            value,
        };
        match AnyClientState::try_from(client_state) {
            Ok(client_state) => Some(client_state),
            Err(_) => None,
        }
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
        println!("{:?}", consensus_state);
        match AnyConsensusState::try_from(consensus_state) {
            Ok(consensus_state) => Some(consensus_state),
            Err(_) => None,
        }
    }
}

impl ClientKeeper for dyn Storage {
    fn store_client_type(
        &mut self,
        client_id: ClientId,
        client_type: ClientType,
    ) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
        let path = format!(
            "clients/{}/consensusState/{}",
            client_id.to_string(),
            height.to_string()
        );
        let data: Any = consensus_state.into();
        println!("{:?}", data);
        self.set(0, path.into_bytes(), data.value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::InMemoryStore;
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
        let consensus_state = dummy_consensus_state();
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
}
