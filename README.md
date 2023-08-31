# Rust_simulation

# Get started

You can clone the source code from this [repository](https://github.com/Sandraak/Rust_simulation/tree/main). Before you can run it, Rust, Bevy and some dependencies need to be installed. Below are instructions on how to get this code running on either Ubuntu or Windows.

## Ubuntu

### Rust Installation

To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.
```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You can update your Rust toolchain by running ```console rustup update```

When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo. Cargo does lots of things:

   * build your project with ```console cargo build ```
   * run your project with ```console cargo run ```
   * test your project with ```console cargo test ```
   * build en open documentation for your project with ```console cargo doc --open```

### Bevy Installation

Before you can run Bevy code you need to install some depencies. You can do so by running the following in your terminal: ```console
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev```

if using Wayland, you will also need to install

```console sudo apt-get install libwayland-dev libxkbcommon-dev```

Depending on your graphics card, you may have to install one of the following: ```console vulkan-radeon```, ```console vulkan-intel```, or ```console mesa-vulkan-drivers```

Compiling with clang is also possible - replace the ```console g++ ``` package with ```console clang```

## Windows

### Rust Installation

### Bevy Installation

