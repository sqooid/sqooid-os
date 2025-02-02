if [ ! -d build ]; then
  mkdir -p build/esp/efi/boot
fi

cp target/x86_64-unknown-uefi/debug/uefi.efi build/esp/efi/boot/bootx64.efi
ovmf_dir="/usr/share/OVMF"

(
cd build
qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=$ovmf_dir/OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=$ovmf_dir/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
)