use http_body_util::Full;
use hyper::{Request, Response};
use hyper::body::Bytes;
use hyper::http;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use crate::application::model::request::Person;

pub async fn get_routes(request:Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, http::Error> {
    let path = request.uri().path();

    let response =
        match path{
            "/version" =>
                Response::builder()
                    .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                    .body(Full::from(Bytes::from("{\"version\":\"0.0.1\"}"))),
            _ =>
                Response::builder()
                    .status(404)
                    .body(Full::from(Bytes::from("Not Found")))

        };

    match response {
        Ok(r) => Ok(r),
        Err(e) => {
            eprintln!("Error creating response: {}", e);
            Err(e)
        }
    }
}