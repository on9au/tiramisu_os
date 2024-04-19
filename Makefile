# Default target architecture and boot method.
# ARCH: Can be "x86_64", or "aarch64"
ARCH ?= x86_64
# BOOT_METHOD: Can be "bios", "uefi", or "arm".
BOOT_METHOD ?= bios

# Directories
BUILD_DIR := build
SRC_DIR := src
BOOT_DIR := $(SRC_DIR)/boot/$(BOOT_METHOD)
KERNEL_DIR := $(SRC_DIR)/kernel
ARCH_DIR := $(SRC_DIR)/arch/$(ARCH)

# Filenames
KERNEL_BIN := $(BUILD_DIR)/kernel-$(ARCH).bin
ISO := $(BUILD_DIR)/os-$(ARCH).iso
TARGET := $(ARCH)-tiramisu_os
RUST_OS := target/$(TARGET)/debug/libtiramisu_os.a

# Tools
LD := ld
NASM := nasm
MKDIR_P := mkdir -p

# Source files
LINKER_SCRIPT := $(BOOT_DIR)/linker.ld
GRUB_CFG := $(BOOT_DIR)/grub.cfg
ASSEMBLY_SOURCE_FILES := $(wildcard $(BOOT_DIR)/*.asm)
ASSEMBLY_OBJECT_FILES := $(patsubst $(BOOT_DIR)/%.asm, \
    $(BUILD_DIR)/arch/$(ARCH)/%.o, $(ASSEMBLY_SOURCE_FILES))

.PHONY: all clean run iso kernel

all: $(KERNEL_BIN)

kernel:
	@RUST_TARGET_PATH=$(shell pwd) xargo build --target $(TARGET)

clean:
	@rm -r $(BUILD_DIR)

run: $(ISO)
	@qemu-system-x86_64 -cdrom $(ISO)

iso: $(ISO)

$(ISO): $(KERNEL_BIN) $(GRUB_CFG)
	@$(MKDIR_P) $(BUILD_DIR)/isofiles/boot/grub
	@cp $(KERNEL_BIN) $(BUILD_DIR)/isofiles/boot/kernel.bin
	@cp $(GRUB_CFG) $(BUILD_DIR)/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) -d /usr/lib/grub/i386-pc $(BUILD_DIR)/isofiles
	@rm -r $(BUILD_DIR)/isofiles

$(KERNEL_BIN): kernel $(RUST_OS) $(ASSEMBLY_OBJECT_FILES) $(LINKER_SCRIPT)
	@$(LD) -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL_BIN) \
		$(ASSEMBLY_OBJECT_FILES) $(RUST_OS)

# compile assembly files
$(BUILD_DIR)/arch/$(ARCH)/%.o: $(BOOT_DIR)/%.asm
	@$(MKDIR_P) $(shell dirname $@)
	@$(NASM) -felf64 $< -o $@