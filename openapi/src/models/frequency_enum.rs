/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FrequencyEnum : * `Daily` - Daily * `Weekly` - Weekly * `EveryTwoWeeks` - EveryTwoWeeks * `Monthly` - Monthly * `EveryTwoMonths` - EveryTwoMonths * `Quarterly` - Quarterly * `SemiAnnual` - SemiAnnual * `Annual` - Annual * `MonthlyVariable` - MonthlyVariable

/// * `Daily` - Daily * `Weekly` - Weekly * `EveryTwoWeeks` - EveryTwoWeeks * `Monthly` - Monthly * `EveryTwoMonths` - EveryTwoMonths * `Quarterly` - Quarterly * `SemiAnnual` - SemiAnnual * `Annual` - Annual * `MonthlyVariable` - MonthlyVariable
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FrequencyEnum {
    #[serde(rename = "Daily")]
    Daily,
    #[serde(rename = "Weekly")]
    Weekly,
    #[serde(rename = "EveryTwoWeeks")]
    EveryTwoWeeks,
    #[serde(rename = "Monthly")]
    Monthly,
    #[serde(rename = "EveryTwoMonths")]
    EveryTwoMonths,
    #[serde(rename = "Quarterly")]
    Quarterly,
    #[serde(rename = "SemiAnnual")]
    SemiAnnual,
    #[serde(rename = "Annual")]
    Annual,
    #[serde(rename = "MonthlyVariable")]
    MonthlyVariable,

}

impl ToString for FrequencyEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Daily => String::from("Daily"),
            Self::Weekly => String::from("Weekly"),
            Self::EveryTwoWeeks => String::from("EveryTwoWeeks"),
            Self::Monthly => String::from("Monthly"),
            Self::EveryTwoMonths => String::from("EveryTwoMonths"),
            Self::Quarterly => String::from("Quarterly"),
            Self::SemiAnnual => String::from("SemiAnnual"),
            Self::Annual => String::from("Annual"),
            Self::MonthlyVariable => String::from("MonthlyVariable"),
        }
    }
}

impl Default for FrequencyEnum {
    fn default() -> FrequencyEnum {
        Self::Daily
    }
}



