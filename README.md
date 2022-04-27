## actix web starter

## web_service

```bash
# install cargo-watch
cargo install cargo-watch

# run development
cargo watch -x 'run'

# run development
cargo run

```

## catalogue

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── pages       # static files pages
│   └── index.html
├── src  
│   ├── handlers.rs  # handlers
│   ├── main.rs      # entry
│   ├── routers.rs   # router
│   └── state.rs     # state
└── static     # static files
    └── index.css
```