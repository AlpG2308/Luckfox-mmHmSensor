import serial
import time

DEVICE = "/dev/ttyS3"
BAUDRATE = 115200
COMMAND = "FDFCFBFA0800120000000000000004030201"
SECONDS = 60
start = time.time()
with serial.Serial(
    DEVICE,
    baudrate=BAUDRATE,
    timeout=1,
) as target:
    # target.write()
    target.reset_input_buffer()
    target.write(bytes.fromhex(COMMAND))
    target.flush()
    time.sleep(0.5)
    while time.time() - start < SECONDS:
        buf = target.read(2048)
        # print(hex(int.from_bytes(buf,byteorder='little')))
        print(buf.hex())
