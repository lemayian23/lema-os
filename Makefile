KERNEL := target/x86_64-unknown-none/debug/lema-kernel
ISO := lema-os.iso
LIMINE_DIR := limine-binary/limine-binary
LIMINE_TOOL := $(LIMINE_DIR)/limine-tool-windows-x86/limine.exe

.PHONY: all kernel iso_root clean run

all: $(ISO)

kernel:
	cargo build --target x86_64-unknown-none

iso_root: kernel
	rm -rf iso_root
	mkdir -p iso_root/boot/limine
	mkdir -p iso_root/EFI/BOOT
	cp $(KERNEL) iso_root/boot/lema-kernel
	cp boot/limine.conf iso_root/boot/limine/limine.conf
	cp $(LIMINE_DIR)/limine-bios.sys iso_root/boot/limine/
	cp $(LIMINE_DIR)/limine-bios-cd.bin iso_root/boot/limine/
	cp $(LIMINE_DIR)/limine-uefi-cd.bin iso_root/boot/limine/
	cp $(LIMINE_DIR)/BOOTX64.EFI iso_root/EFI/BOOT/
	cp $(LIMINE_DIR)/BOOTIA32.EFI iso_root/EFI/BOOT/

$(ISO): iso_root
	xorriso -as mkisofs -R -r -J \
		-b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		-hfsplus -apm-block-size 2048 \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image \
		--protective-msdos-label \
		iso_root -o $(ISO)
	$(LIMINE_TOOL) bios-install $(ISO)

run: $(ISO)
	qemu-system-x86_64 -cdrom $(ISO) -m 512M -serial stdio

clean:
	rm -rf iso_root $(ISO) target