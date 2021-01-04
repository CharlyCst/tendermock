//! # gRPC Staking
//!
//! The staking tendermint gRPC API.
use crate::node;
use crate::store::Storage;
use ibc_proto::cosmos::staking::v1beta1;
use ibc_proto::cosmos::staking::v1beta1::query_server::{Query, QueryServer};

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
    async fn validator(
        &self,
        request: tonic::Request<v1beta1::QueryValidatorRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryValidatorResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/validator {:?}", request);
        }
        unimplemented!();
    }

    async fn validators(
        &self,
        request: tonic::Request<v1beta1::QueryValidatorsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryValidatorsResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/validators {:?}", request);
        }
        unimplemented!();
    }

    async fn validator_delegations(
        &self,
        request: tonic::Request<v1beta1::QueryValidatorDelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryValidatorDelegationsResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/validator_delegations {:?}", request);
        }
        unimplemented!();
    }

    async fn validator_unbonding_delegations(
        &self,
        request: tonic::Request<v1beta1::QueryValidatorUnbondingDelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryValidatorUnbondingDelegationsResponse>, tonic::Status>
    {
        if self.verbose {
            println!("[gRPC] staking/validator_unbounding_delegations {:?}", request);
        }
        unimplemented!();
    }

    async fn delegation(
        &self,
        request: tonic::Request<v1beta1::QueryDelegationRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegationResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/delegation {:?}", request);
        }
        unimplemented!();
    }

    async fn unbonding_delegation(
        &self,
        request: tonic::Request<v1beta1::QueryUnbondingDelegationRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryUnbondingDelegationResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/unbounding_delegation {:?}", request);
        }
        unimplemented!();
    }

    async fn delegator_validator(
        &self,
        request: tonic::Request<v1beta1::QueryDelegatorValidatorRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegatorValidatorResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/delegator_validator {:?}", request);
        }
        unimplemented!();
    }

    async fn delegator_delegations(
        &self,
        request: tonic::Request<v1beta1::QueryDelegatorDelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegatorDelegationsResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/delegator_delegations {:?}", request);
        }
        unimplemented!();
    }

    async fn delegator_unbonding_delegations(
        &self,
        request: tonic::Request<v1beta1::QueryDelegatorUnbondingDelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegatorUnbondingDelegationsResponse>, tonic::Status>
    {
        if self.verbose {
            println!("[gRPC] staking/delegator_unbounding_delegations {:?}", request);
        }
        unimplemented!();
    }

    async fn redelegations(
        &self,
        request: tonic::Request<v1beta1::QueryRedelegationsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryRedelegationsResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/redelegations {:?}", request);
        }
        unimplemented!();
    }

    async fn delegator_validators(
        &self,
        request: tonic::Request<v1beta1::QueryDelegatorValidatorsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryDelegatorValidatorsResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/delegator_validators {:?}", request);
        }
        unimplemented!();
    }

    async fn historical_info(
        &self,
        request: tonic::Request<v1beta1::QueryHistoricalInfoRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryHistoricalInfoResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/historical_info {:?}", request);
        }
        unimplemented!();
    }

    async fn pool(
        &self,
        request: tonic::Request<v1beta1::QueryPoolRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryPoolResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/pool   {:?}", request);
        }
        unimplemented!();
    }

    async fn params(
        &self,
        request: tonic::Request<v1beta1::QueryParamsRequest>,
    ) -> Result<tonic::Response<v1beta1::QueryParamsResponse>, tonic::Status> {
        if self.verbose {
            println!("[gRPC] staking/params {:?}", request);
        }
        let response = v1beta1::QueryParamsResponse {
            params: Some(v1beta1::Params {
                bond_denom: "bond_denom".to_owned(),
                historical_entries: 0,
                max_entries: 3,
                max_validators: 3,
                unbonding_time: Some(std::time::Duration::new(3600 * 24 * 30, 0).into()),
            }),
        };
        Ok(tonic::Response::new(response))
    }
}
