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
pub struct BlacklistUserRequest {
    #[serde(rename = "steam_id")]
    pub steam_id: String,
}

impl BlacklistUserRequest {
    pub fn new(steam_id: String) -> BlacklistUserRequest {
        BlacklistUserRequest { steam_id }
    }
}
