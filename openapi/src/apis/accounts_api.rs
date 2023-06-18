/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`retrieve_account_balances_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveAccountBalancesV2Error {
    Status404(crate::models::AccountNotFoundError),
    Status429(crate::models::RateLimitError),
    Status401(crate::models::SsnverificationFailed),
    Status403(crate::models::AccountAccessForbidden),
    Status400(crate::models::DateRangeError),
    Status500(crate::models::UnknownRequestError),
    Status409(crate::models::AccountStateError),
    Status503(crate::models::ConnectionError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_account_details_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveAccountDetailsV2Error {
    Status404(crate::models::AccountNotFoundError),
    Status429(crate::models::RateLimitError),
    Status401(crate::models::SsnverificationFailed),
    Status403(crate::models::AccountAccessForbidden),
    Status400(crate::models::DateRangeError),
    Status500(crate::models::UnknownRequestError),
    Status409(crate::models::AccountStateError),
    Status503(crate::models::ConnectionError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_account_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveAccountMetadataError {
    Status404(crate::models::AccountNotFoundError),
    Status429(crate::models::NordigenRateLimitExceeded),
    Status401(crate::models::InvalidToken),
    Status403(crate::models::IpAccessDenied),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_account_transactions_v22`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveAccountTransactionsV22Error {
    Status404(crate::models::AccountNotFoundError),
    Status429(crate::models::RateLimitError),
    Status401(crate::models::SsnverificationFailed),
    Status403(crate::models::AccountAccessForbidden),
    Status400(crate::models::DateRangeError),
    Status500(crate::models::UnknownRequestError),
    Status409(crate::models::AccountStateError),
    Status503(crate::models::ConnectionError),
    UnknownValue(serde_json::Value),
}


/// Access account balances.  Balances will be returned in Berlin Group PSD2 format.
pub async fn retrieve_account_balances_v2(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::RetrieveAccountBalances, Error<RetrieveAccountBalancesV2Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/accounts/{id}/balances/", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveAccountBalancesV2Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Access account details.  Account details will be returned in Berlin Group PSD2 format.
pub async fn retrieve_account_details_v2(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::RetrieveAccountDetails, Error<RetrieveAccountDetailsV2Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/accounts/{id}/details/", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveAccountDetailsV2Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Access account metadata.  Information about the account record, such as the processing status and IBAN.  Account status is recalculated based on the error count in the latest req.
pub async fn retrieve_account_metadata(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::Account, Error<RetrieveAccountMetadataError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/accounts/{id}/", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveAccountMetadataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Access account transactions.  Transactions will be returned in Berlin Group PSD2 format.
pub async fn retrieve_account_transactions_v22(configuration: &configuration::Configuration, id: &str, date_from: Option<String>, date_to: Option<String>) -> Result<crate::models::RetrieveAccountTransactions, Error<RetrieveAccountTransactionsV22Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/accounts/{id}/transactions/", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = date_from {
        local_var_req_builder = local_var_req_builder.query(&[("date_from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = date_to {
        local_var_req_builder = local_var_req_builder.query(&[("date_to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveAccountTransactionsV22Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

