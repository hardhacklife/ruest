# Features HTTP Axum (RUEST)

RUEST active **toutes** les features optionnelles d’Axum 0.7 au niveau du workspace. Les applications qui dépendent de `ruest` héritent de ces capacités sans reconfigurer `axum`.

## Features activées

| Feature | Usage |
|---------|--------|
| `http1` | HTTP/1.1 (défaut Hyper) |
| `http2` | HTTP/2 (négo via Hyper ; TLS en production recommandé) |
| `json` | `Json<T>` |
| `macros` | Macros Axum (`#[debug_handler]`, …) |
| `matched-path` | `MatchedPath` — chemin matché par le routeur |
| `multipart` | `Multipart` — upload fichiers |
| `original-uri` | `OriginalUri` — URI avant rewrite |
| `tokio` | `axum::serve`, SSE, `ConnectInfo` |
| `tower-log` | Logs Tower |
| `tracing` | Traces des rejets d’extracteurs |
| `ws` | `WebSocketUpgrade`, `WebSocket`, `Message` |
| `form` | `Form<T>` — formulaires URL-encodés |
| `query` | `Query<T>` — query string |

## Import

```rust
use ruest::prelude::*;
// ou
use ruest::http::{Json, Form, Query, Path, Multipart, WebSocketUpgrade, MatchedPath, OriginalUri};
```

## Exemples

### Query + JSON

```rust
async fn search(Query(params): Query<SearchParams>) -> Json<Vec<Item>> { /* ... */ }
```

### Multipart

```rust
async fn upload(mut multipart: Multipart) -> Result<(), HttpError> {
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        let data = field.bytes().await?;
        // traiter name / data
    }
    Ok(())
}
```

### WebSocket

```rust
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(Message::Text(text)) = msg {
            let _ = socket.send(Message::Text(text)).await;
        }
    }
}
```

### ConnectInfo (IP client)

Utiliser `axum::serve` avec `into_make_service_with_connect_info::<SocketAddr>()` — voir la doc Axum ; RUEST expose le type via `ConnectInfo`.

## Hyper

`hyper` est compilé avec `http1`, `http2`, `server`, `client` pour supporter la pile complète utilisée par Axum.
