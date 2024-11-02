import numpy as np
import imageio.v3 as iio
from bitarray import bitarray


def transject(image: np.ndarray, pattern: callable, message: bitarray) -> int:
    """Contains the log wrapped by the inject and eject functions

    Args:
        image: The image to be manipulated
        pattern:
            A function that determines which bits in the image should be overwritten
            or returned.

            The function should take 4 arguments as input:
                - the row the pixel
                - the column of the pixel
                - the channel (either r,g, or b, corresponding to 0, 1, 2)
                - the index of the bit inside of the channel (value from 0 to 7)

                The bits of the channel are in little endian ordering, meaning that
                an index of 0 corresponds to the list significant bit of the channel

            If the function returns `True` given the coordinates of the given bit, then that
            can be overwritten with a bit from `message`, if `False`, then not.

            Here's an example:

            ```py
            # Would target every single bit in each channel 
            def pattern(row: int, col: int, chn: int, idx: int):
                return True

            # Would only target the least significant bit of each channel 
            def pattern(row: int, col: int, chn: int, idx: int):
                return idx == 0
            ```
        message: The data to be encoded into the image, in binary form

    Returns: The number of bits of the message successfully encoded into the image

    """

    # multi_index flattens the array so we can iterate over it with a single loop,
    # but still keep track of where we are 3 dimensionally
    it = np.nditer(image, order="C", flags=["multi_index"])
    # unsure if the C order is relevant, but we need the ordering of the array to be deterministic
    # so we get the pixels in the right order (fairly certain the default behavior doesn't work this way,
    # prioritizes memory efficiency instead)

    message_index = 0

    # iterate over the flattened array
    for channel in it:
        if message_index >= len(message):
            break

        bits = bitarray(endian="little")
        bits.frombytes(int(channel).to_bytes())

        for i, bit in enumerate(bits):
            if message_index >= len(message):
                break

            # i here is the location in the binary representation of the channel
            if pattern(*it.multi_index, i):
                bits[i], message[message_index] = message[message_index], bits[i]
                message_index += 1

        new_channel = int.from_bytes(bits.tobytes())
        image[*it.multi_index] = new_channel

    if message_index < len(message):
        print(
            f"WARNING: Length mismatch: {len(message)} message bits but only {message_index} suitable bits were found in cover image."
        )

    return message_index


def inject(image: np.ndarray, pattern: callable, message: bitarray) -> np.ndarray:
    """Inject data into an image

    Args:
        image: The image to be modified
        pattern:
            A function that dictates which bits to write the message into
            See the documentation for `transject` for valid patterns
        message: The message being encoded

    Returns: The modified image
    """

    new_image = image.copy()
    transject(new_image, pattern, message.copy())
    return new_image


def eject(image: np.ndarray, pattern: callable, length: int = -1) -> bitarray:
    """Parses a message out of an image

    Args:
        image: The image to parse
        pattern:
            A function that dictates which bits to look for the message in
            See the documentation for `transject` for valid patterns
        length:

    Returns: the requested data
    """

    # I hate sentinel values, but the lack of well designed Enums like a *real language* makes this kind of necessary
    # (will still be judging you)
    if length == -1:
        length = image.size * 8

    message = bitarray(length)
    true_length = transject(image.copy(), pattern, message)
    return message[:true_length]
