use std::net::TcpListener;
use anisoc::run;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use anisoc::telemetry::init_subscriber;

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        init_subscriber(std::io::stdout);
    }
    else {
        init_subscriber(std::io::sink);
    }
    
});

fn spawn_app() -> String {
    Lazy::force(&TRACING);
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

use anisoc::Info;
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
    let map = Info {
        id,
        from: 0,
        to: 0
    };
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
    assert_eq!("reseted".to_string(), response_json.res);
}

#[actix_rt::test]
async fn reset_dont_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    
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

#[actix_rt::test]
async fn mov_works() {
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
    let map = Info {
        id,
        from: 0,
        to: 1
    };
    let response = client
        // Use the returned application address
        .post(&format!("{}/mov", &address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    
    let response_json = response.json::<MakeResult>().await.expect("Not a JSON");

    assert_eq!(id, response_json.id);
    assert_ne!(ref_board, response_json.board);
    assert_eq!("OK move".to_string(), response_json.res);
}

#[actix_rt::test]
async fn mov_dont_works() {
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
    let map = Info {
        id,
        from: 0,
        to: 2
    };
    let response = client
        // Use the returned application address
        .post(&format!("{}/mov", &address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    
    let response_json = response.json::<MakeResult>().await.expect("Not a JSON");

    assert_eq!(id, response_json.id);
    assert_eq!(ref_board, response_json.board);
    assert_eq!("NG move".to_string(), response_json.res);
}