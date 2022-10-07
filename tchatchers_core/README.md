# tchatchers_core

This project is a common library shared by both `tchatchers_back` and `tchatchers_front`

## Project structure

```
.
├── Cargo.toml => define dependencies
├── README.md
└── src
    ├── jwt.rs => JWT related logic
    ├── lib.rs
    ├── pool.rs => the connection pools logic 
    ├── room.rs => the application's room logic
    ├── user.rs => the user related logic
    └── ws_message.rs => the websocket messages related logic
```
