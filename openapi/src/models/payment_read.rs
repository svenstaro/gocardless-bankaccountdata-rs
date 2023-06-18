/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentRead : PaymentReadSerializer.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentRead {
    /// Payment ID
    #[serde(rename = "payment_id", skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(rename = "payment_status", skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<crate::models::PaymentStatusEnum>,
    #[serde(rename = "payment_product", skip_serializing_if = "Option::is_none")]
    pub payment_product: Option<crate::models::PaymentProductEnum>,
    #[serde(rename = "payment_type", skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<crate::models::PaymentTypeEnum>,
    /// Redirect URL to your application after payment is done
    #[serde(rename = "redirect", deserialize_with = "Option::deserialize")]
    pub redirect: Option<String>,
    /// Payment description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Payment Custom Payment ID
    #[serde(rename = "custom_payment_id", skip_serializing_if = "Option::is_none")]
    pub custom_payment_id: Option<String>,
    /// Registered creditor account
    #[serde(rename = "creditor_account", skip_serializing_if = "Option::is_none")]
    pub creditor_account: Option<uuid::Uuid>,
    #[serde(rename = "creditor_object", skip_serializing_if = "Option::is_none")]
    pub creditor_object: Option<Box<crate::models::PaymentReadCreditorObject>>,
    #[serde(rename = "debtor_account")]
    pub debtor_account: Box<crate::models::DebtorAccountWrite>,
    #[serde(rename = "instructed_amount")]
    pub instructed_amount: Box<crate::models::PaymentReadInstructedAmount>,
}

impl PaymentRead {
    /// PaymentReadSerializer.
    pub fn new(redirect: Option<String>, debtor_account: crate::models::DebtorAccountWrite, instructed_amount: crate::models::PaymentReadInstructedAmount) -> PaymentRead {
        PaymentRead {
            payment_id: None,
            payment_status: None,
            payment_product: None,
            payment_type: None,
            redirect,
            description: None,
            custom_payment_id: None,
            creditor_account: None,
            creditor_object: None,
            debtor_account: Box::new(debtor_account),
            instructed_amount: Box::new(instructed_amount),
        }
    }
}


