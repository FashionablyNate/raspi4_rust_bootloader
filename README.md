
# 🧵 raspi4_rust_bootloader

A bare-metal operating system for the Raspberry Pi 4 written in [Rust](https://www.rust-lang.org/), with framebuffer graphics, double buffering, and no `std` dependency. This project includes unit-tested low-level graphics routines using a mocked framebuffer implementation.

---

## 📁 Project Structure

```
src/
├── boot.s # Assembly startup code (entry point before Rust)
├── font8x8_basic.rs # 8x8 bitmap font used for text rendering
├── frame_buffer.rs # Framebuffer mailbox init + pixel/drawing logic
├── lib.rs # #![no_std] and common declarations
├── mailbox.rs # Mailbox interface with VC property tags
├── main.rs # Kernel main() logic
├── text_buffer.rs # Line-wrapped text rendering buffer using framebuffer
└── timer.rs # Access to the ARM generic timer
```

---

## 🚀 Building the Kernel

You'll need a cross-compiler for `aarch64-unknown-none`:

```bash
rustup target add aarch64-unknown-none
```

Then build with:

```bash
make
```

This produces a raw binary kernel in:

```bash
target/aarch64-unknown-none/release/raspi4_rust_bootloader
```

Then uses `aarch64-unknown-linux-gnu-objcopy` to create an elf image in:

```bash
target/kernel.img
```

Then attempts to copy the image to:

```bash
/Volumes/bootfs
```

Where it expects to find a bootable raspberry pi micro SD card

## 🧪 Running Unit Tests

Unit tests can run on x86_64 using mocks, just don't specify a target. Example:

```bash
cargo test
```

## 💡 Why This Exists

It's a fun project to improve my embedded programming skills and learn about low level protocols.

## 🛠 Requirements

Rust nightly (for inline asm!)
Cross-compiler: aarch64-none-elf-gcc or similar
Raspberry Pi 4 Model B with SD card
