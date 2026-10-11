#!/bin/bash
set -e
cargo build

rm -rf iso_root
mkdir -p iso_root/boot/limine iso_root/EFI/BOOT
cp target/x86_64-unknown-none/debug/ferox iso_root/boot/ferox
cp limine.conf iso_root/boot/limine/
cp /usr/share/limine/limine-bios.sys \
   /usr/share/limine/limine-bios-cd.bin \
   /usr/share/limine/limine-uefi-cd.bin iso_root/boot/limine/
cp /usr/share/limine/BOOTX64.EFI iso_root/EFI/BOOT/

xorriso -as mkisofs -R -r -J -b boot/limine/limine-bios-cd.bin \
  -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus \
  -apm-block-size 2048 --efi-boot boot/limine/limine-uefi-cd.bin \
  -efi-boot-part --efi-boot-image --protective-msdos-label \
  iso_root -o ferox.iso
limine bios-install ferox.iso

qemu-system-x86_64 -cdrom ferox.iso -serial stdio
