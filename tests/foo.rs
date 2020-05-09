use anyhow::*;
use assert_cmd::Command;

#[test]
#[ignore]
fn sample() -> Result<()> {
    dotenv::dotenv()?;

    let mut cmd = Command::cargo_bin("rust-grpc-server-example")?;

    let handle = std::thread::spawn(move || {
        cmd.timeout(std::time::Duration::from_secs(3))
            .assert()
            .interrupted();
    });

    std::thread::sleep(std::time::Duration::from_secs(1));

    Command::new("grpcurl")
        .args(&[
            "-plaintext",
            "-import-path",
            "./proto",
            "-proto",
            "greet.proto",
            "-d",
            r#"{"name": "foo"}"#,
            "localhost:5001",
            "greet.Greet/SayHello",
        ])
        .assert()
        .success();

    handle.join().map_err(|_| anyhow!(""))
}
