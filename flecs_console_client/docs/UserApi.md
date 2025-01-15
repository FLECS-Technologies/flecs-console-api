# \UserApi

All URIs are relative to *https://console.flecs.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_v2_my_products**](UserApi.md#get_api_v2_my_products) | **GET** /api/v2/my/products | Get all products for a specific user
[**get_api_v2_my_products_apps**](UserApi.md#get_api_v2_my_products_apps) | **GET** /api/v2/my/products/apps | Get all app products for a specific user
[**get_api_v2_my_subscriptions**](UserApi.md#get_api_v2_my_subscriptions) | **GET** /api/v2/my/subscriptions | Get all subscriptions of a specific user



## get_api_v2_my_products

> models::GetApiV2Products200Response get_api_v2_my_products(authorization, store_id)
Get all products for a specific user

Get all products for a specific user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**store_id** | Option<**i32**> |  |  |

### Return type

[**models::GetApiV2Products200Response**](get_api_v2_products_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v2_my_products_apps

> models::GetApiV2Products200Response get_api_v2_my_products_apps(authorization, store_id)
Get all app products for a specific user

Get all app products for a specific user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**store_id** | Option<**i32**> |  |  |

### Return type

[**models::GetApiV2Products200Response**](get_api_v2_products_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v2_my_subscriptions

> models::GetApiV2MySubscriptions200Response get_api_v2_my_subscriptions(authorization, store_id)
Get all subscriptions of a specific user

Get all subscriptions of a specific user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**store_id** | Option<**i32**> |  |  |

### Return type

[**models::GetApiV2MySubscriptions200Response**](get_api_v2_my_subscriptions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

