//! # gRPC Auth
//!
//! The auth tendermint gRPC API.
use crate::node;
use crate::store::Storage;
use ibc_proto::cosmos::auth::v1beta1;
use ibc_proto::cosmos::auth::v1beta1::query_server::{Query, QueryServer};

pub fn get_service<S: 'static + Storage + Sync + Send>(
    node: node::SharedNode<S>,
    verbose: bool,
) -> QueryServer<QueryService<S>> {
    let query_service = QueryService::new(node, verbose);
    QueryServer::new(query_service)
}

/// A struct handling the `Query` service.
#[derive(Clone)]
pub struct QueryService<S: Storage> {
    #[allow(dead_code)]
    node: node::SharedNode<S>,
    verbose: bool,
}

impl<S: Storage> QueryService<S> {
    fn new(node: node::SharedNode<S>, verbose: bool) -> Self {
        QueryService { node, verbose }
    }
}

#[tonic::async_trait]
impl<S: 'static + Storage + Sync + Send> Query for QueryService<S> {
    async fn account(
        &self,
        request: tonic::Request<v1beta1::QueryAccountRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryAccountResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] /account {:?}", request);
        }
        // let response = v1beta1::QueryAccountResponse {
        //     account: Some(v1beta1::BaseAccount {
        //         address: String::from("ACCOUNT_ADDRESS"),
        //         pub_key: None,
        //         account_number: 42,
        //         sequence: 42,
        //     }),
        // };
        let response = v1beta1::QueryAccountResponse {
            account: None,
        };
        Ok(tonic::Response::new(response))
    }

    async fn params(
        &self,
        request: tonic::Request<v1beta1::QueryParamsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryParamsResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] /params {:?}", request);
        }
        unimplemented!()
    }
}
