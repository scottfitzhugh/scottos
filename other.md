
## Run qemu with a USB boot device
```
sudo qemu-system-x86_64 -drive format=raw,file=/dev/sdb
```

## Recompile the project as a bootable image
```
cargo bootimage
```

## Run the boot image in qemu
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-scottos/debug/bootimage-ScottOS.bin
```
or

```
cargo run
```