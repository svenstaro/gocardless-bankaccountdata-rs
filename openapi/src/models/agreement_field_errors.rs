/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AgreementFieldErrors {
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "status_code")]
    pub status_code: i32,
}

impl AgreementFieldErrors {
    pub fn new(summary: String, detail: String, status_code: i32) -> AgreementFieldErrors {
        AgreementFieldErrors {
            summary,
            detail,
            r#type: None,
            status_code,
        }
    }
}


