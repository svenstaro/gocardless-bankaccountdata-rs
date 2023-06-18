# PaymentReadRequestCreditorObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Creditor account name | 
**r#type** | Option<[**crate::models::TypeEnum**](TypeEnum.md)> |  | [optional][default to Iban]
**account** | **String** | Creditor account type identifier | 
**currency** | **String** | Creditor account currency | 
**address_country** | Option<[**crate::models::AddressCountryEnum**](AddressCountryEnum.md)> |  | [optional]
**institution_id** | Option<**String**> | an Institution ID for this CreditorAccount | [optional]
**agent** | Option<**String**> | Creditor account BICFI Identifier | [optional]
**agent_name** | Option<**String**> | Creditor account agent name | [optional]
**address_street** | Option<**String**> | Creditor account address street | [optional]
**post_code** | Option<**String**> | Creditor account address post code | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


