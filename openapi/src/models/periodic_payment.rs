/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PeriodicPayment : Periodic Payment Serializer.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PeriodicPayment {
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<crate::models::FrequencyEnum>,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "execution_rule", skip_serializing_if = "Option::is_none")]
    pub execution_rule: Option<crate::models::ExecutionRuleEnum>,
    #[serde(rename = "day_of_execution", skip_serializing_if = "Option::is_none")]
    pub day_of_execution: Option<String>,
}

impl PeriodicPayment {
    /// Periodic Payment Serializer.
    pub fn new(start_date: String) -> PeriodicPayment {
        PeriodicPayment {
            frequency: None,
            start_date,
            end_date: None,
            execution_rule: None,
            day_of_execution: None,
        }
    }
}


