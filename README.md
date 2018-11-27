# raspi-zero-garage-control
(INCOMPLETE)
Garage Status Indicator/Controller, using a Raspberry Pi Zero W custom Bonnet, with Rust-based software stack.

## Hardware
KiCAD schematics & PCB layout are provided. PCB designed for 10mmx10mm cheap from EasyEDA/JLCPCB. 
Simply generate the GERBERs using KiCAD 5.0 or later, and order from them, and you'll get 20 copies of this Bonnet.

## Software
GPIO & HTTP stack use Rust. While compiling this on the Pi does work, it will take a long time, so setting up a development computer for cross-compiling is recommended.
