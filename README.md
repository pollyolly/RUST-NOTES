# RUST-NOTES

### Installation
#### M1
Install
```vim
https://www.rust-lang.org/tools/install
```
```vim
$curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Check installation
```vim
$which rustc
$which cargo
```
Running in Commandline M1
```vim
$rustc test.rs
$./test
```
Creating new Rust Project
```vim
$cargo new rust-jwt
```
Creating Rust Binary Program
```vim
$cargo new sample_program --bin
```
Creating Rust library for Python
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

[dependencies]
version = "0.14.4"
features = ["extension-module"]
```
Then Run
```vim
$cargo build --release
```
### Reference

[Google Rust Tutorial Documentation](https://doc.rust-lang.org/book/title-page.html)

[Rust Book](https://doc.rust-lang.org/1.30.0/book/first-edition/patterns.html)
