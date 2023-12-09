use lambda_http::{Body, Response};

pub fn response_ok<T: serde::Serialize>(body: T) -> Result<Response<Body>, Box<lambda_http::http::Error>> {
    return Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&body).unwrap().into())
        .map_err(Box::new);
}