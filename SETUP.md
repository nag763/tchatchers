# Setting up the application

This folder contains the documentation that and scripts that can be helpful if you try to install this app on your workstation.

## Foreword

It is to be noted that the setting up has only been made available for apt, rpm and pacman based distros. If you want your distro to be supported, I recommend to check the objectives and install the application on your end with your knowledge and then update the setup script.

## Objectives

- Ensure that all the following packages are installed : gcc, ssl tools in developer mode, rustup (includes cargo), npm
- Add the target `wasm32-unknown-unknown` from rustup
- Install `trunk` and (recommended) `cargo-watch` from cargo
- Setting up the environment file
- Starting the database
- Running the application in developper mode

## 0. Prerequisites

- Git is needed, if you didn't clone yet this repo, use `git clone https://github.com/nag763/tchatchers`, and then position yourself at the root of the project.
- Make sure your distro and its packages are up to date
- Have cargo and rustup installed. Follow [the official guide](https://www.rust-lang.org/tools/install) if needed.
- Install `docker` (guide [here](https://docs.docker.com/engine/install/)) and care about having rights with your user on dockerd if you are on linux (guide [here](https://docs.docker.com/engine/install/linux-postinstall/)).

Then you can either setup your environment by :
- Running the following command : `curl -ssl https://raw.githubusercontent.com/nag763/tchatchers/main/setup.sh | sh`
- Or using `cargo-make`
    1. Install cargo-make : `cargo install cargo-make`
    2. Run the following : `cargo-make make install-native-pkgs`

## 1. Setting up the environment

Now that all the prequesites have been installed, I recommend you to run the following before starting the app in either dev or prod mode 

```bash
cargo-make make setup-env
```

If you want to set up a new environment, it is recommanded to first run the following command :

```bash
cargo-make make clear-symlink
```

## 2. Starting the db, back and front as observable processes (dev mode)

So now, you most likely want to start the application as an observable process, meaning that both the front and the back will be running, and they will recompile everytime a change is performed.

For that run 

```bash
cargo-make start-dev
```

And you should be all good to start the application locally. :happy:

If you access http://localhost:3000/ , you should see the login screen of the app.

## Recommandation

It is recommended to create an administrator profile with the help of this command :

```bash
cargo r --bin tct user create
```