import numpy as np
import imageio.v3 as iio
from bitarray import bitarray
import random


def eject(image: np.ndarray, pattern: callable, count: int = None) -> bitarray:
    message = bitarray()
    it = np.nditer(image, flags=['multi_index'])

    for value in it:
        bits = bitarray(endian='little')
        bits.frombytes(int(value).to_bytes())

        for index, bit in enumerate(bits):
            if count is not None and len(message) >= count:
                return message
            if pattern(*it.multi_index, index):
                print(*it.multi_index, index, ' -> ', bit)
                message.append(bit)
        print(bits, value)

    #error if requested count too long

    return message


def inject(image: np.ndarray, pattern: callable, message: bitarray):
    it = np.nditer(image, flags=['multi_index'])

    new_image = np.copy(image)
    message_index = 0

    for value in it:
        bits = bitarray(endian='little')
        bits.frombytes(int(value).to_bytes())

        new_bits = bitarray(8, endian='little')
        for i, bit in enumerate(bits):
            new_bits[i] = bits[i]

        for index, bit in enumerate(bits):
            if message_index >= len(message):
                break

            if pattern(*it.multi_index, index):
                print(*it.multi_index, index, ' <- ', message[message_index])
                new_bits[index] = message[message_index]
                message_index += 1

        new_value = int.from_bytes(new_bits.tobytes())
        print(new_bits, new_value)
        new_image[*it.multi_index] = new_value

        if message_index >= len(message):
            return new_image

    if message_index < len(message) - 1:
        raise Exception(f'Message does not fit! Remaining bits: {(len(message) - 1) - message_index}')

    return new_image

def f(row, col, channel, index):
    return row % 5 == 0 and col <= 10

def transject(image: np.ndarray, message: bitarray, pattern: callable):
    it = np.nditer(image, order='C', flags=['multi_index'])

    message_index = 0

    for subpixel in it:
        if message_index >= len(message):
            break

        bits = bitarray(endian='little')
        bits.frombytes(int(subpixel).to_bytes())

        #new_bits = bitarray(8, endian='little')
        #for i, bit in enumerate(bits):
        #    new_bits[i] = bits[i]

        for i, bit in enumerate(bits):
            if message_index >= len(message):
                break

            if pattern(*it.multi_index, i):
                bits[i], message[message_index] = message[message_index], bits[i]
                message_index += 1
            if message_index <= 100:
                print(*it.multi_index, i)
        new_subpixel = int.from_bytes(bits.tobytes())
        image[*it.multi_index] = new_subpixel

    if message_index < len(message):
        raise Exception(f'Length mismatch: {len(message)} message bits but only {message_index} suitable bits were found in cover image.')

random.seed(0)

im = iio.imread('imageio:chelsea.png')
msg = bitarray([True for i in range(50 * 10 * 24)])
im2 = im.copy()
msg2 = msg.copy()

random.seed(0)
transject(im2, msg2, f)
print(np.array_equal(im2, im), msg2 == msg)

iio.imwrite('test-out.png', im2)

random.seed(0)
transject(im2, msg2, f)
print(np.array_equal(im2, im), msg2 == msg)

#message = bitarray()
#message.frombytes(bytes(random.randrange(256) for _ in range(2**10)))
#random.seed(1)
#im2 = inject(im, lsb, message)
#iio.imwrite("test-out.png", im2)
#print('-------')
#random.seed(1)
#out = eject(im2, lsb, len(message))
#print(message == out)
