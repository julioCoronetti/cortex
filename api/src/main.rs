use lambda_http::{service_fn, LambdaEvent, Request, Response, Body};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    let func = service_fn(handler);
    lambda_http::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Request>) -> Result<Response<Body>, lambda_http::Error> {
    let (parts, _body) = event.into_parts();
    
    match parts.uri.path.as_str() {
        "/health" => {
            let body = json!({ "status": "ok" });
            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body)?))
                .unwrap())
        }
        _ => {
            let body = json!({ "error": "not found" });
            Ok(Response::builder()
                .status(404)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body)?))
                .unwrap())
        }
    }
}