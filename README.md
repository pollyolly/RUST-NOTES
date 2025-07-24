# RUST-NOTES

## M1 Installation
### Install
```vim
https://www.rust-lang.org/tools/install
```
```vim
$curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
#### Check installation
```vim
$which rustc
$which cargo
```
#### Running Rust File in Commandline
Run in Debug Mode
```
$rustc test.rs
```
Compile Rust file 
```vim
$rustc -O test.rs
```
Run the Debug or Compiled rust file
```
$./test
```
## Cargo
#### Creating Rust Cargo Project
```vim
$cargo new rust-jwt
```
#### Creating Rust Cargo Binary Program
```vim
$cargo new sample_progra --bin

//To Run
$cargo run --bin server
```
```vim
#Cargo.toml
[package]
name = "rust-grpc-chat-auth"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "server"
path = "src/server.rs"

[[bin]]
name ="client"
path = "src/client.rs"
```
#### Creating Rust Cargo library for Python
```vim
$cd proj_dir
$cargo new "library_name" --lib
```
```vim
#Cargo.toml
[package]
name = "pyo3"
version = "0.1.0"
edition = "2021"

[lib]
name = "calculate_pi"
create-type = ["cdylib"]
```
#### Cargo Clean and Release Build
```vim
$cargo clean
$cargo build --release
```
```vim
//Build Location
$cd rust-project/target/debug/rust-project-name

//Execute The Project
$./rust-project-name
```
## Cargo Hot Reload
```
$cargo install cargo-watch
```
To run
```
$cargo watch -x run
```
## Reference
[Shuttle Rs Tutorials](https://docs.shuttle.rs/tutorials/websocket-chat-app-js)

[Rust Lang Nursery](https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html)

[Google Rust Tutorial Documentation](https://doc.rust-lang.org/book/title-page.html)

[Rust Book](https://doc.rust-lang.org/1.30.0/book/first-edition/patterns.html)

[Rust Async/Await Book](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)

[Tokio Runtime](https://tokio.rs/tokio/tutorial)

### Recommended by Lets get rusty
[Rust by example](https://doc.rust-lang.org/stable/rust-by-example/)

[Rust by Practice](https://practice.course.rs/basic-types/numbers.html)

[Lets Get Rusty](https://learn.letsgetrusty.com/index.html)
