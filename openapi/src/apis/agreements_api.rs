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


/// struct for typed errors of method [`accept_eua`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcceptEuaError {
    Status405(crate::models::EuaAccepted),
    Status403(crate::models::IpAccessDenied),
    Status400(crate::models::InvalidId),
    Status404(crate::models::NotFoundError),
    Status429(crate::models::NordigenRateLimitExceeded),
    Status401(crate::models::InvalidToken),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_eua_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEuaV2Error {
    Status400(crate::models::AgreementFieldErrors),
    Status429(crate::models::NordigenRateLimitExceeded),
    Status402(crate::models::PaymentRequired),
    Status401(crate::models::InvalidToken),
    Status403(crate::models::IpAccessDenied),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_eua_by_id_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEuaByIdV2Error {
    Status400(crate::models::InvalidId),
    Status404(crate::models::NotFoundError),
    Status429(crate::models::NordigenRateLimitExceeded),
    Status401(crate::models::InvalidToken),
    Status403(crate::models::IpAccessDenied),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_all_euas_for_an_end_user_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveAllEuasForAnEndUserV2Error {
    Status404(crate::models::NotFoundError),
    Status429(crate::models::NordigenRateLimitExceeded),
    Status401(crate::models::InvalidToken),
    Status403(crate::models::IpAccessDenied),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_eua_by_id_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveEuaByIdV2Error {
    Status404(crate::models::NotFoundError),
    Status400(crate::models::InvalidId),
    Status429(crate::models::NordigenRateLimitExceeded),
    Status401(crate::models::InvalidToken),
    Status403(crate::models::IpAccessDenied),
    UnknownValue(serde_json::Value),
}


/// Accept an end-user agreement via the API
pub async fn accept_eua(configuration: &configuration::Configuration, id: &str, enduser_acceptance_details_request: crate::models::EnduserAcceptanceDetailsRequest) -> Result<crate::models::EndUserAgreement, Error<AcceptEuaError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/agreements/enduser/{id}/accept/", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&enduser_acceptance_details_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AcceptEuaError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// API endpoints related to end-user agreements.
pub async fn create_eua_v2(configuration: &configuration::Configuration, end_user_agreement_request: crate::models::EndUserAgreementRequest) -> Result<crate::models::EndUserAgreement, Error<CreateEuaV2Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/agreements/enduser/", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&end_user_agreement_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateEuaV2Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an end user agreement
pub async fn delete_eua_by_id_v2(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<DeleteEuaByIdV2Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/agreements/enduser/{id}/", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        Ok(())
    } else {
        let local_var_entity: Option<DeleteEuaByIdV2Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// API endpoints related to end-user agreements.
pub async fn retrieve_all_euas_for_an_end_user_v2(configuration: &configuration::Configuration, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::PaginatedEndUserAgreementList, Error<RetrieveAllEuasForAnEndUserV2Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/agreements/enduser/", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
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
        let local_var_entity: Option<RetrieveAllEuasForAnEndUserV2Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve end user agreement by ID
pub async fn retrieve_eua_by_id_v2(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::EndUserAgreement, Error<RetrieveEuaByIdV2Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/agreements/enduser/{id}/", local_var_configuration.base_path, id=crate::apis::urlencode(id));
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
        let local_var_entity: Option<RetrieveEuaByIdV2Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
