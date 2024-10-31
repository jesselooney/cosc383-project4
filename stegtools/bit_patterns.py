import numpy as np
import imageio.v3 as iio
from bitarray import bitarray
import random

im = iio.imread('imageio:chelsea.png')

def eject(image: np.ndarray, pattern: callable, count: int = None) -> bitarray:
    message = bitarray()
    it = np.nditer(image, flags=['multi_index'])

    for value in it:
        bits = bitarray(endian='big')
        bits.frombytes(bytes([value]))

        for index, bit in enumerate(bits):
            if count is not None and len(message) >= count:
                return message
            if pattern(*it.multi_index, index):
                print(*it.multi_index, bit)
                message.append(bit)

        print(value)
                    
    return message


def inject(image: np.ndarray, pattern: callable, message: bitarray):
    it = np.nditer(image, flags=['multi_index'])

    new_image = np.copy(image)
    message_index = 0

    for value in it:
        bits = bitarray(endian='big')
        bits.frombytes(bytes([value]))

        new_bits = bitarray(8, endian='big')

        for index, bit in enumerate(bits):
            if message_index >= len(message):
                return new_image

            if pattern(*it.multi_index, index):
                print(*it.multi_index, message[message_index])
                new_bits[index] = message[message_index]
                message_index += 1
            else:
                new_bits[index] = bits[index]

        new_value = int.from_bytes(new_bits.tobytes(), 'big')
        print(bits, new_bits)
        print(value, new_value)
        new_image[*it.multi_index] = new_value

    if message_index < len(message) - 1:
        raise Exception(f'Message does not fit! Remaining bits: {(len(message) - 1) - message_index}')

    return new_image

def lsb(row, col, channel, index):
    return random.choice([True, False])

message = bitarray()
message.frombytes(bytes(b'Hello'))
random.seed(0)
im2 = inject(im, lsb, message)
print('-------')
random.seed(0)
out = eject(im2, lsb, len(message))
print(message)
print(out)
print(message == out)
