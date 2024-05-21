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
pub struct ModeratorCredentials {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl ModeratorCredentials {
    pub fn new(login: String, password: String) -> ModeratorCredentials {
        ModeratorCredentials { login, password }
    }
}
