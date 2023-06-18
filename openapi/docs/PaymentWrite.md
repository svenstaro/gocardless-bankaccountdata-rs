# PaymentWrite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**link** | Option<**String**> | Link to initiate authorization with Institution | [optional][readonly][default to https://ob.nordigen.com/pis/start/2d1478d0-0ae0-4500-82d3-7ea8ebfcf9ba/{$INSTITUTION_ID}]
**payment_id** | Option<**String**> | Payment ID | [optional][readonly]
**payment_status** | Option<[**crate::models::PaymentStatusEnum**](PaymentStatusEnum.md)> |  | [optional][readonly]
**payment_product** | Option<[**crate::models::PaymentProductEnum**](PaymentProductEnum.md)> |  | [optional][default to Isct]
**payment_type** | Option<[**crate::models::PaymentTypeEnum**](PaymentTypeEnum.md)> |  | [optional][readonly]
**instructed_amount** | [**crate::models::PaymentReadInstructedAmount**](PaymentRead_instructed_amount.md) |  | 
**creditor_account** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Registered creditor account | [optional]
**creditor_object** | Option<[**crate::models::PaymentReadCreditorObject**](PaymentRead_creditor_object.md)> |  | [optional]
**redirect** | Option<**String**> | Redirect URL to your application after payment is done | 
**description** | Option<**String**> | Payment description | [optional][default to GOCARDLESS]
**custom_payment_id** | Option<**String**> | Payment Custom Payment ID | [optional]
**requested_execution_date** | Option<[**String**](string.md)> | Payment Execution date (for periodic payments) | [optional]
**periodic_payment** | Option<[**crate::models::PeriodicPayment**](PeriodicPayment.md)> |  | [optional]
**submit_payment** | Option<**bool**> | Indicates whether payment should be submitted separately | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


