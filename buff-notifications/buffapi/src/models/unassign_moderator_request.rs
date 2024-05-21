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
pub struct UnassignModeratorRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
}

impl UnassignModeratorRequest {
    pub fn new(order_id: String) -> UnassignModeratorRequest {
        UnassignModeratorRequest { order_id }
    }
}
