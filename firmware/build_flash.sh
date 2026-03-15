#!/usr/bin/env bash
#empty file allocate space based on bytes times 512
cd "$(dirname $0)"
dd if=/dev/zero of=luckfox_sd.img bs=1M count=2048
echo "Allocated 2gb void to image"
dd if=env.img of=luckfox_sd.img bs=512 seek=$((0x0)) conv=notrunc
dd if=idblock.img of=luckfox_sd.img bs=512 seek=$((0x40)) conv=notrunc
dd if=uboot.img of=luckfox_sd.img bs=512 seek=$((0x440)) conv=notrunc
dd if=boot.img of=luckfox_sd.img bs=512 seek=$((0x840)) conv=notrunc
dd if=oem.img of=luckfox_sd.img bs=512 seek=$((0x10840)) conv=notrunc
dd if=userdata.img of=luckfox_sd.img bs=512 seek=$((0x110840)) conv=notrunc
dd if=rootfs.img of=luckfox_sd.img bs=512 seek=$((0x190840)) conv=notrunc

### truncate image to generate sparse layout better performance
#truncate -s 2G luckfox_sd.img
