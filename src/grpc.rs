use ibc_proto::cosmos::staking::v1beta1;
use ibc_proto::cosmos::staking::v1beta1::query_server::{Query, QueryServer};

type GrpcServer<T> = tonic::transport::server::Router<T, tonic::transport::server::Unimplemented>;

/// A grcp server.
pub struct Server {}

impl Server {
    pub fn new() -> GrpcServer<QueryServer<QueryService>> {
        let query_service = QueryService::new();
        tonic::transport::Server::builder().add_service(QueryServer::new(query_service))
    }
}

/// A struct handling the `Query` service.
pub struct QueryService {}

impl QueryService {
    fn new() -> Self {
        QueryService {}
    }
}

#[tonic::async_trait]
impl Query for QueryService {
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
        unimplemented!();
    }
}
