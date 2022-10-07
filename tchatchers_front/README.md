# tchatchers_front

Contains the front end logic

## How to run

```
cargo install trunk
trunk serve
```

## Project structure

```
├── assets => images and stylesheets deployed with the application
│   ├── no_pfp.webp
│   └── tailwind.css => stylesheet
├── Cargo.lock
├── Cargo.toml => dependencies
├── favicon.ico
├── index.html => base of the project, defines which assets have to be built
├── install_assets.sh => build the assets from the source code
├── src
│   ├── components => application components, updates the view
│   ├── main.rs => entry point
│   ├── router.rs => handles route logic
│   ├── services => distant and frontend components related logic 
│   └── utils => common tools 
├── tailwind.config.js => tailwind preferences
└── Trunk.toml => development preferences
```
