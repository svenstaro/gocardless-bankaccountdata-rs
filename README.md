# gocardless-bankaccountdata-rs
Rust API client for GoCardless Bank Account Data

This crate wraps the output of OpenAPI generator. In order to make this crate convenient to use,
the generated code is checked in. Everything in `openapi/` is generated and can be re-created using
`just buil-api`.

## Usage
- Register for an account at https://bankaccountdata.gocardless.com
- Go to `Project -> User secrets` and create a new secret
- It will show you your `Secret Id` and `Secret Key`
