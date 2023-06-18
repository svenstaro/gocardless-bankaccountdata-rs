build-api:
    # Download the schema
    curl https://bankaccountdata.gocardless.com/api/schema --create-dirs -o target/gocardless-bankaccountdata-schema.yaml

    # Fix up broken schema *sigh*
    sed -i "s/version: 2.0 (v2)/version: 2.0.0/" target/gocardless-bankaccountdata-schema.yaml

    # Generate the schema to a subcrate
    openapi-generator-cli generate -i target/gocardless-bankaccountdata-schema.yaml -g rust -o openapi
