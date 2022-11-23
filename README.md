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

## Foreword (10/10/2022)

I know there is room for improvement in some areas for this project notably :
* some security points (ie CSRF protection)
* some code quality (some elements of my frontend project could be better)
* some logic (websocket logic is a bit shaky sometimes)

But I think that overall the goals I wanted to attain with this project have been achieved and I don't feel like improving this project would be worth it since the only and sole goal of this project was to be an entry point into yew.rs and WASM.

I hope this project can be useful to anyone wanting to build any more or less advanced similar application. :blush:

## How to access the application

The application is deployed on https://tchatche.rs and should be compatible with any modern navigator.

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

## Production project architecture

![](https://raw.githubusercontent.com/nag763/tchatchers/17a4e86adb1c26259c3890e3303d6a67d3dd70df/.github/application_schema.jpg)

The production architecture consists of several layers :
* <u>The client</u>, who uses the application and interacts with the different ressources.
* <u>The proxy layer</u>, that defines some security constraints such as BruteForce mitigation, the HTTPS connection, the read time out and HTTP headers. This layer is the sole entry point for the client to the application, as others aren't publicly reachable since they are on another network.
* <u>The applicative layer :</u> One part being the frontend built in WASM, so static assets in production, another part being the backend, built in axum, so running as a task. It is important to note that in production, only the backend part exists, since the frontend and proxy are on the same image. Besides, only the backend can access to the data layer within the network.
* <u>The data layer :</u> Mainly used for persistence. While Postgres will contain the data about the users, redis will store the messages that have been sent in the chatrooms. Postgres schema can be found right below, redis one is as simple as `ROOM_NAME[KEY]=MESSAGES[LIST]`.

## Postgres schema

Made with one of my other tools, [doteur](https://github.com/nag763/doteur).

![img](https://raw.githubusercontent.com/nag763/tchatchers/main/.github/schema.jpeg)

## Personnal objectives behind this project, feedback

My goal with this project was to learn more about both WASM and Websocket technologies besides of perfecting my Rust knowledge. It was really nice to build such a project and I hope it can inspire or help other peoples to built similar tools. I think the code is pretty good (even though it should be perfectible) on the back and core projects, but quite perfectible on the front one. It is to note that there are still some bugs remaining.

My feeling is that both Rust and WASM have a pretty good future when it comes to frontend apps. The backend side of Rust already has several frameworks that are making it a reliable language when it comes to handling data logic, and the development of yew.rs or similar technologies in the future could be a game changer for the interest users and firms have toward the language. 

## Special thanks

Thanks to the Rust community for all their advices and their time reviewing my project, it's always a pleasure to be part of such a community. :blush: :crab:
