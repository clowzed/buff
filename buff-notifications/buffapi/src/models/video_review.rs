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
pub struct VideoReview {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "avatar")]
    pub avatar: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "subscribers")]
    pub subscribers: String,
}

impl VideoReview {
    pub fn new(
        id: String,
        url: String,
        avatar: String,
        name: String,
        subscribers: String,
    ) -> VideoReview {
        VideoReview {
            id,
            url,
            avatar,
            name,
            subscribers,
        }
    }
}