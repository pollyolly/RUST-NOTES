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
```vim
$rustc test.rs
$./test
```
#### Creating new Rust Project
```vim
$cargo new rust-jwt
```
#### Creating Rust Binary Program
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
#### Creating Rust library for Python
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
#### Clean and Building Rust Project
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
### Reference
[Rust Lang Nursery](https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html)

[Google Rust Tutorial Documentation](https://doc.rust-lang.org/book/title-page.html)

[Rust Book](https://doc.rust-lang.org/1.30.0/book/first-edition/patterns.html)

[Rust Async/Await Book](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)

[Tokio Runtime](https://tokio.rs/tokio/tutorial)
