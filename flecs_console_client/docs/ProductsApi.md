# \ProductsApi

All URIs are relative to *https://console.flecs.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_v2_products**](ProductsApi.md#get_api_v2_products) | **GET** /api/v2/products | Get all products
[**get_api_v2_products_apps**](ProductsApi.md#get_api_v2_products_apps) | **GET** /api/v2/products/apps | Get all app products
[**get_api_v2_products_apps_reviews**](ProductsApi.md#get_api_v2_products_apps_reviews) | **POST** /api/v2/products/apps/reviews | Post a review for an app



## get_api_v2_products

> models::GetApiV2Products200Response get_api_v2_products(store_id)
Get all products

Get all products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | Option<**i32**> |  |  |

### Return type

[**models::GetApiV2Products200Response**](get_api_v2_products_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v2_products_apps

> models::GetApiV2ProductsApps200Response get_api_v2_products_apps(store_id)
Get all app products

Get all app products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | Option<**i32**> |  |  |

### Return type

[**models::GetApiV2ProductsApps200Response**](get_api_v2_products_apps_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v2_products_apps_reviews

> models::GetApiV2ProductsAppsReviews201Response get_api_v2_products_apps_reviews(review_request)
Post a review for an app

Post a review for an app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_request** | Option<[**ReviewRequest**](ReviewRequest.md)> |  |  |

### Return type

[**models::GetApiV2ProductsAppsReviews201Response**](get_api_v2_products_apps_reviews_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

