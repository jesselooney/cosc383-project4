"""
This file contains all of the commonly used patterns for use with the `transject`, `inject`, and `eject`
functions from `./bit_patterns.py`
"""


def access_all(row, col, chn, idx):
    return True


def access_least_significant_bits(row, col, chn, idx):
    return idx == 0
