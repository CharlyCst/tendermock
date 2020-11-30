use crate::node;
use crate::store::Storage;
use ibc_proto::cosmos::staking::v1beta1;
use ibc_proto::cosmos::staking::v1beta1::query_server::{Query, QueryServer};

type GrpcServer<T> = tonic::transport::server::Router<T, tonic::transport::server::Unimplemented>;

pub fn new<S: 'static + Storage + Sync + Send>(
    node: node::SharedNode<S>,
) -> GrpcServer<QueryServer<QueryService<S>>> {
    let query_service = QueryService::new(node);
    tonic::transport::Server::builder().add_service(QueryServer::new(query_service))
}

/// A struct handling the `Query` service.
pub struct QueryService<S: Storage> {
    node: node::SharedNode<S>,
}

impl<S: Storage> QueryService<S> {
    fn new(node: node::SharedNode<S>) -> Self {
        QueryService { node }
    }
}

#[tonic::async_trait]
impl<S: 'static + Storage + Sync + Send> Query for QueryService<S> {
    async fn validator(
        &self,
        _request: tonic::Request<v1beta1::QueryValidatorRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryValidatorResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn validators(
        &self,
        _request: tonic::Request<v1beta1::QueryValidatorsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryValidatorsResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn validator_delegations(
        &self,
        _request: tonic::Request<v1beta1::QueryValidatorDelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryValidatorDelegationsResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn validator_unbonding_delegations(
        &self,
        _request: tonic::Request<v1beta1::QueryValidatorUnbondingDelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryValidatorUnbondingDelegationsResponse>, tonic::Status>
    {
        unimplemented!();
    }

    async fn delegation(
        &self,
        _request: tonic::Request<v1beta1::QueryDelegationRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegationResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn unbonding_delegation(
        &self,
        _request: tonic::Request<v1beta1::QueryUnbondingDelegationRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryUnbondingDelegationResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn delegator_validator(
        &self,
        _request: tonic::Request<v1beta1::QueryDelegatorValidatorRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegatorValidatorResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn delegator_delegations(
        &self,
        _request: tonic::Request<v1beta1::QueryDelegatorDelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegatorDelegationsResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn delegator_unbonding_delegations(
        &self,
        _request: tonic::Request<v1beta1::QueryDelegatorUnbondingDelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegatorUnbondingDelegationsResponse>, tonic::Status>
    {
        unimplemented!();
    }

    async fn redelegations(
        &self,
        _request: tonic::Request<v1beta1::QueryRedelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryRedelegationsResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn delegator_validators(
        &self,
        _request: tonic::Request<v1beta1::QueryDelegatorValidatorsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegatorValidatorsResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn historical_info(
        &self,
        _request: tonic::Request<v1beta1::QueryHistoricalInfoRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryHistoricalInfoResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn pool(
        &self,
        _request: tonic::Request<v1beta1::QueryPoolRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryPoolResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn params(
        &self,
        _request: tonic::Request<v1beta1::QueryParamsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryParamsResponse>, tonic::Status> {
        println!("Param request!");
        //let node = self.node.read().unwrap();
        let response = v1beta1::QueryParamsResponse {
            params: Some(v1beta1::Params {
                bond_denom: "bond_denom".to_owned(),
                historical_entries: 0,
                max_entries: 3,
                max_validators: 3,
                unbonding_time: Some(std::time::Duration::new(3600, 0).into()),
            }),
        };
        Ok(tonic::Response::new(response))
    }
}
