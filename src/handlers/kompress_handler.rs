//! Axum handlers for `/`, `/ws`, `/api/stats`, `/api/history`, and `/api/kompress`.

use crate::{
    config::FoldMode,
    error::{AppError, Result},
    models::{CompressionLogEntry, KompressRequest, KompressResponse, KompressStats},
    services::AppState,
};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Html, IntoResponse, Json},
};

/// Upgrade an incoming HTTP connection to a live telemetry WebSocket stream (`/ws`).
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();
    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}

/// Return current cumulative `KompressStats` (`GET /api/stats`).
pub async fn get_stats(State(state): State<AppState>) -> Result<Json<KompressStats>> {
    let stats = state
        .stats
        .lock()
        .map_err(|_| AppError::StatePoisoned("stats lock poisoned".to_string()))?
        .clone();
    Ok(Json(stats))
}

/// Return recent `CompressionLogEntry` ring buffer (`GET /api/history`).
pub async fn get_history(State(state): State<AppState>) -> Result<Json<Vec<CompressionLogEntry>>> {
    let hist = state
        .history
        .lock()
        .map_err(|_| AppError::StatePoisoned("history lock poisoned".to_string()))?
        .clone();
    Ok(Json(hist))
}

/// Fold raw agent/tool context and emit live WebSocket telemetry (`POST /api/kompress`).
pub async fn handle_kompress(
    State(state): State<AppState>,
    Json(req): Json<KompressRequest>,
) -> Result<Json<KompressResponse>> {
    if req.text.is_empty() {
        return Err(AppError::InvalidInput("request text must not be empty".to_string()));
    }

    let mode = req
        .mode
        .as_deref()
        .map(|m| FoldMode::parse_opt(Some(m)))
        .unwrap_or(state.config.default_fold_mode);

    let outcome = state.engine.fold_with_metrics(&req.text, mode);
    state.record_fold(req.text.len(), &outcome)?;

    Ok(Json(KompressResponse {
        compressed: outcome.compressed,
        original_tokens: outcome.original_tokens,
        compressed_tokens: outcome.compressed_tokens,
        saved_tokens: outcome.saved_tokens,
        compression_ratio_pq: format!(
            "{}/{}",
            outcome.compression_ratio.numer(),
            outcome.compression_ratio.denom()
        ),
        savings_ratio_pq: format!(
            "{}/{}",
            outcome.savings_ratio.numer(),
            outcome.savings_ratio.denom()
        ),
        compression_savings_bps: outcome.savings_bps,
        bonobo_resonance_trit: outcome.resonance_trit,
        latency_us: outcome.latency_us,
    }))
}

/// Serve the live Techno-Zen-Bonobo WebUI dashboard (`GET /`).
pub async fn serve_webui() -> impl IntoResponse {
    Html(WEBUI_HTML)
}

const WEBUI_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>⚡ KOMPRESS-SUPER-ULTRA CORE // LIVE DASHBOARD</title>
  <style>
    :root {
      --bg: #07090e;
      --card: #0f141f;
      --border: #1e2638;
      --accent: #ff4757;
      --gold: #ffa502;
      --green: #2ed573;
      --cyan: #00d2d3;
      --text: #f1f2f6;
      --dim: #747d8c;
    }
    body {
      margin: 0;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, monospace;
      background: var(--bg);
      color: var(--text);
      display: flex;
      flex-direction: column;
      height: 100vh;
    }
    header {
      padding: 1rem 2rem;
      background: var(--card);
      border-bottom: 1px solid var(--border);
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    h1 { margin: 0; font-size: 1.2rem; letter-spacing: 0.05em; color: var(--gold); }
    .badge {
      background: #182030;
      padding: 0.35rem 0.75rem;
      border-radius: 4px;
      font-size: 0.8rem;
      border: 1px solid var(--border);
    }
    .metrics {
      display: grid;
      grid-template-columns: repeat(5, 1fr);
      gap: 1rem;
      padding: 1.25rem 2rem;
    }
    .metric-card {
      background: var(--card);
      border: 1px solid var(--border);
      padding: 1rem;
      border-radius: 8px;
    }
    .metric-label { font-size: 0.75rem; color: var(--dim); text-transform: uppercase; letter-spacing: 0.05em; }
    .metric-val { font-size: 1.5rem; font-weight: bold; margin-top: 0.3rem; color: var(--green); }
    .workspace {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 1rem;
      padding: 0 2rem 1rem;
      flex: 1;
      min-height: 0;
    }
    .editor-pane {
      background: var(--card);
      border: 1px solid var(--border);
      border-radius: 8px;
      display: flex;
      flex-direction: column;
      overflow: hidden;
    }
    .pane-header {
      padding: 0.75rem 1rem;
      background: #141b2a;
      font-size: 0.85rem;
      font-weight: bold;
      border-bottom: 1px solid var(--border);
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    textarea {
      flex: 1;
      background: transparent;
      border: none;
      color: var(--text);
      font-family: inherit;
      padding: 1rem;
      resize: none;
      font-size: 0.95rem;
      line-height: 1.5;
      outline: none;
    }
    .output-box {
      flex: 1;
      padding: 1rem;
      font-size: 0.95rem;
      line-height: 1.5;
      overflow-y: auto;
      white-space: pre-wrap;
      color: #ced6e0;
    }
    button {
      background: var(--accent);
      color: white;
      border: none;
      padding: 0.45rem 1.1rem;
      border-radius: 4px;
      cursor: pointer;
      font-weight: bold;
      transition: opacity 0.2s;
    }
    button:hover { opacity: 0.9; }
    .live-stream-bar {
      margin: 0 2rem 1rem;
      background: var(--card);
      border: 1px solid var(--border);
      border-radius: 6px;
      padding: 0.5rem 1rem;
      font-size: 0.85rem;
      color: var(--cyan);
      display: flex;
      align-items: center;
      gap: 0.75rem;
    }
    .live-dot {
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: var(--green);
      animation: pulse 1.5s infinite;
    }
    @keyframes pulse {
      0% { opacity: 0.4; }
      50% { opacity: 1; }
      100% { opacity: 0.4; }
    }
  </style>
</head>
<body>
  <header>
    <h1>⚡ KOMPRESS-SUPER-ULTRA CORE // TECHNO-ZEN-BONOBO ERA {&lt;3,&lt;3,&lt;3}+1</h1>
    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
      <span class="badge" style="color: var(--cyan);">WebSocket: <span id="ws-status">Connecting...</span></span>
      <span class="badge">Techno-Zen-Bonobo: (3/2)×(3/4)=9/8 · 432Hz/180WPM · p/qT=375/1</span>
      <span class="badge">SWE: 447/500 · MATH: 493/500 · AIME: 29/30 · SPARK: 117/117</span>
      <a class="badge" href="https://huggingface.co/PeetPedro/qwen4_exp-8b-obliterated-abliterated-kompress-ultra" target="_blank" style="text-decoration:none;color:var(--gold);">HF: qwen4_exp-8b-kompress-ultra</a>
      <a class="badge" href="https://codeberg.org/8b-is/eightb-agy-fanout" target="_blank" style="text-decoration:none;color:var(--green);">8b-agy v0.4.0</a>
    </div>
  </header>

  <div class="metrics">
    <div class="metric-card">
      <div class="metric-label">Savings Ratio (p/q)</div>
      <div class="metric-val" id="metric-ratio">0/1</div>
    </div>
    <div class="metric-card">
      <div class="metric-label">Token Savings</div>
      <div class="metric-val" id="metric-saved">0</div>
    </div>
    <div class="metric-card">
      <div class="metric-label">Compression % (bps)</div>
      <div class="metric-val" id="metric-pct">0.00%</div>
    </div>
    <div class="metric-card">
      <div class="metric-label">Avg Latency</div>
      <div class="metric-val" id="metric-lat">0 μs</div>
    </div>
    <div class="metric-card">
      <div class="metric-label">Processed Requests</div>
      <div class="metric-val" id="metric-reqs" style="color: var(--gold);">0</div>
    </div>
  </div>

  <div class="live-stream-bar">
    <div class="live-dot"></div>
    <span id="live-stream-text">Listening for live agent token folds across the cluster...</span>
  </div>

  <div class="workspace">
    <div class="editor-pane">
      <div class="pane-header">
        <span>RAW AGENT CONTEXT / TOOLS / ANSI LOGS</span>
        <button onclick="kompress()">KOMPRESS NOW</button>
      </div>
      <textarea id="raw-input" placeholder="Paste verbose tool output, JSON, stack trace or agent logs here..."></textarea>
    </div>
    <div class="editor-pane">
      <div class="pane-header">
        <span>SOVEREIGN FOLDED CONTEXT</span>
        <span id="token-diff" style="color: var(--gold);">Ready</span>
      </div>
      <div class="output-box" id="folded-output">// Folded output will appear here with exact p/q accounting</div>
    </div>
  </div>

  <script>
    async function updateStats() {
      try {
        const res = await fetch('/api/stats');
        const s = await res.json();
        document.getElementById('metric-ratio').innerText = s.savings_ratio_pq;
        document.getElementById('metric-saved').innerText = s.saved_tokens.toLocaleString();
        document.getElementById('metric-pct').innerText = (s.savings_bps / 100).toFixed(2) + '%';
        document.getElementById('metric-lat').innerText = s.latency_us_avg + ' μs';
        document.getElementById('metric-reqs').innerText = s.total_requests;
      } catch (e) {}
    }

    async function kompress() {
      const text = document.getElementById('raw-input').value;
      if (!text) return;
      document.getElementById('token-diff').innerText = 'Folding...';
      const res = await fetch('/api/kompress', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text })
      });
      const data = await res.json();
      document.getElementById('folded-output').innerText = data.compressed;
      const pct = (data.compression_savings_bps / 100).toFixed(2);
      document.getElementById('token-diff').innerText = `${data.original_tokens} → ${data.compressed_tokens} tokens (p/q: ${data.compression_ratio_pq} · -${pct}% · trit: ${data.bonobo_resonance_trit}) in ${data.latency_us}μs`;
      updateStats();
    }

    function connectWS() {
      const proto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const ws = new WebSocket(`${proto}//${window.location.host}/ws`);
      ws.onopen = () => {
        document.getElementById('ws-status').innerText = 'Online';
        document.getElementById('ws-status').style.color = 'var(--green)';
      };
      ws.onmessage = (ev) => {
        const data = JSON.parse(ev.data);
        const pct = (data.savings_bps / 100).toFixed(2);
        document.getElementById('live-stream-text').innerText = `[Live Event #${data.id}] Folded ${data.input_tokens} → ${data.output_tokens} tokens (-${pct}% · p/q: ${data.ratio_pq}) in ${data.latency_us}μs: "${data.snippet}"`;
        updateStats();
      };
      ws.onclose = () => {
        document.getElementById('ws-status').innerText = 'Reconnecting...';
        document.getElementById('ws-status').style.color = 'var(--accent)';
        setTimeout(connectWS, 2000);
      };
    }

    connectWS();
    setInterval(updateStats, 3000);
    updateStats();
  </script>
</body>
</html>"#;
