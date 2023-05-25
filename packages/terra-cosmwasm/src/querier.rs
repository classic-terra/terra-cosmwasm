use cosmwasm_std::{Coin, QuerierWrapper, StdResult, QueryRequest};

use crate::query::{
    /*ContractInfoResponse, */ExchangeRatesResponse, SwapResponse, TaxCapResponse, TaxRateResponse,
     TerraQueryWrapper
};

/// This is a helper wrapper to easily use our custom queries
pub struct TerraQuerier<'a> {
    querier: &'a QuerierWrapper<'a, TerraQueryWrapper>,
}

impl<'a> TerraQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, TerraQueryWrapper>) -> Self {
        TerraQuerier { querier }
    }

    pub fn query_swap<T: Into<String>>(
        &self,
        offer_coin: Coin,
        ask_denom: T,
    ) -> StdResult<SwapResponse> {
        let request = QueryRequest::Custom(TerraQueryWrapper::Swap {
            offer_coin,
            ask_denom: ask_denom.into(),
        })
        .into();

        self.querier.query(&request)
    }

    pub fn query_tax_cap<T: Into<String>>(&self, denom: T) -> StdResult<TaxCapResponse> {
        let request = QueryRequest::Custom(TerraQueryWrapper::TaxCap {
            denom: denom.into(),
        })
        .into();

        self.querier.query(&request)
    }

    pub fn query_tax_rate(&self) -> StdResult<TaxRateResponse> {
        let request = QueryRequest::Custom(TerraQueryWrapper::TaxRate {
        })
        .into();

        self.querier.query(&request)
    }

    pub fn query_exchange_rates<T: Into<String>>(
        &self,
        base_denom: T,
        quote_denoms: Vec<T>,
    ) -> StdResult<ExchangeRatesResponse> {
        let request = QueryRequest::Custom(TerraQueryWrapper::ExchangeRates { 
            base_denom: base_denom.into(),
            quote_denoms: quote_denoms.into_iter().map(|x| x.into()).collect(),
        })
        .into();

        self.querier.query(&request)
    }

    // not supported
    /*pub fn query_contract_info<T: Into<String>>(
        &self,
        contract_address: T,
    ) -> StdResult<ContractInfoResponse> {
        let request = QueryRequest::Custom(TerraQueryWrapper::ContractInfo {
            contract_address: contract_address.into(),
        })
        .into();

        self.querier.query(&request)
    }*/
}
