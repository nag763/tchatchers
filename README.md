[![tchatchers-stars](https://img.shields.io/github/stars/nag763/tchatchers?style=social)](https://github.com/nag763/tchatchers/stargazers)
[![tchatchers-license](https://img.shields.io/github/license/nag763/tchatchers)](https://raw.githubusercontent.com/nag763/tchatchers/main/LICENSE.MD)
[![github-issues](https://img.shields.io/github/issues/nag763/tchatchers)](https://github.com/nag763/tchatchers/issues)

<p align="center"><img height="300" src="https://raw.githubusercontent.com/nag763/tchatchers/main/.github/gh_logo.png"></img></p>

<h2 align="center">tchatche.rs</h2>
<h4 align="center">A blazing fast chat application built with Axum and yew.rs :rocket:</h4>

<p align="center"><img src="https://raw.githubusercontent.com/nag763/tchatchers/main/.github/app_screens.png"></img></p>

## TL;DR

* :speech_balloon: tchatche.rs is a realtime chat application built with yew.rs (frontend) and axum (backend) frameworks.
* :heavy_check_mark: Easy to use, only requires authentication informations to be used.
* :rocket: Blazing fast, completely built on Rust.
* :moon: Supports browser's dark mode.
* :sparkles: Simple yet elegant UI.

## About

tchatche.rs is an application used to help clients to communicate between each others. It is built with yew.rs and axum servers in order to provide blazing fast responses and strong API. It is also using Postgres and Redis services in order to store the user data and retrieve the room messages.

## Project structure

```
.
├── Cargo.lock
├── Cargo.toml
├── CODE_OF_CONDUCT.md
├── docker-compose_dev.yml => in order to launch redis and pg services in
development
├── docker-compose.yml => deployment docker-compose file
├── Dockerfile_back => deployment backend dockerfile
├── Dockerfile_front => deployment frontend dockerfile
├── Dockerfile_migrations => deployment migrations dockerfile
├── LICENSE.md
├── migrations => PG migration projet, has to be ran with sqlx
├── nginx.conf => the nginx configuration
├── README.md
├── tchatchers_back => the backend project
├── tchatchers_core => the core project, gathers the APi shared both by the
front and backend
└── tchatchers_front => the frontend project
```

## Technologies used

|Technology/Framework|Utility                     |Version|
|--------------------|----------------------------|-------|
|Rust                |Programming language        |1.64   |
|Tailwind            |Stylesheets                 |3.X    |
|yew.rs              |WASM Frontend framework     |0.19   |
|axum                |rust server                 |0.5.4  |
|trunk-s             |Rust development WASM server|0.16   |
|nginx               |Reverse proxy server        |latest |
|Postgres            |SQL engine                  |latest |
|Redis               |Key value NoSQL engine      |latest |

## Postgres schema

Made with one of my other tools, [doteur](https://github.com/nag763/doteur).

![img](https://raw.githubusercontent.com/nag763/tchatchers/main/.github/schema.jpeg)
