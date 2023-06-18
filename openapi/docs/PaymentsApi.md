# \PaymentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment**](PaymentsApi.md#create_payment) | **POST** /api/v2/payments/ | 
[**delete_periodic_payment**](PaymentsApi.md#delete_periodic_payment) | **DELETE** /api/v2/payments/{id}/ | 
[**list_minimum_required_fields_for_institution**](PaymentsApi.md#list_minimum_required_fields_for_institution) | **GET** /api/v2/payments/fields/{institution_id}/ | 
[**list_payments**](PaymentsApi.md#list_payments) | **GET** /api/v2/payments/ | 
[**payments_creditors_create**](PaymentsApi.md#payments_creditors_create) | **POST** /api/v2/payments/creditors/ | 
[**payments_creditors_destroy**](PaymentsApi.md#payments_creditors_destroy) | **DELETE** /api/v2/payments/creditors/{id}/ | 
[**payments_creditors_list**](PaymentsApi.md#payments_creditors_list) | **GET** /api/v2/payments/creditors/ | 
[**payments_creditors_retrieve**](PaymentsApi.md#payments_creditors_retrieve) | **GET** /api/v2/payments/creditors/{id}/ | 
[**payments_submit_create**](PaymentsApi.md#payments_submit_create) | **POST** /api/v2/payments/{id}/submit/ | 
[**retrieve_all_payment_creditor_accounts**](PaymentsApi.md#retrieve_all_payment_creditor_accounts) | **GET** /api/v2/payments/account/ | 
[**retrieve_payment**](PaymentsApi.md#retrieve_payment) | **GET** /api/v2/payments/{id}/ | 



## create_payment

> crate::models::PaymentWrite create_payment(payment_write_request)


Create payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_write_request** | [**PaymentWriteRequest**](PaymentWriteRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentWrite**](PaymentWrite.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_periodic_payment

> crate::models::PaymentDeleted delete_periodic_payment(id)


Delete periodic payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::PaymentDeleted**](PaymentDeleted.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_minimum_required_fields_for_institution

> crate::models::MinimumRequiredFields list_minimum_required_fields_for_institution(institution_id)


List minimum required fields for institution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**institution_id** | **String** |  | [required] |

### Return type

[**crate::models::MinimumRequiredFields**](MinimumRequiredFields.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payments

> crate::models::PaginatedPaymentReadList list_payments(limit, offset)


Retrieve all payments belonging to the company

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to return per page. |  |[default to 100]
**offset** | Option<**i32**> | The initial index from which to return the results. |  |[default to 1]

### Return type

[**crate::models::PaginatedPaymentReadList**](PaginatedPaymentReadList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_creditors_create

> crate::models::CreditorAccountWrite payments_creditors_create(creditor_account_write_request)


API endpoints related to creditor accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**creditor_account_write_request** | [**CreditorAccountWriteRequest**](CreditorAccountWriteRequest.md) |  | [required] |

### Return type

[**crate::models::CreditorAccountWrite**](CreditorAccountWrite.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_creditors_destroy

> payments_creditors_destroy(id)


API endpoints related to creditor accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_creditors_list

> crate::models::PaginatedCreditorAccountList payments_creditors_list(account, address_country, agent, currency, limit, name, offset, r#type)


API endpoints related to creditor accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account** | Option<**String**> |  |  |
**address_country** | Option<**String**> |  |  |
**agent** | Option<**String**> |  |  |
**currency** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |[default to 100]
**name** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |[default to 1]
**r#type** | Option<**String**> |  |  |

### Return type

[**crate::models::PaginatedCreditorAccountList**](PaginatedCreditorAccountList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_creditors_retrieve

> crate::models::CreditorAccount payments_creditors_retrieve(id)


API endpoints related to creditor accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::CreditorAccount**](CreditorAccount.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_submit_create

> crate::models::PaymentRead payments_submit_create(id, payment_read_request)


Initiate the payment on bank's side.  Complete the payment and return payment details as a response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**payment_read_request** | [**PaymentReadRequest**](PaymentReadRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentRead**](PaymentRead.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_all_payment_creditor_accounts

> Vec<crate::models::CreditorAccount> retrieve_all_payment_creditor_accounts()


Retrieve all payment creditor accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CreditorAccount>**](CreditorAccount.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_payment

> crate::models::PaymentRead retrieve_payment(id)


Retrieve payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::PaymentRead**](PaymentRead.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

