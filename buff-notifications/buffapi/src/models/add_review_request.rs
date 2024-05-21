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
pub struct AddReviewRequest {
    #[serde(rename = "review")]
    pub review: String,
    #[serde(rename = "stars")]
    pub stars: i32,
}

impl AddReviewRequest {
    pub fn new(review: String, stars: i32) -> AddReviewRequest {
        AddReviewRequest { review, stars }
    }
}
