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

 The code will build and run now, it might however still panic. This is because the system is meant to communicate with server in the hardware implementation. When it can't it might show show some unexpected behaviour.

 ### Run simulation without hardware

 If you want to run the simulation without the hardware, you can comment out the following lines
 * ``` .add_system(poll_system)``` (line 42 in controller.rs).
 * ```magnet_status.real = false;``` (line 262 in controller.rs).


 ## Controlling the hardware

The system will send HTTP Requests to an Arduino, which controls two steppermotors and an elektromagnet to show the same behaviour as in the simulation. You can download the code from this [repository](https://github.com/Sandraak/Automated_chessboard).

### Hardware
For this code to function you need to flash it to an Arduino Uno with Ethernetshield that is connected to the hardware according to the following scheme.

### Arduino IDE

You can install the Arduino IDE using the instructins on the [downloadpage](https://www.arduino.cc/en/software) of the Arduino website. Open the Arduino IDE and connect your Arduino to your laptop using USB, you can now select a port and board, the boardshoud be "Arduino Uno".
>   
    NOTE FOR LINUX USERS
    Installing from the ubuntu store can give some weird behaviour, the Arduino IDE works better when you install it using the appimage in the previously posted link.
>

#### Dependencies
The code uses some libraries you still need to include using the library manager in the Arduino IDE. You can do so by clicking on ``Sketch -> Include Libraries -> Manage Libraries ...``. A search bar will appear, search and add the following libraries:
* ArduinoUnit.h
* Ethernet.h

>
    NOTE, the library you need is not always listed first, you should pay attention to the names.
>

### Establishing a connection
When this is all setup you need to be able to setup a connection between your laptop and the Arduino. You can do this by executing the following steps:

1. Assign IP 192.168.1.1 and subnetmask 255.255.255.0 to the ethernet card (not your WiFI connection) van de laptop. You can find information on how to this this by searching on “assign static ethernet IP address in <operating system>” in your search engine. If needed, you can give your WiFi connection a higher priority than ethernet.

2. Install the DHCP server
```console sudo apt install isc-dhcp-server```
The server might try to start and fail, you can ignore this.

3. Configure the DHCP server
```console sudo nano /etc/default/isc-dhcp-server ```
Replace the line ``` INTERFACESv4="" ``` to ```INTERFACESv4="eth0"```
Exit with ``ctrl-X`` en save your changes.
4. Stop the DHCP services
```console sudo systemctl stop isc-dhcp-server.service```
5. Alter the DCHP configuration
    * Open the file:
        ```console sudo nano /etc/dhcp/dhcpd.conf ```
    * Remove the # in the line: ```#authoritative;```, there will just be ```authoritative;``` now.

    * Add the following at the end of the file:
        >
            subnet 192.168.1.0 netmask 255.255.255.0 {
            range 192.168.1.51 192.168.1.200;
            option broadcast-address 192.168.1.255;
            default-lease-time 600;
            max-lease-time 7200;
            }
        >

6.Start the DHCP service
```console sudo service isc-dhcp-server start```

    # Ready!

    Once you have completed all previously listed steps, you can run the whole system. There are a few things to pay attention to for a smooth run.
    
    ## Hardware
    1. Place the physical magnet neatly beneath square ``(0,0)``.
    2. Upload the [code](https://github.com/Sandraak/Automated_chessboard) to the Arduino and wait for the server to start.
    3. Plugin the power wupply for the magnet and motors.
    ## Simulation
    1. Start the simulation.
    2. Wait for the magnet to reach position (0,0).
    2. Perform a move (default: playing as White).


