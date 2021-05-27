#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_const_for_fn)]
#![warn(missing_docs)]

//! Tiny Json-RPC implementation

mod data;
mod handlers;

use handlers::WrappedHandler;
use serde_json::json;
use std::collections::HashMap;

pub use data::{Request, ResponseResult};
pub use handlers::Handler;

/// This struct stores your handlers and invokes the necessary ones
#[derive(Default)]
pub struct HandlersEngine {
    map: HashMap<&'static str, Box<dyn WrappedHandler + Send + Sync>>,
}

impl HandlersEngine {
    /// Adds handler to the engine
    #[inline]
    pub fn add_handler<H>(&mut self, handler: H)
    where
        H: Handler + Send + Sync + 'static,
    {
        self.map.insert(H::METHOD, Box::new(handler));
    }

    /// Accepts a json request and returns a json response
    #[inline]
    pub async fn handle_request(&self, request: Request) -> ResponseResult {
        let handler = match self.map.get(&request.method as &str) {
            Some(h) => h,
            None => {
                return ResponseResult::Err {
                    error: json!("No such handler"),
                    id: request.id,
                }
            }
        };
        match handler.handle_json(request.params).await {
            Ok(result) => ResponseResult::Ok {
                result,
                id: request.id,
            },
            Err(error) => ResponseResult::Err {
                error,
                id: request.id,
            },
        }
    }
}
