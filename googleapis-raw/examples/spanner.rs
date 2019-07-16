use std::error::Error;
use std::sync::Arc;

use futures::Future;
use googleapis_raw::spanner::v1::{
    spanner::CreateSessionRequest, spanner::ExecuteSqlRequest, spanner_grpc::SpannerClient,
};
use grpcio::{CallOption, ChannelBuilder, ChannelCredentials, EnvBuilder, MetadataBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    // Mozilla's Spanner instance configuration.
    let project = "mozilla-rust-sdk-dev";
    let instance = "mozilla-spanner-dev";
    let database = "mydb";

    // Google Cloud configuration.
    let endpoint = "spanner.googleapis.com:443";
    let db_name = "".to_string()
        + "projects/"
        + project
        + "/instances/"
        + instance
        + "/databases/"
        + database;

    // Set up the gRPC environment.
    let env = Arc::new(EnvBuilder::new().build());
    let creds = ChannelCredentials::google_default_credentials()?;

    // Create a Spanner client.
    let chan = ChannelBuilder::new(env.clone())
        .max_send_message_len(100 << 20)
        .max_receive_message_len(100 << 20)
        .secure_connect(&endpoint, creds);
    let client = SpannerClient::new(chan);

    // Connect to the instance and create a Spanner session.
    let mut req = CreateSessionRequest::new();
    req.set_database(db_name.to_string());
    let mut meta = MetadataBuilder::new();
    meta.add_str("google-cloud-resource-prefix", &db_name)?;
    meta.add_str("x-goog-api-client", "googleapis-rs")?;
    let opt = CallOption::default().headers(meta.build());
    let session = client.create_session_opt(&req, opt)?;

    // Prepare a SQL command to execute.
    let mut req = ExecuteSqlRequest::new();
    req.set_session(session.get_name().to_string());
    req.set_sql("select * from planets".to_string());

    // Execute the command synchronously.
    let out = client.execute_sql(&req)?;
    dbg!(out);

    // Execute the command asynchronously.
    let fut = client.execute_sql_async(&req)?;
    let out = fut.wait()?;
    dbg!(out);

    Ok(())
}
