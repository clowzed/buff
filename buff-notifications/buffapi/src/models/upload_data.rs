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
pub struct UploadData {
    #[serde(
        rename = "image",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image: Option<Option<std::path::PathBuf>>,
    #[serde(rename = "text")]
    pub text: String,
}

impl UploadData {
    pub fn new(text: String) -> UploadData {
        UploadData { image: None, text }
    }
}
