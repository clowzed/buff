/*
 * buff
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyForm {
    #[serde(rename = "openid.ns")]
    pub openid_period_ns: String,
    #[serde(rename = "openid.mode")]
    pub openid_period_mode: String,
    #[serde(rename = "openid.op_endpoint")]
    pub openid_period_op_endpoint: String,
    #[serde(rename = "openid.claimed_id")]
    pub openid_period_claimed_id: String,
    #[serde(
        rename = "openid.identity",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub openid_period_identity: Option<Option<String>>,
    #[serde(rename = "openid.return_to")]
    pub openid_period_return_to: String,
    #[serde(rename = "openid.response_nonce")]
    pub openid_period_response_nonce: String,
    #[serde(
        rename = "openid.invalidate_handle",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub openid_period_invalidate_handle: Option<Option<String>>,
    #[serde(rename = "openid.assoc_handle")]
    pub openid_period_assoc_handle: String,
    #[serde(rename = "openid.signed")]
    pub openid_period_signed: String,
    #[serde(rename = "openid.sig")]
    pub openid_period_sig: String,
}

impl VerifyForm {
    pub fn new(
        openid_period_ns: String,
        openid_period_mode: String,
        openid_period_op_endpoint: String,
        openid_period_claimed_id: String,
        openid_period_return_to: String,
        openid_period_response_nonce: String,
        openid_period_assoc_handle: String,
        openid_period_signed: String,
        openid_period_sig: String,
    ) -> VerifyForm {
        VerifyForm {
            openid_period_ns,
            openid_period_mode,
            openid_period_op_endpoint,
            openid_period_claimed_id,
            openid_period_identity: None,
            openid_period_return_to,
            openid_period_response_nonce,
            openid_period_invalidate_handle: None,
            openid_period_assoc_handle,
            openid_period_signed,
            openid_period_sig,
        }
    }
}
