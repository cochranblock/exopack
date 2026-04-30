// Unlicense — public domain — cochranblock.org
//! Mock APIs on demand. wiremock wrapper.
//!
//! Public API: [`start_server`], [`mount_get`], [`mount_get_json`],
//! [`mount_post`], [`mount_post_json`], [`mount_status`].
//! P13 aliases (`f82`–`f87`) retained.

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Start a mock HTTP server on a random port. Returns `(server, base_uri)`.
pub async fn start_server() -> (MockServer, String) {
    f82().await
}

/// Mount a GET handler returning 200 + a text body.
pub async fn mount_get(server: &MockServer, path_pattern: &str, body: &str) {
    f83(server, path_pattern, body).await
}

/// Mount a GET handler returning 200 + a JSON body.
pub async fn mount_get_json(server: &MockServer, path_pattern: &str, json: serde_json::Value) {
    f84(server, path_pattern, json).await
}

/// Mount a POST handler returning 200 + a text body.
pub async fn mount_post(server: &MockServer, path_pattern: &str, body: &str) {
    f85(server, path_pattern, body).await
}

/// Mount a POST handler returning 200 + a JSON body.
pub async fn mount_post_json(server: &MockServer, path_pattern: &str, json: serde_json::Value) {
    f86(server, path_pattern, json).await
}

/// Mount a handler for any method returning a custom status code + body.
pub async fn mount_status(server: &MockServer, path_pattern: &str, status: u16, body: &str) {
    f87(server, path_pattern, status, body).await
}

/// f82 = start_mock_server. Start a mock HTTP server on a random port. Returns (server, base_uri).
pub async fn f82() -> (MockServer, String) {
    let server = MockServer::start().await;
    let uri = server.uri();
    (server, uri)
}

/// f83 = mount_get. Mount a simple GET handler returning 200 + body.
pub async fn f83(server: &MockServer, path_pattern: &str, body: &str) {
    Mock::given(method("GET"))
        .and(path(path_pattern))
        .respond_with(ResponseTemplate::new(200).set_body_string(body.to_string()))
        .mount(server)
        .await;
}

/// f84 = mount_get_json. Mount a simple GET handler returning JSON.
pub async fn f84(server: &MockServer, path_pattern: &str, json: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path(path_pattern))
        .respond_with(ResponseTemplate::new(200).set_body_json(json))
        .mount(server)
        .await;
}

/// f85 = mount_post. Mount a POST handler returning 200 + body.
pub async fn f85(server: &MockServer, path_pattern: &str, body: &str) {
    Mock::given(method("POST"))
        .and(path(path_pattern))
        .respond_with(ResponseTemplate::new(200).set_body_string(body.to_string()))
        .mount(server)
        .await;
}

/// f86 = mount_post_json. Mount a POST handler returning JSON.
pub async fn f86(server: &MockServer, path_pattern: &str, json: serde_json::Value) {
    Mock::given(method("POST"))
        .and(path(path_pattern))
        .respond_with(ResponseTemplate::new(200).set_body_json(json))
        .mount(server)
        .await;
}

/// f87 = mount_status. Mount a handler for any method returning a specific status code + body.
pub async fn f87(server: &MockServer, path_pattern: &str, status: u16, body: &str) {
    Mock::given(path(path_pattern))
        .respond_with(ResponseTemplate::new(status).set_body_string(body.to_string()))
        .mount(server)
        .await;
}
