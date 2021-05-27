use serde_json::Value as JsonValue;

#[derive(serde::Deserialize)]
/// Json-RPC request
pub struct Request {
    /// A String with the name of the method to be invoked
    pub method: String,
    /// An Object or Array of values to be passed as parameters to the defined method
    pub params: JsonValue,
    /// Something to match the response with the request that it is replying to
    pub id: JsonValue,
}

#[derive(serde::Serialize)]
#[serde(untagged)]
/// Json-RPC response
pub enum ResponseResult {
    /// If handler returned ok
    Ok {
        /// The data returned by the invoked method.
        result: JsonValue,
        /// ID from request
        id: JsonValue,
    },
    /// If handler returned error
    Err {
        /// An error object if there was an error invoking the method
        error: JsonValue,
        /// ID from request
        id: JsonValue,
    },
}
