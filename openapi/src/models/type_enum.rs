/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TypeEnum : * `IBAN` - IBAN * `SCAN` - SortCodeAccountNumber

/// * `IBAN` - IBAN * `SCAN` - SortCodeAccountNumber
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "IBAN")]
    Iban,
    #[serde(rename = "SCAN")]
    Scan,

}

impl ToString for TypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Iban => String::from("IBAN"),
            Self::Scan => String::from("SCAN"),
        }
    }
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Iban
    }
}



