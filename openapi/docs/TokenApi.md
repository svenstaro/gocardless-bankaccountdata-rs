# \TokenApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**j_wt_obtain**](TokenApi.md#j_wt_obtain) | **POST** /api/v2/token/new/ | 
[**j_wt_refresh**](TokenApi.md#j_wt_refresh) | **POST** /api/v2/token/refresh/ | 



## j_wt_obtain

> crate::models::SpectacularJwtObtain j_wt_obtain(jwt_obtain_pair_request)


Obtain JWT pair

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jwt_obtain_pair_request** | [**JwtObtainPairRequest**](JwtObtainPairRequest.md) |  | [required] |

### Return type

[**crate::models::SpectacularJwtObtain**](SpectacularJWTObtain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## j_wt_refresh

> crate::models::SpectacularJwtRefresh j_wt_refresh(jwt_refresh_request)


Refresh access token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jwt_refresh_request** | [**JwtRefreshRequest**](JwtRefreshRequest.md) |  | [required] |

### Return type

[**crate::models::SpectacularJwtRefresh**](SpectacularJWTRefresh.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

