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
pub struct StatusRequest {
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
}

impl StatusRequest {
    pub fn new(ids: Vec<String>) -> StatusRequest {
        StatusRequest { ids }
    }
}
