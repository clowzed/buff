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
pub struct SetSocialUrlRequest {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(
        rename = "url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<Option<String>>,
}

impl SetSocialUrlRequest {
    pub fn new(id: String) -> SetSocialUrlRequest {
        SetSocialUrlRequest { id, url: None }
    }
}
