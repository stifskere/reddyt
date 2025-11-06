use std::collections::HashMap;
use std::backtrace::Backtrace;

use actix_web::{HttpResponse, HttpResponseBuilder};

/// JSON error formatter for `actix_failwrap`.
///
/// This formats the errors HTTP error deriving from `actix_failwrap`
/// that use this as a formater as JSON.
///
/// The JSON structure is the folllwing
/// ```json
/// {
///     #[cfg(debug_assertions)]
///     "backtrace": "..."
///     "error": "<_ as Display>::to_string()"
/// }
/// ```
pub fn json_formatter(mut builder: HttpResponseBuilder, display: String) -> HttpResponse {
    let mut data = HashMap::new();
    data.insert("error", display);

    #[cfg(debug_assertions)]
    {
        let backtrace = Backtrace::capture();
        data.insert("backtrace", backtrace.to_string());
    }

    builder
        .json(data)
}
