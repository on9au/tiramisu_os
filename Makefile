# Default target architecture and boot method.
# ARCH: Can be "x86_64", or "aarch64"
ARCH ?= x86_64
# BOOT_METHOD: Can be "bios", "uefi", or "arm".
BOOT_METHOD ?= bios

# Directories
BUILD_DIR := build
KERNEL_DIR := kernel
BOOT_MODULE:= tiramisu_bootloader
BOOT_DIR := $(KERNEL_DIR)/$(BOOT_MODULE)/src/$(BOOT_METHOD)
ARCH_DIR := arch/$(ARCH)

# Filenames
KERNEL_BIN := $(BUILD_DIR)/kernel-$(ARCH).bin
TEST_KERNEL_BIN := $(BUILD_DIR)/test-kernel-$(ARCH).bin
ISO := $(BUILD_DIR)/os-$(ARCH).iso
TEST_ISO := $(BUILD_DIR)/test-os-$(ARCH).iso
TARGET := $(ARCH)-tiramisu_os
BOOT_RUST_ENTRY_POINT := target/$(TARGET)/debug/libtiramisu_bootloader.a

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
	@RUST_TARGET_PATH=$(shell pwd) cargo build --target $(TARGET).json

test_kernel:
	@RUST_TARGET_PATH=$(shell pwd) cargo build -F test --target $(TARGET).json 

clean:
	@rm -r $(BUILD_DIR)

run: $(ISO)
	@qemu-system-x86_64 -cdrom $(ISO) -serial stdio

test: $(TEST_ISO)
	@qemu-system-x86_64 -cdrom $(TEST_ISO) -serial stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 || [ $$? -eq 33 ]

iso: $(ISO)

$(ISO): $(KERNEL_BIN) $(GRUB_CFG)
	@$(MKDIR_P) $(BUILD_DIR)/isofiles/boot/grub
	@cp $(KERNEL_BIN) $(BUILD_DIR)/isofiles/boot/kernel.bin
	@cp $(GRUB_CFG) $(BUILD_DIR)/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) -d /usr/lib/grub/i386-pc $(BUILD_DIR)/isofiles
	@rm -r $(BUILD_DIR)/isofiles

$(TEST_ISO): $(TEST_KERNEL_BIN) $(GRUB_CFG)
	@$(MKDIR_P) $(BUILD_DIR)/isofiles/boot/grub
	@cp $(TEST_KERNEL_BIN) $(BUILD_DIR)/isofiles/boot/kernel.bin
	@cp $(GRUB_CFG) $(BUILD_DIR)/isofiles/boot/grub
	@grub-mkrescue -o $(TEST_ISO) -d /usr/lib/grub/i386-pc $(BUILD_DIR)/isofiles
	@rm -r $(BUILD_DIR)/isofiles

$(KERNEL_BIN): kernel $(BOOT_RUST_ENTRY_POINT) $(ASSEMBLY_OBJECT_FILES) $(LINKER_SCRIPT)
	@$(LD) -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL_BIN) \
		$(ASSEMBLY_OBJECT_FILES) $(BOOT_RUST_ENTRY_POINT)

$(TEST_KERNEL_BIN): test_kernel $(BOOT_RUST_ENTRY_POINT) $(ASSEMBLY_OBJECT_FILES) $(LINKER_SCRIPT)
	@$(LD) -n --gc-sections -T $(LINKER_SCRIPT) -o $(TEST_KERNEL_BIN) \
		$(ASSEMBLY_OBJECT_FILES) $(BOOT_RUST_ENTRY_POINT)

# compile assembly files
$(BUILD_DIR)/arch/$(ARCH)/%.o: $(BOOT_DIR)/%.asm
	@$(MKDIR_P) $(shell dirname $@)
	@$(NASM) -felf64 $< -o $@