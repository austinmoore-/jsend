//! The `jsend` crate provides an implementation of the [JSend specification](https://github.com/omniti-labs/jsend)
//! for API responses in Rust applications.
//!
//! ## Usage
//!
//! Add `jsend` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! jsend = "0.1.0"
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! ### Basic Example
//!
//! ```rust
//! use jsend::JSendResponse;
//! use std::collections::HashMap;
//!
//! // Success response with data
//! let data = Some(HashMap::from([("key", "value")]));
//! let response = JSendResponse::success(data);
//! println!("{}", serde_json::to_string(&response).unwrap());
//!
//! // Error response
//! let error_response = JSendResponse::error("An error occurred".to_string(), Some(100), None::<String>);
//! println!("{}", serde_json::to_string(&error_response).unwrap());
//! ```
//!
//! A more in-depth example of how this crate could be used with a framework
//! like [axum](https://crates.io/crates/axum) can be found in the `examples/`
//! directory.
//!
//! ## Features
//! - `serde`: Enabled by default. Adds [serde::Serialize] and
//! [serde::Deserialize] derives, along with attributes to serialize into JSON
//! according to the JSend specification.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The `JSendResponse` enum provides a way to model JSend compliant responses.
///
/// It supports the three JSend response types as variants: `Success`, `Fail`,
/// and `Error`.
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "status", rename_all = "lowercase"))]
pub enum JSendResponse<T> {
    Success {
        /// Acts as the wrapper for any data returned by the API call. If the
        /// call returns no data, `data` should be set to `None`.
        data: Option<T>,
    },
    Fail {
        /// Provides the wrapper for the details of why the request failed. If
        /// the reasons for failure correspond to POST values, the response
        /// object's keys SHOULD correspond to those POST values.
        data: T,
    },
    Error {
        /// A meaningful, end-user-readable (or at the least log-worthy)
        /// message, explaining what went wrong.
        message: String,
        /// A numeric code corresponding to the error, if applicable.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        code: Option<i64>,
        /// A generic container for any other information about the error, i.e.
        /// the conditions that caused the error, stack traces, etc.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        data: Option<T>,
    },
}

impl<T> JSendResponse<T> {
    /// Constructs the [JSendResponse::Success] variant.
    pub fn success(data: Option<T>) -> JSendResponse<T> {
        JSendResponse::Success { data }
    }

    /// Constructs the [JSendResponse::Fail] variant.
    pub fn fail(data: T) -> JSendResponse<T> {
        JSendResponse::Fail { data }
    }

    /// Constructs the [JSendResponse::Error] variant.
    pub fn error(message: String, code: Option<i64>, data: Option<T>) -> JSendResponse<T> {
        JSendResponse::Error {
            message,
            code,
            data,
        }
    }

    /// Returns a reference to the underlying `Option` value if set, and `None`
    /// otherwise.
    ///
    /// This getter "flattens" the structure of the enum:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use jsend::JSendResponse;
    /// # let data = HashMap::from([("key", "value")]);
    /// let response_with_data = JSendResponse::success(Some(data.clone()));
    /// assert_eq!(response_with_data.data(), Some(data).as_ref());
    ///
    /// let response_without_data = JSendResponse::success(None::<HashMap<&str, &str>>);
    /// assert_eq!(response_without_data.data(), None)
    /// ```
    pub fn data(&self) -> Option<&T> {
        match self {
            JSendResponse::Success { data } => data.as_ref(),
            JSendResponse::Fail { data } => Some(data),
            JSendResponse::Error { data, .. } => data.as_ref(),
        }
    }

    /// Returns a reference to `message` for the `Error` variant, and `None`
    /// for the other variants.
    pub fn message(&self) -> Option<&String> {
        match self {
            JSendResponse::Error { message, .. } => Some(message),
            _ => None,
        }
    }

    /// Returns a reference to the value of `code`for the `Error` variant, and
    /// `None` for the other variants.
    ///
    /// This getter "flattens" the structure of the enum:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use jsend::JSendResponse;
    /// # let message = "error message".to_string();
    /// # let code = 123;
    /// # let data = HashMap::from([("key", "value")]);
    /// let response_with_code = JSendResponse::error(message.clone(), Some(code), Some(data.clone()));
    /// assert_eq!(response_with_code.code(), Some(code).as_ref());
    ///
    /// let response_without_code = JSendResponse::error(message.clone(), None, Some(data.clone()));
    /// assert_eq!(response_without_code.code(), None);
    /// ```
    pub fn code(&self) -> Option<&i64> {
        match self {
            JSendResponse::Error { code, .. } => code.as_ref(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_success_variant() {
        let data = HashMap::from([("key", "value")]);
        let response = JSendResponse::success(Some(data.clone()));
        assert_eq!(Some(data).as_ref(), response.data());
        assert_eq!(None, response.code());
        assert_eq!(None, response.message());
    }

    #[test]
    fn test_success_variant_no_data() {
        let response: JSendResponse<HashMap<&str, &str>> = JSendResponse::success(None);
        assert_eq!(None, response.data());
        assert_eq!(None, response.code());
        assert_eq!(None, response.message());
    }

    #[test]
    fn test_fail_variant() {
        let data = HashMap::from([("key", "value")]);
        let response = JSendResponse::fail(data.clone());
        assert_eq!(Some(data).as_ref(), response.data());
        assert_eq!(None, response.code());
        assert_eq!(None, response.message());
    }

    #[test]
    fn test_fail_variant_no_data() {
        let data: Option<String> = None;
        let response = JSendResponse::fail(data.clone());
        assert_eq!(Some(data).as_ref(), response.data());
        assert_eq!(None, response.code());
        assert_eq!(None, response.message());
    }

    #[test]
    fn test_error_variant() {
        let message = "error message".to_string();
        let code = 123;
        let data = HashMap::from([("key", "value")]);
        let response = JSendResponse::error(message.clone(), Some(code), Some(data.clone()));
        assert_eq!(Some(message).as_ref(), response.message());
        assert_eq!(Some(code).as_ref(), response.code());
        assert_eq!(Some(data).as_ref(), response.data());
    }

    #[test]
    fn test_error_variant_only_message() {
        let message = "error message".to_string();
        let response: JSendResponse<String> = JSendResponse::error(message.clone(), None, None);
        assert_eq!(Some(message).as_ref(), response.message());
        assert_eq!(None, response.code());
        assert_eq!(None, response.data());
    }
}
