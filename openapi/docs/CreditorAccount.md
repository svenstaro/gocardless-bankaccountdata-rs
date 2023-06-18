# CreditorAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique entry ID | [optional][readonly]
**name** | **String** | Creditor account name | 
**r#type** | Option<[**crate::models::TypeEnum**](TypeEnum.md)> |  | [optional][default to Iban]
**account** | **String** | Creditor account type identifier | 
**currency** | **String** | Creditor account currency | 
**address_country** | Option<**String**> | Creditor account address country | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


