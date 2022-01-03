use std::net::TcpListener;
use anisoc::run;
use std::collections::HashMap;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

use uuid::Uuid;
use anisoc::MakeResult;
#[actix_rt::test]
async fn make_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/make", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    
    let response_json = response.json::<MakeResult>().await.expect("Not a JSON");

    println!("Uuid is {:?}", response_json);
}


#[actix_rt::test]
async fn reset_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    
    let response = client
        // Use the returned application address
        .post(&format!("{}/make", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    let response_json = response.json::<MakeResult>().await.expect("Not a JSON");
    let ref_board = response_json.board;
    let id = response_json.id;
    
    // reset送信
    let mut map = HashMap::new();
    map.insert("id", id);
    let response = client
        // Use the returned application address
        .post(&format!("{}/reset", &address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    
    let response_json = response.json::<MakeResult>().await.expect("Not a JSON");

    assert_eq!(id, response_json.id);
    assert_eq!(ref_board, response_json.board);
}

#[actix_rt::test]
async fn reset_dont_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    
    let response = client
        // Use the returned application address
        .post(&format!("{}/make", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    
    // reset送信
    let mut map = HashMap::new();
    map.insert("id", "bad request");
    let response = client
        // Use the returned application address
        .post(&format!("{}/reset", &address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(!response.status().is_success());
}