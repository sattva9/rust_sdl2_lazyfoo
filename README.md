# SDL2 Rust

These are [LazyFoo](https://crates.io/crates/sdl2) sdl2 tutorials ported to Rust

- Install sdl2 by following the [installation steps](https://crates.io/crates/sdl2)
- If you use Windows or macOS, ensure the `LIBRARY_PATH` environment variable is set

```shell
# macOS
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

### Static Linking

For static linking using vcpkg, run the following commands and update Cargo.toml
- vcpkg build needs `VCPKG_ROOT` environment variable to be set

```shell
cargo install cargo-vcpkg
cargo vcpkg build
cargo build
```

Cargo.toml
```
[dependencies]
sdl2 = { version = "0.37.0", features = ["image", "ttf", "mixer", "static-link", "use-vcpkg"] }

[package.metadata.vcpkg]
git = "https://github.com/microsoft/vcpkg"
branch = "master"
dependencies = [
    "sdl2",
    "sdl2-image[libjpeg-turbo,tiff,libwebp]",
    "sdl2-ttf",
    "sdl2-mixer[mpg123]",
]
```

## Run Command

```shell
cargo build --release
./target/release/open_window
```
