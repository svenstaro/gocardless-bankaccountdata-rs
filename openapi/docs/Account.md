# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of this Account, used to refer to this account in other API calls. | [optional][readonly]
**created** | Option<**String**> | The date & time at which the account object was created. | [optional][readonly]
**last_accessed** | Option<**String**> | The date & time at which the account object was last accessed. | [optional][readonly]
**iban** | Option<**String**> | The Account IBAN | [optional][readonly]
**institution_id** | Option<**String**> | The ASPSP associated with this account. | [optional][readonly]
**status** | Option<[**crate::models::AccountStatusEnum**](AccountStatusEnum.md)> |  | [optional][readonly]
**owner_name** | Option<**String**> | The name of the account owner. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


