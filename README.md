# RUST-NOTES

### Installation
#### M1
```vim
go to https://www.rust-lang.org/tools/install
run command: $curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
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
