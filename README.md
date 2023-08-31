# Rust_simulation

# Get started

You can clone the source code from this [repository](https://github.com/Sandraak/Rust_simulation/tree/main). Before you can run it, Rust, Bevy and some dependencies need to be installed. Below are instructions on how to get this code running on either Ubuntu or Windows.

## Rust and Bevy on Ubuntu

### Rust Installation

Instructions on how to install Rust can be found on [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install), but are also listed below.

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

## Rust and Bevy on Windows

### Rust Installation

Instructions on how to install Rust can be found on [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install), but are also listed below.

To start using Rust, download the installer found on the [installpage](https://www.rust-lang.org/tools/install), then run the program and follow the onscreen instructions. You may need to install the Visual Studio C++ Build tools when prompted to do so, this can take a while. 

You can update your Rust toolchain by running ```console rustup update```

When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo. Cargo does lots of things:

   * build your project with ```console cargo build ```
   * run your project with ```console cargo run ```
   * test your project with ```console cargo test ```
   * build en open documentation for your project with ```console cargo doc --open```

### Bevy Installation

No additional steps neeed to be performed.

## Running the code
After succesfully installing Rust and Bevy, you can run the code by heading into the folder "Rust_simulation-main" and running ```console cargo run ```. You might get the following error:
```
    PS C:\Users\Sandra ter Maat\Documents\workspace\Rust_simulation> cargo run
    error: failed to run `rustc` to learn about target-specific information

    Caused by:
        process didn't exit successfully: `rustc - --crate-name ___ --print=file-names -Zshare-generics=n --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit code: 1)
        --- stderr
        error: the option `Z` is only accepted on the nightly compiler

        note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

        help: consider switching to a nightly toolchain: `rustup default nightly`

        note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>
```

You can fix this error by using Rust's nightly toolchain, more information about the nightly compiler can be found in the [rustup book](https://rust-lang.github.io/rustup/concepts/channels.html). You can switch to the nightly toolchain:
 ```console rustup default nightly```

 OR head over to the config.toml file and remove the "-Zshare-generics=y" listed in the rustflag. This will be line 8 for linux and line 28 for windows. NOTE that running in nightly will give a better performance.

 ```cargo build --release``` will result in a longer compile time, but better optimized code.

 ## Controlling the hardware