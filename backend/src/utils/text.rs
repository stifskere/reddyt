use serde::{Deserialize, Serialize};
use dyn_path::dyn_access;
use reqwest::{Client as HttpClient, Error as HttpError};
use thiserror::Error;

const GEMINI_URL: &'static str = "https://gemin-rest.vercel.app/api/ai?prompt=$PROMPT";

#[derive(Error, Debug)]
enum VideoTextError {
    #[error("{0:#}")]
    Request(#[from] HttpError)
}

#[derive(Deserialize, PartialEq, PartialOrd, Hash, Clone)]
struct VideoText {
    prompt: String,
    answer: String
}

impl VideoText {
    pub async fn fetch_new() -> Result<Self, VideoText> {
        let client = HttpClient::new();

        client.get(GEMINI_URL.replace("$PROMPT", ""));

        todo!()
    }

    #[inline(always)]
    fn prompt(&self) -> &str {
        &self.prompt
    }

    #[inline(always)]
    fn answer(&self) -> &str {
        &self.answer
    }
}
