# PaymentWriteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**institution_id** | Option<**String**> | Institution ID for Payment | [optional][default to SWEDBANK_SANDBOX_SANDLV22]
**payment_product** | Option<[**crate::models::PaymentProductEnum**](PaymentProductEnum.md)> |  | [optional][default to Isct]
**instructed_amount** | [**crate::models::PaymentReadRequestInstructedAmount**](PaymentReadRequest_instructed_amount.md) |  | 
**creditor_account** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Registered creditor account | [optional]
**creditor_object** | Option<[**crate::models::PaymentReadRequestCreditorObject**](PaymentReadRequest_creditor_object.md)> |  | [optional]
**debtor_account** | Option<[**crate::models::PaymentWriteRequestDebtorAccount**](PaymentWriteRequest_debtor_account.md)> |  | [optional]
**redirect** | Option<**String**> | Redirect URL to your application after payment is done | 
**description** | Option<**String**> | Payment description | [optional][default to GOCARDLESS]
**custom_payment_id** | Option<**String**> | Payment Custom Payment ID | [optional]
**requested_execution_date** | Option<[**String**](string.md)> | Payment Execution date (for periodic payments) | [optional]
**periodic_payment** | Option<[**crate::models::PeriodicPaymentRequest**](PeriodicPaymentRequest.md)> |  | [optional]
**submit_payment** | Option<**bool**> | Indicates whether payment should be submitted separately | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


