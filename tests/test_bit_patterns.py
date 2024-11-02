import random
from stegtools.bit_patterns import transject, inject, eject
from bitarray import bitarray
import imageio.v3 as iio
import numpy as np


def test_transject_invertible():
    img = iio.imread("imageio:chelsea.png")
    
    random.seed(0)
    msg = bitarray([random.choice([True, False]) for _ in range(100_000)])

    def pattern(row, col, chn, idx):
        return True
    
    new_img = img.copy()
    new_msg = msg.copy()
    transject(new_img, pattern, new_msg)

    assert not np.array_equal(new_img, img), 'transjection should modify the image'
    assert new_msg != msg, 'transjection should modify the message'

    transject(new_img, pattern, new_msg)

    assert np.array_equal(new_img, img), 'double transjection should be an identity'
    assert new_msg == msg, 'double transjection should be an identity'

def test_inject_eject_invertible():
    img = iio.imread("imageio:chelsea.png")
    
    random.seed(0)
    msg = bitarray([random.choice([True, False]) for _ in range(100_000)])

    def pattern(row, col, chn, idx):
        return True
    
    modified_img = inject(img, pattern, msg)
    assert not np.array_equal(modified_img, img), 'injection should modify the image'

    parsed_msg = eject(modified_img, pattern, 100_000)
    assert parsed_msg == msg, 'ejecting the injected image should retrieve the original message'
