
# 🧵 raspi4_rust_bootloader

A bare-metal operating system for the Raspberry Pi 4 written in [Rust](https://www.rust-lang.org/), with framebuffer graphics, double buffering, and no `std` dependency. This project includes unit-tested low-level graphics routines using a mocked framebuffer implementation.

---

## 📁 Project Structure

src/
├── frame_buffer.rs # Framebuffer init, drawing, glyph rendering
├── mailbox.rs # Mailbox interface (mocked for tests)
├── timer.rs # ARM timer access (uses inline asm!)
├── main.rs # Entry point (or your boot/startup logic)

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
