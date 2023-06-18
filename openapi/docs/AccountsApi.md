# \AccountsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_account_balances_v2**](AccountsApi.md#retrieve_account_balances_v2) | **GET** /api/v2/accounts/{id}/balances/ | 
[**retrieve_account_details_v2**](AccountsApi.md#retrieve_account_details_v2) | **GET** /api/v2/accounts/{id}/details/ | 
[**retrieve_account_metadata**](AccountsApi.md#retrieve_account_metadata) | **GET** /api/v2/accounts/{id}/ | 
[**retrieve_account_transactions_v22**](AccountsApi.md#retrieve_account_transactions_v22) | **GET** /api/v2/accounts/{id}/transactions/ | 



## retrieve_account_balances_v2

> crate::models::RetrieveAccountBalances retrieve_account_balances_v2(id)


Access account balances.  Balances will be returned in Berlin Group PSD2 format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::RetrieveAccountBalances**](RetrieveAccountBalances.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_account_details_v2

> crate::models::RetrieveAccountDetails retrieve_account_details_v2(id)


Access account details.  Account details will be returned in Berlin Group PSD2 format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::RetrieveAccountDetails**](RetrieveAccountDetails.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_account_metadata

> crate::models::Account retrieve_account_metadata(id)


Access account metadata.  Information about the account record, such as the processing status and IBAN.  Account status is recalculated based on the error count in the latest req.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_account_transactions_v22

> crate::models::RetrieveAccountTransactions retrieve_account_transactions_v22(id, date_from, date_to)


Access account transactions.  Transactions will be returned in Berlin Group PSD2 format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |

### Return type

[**crate::models::RetrieveAccountTransactions**](RetrieveAccountTransactions.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

