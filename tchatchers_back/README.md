# tchatchers_back

This project gathers all the backend logic.

## How to run

```
cargo run
```

It is however advised to run forehand :

```
docker-compose up -d --build -f ./docker-compose_dev.yml
```

And then

```
sqlx migrate run
```

## Project structure

```
├── Cargo.toml => define dependencies
├── README.md
├── src
│   ├── api => HTTP apis used by the server
│   ├── extractor.rs => extractors, used to improve readability
│   ├── main.rs => application entry point
│   └── ws.rs => websocket logic
└── static => static assets, written and read during execution
```
