import random
from stegtools.bit_patterns import transject
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

