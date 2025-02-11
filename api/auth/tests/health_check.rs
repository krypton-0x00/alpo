use std::{fmt::format, net::TcpListener};

use auth::run;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    dbg!(&address);
    let response = client.get(&format!("{}/health",&address)).send().await.expect("
    Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0),response.content_length());

}
fn spawn_app()-> String{
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    //Get port assigned by OS.
    let port = listner.local_addr().unwrap().port();
    let server = auth::run(listner).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
}

