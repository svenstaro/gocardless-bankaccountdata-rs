/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentWriteRequest : PaymentWriteSerializer.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentWriteRequest {
    /// Institution ID for Payment
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    #[serde(rename = "payment_product", skip_serializing_if = "Option::is_none")]
    pub payment_product: Option<crate::models::PaymentProductEnum>,
    #[serde(rename = "instructed_amount")]
    pub instructed_amount: Box<crate::models::PaymentReadRequestInstructedAmount>,
    /// Registered creditor account
    #[serde(rename = "creditor_account", skip_serializing_if = "Option::is_none")]
    pub creditor_account: Option<uuid::Uuid>,
    #[serde(rename = "creditor_object", skip_serializing_if = "Option::is_none")]
    pub creditor_object: Option<Box<crate::models::PaymentReadRequestCreditorObject>>,
    #[serde(rename = "debtor_account", skip_serializing_if = "Option::is_none")]
    pub debtor_account: Option<Box<crate::models::PaymentWriteRequestDebtorAccount>>,
    /// Redirect URL to your application after payment is done
    #[serde(rename = "redirect", deserialize_with = "Option::deserialize")]
    pub redirect: Option<String>,
    /// Payment description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Payment Custom Payment ID
    #[serde(rename = "custom_payment_id", skip_serializing_if = "Option::is_none")]
    pub custom_payment_id: Option<String>,
    /// Payment Execution date (for periodic payments)
    #[serde(rename = "requested_execution_date", skip_serializing_if = "Option::is_none")]
    pub requested_execution_date: Option<String>,
    #[serde(rename = "periodic_payment", skip_serializing_if = "Option::is_none")]
    pub periodic_payment: Option<Box<crate::models::PeriodicPaymentRequest>>,
    /// Indicates whether payment should be submitted separately
    #[serde(rename = "submit_payment", skip_serializing_if = "Option::is_none")]
    pub submit_payment: Option<bool>,
}

impl PaymentWriteRequest {
    /// PaymentWriteSerializer.
    pub fn new(instructed_amount: crate::models::PaymentReadRequestInstructedAmount, redirect: Option<String>) -> PaymentWriteRequest {
        PaymentWriteRequest {
            institution_id: None,
            payment_product: None,
            instructed_amount: Box::new(instructed_amount),
            creditor_account: None,
            creditor_object: None,
            debtor_account: None,
            redirect,
            description: None,
            custom_payment_id: None,
            requested_execution_date: None,
            periodic_payment: None,
            submit_payment: None,
        }
    }
}


