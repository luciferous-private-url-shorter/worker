use worker::{event, Context, Env, Request, Response, Result};

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let payload = serde_json::json!({
        "message": "Hello World"
    });
    Response::from_json(&payload)
}
