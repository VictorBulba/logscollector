mod inserter;
mod record;
mod seek;

use actix_web::{post, web};
use jsonrpc::{HandlersEngine, Request, ResponseResult};

pub fn make_handlers_engine() -> HandlersEngine {
    let mut engine = HandlersEngine::default();
    let db = sled::open("./logs").expect("Could not open db");
    engine.add_handler(inserter::Inserter(db.clone()));
    engine.add_handler(seek::Seeker(db));
    engine
}

#[post("/jsonrpc")]
pub async fn jsonrpc_handler(
    req: web::Json<Request>,
    engine: web::Data<HandlersEngine>,
) -> web::Json<ResponseResult> {
    web::Json(engine.handle_request(req.0).await)
}
