use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

/// Main body for the function.
async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract name from URL parameter
    let who = _event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, web request received at Lambda function");

    // Return response
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("<h2>{}</h2>",message).into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
