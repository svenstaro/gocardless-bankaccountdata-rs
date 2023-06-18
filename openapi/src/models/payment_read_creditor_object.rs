/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentReadCreditorObject : Creditor account



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentReadCreditorObject {
    /// Unique entry ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Creditor account name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::TypeEnum>,
    /// Creditor account type identifier
    #[serde(rename = "account")]
    pub account: String,
    /// Creditor account currency
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "address_country", skip_serializing_if = "Option::is_none")]
    pub address_country: Option<crate::models::AddressCountryEnum>,
    /// an Institution ID for this CreditorAccount
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    /// Creditor account BICFI Identifier
    #[serde(rename = "agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Creditor account agent name
    #[serde(rename = "agent_name", skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// Creditor account address street
    #[serde(rename = "address_street", skip_serializing_if = "Option::is_none")]
    pub address_street: Option<String>,
    /// Creditor account address post code
    #[serde(rename = "post_code", skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
}

impl PaymentReadCreditorObject {
    /// Creditor account
    pub fn new(name: String, account: String, currency: String) -> PaymentReadCreditorObject {
        PaymentReadCreditorObject {
            id: None,
            name,
            r#type: None,
            account,
            currency,
            address_country: None,
            institution_id: None,
            agent: None,
            agent_name: None,
            address_street: None,
            post_code: None,
        }
    }
}


