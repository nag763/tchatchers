# Setting up the application

This folder contains the documentation that and scripts that can be helpful if you try to install this app on your workstation.

## Foreword

It is to be noted that the setting up has only been made available for apt, rpm and pacman based distros. If you want your distro to be supported, I recommend to check the objectives and install the application on your end with your knowledge and then update the setup script.

## Setting up for developping

### Objectives

- Ensure that all the following packages are installed : gcc, ssl tools in developer mode, rustup (includes cargo), npm
- Add the target `wasm32-unknown-unknown` from rustup
- Install `trunk` and (recommended) `cargo-watch` from cargo
- Setting up the environment file
- Starting the database
- Running the application in developper mode


### 0. Prerequisites

- Git is needed, if you didn't clone yet this repo, use `git clone https://github.com/nag763/tchatchers`, and then position yourself at the root of the project.
- Make sure your distro and its packages are up to date
- Have cargo and rustup installed. Follow [the official guide](https://www.rust-lang.org/tools/install) if needed.
- Install `docker` (guide [here](https://docs.docker.com/engine/install/)) and care about having rights with your user on dockerd if you are on linux (guide [here](https://docs.docker.com/engine/install/linux-postinstall/)).

Then you can either setup your environment by :
- Running the following command : `curl -ssl https://raw.githubusercontent.com/nag763/tchatchers/main/setup.sh | sh`
- Or using `cargo-make`
    1. Install cargo-make : `cargo install cargo-make`
    2. Run the following : `cargo-make make install-native-pkgs`

### 1. Setting up the environment

Now that all the prequesites have been installed, I recommend you to run the following before starting the app in either dev or prod mode 

```bash
cargo-make make setup-env
```

If you want to set up a new environment, it is recommanded to first run the following command :

```bash
cargo-make make clear-symlink
```

### 2. Starting the db, back and front as observable processes (dev mode)

So now, you most likely want to start the application as an observable process, meaning that both the front and the back will be running, and they will recompile everytime a change is performed.

For that run 

```bash
cargo-make start-dev
```

And you should be all good to start the application locally. :happy:

If you access http://localhost:3000/ , you should see the login screen of the app.

### Recommandation

It is recommended to create an administrator profile with the help of this command :

```bash
cargo r --bin tct user create
```

---

## Setting up for production

This tutorial is dedicated to users who would rather install the application on a production server.

Most of the ways to install dependencies or packages won't be rexplained here since it is installed on the developer section. Refer to it if needed.

### Objectives

- Creating an environment file exploitable by the end application.
- Setting up the nginx config.
- Starting the application.

### 0. Prerequisites

- Have the project installed and be on the project workspace root path.
- (Prefered) Have `cargo-make` installed.
- Being able to run the tct crate of this project, this should print the help : `cargo r --bin tct`.
- Docker and Docker Compose being installed on the station.

### Optional : Generate a self signed certificate

You might want to generate a self signed certificate for HTTPs config, to do that, run the following with cargo-make :

```
cargo-make make generate-self-signed-certificate
```

Be aware that self signed certificates might not be suitable for public production usages.

### 1. Generate an environment file

Run the following for that, and follow the dialogs.

```
cargo r --bin tct env create
```

Be aware that for the HTTPs config, you need to precise all paths as relative './myfile' or absolute '/path/to/myfile/'.

### 2. Generate the nginx config

Run the following command, and follow again the dialogs.

```
cargo r --bin tct env build-nginx-conf
```

### 3. Start the server

Your config should now be ready for prod, run the following to start your server with the configuration you parametarized :

For first run, run the following :

```
docker-compose up --build 
```

If no error is thrown, for subsequent stop/restarts, run the following to detach the process to the current shell

```
docker-compose up --build -d
```