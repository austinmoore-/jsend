//! Tests to ensure that serde attributes on [JSendResponse] are correctly
//! configured. And since we're already doing all the work to serialize, these
//! tests also do a simple deserialization and comparison to ensure that's
//! supported too.
use jsend::JSendResponse;
use serde_json::{from_str, json, to_string, Value};

#[test]
fn test_serde_success() {
    let response: JSendResponse<Value> = JSendResponse::success(Some(json!({
        "post":{
            "id": 1,
            "title": "A blog post",
            "body": "Some useful content",
        }
    })));
    let serialized = to_string(&response).unwrap();
    assert_eq!(
        serialized,
        r#"{"status":"success","data":{"post":{"body":"Some useful content","id":1,"title":"A blog post"}}}"#
    );

    let deserialized: JSendResponse<Value> = from_str(&serialized).unwrap();
    assert_eq!(response, deserialized);
}

#[test]
fn test_serde_success_no_data() {
    let response: JSendResponse<Value> = JSendResponse::success(None);
    let serialized = to_string(&response).unwrap();
    assert_eq!(serialized, r#"{"status":"success","data":null}"#);

    let deserialized: JSendResponse<Value> = from_str(&serialized).unwrap();
    assert_eq!(response, deserialized);
}

#[test]
fn test_serialization_fail() {
    let response: JSendResponse<Value> =
        JSendResponse::fail(json!({"title": "A title is required"}));
    let serialized = to_string(&response).unwrap();
    assert_eq!(
        serialized,
        r#"{"status":"fail","data":{"title":"A title is required"}}"#
    );

    let deserialized: JSendResponse<Value> = from_str(&serialized).unwrap();
    assert_eq!(response, deserialized);
}

#[test]
fn test_serialization_error() {
    let response: JSendResponse<Value> = JSendResponse::error(
        "Unable to communicate with database".to_string(),
        None,
        None,
    );
    let serialized = to_string(&response).unwrap();
    assert_eq!(
        serialized,
        r#"{"status":"error","message":"Unable to communicate with database"}"#
    );

    let deserialized: JSendResponse<Value> = from_str(&serialized).unwrap();
    assert_eq!(response, deserialized);
}

#[test]
fn test_serialization_error_with_optional_fields() {
    let response: JSendResponse<Value> = JSendResponse::error(
        "Unable to communicate with database".to_string(),
        Some(100),
        Some(json!({"cause": "timeout"})),
    );
    let serialized = to_string(&response).unwrap();
    assert_eq!(
        serialized,
        r#"{"status":"error","message":"Unable to communicate with database","code":100,"data":{"cause":"timeout"}}"#
    );

    let deserialized: JSendResponse<Value> = from_str(&serialized).unwrap();
    assert_eq!(response, deserialized);
}
