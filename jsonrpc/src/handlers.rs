use serde_json::{json, Value as JsonValue};

/// Implement this trait for your handler
#[async_trait::async_trait]
pub trait Handler {
    /// `params` field of the request
    type Input: serde::de::DeserializeOwned + Send;

    /// `result` field of the request
    type Output: serde::Serialize;

    /// `error` field of the request
    type Error: serde::Serialize;

    /// `method` field of the request
    const METHOD: &'static str;

    /// Handles the request
    async fn handle(&self, params: Self::Input) -> Result<Self::Output, Self::Error>;
}

// Trick to make dyn handlers
#[async_trait::async_trait]
pub trait WrappedHandler {
    async fn handle_json(&self, json: JsonValue) -> Result<JsonValue, JsonValue>;
}

#[async_trait::async_trait]
impl<H> WrappedHandler for H
where
    H: Handler + Send + Sync,
{
    async fn handle_json(&self, json: JsonValue) -> Result<JsonValue, JsonValue> {
        let params: H::Input = serde_json::from_value(json).map_err(to_json_err)?;
        self.handle(params)
            .await
            .map_err(|err| serde_json::to_value(err).unwrap_or_else(to_json_err))
            .and_then(|output| serde_json::to_value(output).map_err(to_json_err))
    }
}

fn to_json_err<E: std::error::Error>(err: E) -> JsonValue {
    json!(err.to_string())
}
