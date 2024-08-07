/*
 * buff
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};

/// struct for typed errors of method [`login`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginError {
    Status400(models::Details),
    Status403(models::Details),
    Status500(models::Details),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`login_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginLinkError {
    UnknownValue(serde_json::Value),
}

pub async fn login(
    configuration: &configuration::Configuration,
    openid_period_ns: &str,
    openid_period_mode: &str,
    openid_period_op_endpoint: &str,
    openid_period_claimed_id: &str,
    openid_period_return_to: &str,
    openid_period_response_nonce: &str,
    openid_period_assoc_handle: &str,
    openid_period_signed: &str,
    openid_period_sig: &str,
    openid_period_identity: Option<&str>,
    openid_period_invalidate_handle: Option<&str>,
) -> Result<models::JwtResponse, Error<LoginError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/auth/user/callback",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder =
        local_var_req_builder.query(&[("openid.ns", &openid_period_ns.to_string())]);
    local_var_req_builder =
        local_var_req_builder.query(&[("openid.mode", &openid_period_mode.to_string())]);
    local_var_req_builder = local_var_req_builder
        .query(&[("openid.op_endpoint", &openid_period_op_endpoint.to_string())]);
    local_var_req_builder = local_var_req_builder
        .query(&[("openid.claimed_id", &openid_period_claimed_id.to_string())]);
    if let Some(ref local_var_str) = openid_period_identity {
        local_var_req_builder =
            local_var_req_builder.query(&[("openid.identity", &local_var_str.to_string())]);
    }
    local_var_req_builder =
        local_var_req_builder.query(&[("openid.return_to", &openid_period_return_to.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[(
        "openid.response_nonce",
        &openid_period_response_nonce.to_string(),
    )]);
    if let Some(ref local_var_str) = openid_period_invalidate_handle {
        local_var_req_builder = local_var_req_builder
            .query(&[("openid.invalidate_handle", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[(
        "openid.assoc_handle",
        &openid_period_assoc_handle.to_string(),
    )]);
    local_var_req_builder =
        local_var_req_builder.query(&[("openid.signed", &openid_period_signed.to_string())]);
    local_var_req_builder =
        local_var_req_builder.query(&[("openid.sig", &openid_period_sig.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LoginError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn login_link(
    configuration: &configuration::Configuration,
) -> Result<models::LoginLinkResponse, Error<LoginLinkError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/auth/user/link", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LoginLinkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
