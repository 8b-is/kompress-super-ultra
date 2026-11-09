//! End-to-end Axum Web API tests over an ephemeral local TCP socket.

mod common;

use kompress_super_ultra::{build_router, HealthResponse, KompressResponse, KompressStats};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

async fn send_raw_http(addr: std::net::SocketAddr, request: &str) -> String {
    let mut stream = TcpStream::connect(addr).await.expect("connect to test server");
    stream.write_all(request.as_bytes()).await.expect("write HTTP request");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.expect("read HTTP response");
    String::from_utf8_lossy(&buf).into_owned()
}

fn extract_json_body(raw_http: &str) -> &str {
    raw_http.split("\r\n\r\n").nth(1).unwrap_or("")
}

#[tokio::test]
async fn axum_api_health_kompress_and_stats_e2e() {
    let state = common::test_state();
    let app = build_router(state);

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind ephemeral port");
    let addr = listener.local_addr().expect("local addr");

    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    // 1. Verify GET /health
    let health_raw =
        send_raw_http(addr, "GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
            .await;
    assert!(health_raw.starts_with("HTTP/1.1 200 OK"));
    let health: HealthResponse =
        serde_json::from_str(extract_json_body(&health_raw)).expect("valid HealthResponse JSON");
    assert_eq!(health.status, "ok");
    assert_eq!(health.bonobo_harmonic_pq, "3/2");
    assert_eq!(health.techno_zen_bonobo_pq, "9/8");

    // 2. Verify POST /api/kompress
    let body = serde_json::json!({
        "text": common::SAMPLE_NOISY_PAYLOAD,
        "mode": "aggressive"
    })
    .to_string();
    let post_req = format!(
        "POST /api/kompress HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let kompress_raw = send_raw_http(addr, &post_req).await;
    assert!(kompress_raw.starts_with("HTTP/1.1 200 OK"));
    let resp: KompressResponse = serde_json::from_str(extract_json_body(&kompress_raw))
        .expect("valid KompressResponse JSON");
    assert!(resp.original_tokens > resp.compressed_tokens);
    assert!(resp.saved_tokens > 0);
    assert!(resp.compression_savings_bps > 3000);
    assert_eq!(resp.bonobo_resonance_trit, 1);

    // 3. Verify GET /api/stats reflects the recorded fold
    let stats_raw = send_raw_http(
        addr,
        "GET /api/stats HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(stats_raw.starts_with("HTTP/1.1 200 OK"));
    let stats: KompressStats =
        serde_json::from_str(extract_json_body(&stats_raw)).expect("valid KompressStats JSON");
    assert_eq!(stats.total_requests, 1);
    assert_eq!(stats.saved_tokens, resp.saved_tokens);
    assert_eq!(stats.savings_bps, resp.compression_savings_bps);
}
