use worker::{event, Context, Env, Request, Response, Result};

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    let path = url.path();

    let payload = serde_json::json!({
        "message": "Hello World",
        "path": path.to_string()
    });
    Response::from_json(&payload)
}
