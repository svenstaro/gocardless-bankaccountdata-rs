# PaymentRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payment_id** | Option<**String**> | Payment ID | [optional][readonly]
**payment_status** | Option<[**crate::models::PaymentStatusEnum**](PaymentStatusEnum.md)> |  | [optional][readonly]
**payment_product** | Option<[**crate::models::PaymentProductEnum**](PaymentProductEnum.md)> |  | [optional][default to Isct]
**payment_type** | Option<[**crate::models::PaymentTypeEnum**](PaymentTypeEnum.md)> |  | [optional][readonly]
**redirect** | Option<**String**> | Redirect URL to your application after payment is done | 
**description** | Option<**String**> | Payment description | [optional][default to GOCARDLESS]
**custom_payment_id** | Option<**String**> | Payment Custom Payment ID | [optional]
**creditor_account** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Registered creditor account | [optional]
**creditor_object** | Option<[**crate::models::PaymentReadCreditorObject**](PaymentRead_creditor_object.md)> |  | [optional]
**debtor_account** | [**crate::models::DebtorAccountWrite**](DebtorAccountWrite.md) |  | 
**instructed_amount** | [**crate::models::PaymentReadInstructedAmount**](PaymentRead_instructed_amount.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


