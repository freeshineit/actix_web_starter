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


## api

- [x] /api/register
- [x] /api/login
- [] /api/logout
- [] /api/v1/form
- [] /api/v1/json
- [] /api/v1/user/[id] (GET)
- [] /api/v1/user/[id] (PUT)
- [] /api/v1/user/[id] (DELETE)

- [] /api/v1/upload
- [] /api/v1/upload/chunk
  