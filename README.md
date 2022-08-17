# Test ARM peripherals

This is a sample code for testing your peripherals on arm boards like Raspberry pi/ IMX/ SUNXI boards.

All Tests run simultaneously for checking boot time 

Here is a list of testing:
 - CAN
 - Ethernet
 - GPIO
 - UART
 - Industrial IO (iio)

In ```main.rs```  change const values based on your board configuration

# Cross Compiling 

For cross compiling rust project base on your board please change config.toml

if you use Yocto

```bash

source /opt/poky/3.1.16/environment-setup-cortexa7hf-neon-vfpv4-poky-linux-gnueabi
cargo build

```
