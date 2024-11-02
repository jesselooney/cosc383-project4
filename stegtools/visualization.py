from bit_patterns import inject, eject
import patterns
import numpy as np
from bitarray import bitarray

def amplify_least_signigicant_bits(image: np.ndarray) -> np.ndarray:
    least_significant_bits = eject(
        image, patterns.access_least_significant_bits, image.size
    )

    amplified_bits = bitarray()
    for i in least_significant_bits:
        amplified_bits.extend([i] * 8)

    amplified_image = inject(image, patterns.access_all, amplified_bits)
    return amplified_image

