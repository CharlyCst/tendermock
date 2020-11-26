use crate::chain::Chain;
use crate::config::Config;
use crate::store::{InMemoryStore, Storage};
use ibc::ics02_client::client_def::{AnyClientState, AnyConsensusState};
use ibc::ics02_client::client_type::ClientType;
use ibc::ics02_client::context::{ClientKeeper, ClientReader};
use ibc::ics02_client::error::Error as ClientError;
use ibc::ics03_connection::connection::ConnectionEnd;
use ibc::ics03_connection::context::{ConnectionKeeper, ConnectionReader};
use ibc::ics03_connection::error::{Error as ConnectionError, Kind as ConnectionErrorKind};
use ibc::ics23_commitment::commitment::CommitmentPrefix;
use ibc::ics24_host::identifier::{ClientId, ConnectionId};
use ibc::Height;
use ibc_proto::ibc::core::connection::v1::ConnectionEnd as RawConnectionEnd;
use jsonrpc_core::serde::{Deserialize, Serialize};
use jsonrpc_core::serde_json;
use prost::Message;
use prost_types::Any;
use std::convert::TryFrom;
use std::str::FromStr;
use tendermint::chain;
use tendermint::net::Address;
use tendermint::node;
use tendermint_rpc::endpoint::status::SyncInfo;

// protobuf URL
const CONSENSUS_STATE_URL: &'static str = "/ibc.lightclients.tendermint.v1.ConsensusState";
const CLIENT_STATE_URL: &'static str = "/ibc.lightclients.tendermint.v1.ClientState";

// System constant
const COMMITMENT_PREFIX: &'static str = "store/ibc/key";

pub type SharedNode<S> = std::sync::Arc<std::sync::RwLock<Node<S>>>;

/// A type representing connections in memory
#[derive(Serialize, Deserialize)]
struct Connections {
    pub connections: Vec<String>,
}

impl Connections {
    pub fn new() -> Self {
        Connections {
            connections: Vec::new(),
        }
    }
}

pub struct Node<S: Storage> {
    store: S,
    chain: Chain,
    chain_id: tendermint::chain::Id,
    host_client_id: String,
    info: node::Info,
    consensus_params: tendermint::consensus::Params,
}

impl Node<InMemoryStore> {
    pub fn new(config: &Config) -> Self {
        // TODO: allow to pass custimized values
        let info = node::Info {
            // Node id
            id: node::Id::new([61; 20]),
            listen_addr: node::info::ListenAddress::new(String::from("localhost:26657")),
            network: chain::Id::from_str(&config.chain_id).unwrap(),
            protocol_version: node::info::ProtocolVersionInfo {
                p2p: 0,
                block: 0,
                app: 0,
            },
            version: serde_json::from_value(serde_json::Value::String("v0.1.0".to_string()))
                .unwrap(),
            channels: serde_json::from_value(serde_json::Value::String("channels".to_string()))
                .unwrap(),
            moniker: tendermint::Moniker::from_str("moniker").unwrap(),
            other: node::info::OtherInfo {
                tx_index: node::info::TxIndexStatus::Off,
                rpc_address: Address::from_str("tcp://127.0.0.1:26657").unwrap(),
            },
        };
        Node {
            store: InMemoryStore::new(),
            chain: Chain::new(),
            chain_id: tendermint::chain::Id::try_from(config.chain_id.to_owned()).unwrap(),
            host_client_id: config.host_client.id.to_owned(),
            consensus_params: config.consensus_params.clone(),
            info,
        }
    }
}

impl<S: Storage> Node<S> {
    pub fn get_store(&self) -> &S {
        &self.store
    }

    pub fn get_chain(&self) -> &Chain {
        &self.chain
    }

    pub fn get_info(&self) -> &node::Info {
        &self.info
    }

    pub fn get_chain_id(&self) -> &chain::Id {
        &self.chain_id
    }

    pub fn get_consensus_params(&self) -> &tendermint::consensus::Params {
        &self.consensus_params
    }

    /// Get sync infos. For now only the field `latest_block_height` contains a valid value.
    pub fn get_sync_info(&self) -> SyncInfo {
        let latest_block_height = self.chain.get_height();
        let block = self
            .chain
            .get_block(0)
            .expect("The chain should always contain a block");
        let hash = block.signed_header.header.hash();
        SyncInfo {
            latest_block_hash: hash,
            latest_app_hash: tendermint::AppHash::try_from(vec![61 as u8; 32]).unwrap(),
            latest_block_height: (latest_block_height.version_height as u32).into(),
            latest_block_time: block.signed_header.header.time,
            catching_up: false,
        }
    }
}

impl<S: Storage> ClientReader for Node<S> {
    fn client_type(&self, client_id: &ClientId) -> Option<ClientType> {
        let path = format!("clients/{}/clientType", client_id.as_str());
        let client_type = self.store.get(0, path.as_bytes())?;
        let client_type = String::from_utf8(client_type.to_vec());
        match client_type {
            Err(_) => None,
            Ok(client_type) => ClientType::from_str(&client_type).ok(),
        }
    }

    fn client_state(&self, client_id: &ClientId) -> Option<AnyClientState> {
        let path = format!("clients/{}/clientState", client_id.as_str());
        let value = self.store.get(0, path.as_bytes())?;
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
        let value = self.store.get(0, path.as_bytes())?;
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
        self.store.set(
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
        self.store.set(0, path.into_bytes(), data.value);
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
        self.store.set(0, path.into_bytes(), data.value);
        Ok(())
    }
}

impl<S: Storage> ConnectionKeeper for Node<S> {
    fn store_connection(
        &mut self,
        connection_id: &ConnectionId,
        connection_end: &ConnectionEnd,
    ) -> Result<(), ConnectionError> {
        let mut buffer = Vec::new();
        let path = format!("connections/{}", connection_id.as_str());
        let raw: RawConnectionEnd = connection_end.to_owned().into();
        raw.encode(&mut buffer).unwrap();
        self.get_store().set(0, path.into_bytes(), buffer);
        Ok(())
    }

    fn store_connection_to_client(
        &mut self,
        connection_id: &ConnectionId,
        client_id: &ClientId,
    ) -> Result<(), ConnectionError> {
        let path = format!("clients/{}/connections", client_id.as_str());
        let store = self.get_store();
        let connections = store.get(0, path.as_bytes()).unwrap_or(vec![]);
        let connections = String::from_utf8(connections).unwrap_or(String::from(""));
        let mut connections =
            serde_json::from_str::<Connections>(&connections).unwrap_or(Connections::new());
        connections
            .connections
            .push(connection_id.as_str().to_owned());
        self.get_store()
            .set(0, path.into_bytes(), connection_id.as_bytes().to_owned());
        Ok(())
    }
}

impl<S: Storage> ConnectionReader for Node<S> {
    fn connection_end(&self, connection_id: &ConnectionId) -> Option<ConnectionEnd> {
        let path = format!("connections/{}", connection_id.as_str());
        let value = self.store.get(0, path.as_bytes())?;
        let raw = RawConnectionEnd::decode(&*value).ok()?;
        ConnectionEnd::try_from(raw).ok()
    }

    fn client_state(&self, client_id: &ClientId) -> Option<AnyClientState> {
        <Node<S> as ClientReader>::client_state(self, client_id)
    }

    fn host_current_height(&self) -> Height {
        self.chain.get_height()
    }

    fn host_chain_history_size(&self) -> usize {
        100
    }

    fn commitment_prefix(&self) -> CommitmentPrefix {
        CommitmentPrefix::from(COMMITMENT_PREFIX.as_bytes().to_owned())
    }

    fn client_consensus_state(
        &self,
        client_id: &ClientId,
        height: Height,
    ) -> Option<AnyConsensusState> {
        self.consensus_state(client_id, height)
    }

    fn host_consensus_state(&self, height: Height) -> Option<AnyConsensusState> {
        let host_id = ClientId::from_str(&self.host_client_id).unwrap();
        self.consensus_state(&host_id, height)
    }

    // TODO: what is the correct version format?
    fn get_compatible_versions(&self) -> Vec<String> {
        vec![String::from("0.0.1")]
    }

    // TODO: what if there is no compatible versions?
    fn pick_version(
        &self,
        _supported_versions: Vec<String>,
        counterparty_candidate_versions: Vec<String>,
    ) -> Result<String, ConnectionError> {
        match counterparty_candidate_versions.get(0) {
            Some(version) => Ok(version.to_owned()),
            None => Err(ConnectionErrorKind::NoCommonVersion.into()),
        }
    }
}
