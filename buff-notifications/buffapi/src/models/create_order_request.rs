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
pub struct CreateOrderRequest {
    #[serde(rename = "payment_method")]
    pub payment_method: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "requisites_id")]
    pub requisites_id: String,
}

impl CreateOrderRequest {
    pub fn new(
        payment_method: String,
        amount: String,
        currency: String,
        requisites_id: String,
    ) -> CreateOrderRequest {
        CreateOrderRequest {
            payment_method,
            amount,
            currency,
            requisites_id,
        }
    }
}
