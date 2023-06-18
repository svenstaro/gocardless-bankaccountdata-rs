# \EnrichedBankDataInviteOnlyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_account_transactions_v2**](EnrichedBankDataInviteOnlyApi.md#retrieve_account_transactions_v2) | **GET** /api/v2/accounts/premium/{id}/transactions/ | 



## retrieve_account_transactions_v2

> crate::models::RetrieveAccountTransactions retrieve_account_transactions_v2(id, country, date_from, date_to)


Access account premium transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**country** | Option<**String**> | ISO 3166 two-character country code |  |
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

