#!/bin/sh

echo Creating ISO file

if [ ! -d "limine" ]; then
	git submodule update --init --recursive
fi

rm -rf iso_root
mkdir iso_root

(
  cd iso_root || exit 255
  mkdir -p EFI/BOOT

if [[ $1 == "aarch64" ]]; then
	cp ../limine/bin/BOOTAA64.EFI EFI/BOOT
elif [[ $1 == "x64" ]]; then
	cp ../limine/bin/BOOTX64.EFI EFI/BOOT
else
	echo "Wrong '$1' arch provided"
	exit 255
fi

 cp ../limine/limine-uefi-cd.bin .
 cp -v ../target/k64.sys .
 cp ../limine.conf .

  cd ..
  xorriso -as mkisofs \
  		-no-emul-boot -boot-load-size 4 -boot-info-table \
  		--efi-boot limine-uefi-cd.bin \
  		-efi-boot-part --efi-boot-image --protective-msdos-label \
  		iso_root -o ferrux.iso
)
