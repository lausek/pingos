# set if arch doesn't have a value
arch ?= x86_64
target_dir := build
target = $(arch)-target
rust_os := target/$(target)/debug/libpingos.a

# := set immediately
kernel := $(target_dir)/kernel-$(arch).bin
iso := $(target_dir)/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg

# get all asm source files in path
asm_source := $(wildcard src/arch/$(arch)/*.asm)
# translate asm source filenames to target filenames
asm_object := $(patsubst src/arch/$(arch)/%.asm, \
	$(target_dir)/arch/$(arch)/%.o, $(asm_source))

# commands, not files
.PHONY: all clean run iso 

all: $(kernel)

clean:
	@rm -r $(target_dir)

run: $(iso)
	# TODO: don't hardcode arch here
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p $(target_dir)/iso/boot/grub
	@cp $(kernel) $(target_dir)/iso/boot/kernel.bin
	@cp $(grub_cfg) $(target_dir)/iso/boot/grub
	@grub-mkrescue -o $(iso) $(target_dir)/iso 2> /dev/null
	@rm -r $(target_dir)/iso

$(kernel): kernel $(rust_os) $(asm_object) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) \
		$(asm_object) $(rust_os)

kernel:
	@RUST_TARGET_PATH=$(shell pwd) xargo build --target $(target)

$(target_dir)/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
