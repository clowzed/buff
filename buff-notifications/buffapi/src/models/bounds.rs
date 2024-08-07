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
pub struct Bounds {
    #[serde(rename = "limit")]
    pub limit: String,
    #[serde(rename = "offset")]
    pub offset: String,
}

impl Bounds {
    pub fn new(limit: String, offset: String) -> Bounds {
        Bounds { limit, offset }
    }
}
