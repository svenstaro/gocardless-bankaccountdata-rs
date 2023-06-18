# PaymentReadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**institution_id** | Option<**String**> | Institution ID for Payment | [optional][default to SWEDBANK_SANDBOX_SANDLV22]
**payment_product** | Option<[**crate::models::PaymentProductEnum**](PaymentProductEnum.md)> |  | [optional][default to Isct]
**redirect** | Option<**String**> | Redirect URL to your application after payment is done | 
**description** | Option<**String**> | Payment description | [optional][default to GOCARDLESS]
**custom_payment_id** | Option<**String**> | Payment Custom Payment ID | [optional]
**creditor_account** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Registered creditor account | [optional]
**creditor_object** | Option<[**crate::models::PaymentReadRequestCreditorObject**](PaymentReadRequest_creditor_object.md)> |  | [optional]
**debtor_account** | [**crate::models::DebtorAccountWriteRequest**](DebtorAccountWriteRequest.md) |  | 
**instructed_amount** | [**crate::models::PaymentReadRequestInstructedAmount**](PaymentReadRequest_instructed_amount.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


