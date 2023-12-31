/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentReadRequestInstructedAmount : Instructed amount



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentReadRequestInstructedAmount {
    /// Instructed amount
    #[serde(rename = "amount")]
    pub amount: String,
    /// Instructed amount currency
    #[serde(rename = "currency")]
    pub currency: String,
}

impl PaymentReadRequestInstructedAmount {
    /// Instructed amount
    pub fn new(amount: String, currency: String) -> PaymentReadRequestInstructedAmount {
        PaymentReadRequestInstructedAmount {
            amount,
            currency,
        }
    }
}


