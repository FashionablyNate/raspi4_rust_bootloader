# Target and build info
TARGET = aarch64-unknown-none
BINARY_NAME = raspi4_rust_bootloader
BUILD_DIR = target/$(TARGET)/release
OUTPUT = target/kernel.img

# Toolchain
CARGO = cargo
OBJCOPY = aarch64-unknown-linux-gnu-objcopy

# Build flags
BUILD_ARGS = --release -Z build-std=core,compiler_builtins --target $(TARGET)

# SD card mount path
SDCARD_DIR = /Volumes/bootfs
SDCARD_KERNEL = $(SDCARD_DIR)/kernel8.img

# Rust source files
RUST_SRC := $(shell find src -type f -name '*.rs')

.PHONY: all clean run copy

all: $(OUTPUT) copy

$(OUTPUT): $(BUILD_DIR)/$(BINARY_NAME)
	$(OBJCOPY) -O binary $< $@

$(BUILD_DIR)/$(BINARY_NAME): $(RUST_SRC)
	$(CARGO) build $(BUILD_ARGS)

copy: $(OUTPUT)
	@if [ ! -d "$(SDCARD_DIR)" ]; then \
		echo "Error: SD card directory '$(SDCARD_DIR)' not found."; \
		exit 1; \
	fi
	@if [ -f "$(SDCARD_KERNEL)" ]; then \
		read -p "Warning: $(SDCARD_KERNEL) already exists. Overwrite? [y/N] " ans; \
		case $$ans in \
			[yY]*) cp $(OUTPUT) $(SDCARD_KERNEL); echo "Overwritten $(SDCARD_KERNEL)";; \
			*) echo "Aborted."; exit 1;; \
		esac \
	else \
		cp $(OUTPUT) $(SDCARD_KERNEL); \
		echo "Copied $(OUTPUT) to $(SDCARD_KERNEL)"; \
	fi
	diskutil eject $(SDCARD_DIR)

run: all
	@echo "Bootloader built at $(OUTPUT)"

clean:
	$(CARGO) clean
	rm -f $(OUTPUT)
