# \RequisitionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_requisition_by_id_v2**](RequisitionsApi.md#delete_requisition_by_id_v2) | **DELETE** /api/v2/requisitions/{id}/ | 
[**requisition_by_id**](RequisitionsApi.md#requisition_by_id) | **GET** /api/v2/requisitions/{id}/ | 
[**requisition_created**](RequisitionsApi.md#requisition_created) | **POST** /api/v2/requisitions/ | 
[**retrieve_all_requisitions**](RequisitionsApi.md#retrieve_all_requisitions) | **GET** /api/v2/requisitions/ | 



## delete_requisition_by_id_v2

> delete_requisition_by_id_v2(id)


Delete requisition and its end user agreement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this requisition. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## requisition_by_id

> crate::models::Requisition requisition_by_id(id)


Retrieve a requisition by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this requisition. | [required] |

### Return type

[**crate::models::Requisition**](Requisition.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## requisition_created

> crate::models::SpectacularRequisition requisition_created(requisition_request)


Create a new requisition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**requisition_request** | [**RequisitionRequest**](RequisitionRequest.md) |  | [required] |

### Return type

[**crate::models::SpectacularRequisition**](SpectacularRequisition.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_all_requisitions

> crate::models::PaginatedRequisitionList retrieve_all_requisitions(limit, offset)


Retrieve all requisitions belonging to the company

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to return per page. |  |[default to 100]
**offset** | Option<**i32**> | The initial index from which to return the results. |  |[default to 1]

### Return type

[**crate::models::PaginatedRequisitionList**](PaginatedRequisitionList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

