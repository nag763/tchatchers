# TCT - Tchatchers CLI Tool

tct is a CLI tool designed to run the common operations on the tchatchers_app.

## Usage

```bash

tct [OPTIONS] <COMMAND>

Commands:
  user     Manages the users of the application
  room     Manages the rooms of the application
  message  Manages the messages stored in the database
  env      Helper to either set up a new environment or check the current one
  help     Print this message or the help of the given subcommand(s)

Options:
  -e, --env <ENV>  Precise a .env file.
  -h, --help       Print help
  -V, --version    Print version
```

## Usage Notes

This tool is meant to be used within the tchatchers project. Run it only either within the workspace (dev) or as a binary (prod).

## Project Structure

```
├── Cargo.toml
└── src
    ├── actions
    │   ├── env.rs
    │   ├── message.rs
    │   ├── mod.rs
    │   ├── room.rs
    │   └── user.rs
    ├── args
    │   ├── env.rs
    │   ├── message.rs
    │   ├── mod.rs
    │   ├── room.rs
    │   └── user.rs
    ├── common
    │   ├── mod.rs
    │   └── user.rs
    ├── errors.rs
```

## File Descriptions

<ul>
   <li><code>Cargo.toml</code> - Configuration file for Rust's package manager, Cargo.</li>
   <li><code>src/actions</code> - Module used to gather all the actions being able to be performed.</li>
   <li><code>src/actions/env.rs</code> - Module used to manage environment variables.</li>
   <li><code>src/actions/message.rs</code> - Module used to manage messages stored in the database.</li>
   <li><code>src/actions/mod.rs</code> - Module used to import all the actions to the root of the module.</li>
   <li><code>src/actions/room.rs</code> - Module used to manage rooms of the application.</li>
   <li><code>src/actions/user.rs</code> - Module used to manage users of the application.</li>
   <li><code>src/args</code> - Module used to parse CLI arguments.</li>
   <li><code>src/args/env.rs</code> - Module used to parse CLI arguments related to environment variables.</li>
   <li><code>src/args/message.rs</code> - Module used to parse CLI arguments related to messages stored in the database.</li>
   <li><code>src/args/mod.rs</code> - Module used to import all the argument parsers to the root of the module.</li>
   <li><code>src/args/room.rs</code> - Module used to parse CLI arguments related to rooms of the application.</li>
   <li><code>src/args/user.rs</code> - Module used to parse CLI arguments related to users of the application.</li>
   <li><code>src/common</code> - Module used to gather all the common logic.</li>
   <li><code>src/common/mod.rs</code> - Module used to import all the common logic to the root of the module.</li>
   <li><code>src/common/user.rs</code> - Module used to manage user-related operations common to different parts of the CLI tool.</li>
   <li><code>src/errors.rs</code> - Module used to manage errors and exit codes of the CLI tool.</li>
</ul>


