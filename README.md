# TiOS: A toy operating system written in Rust

TiOS is currently under work.

## Introduction

TiOS is a educational basic operating system written mainly in Rust. It aims to be lightweight, robust and extensible.

## Build and Run

### Prerequisites
- ```qemu``` with x86-64 system support
- ```nasm```
- Rust toolchain, including ```rustc```, ```cargo``` and ```xargo```.
- ```gdb``` for debugging purposes.

### Compilation

Clone and enter the repository.

Run ```$ make release``` in a shell.

### Running the kernel

Run ```$ make run-release``` in a shell.

### Debugging

Open two terminals and change directory to the TiOS repository.

One one terminal, run ```$ make run``` and wait for the GRUB boot menu.

After the boot menu appears in ```qemu```, run ```$ make gdb``` on the other terminal.

The second terminal will start ```gdb``` and open a connection to ```qemu```.

## Current Status

TiOS is currently under work and will not be usable for some time. Currently the status on the kernel is listed below.

### System Boot

Achieved. Can boot into protected mode and after setting up all required tables and flags can boot into Rust code in 64-bit mode.

### System Initialization

Under work.

#### Memory Management Initialization

Completed.

#### I/O Systems Initialization

Completed.

#### Interrupts

Completed.

#### File System

Not yet started.

### System Modules

#### Device I/O

Console and CMOS I/O finished. Keyboard currently under work.

#### Interrupt Service Routines

Currently under work.

#### PIC Remapping

Not yet started

#### File System

Not yet started

#### Process Management and Scheduler

Not yet started.

#### Memory Management

Currently under work to switch use kernel heap.


## Acknowledgements

Philipp Oppermann's ```blog-os``` on system bootup, memory management initialization and exception frame handling. Link: https://os.phil-opp.com/

OSDev.org for various articles on OS development. Link: http://wiki.osdev.org


-----------------------------------------------------------------------
Comments and suggestions are welcome.