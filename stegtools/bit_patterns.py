import numpy as np
import imageio.v3 as iio
from bitarray import bitarray


def transject(image: np.ndarray, pattern: callable, message: bitarray) -> int:
    it = np.nditer(image, order='C', flags=['multi_index'])

    message_index = 0

    for subpixel in it:
        if message_index >= len(message):
            break

        bits = bitarray(endian='little')
        bits.frombytes(int(subpixel).to_bytes())

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
        print(f'WARNING: Length mismatch: {len(message)} message bits but only {message_index} suitable bits were found in cover image.')

    return message_index


def inject(image: np.ndarray, pattern: callable, message: bitarray) -> np.ndarray:
    new_image = image.copy()
    transject(new_image, pattern, message.copy())
    return new_image


def eject(image: np.ndarray, pattern: callable, length: int = -1) -> bitarray:
    if length == -1:
        length = image.size * 8

    message = bitarray(length)
    true_length = transject(image.copy(), pattern, message)
    return message[:true_length]


