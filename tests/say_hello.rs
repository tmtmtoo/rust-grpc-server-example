mod proto {
    tonic::include_proto!("greet");
}

use anyhow::*;
use assert_cmd::Command;
pub use proto::greet_client::GreetClient;
pub use proto::{HelloReply, HelloRequest};

#[tokio::test]
#[ignore]
async fn say_hello_response_ok() -> Result<()> {
    dotenv::dotenv()?;

    let mut cmd = Command::cargo_bin("rust-grpc-server-example")?;

    let handle = std::thread::spawn(move || {
        cmd.env("TEST", "true")
            .timeout(std::time::Duration::from_secs(2))
            .assert()
            .interrupted();
    });

    std::thread::sleep(std::time::Duration::from_secs(1));

    let channel = tonic::transport::Channel::from_static("http://localhost:5001")
        .connect()
        .await?;

    let mut client = GreetClient::new(channel);

    let request = tonic::Request::new(HelloRequest { name: "foo".into() });

    let response = client.say_hello(request).await?.into_inner();

    assert!(response.message.find("foo").is_some());

    handle.join().map_err(|_| anyhow!(""))
}

#[tokio::test]
#[ignore]
async fn say_hello_response_err_by_invalid_argument() -> Result<()> {
    dotenv::dotenv()?;

    let mut cmd = Command::cargo_bin("rust-grpc-server-example")?;

    let handle = std::thread::spawn(move || {
        cmd.env("TEST", "true")
            .env("SOCKET_ADDR", "0.0.0.0:5002")
            .timeout(std::time::Duration::from_secs(2))
            .assert()
            .interrupted();
    });

    std::thread::sleep(std::time::Duration::from_secs(1));

    let channel = tonic::transport::Channel::from_static("http://localhost:5002")
        .connect()
        .await?;

    let mut client = GreetClient::new(channel);

    let request = tonic::Request::new(HelloRequest {
        name: (0..256).into_iter().map(|_| "x").collect::<String>(),
    });

    let status = client.say_hello(request).await.unwrap_err();

    let actual = status.code();
    let expected = tonic::Code::InvalidArgument;

    assert_eq!(actual, expected);

    handle.join().map_err(|_| anyhow!(""))
}
