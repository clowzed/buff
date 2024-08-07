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
pub struct JwtResponse {
    #[serde(rename = "token")]
    pub token: String,
}

impl JwtResponse {
    pub fn new(token: String) -> JwtResponse {
        JwtResponse { token }
    }
}
