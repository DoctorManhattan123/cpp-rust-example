# Small CPP Rust integration example

## Requirements

### Arch

```
sudo pacman -S base-devel cmake
```

## Build

This project will use `CMake` as it is the most used CPP software for build automation.

To build your project use the `build.sh` script inside the root folder. You might need to make this file executable:

```
chmod +x build.sh
```

## Include Rust

To add a new Rust library run the following command:

```
cargo new --lib rust_lib
```

You will also need to link the Rust library to the CPP code. This can be done inside of the `CMakeLists.txt` file.
