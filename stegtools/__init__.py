from bit_patterns import inject, eject
import patterns
import imageio as iio
import time

img = iio.imread("./blahaj-in-bando.png")

start = time.time()
test = eject(img, patterns.access_all)
end = time.time()
print(test)
print(end - start)
