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
pub struct AdminLoginResponse {
    #[serde(rename = "token")]
    pub token: String,
}

impl AdminLoginResponse {
    pub fn new(token: String) -> AdminLoginResponse {
        AdminLoginResponse { token }
    }
}
