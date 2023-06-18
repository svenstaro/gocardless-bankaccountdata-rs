//! List all institutions in a given country.
//!
//! Set SECRET_ID and SECRET_KEY as environment variables prior to running.

use std::env;

use anyhow::Result;
use openapi::apis::institutions_api;
use simplelog::{Config, LevelFilter, SimpleLogger};

use gocardless_bankaccountdata::{
    apis::{configuration::Configuration, token_api},
    models::JwtObtainPairRequest,
};

#[tokio::main]
async fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Debug, Config::default())?;

    let api_config = Configuration {
        base_path: "https://bankaccountdata.gocardless.com".to_string(),
        ..Default::default()
    };

    // First we'll have to get an access token with which we can then access most parts of the API.
    let secret_config = JwtObtainPairRequest::new(env::var("SECRET_ID")?, env::var("SECRET_KEY")?);
    let access_token = token_api::j_wt_obtain(&api_config, secret_config).await?;

    // Now that we have an access token, we need a new api_config which contains that token.
    let api_config = Configuration {
        base_path: "https://bankaccountdata.gocardless.com".to_string(),
        bearer_access_token: Some(access_token.access.unwrap()),
        ..Default::default()
    };

    // Finally, get the list of all institutions with this unflattering generated function.
    let institutions = institutions_api::retrieve_all_supported_institutions_in_a_given_country(
        &api_config,
        None,
        None,
        None,
        None,
        None,
        Some("de"),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await?;

    println!("{:#?}", institutions);
    Ok(())
}
