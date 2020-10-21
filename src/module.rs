/*
use ibc::ics02_client::client_def::{AnyClientState, AnyConsensusState};
use ibc::ics02_client::context::ClientReader;
use ibc::ics03_connection::connection::ConnectionEnd;
use ibc::ics03_connection::context::{ConnectionKeeper, ConnectionReader};
use ibc::ics03_connection::error::Error as ConnectionError;
use ibc::ics23_commitment::commitment::CommitmentPrefix;
use ibc::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_proto::ibc::core::connection::v1::ConnectionEnd as RawConnectionEnd;
use prost::Message;

use ibc::Height;
use std::str::FromStr;

use crate::store::Storage;

const CONSENSUS_STATE_URL: &'static str = "/ibc.lightclients.tendermint.v1.ConsensusState";
const CLIENT_STATE_URL: &'static str = "/ibc.lightclients.tendermint.v1.ClientState";

impl ConnectionReader for dyn Storage {
    fn connection_end(&self, conn_id: &ConnectionId) -> Option<&ConnectionEnd> {
        unimplemented!()
    }

    fn client_state(&self, client_id: &ClientId) -> Option<AnyClientState> {
        <dyn Storage as ClientReader>::client_state(self, client_id)
    }

    fn host_current_height(&self) -> Height {
        unimplemented!()
    }

    fn chain_consensus_states_history_size(&self) -> usize {
        unimplemented!()
    }

    // TODO: what is a commitment prefix?
    fn commitment_prefix(&self) -> CommitmentPrefix {
        unimplemented!()
    }

    fn client_consensus_state(
        &self,
        client_id: &ClientId,
        height: Height,
    ) -> Option<AnyConsensusState> {
        self.consensus_state(client_id, height)
    }

    // TODO: what is the host client_id?
    fn host_consensus_state(&self, height: Height) -> Option<AnyConsensusState> {
        let host_id = ClientId::from_str("self").unwrap();
        self.consensus_state(&host_id, height)
    }

    // TODO: what is the correct version format?
    fn get_compatible_versions(&self) -> Vec<String> {
        vec![String::from("0.0.1")]
    }

    // TODO: what if there is no compatible versions?
    fn pick_version(&self, counterparty_candidate_versions: Vec<String>) -> String {
        match counterparty_candidate_versions.get(0) {
            Some(version) => version.to_owned(),
            None => String::from("0.0.1"),
        }
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
*/
