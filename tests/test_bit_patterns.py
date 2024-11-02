from stegtools.bit_patterns import transject


def test_transject_invertible():
    image = iio.imread("imageio:chelsea.png")
