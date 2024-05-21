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
pub struct UpdateVideoReviewRequest {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(
        rename = "url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<Option<String>>,
    #[serde(
        rename = "avatar",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avatar: Option<Option<String>>,
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    #[serde(
        rename = "subscribers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscribers: Option<Option<String>>,
}

impl UpdateVideoReviewRequest {
    pub fn new(id: String) -> UpdateVideoReviewRequest {
        UpdateVideoReviewRequest {
            id,
            url: None,
            avatar: None,
            name: None,
            subscribers: None,
        }
    }
}