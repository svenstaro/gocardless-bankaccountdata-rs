# \AgreementsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_eua**](AgreementsApi.md#accept_eua) | **PUT** /api/v2/agreements/enduser/{id}/accept/ | 
[**create_eua_v2**](AgreementsApi.md#create_eua_v2) | **POST** /api/v2/agreements/enduser/ | 
[**delete_eua_by_id_v2**](AgreementsApi.md#delete_eua_by_id_v2) | **DELETE** /api/v2/agreements/enduser/{id}/ | 
[**retrieve_all_euas_for_an_end_user_v2**](AgreementsApi.md#retrieve_all_euas_for_an_end_user_v2) | **GET** /api/v2/agreements/enduser/ | 
[**retrieve_eua_by_id_v2**](AgreementsApi.md#retrieve_eua_by_id_v2) | **GET** /api/v2/agreements/enduser/{id}/ | 



## accept_eua

> crate::models::EndUserAgreement accept_eua(id, enduser_acceptance_details_request)


Accept an end-user agreement via the API

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this end user agreement. | [required] |
**enduser_acceptance_details_request** | [**EnduserAcceptanceDetailsRequest**](EnduserAcceptanceDetailsRequest.md) |  | [required] |

### Return type

[**crate::models::EndUserAgreement**](EndUserAgreement.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_eua_v2

> crate::models::EndUserAgreement create_eua_v2(end_user_agreement_request)


API endpoints related to end-user agreements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**end_user_agreement_request** | [**EndUserAgreementRequest**](EndUserAgreementRequest.md) |  | [required] |

### Return type

[**crate::models::EndUserAgreement**](EndUserAgreement.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_eua_by_id_v2

> delete_eua_by_id_v2(id)


Delete an end user agreement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this end user agreement. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_all_euas_for_an_end_user_v2

> crate::models::PaginatedEndUserAgreementList retrieve_all_euas_for_an_end_user_v2(limit, offset)


API endpoints related to end-user agreements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to return per page. |  |[default to 100]
**offset** | Option<**i32**> | The initial index from which to return the results. |  |[default to 1]

### Return type

[**crate::models::PaginatedEndUserAgreementList**](PaginatedEndUserAgreementList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_eua_by_id_v2

> crate::models::EndUserAgreement retrieve_eua_by_id_v2(id)


Retrieve end user agreement by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this end user agreement. | [required] |

### Return type

[**crate::models::EndUserAgreement**](EndUserAgreement.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

