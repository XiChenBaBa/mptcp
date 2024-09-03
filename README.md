# Multi-path TCP

![Session](img/session.drawio.png)

## How to use

1. Install Rust: <https://www.rust-lang.org/tools/install>
1. `git clone <REPO_URL>.git`
1. On the server side, run `cargo run -r -p cli --bin server -- config.json`
1. On the client side, run `cargo run -r -p cli --bin client -- config.json`

### Client-side `config.json`

- **servers**: The server endpoints to connect to.
- **local_bind**: The local port to bind to for client access.

```json
{
    "servers": ["1.2.3.4:30001", "1.2.3.4:30001"],
    "local_bind": "0.0.0.0:12811"
}
```

### Server-side `config.json`

- **listen**:The address and port the server will bind to.
- **streams**: The number of streams to be used.
- **remote**: The address of the upstream server.
```json
{
    "listen": "0.0.0.0:30001",
    "streams": 2,
    "remote": "127.0.0.1:27429"
}
```
