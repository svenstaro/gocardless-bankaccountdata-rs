/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CountryEnum : * `AT` - Austria * `BE` - Belgium * `BG` - Bulgaria * `HR` - Croatia * `CY` - Cyprus * `CZ` - Czechia * `DK` - Denmark * `EE` - Estonia * `FI` - Finland * `FR` - France * `DE` - Germany * `GR` - Greece * `HU` - Hungary * `IS` - Iceland * `IE` - Ireland * `IT` - Italy * `LV` - Latvia * `LI` - Liechtenstein * `LT` - Lithuania * `LU` - Luxembourg * `MT` - Malta * `NL` - Netherlands * `NO` - Norway * `PL` - Poland * `PT` - Portugal * `RO` - Romania * `SK` - Slovakia * `SI` - Slovenia * `ES` - Spain * `SE` - Sweden * `GB` - United Kingdom * `US` - United States of America

/// * `AT` - Austria * `BE` - Belgium * `BG` - Bulgaria * `HR` - Croatia * `CY` - Cyprus * `CZ` - Czechia * `DK` - Denmark * `EE` - Estonia * `FI` - Finland * `FR` - France * `DE` - Germany * `GR` - Greece * `HU` - Hungary * `IS` - Iceland * `IE` - Ireland * `IT` - Italy * `LV` - Latvia * `LI` - Liechtenstein * `LT` - Lithuania * `LU` - Luxembourg * `MT` - Malta * `NL` - Netherlands * `NO` - Norway * `PL` - Poland * `PT` - Portugal * `RO` - Romania * `SK` - Slovakia * `SI` - Slovenia * `ES` - Spain * `SE` - Sweden * `GB` - United Kingdom * `US` - United States of America
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CountryEnum {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "US")]
    Us,

}

impl ToString for CountryEnum {
    fn to_string(&self) -> String {
        match self {
            Self::At => String::from("AT"),
            Self::Be => String::from("BE"),
            Self::Bg => String::from("BG"),
            Self::Hr => String::from("HR"),
            Self::Cy => String::from("CY"),
            Self::Cz => String::from("CZ"),
            Self::Dk => String::from("DK"),
            Self::Ee => String::from("EE"),
            Self::Fi => String::from("FI"),
            Self::Fr => String::from("FR"),
            Self::De => String::from("DE"),
            Self::Gr => String::from("GR"),
            Self::Hu => String::from("HU"),
            Self::Is => String::from("IS"),
            Self::Ie => String::from("IE"),
            Self::It => String::from("IT"),
            Self::Lv => String::from("LV"),
            Self::Li => String::from("LI"),
            Self::Lt => String::from("LT"),
            Self::Lu => String::from("LU"),
            Self::Mt => String::from("MT"),
            Self::Nl => String::from("NL"),
            Self::No => String::from("NO"),
            Self::Pl => String::from("PL"),
            Self::Pt => String::from("PT"),
            Self::Ro => String::from("RO"),
            Self::Sk => String::from("SK"),
            Self::Si => String::from("SI"),
            Self::Es => String::from("ES"),
            Self::Se => String::from("SE"),
            Self::Gb => String::from("GB"),
            Self::Us => String::from("US"),
        }
    }
}

impl Default for CountryEnum {
    fn default() -> CountryEnum {
        Self::At
    }
}



