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
pub struct GetChatRequest {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
}

impl GetChatRequest {
    pub fn new(id: String, order_id: String) -> GetChatRequest {
        GetChatRequest { id, order_id }
    }
}
