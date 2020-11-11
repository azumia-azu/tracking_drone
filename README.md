# Two Wheel Tracking Drone

This Project is to control a two wheel drone use:

* Board: STM32FXCXT6 Board V5.02 and 
* Chip: STM32f103C8T6 [datasheet](https://pdf1.alldatasheet.com/datasheet-pdf/view/201596/STMICROELECTRONICS/STM32F103C8T6.html)
* Debug Probes: J-Link [product page](https://www.segger.com/products/debug-probes/j-link/)

## Wire Connection

| FC-35 tracking sensor board | STM32FXCXT6 Board | HL-1 Car Board |
| --------------------------- | ----------------- | -------------- |
| DO1                         | PB05              | -              |
| DO2                         | PB06              | -              |
| DO3                         | PB07              |                |
| GND                         | GND               | GND            |
| VCC                         | +5V               | VCC            |
|                             |                   |                |
|                             | PB10              | IN1            |
|                             | PB11              | IN2            |
|                             | PB12              | IN3            |
|                             | PB13              | IN4            |
|                             | 3.3V              | EN1            |
|                             | 3.3V              | EN2            |





## Download Dependencies

Because of some Driver problem this tutor is only on MacOS

All the tools can be install using [Homebrew](https://brew.sh/):

```bash
# Arm GCC toolchain
$ brew tap ArmMbed/homebrew-formulae
$ brew install arm-none-eabi-gcc

# OpenOCD
$ brew install openocd
```

Rustc & Cargo

```bash
# check you rust toolchain is up to date
$ rustc -V
rustc 1.47.0 (18bf6b4f0 2020-10-07)

# cargo-binutils
$ rustup target add thumbv7m-none-eabi # for the Cortex-M3 processors which stm32f103 use
$ rustup component add llvm-tools-preview
$ cargo install cargo-binutils --vers 0.3.2
$ cargo size -- -version
LLVM (http://llvm.org/):
  LLVM version 11.0.0-rust-1.47.0-stable
  Optimized build.
  Default target: x86_64-apple-darwin19.6.0
  Host CPU: icelake-client
```



OpenOCD connection:

```bash
$ openocd -f interface/jlink.cfg -f target/stm32f1x.cfg
# should get output below
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "jtag". To override use 'transport select <transport>'.
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
jtag_ntrst_delay: 100
none separate
cortex_m reset_config sysresetreq
Info : No device selected, using first device.
Info : J-Link V9 compiled Dec 13 2019 11:14:50
Info : Hardware version: 9.60
Info : VTarget = 3.193 V
Info : clock speed 1000 kHz
Info : JTAG tap: stm32f1x.cpu tap/device found: 0x3ba00477 (mfg: 0x23b (ARM Ltd.), part: 0xba00, ver: 0x3)
Info : JTAG tap: stm32f1x.bs tap/device found: 0x16410041 (mfg: 0x020 (STMicroelectronics), part: 0x6410, ver: 0x1)
Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

openocd will block the terminal. That's fine. now you can kill it

## Build the program:

```bash
# make sure you are in the project dir.
tracking_drone $ cargo build
   Compiling semver-parser v0.7.0
   Compiling typenum v1.12.0
   Compiling version_check v0.9.2
   Compiling proc-macro2 v1.0.24
   Compiling unicode-xid v0.2.1
   Compiling stable_deref_trait v1.2.0
   Compiling syn v1.0.48
   Compiling cortex-m v0.6.4
   Compiling cortex-m-rt v0.6.13
   Compiling vcell v0.1.2
   Compiling r0 v0.2.2
   Compiling nb v1.0.0
   Compiling stm32f1 v0.11.0
   Compiling bitfield v0.13.2
   Compiling void v1.0.2
   Compiling panic-halt v0.2.0
   Compiling embedded-dma v0.1.2
   Compiling volatile-register v0.2.0
   Compiling semver v0.9.0
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling generic-array v0.14.4
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling cast v0.2.3
   Compiling quote v1.0.7
   Compiling generic-array v0.12.3
   Compiling generic-array v0.13.2
   Compiling as-slice v0.1.4
   Compiling aligned v0.3.4
   Compiling cortex-m-rt-macros v0.1.8
   Compiling stm32f1xx-hal v0.7.0
   Compiling drone v0.1.0 ($PROJECT_PATH/tracking_drone)
    Finished dev [unoptimized + debuginfo] target(s) in 31.08s
# Verify that the produced executable is actually an ARM binary
tracking_drone $ cargo readobj --target thumbv7m-none-eabi --bin drone -- -file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x8000131
  Start of program headers:          52 (bytes into file)
  Start of section headers:          2742324 (bytes into file)
  Flags:                             0x5000200
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         5
  Size of section headers:           40 (bytes)
  Number of section headers:         22
  Section header string table index: 20

```



## Flash the Program into microcontroller

First connect the board:

```bash
$ openocd -f interface/jlink.cfg -f target/stm32f1x.cfg
# should get output below
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "jtag". To override use 'transport select <transport>'.
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
jtag_ntrst_delay: 100
none separate
cortex_m reset_config sysresetreq
Info : No device selected, using first device.
Info : J-Link V9 compiled Dec 13 2019 11:14:50
Info : Hardware version: 9.60
Info : VTarget = 3.193 V
Info : clock speed 1000 kHz
Info : JTAG tap: stm32f1x.cpu tap/device found: 0x3ba00477 (mfg: 0x23b (ARM Ltd.), part: 0xba00, ver: 0x3)
Info : JTAG tap: stm32f1x.bs tap/device found: 0x16410041 (mfg: 0x020 (STMicroelectronics), part: 0x6410, ver: 0x1)
Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

The **"6 breakpoints, 4 watchpoints"** part indicates the debugging features the processor has available.

Leave `openocd` process running, and open a new terminal.

Make sure that you are inside the project directory.

the OpenOCD provides a GDB server. Connect to that:

```bash
tracking_drone $ arm-none-eabi-gdb -q target/thumbv7m-none-eabi/debug/drone
```

This only opens a GDB shell. To actually connect to the OpenOCD GDB server, use the following command within the GDB shell:

```shell
(gdb) target remote :3333
Remote debugging using :3333
0x00000000 in ?? ()
```

if you are getting errors like undefined debug reason 7 - target needs reset in `openocd`  you can try running `monitor reset halt` in GDB shell

After entering this command, you'll see new output in the OpenOCD terminal:

```bash
Info : accepting 'gdb' connection on tcp/3333
Info : device id = 0x20036410
Info : flash size = 64kbytes
```

To flash the device, we'll use the `load` command inside the GDB shell:

```
(gdb) load

```

You'll see new output in the OpenOCD terminal:

```bash
Info : JTAG tap: stm32f1x.cpu tap/device found: 0x3ba00477 (mfg: 0x23b (ARM Ltd.), part: 0xba00, ver: 0x3)
Info : JTAG tap: stm32f1x.bs tap/device found: 0x16410041 (mfg: 0x020 (STMicroelectronics), part: 0x6410, ver: 0x1)
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08000130 msp: 0x20005000
```



now the program is flash to the chip