#!/usr/bin/env bash

echo Creating ISO file

curl -SsLO https://ddn.stateprisma.com/limine.tar.gz || exit 1

if [ ! -d "limine" ]; then
  git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1;\
fi
make -C limine

mkdir limine
tar xvf limine.tar.gz -C limine 1>/dev/null
rm limine.tar.gz

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
	exit -1
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
