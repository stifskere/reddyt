use std::{backtrace::Backtrace, collections::HashMap};

use actix_web::{HttpResponse, HttpResponseBuilder};

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
