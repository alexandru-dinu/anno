#!/usr/bin/env python3

import hashlib
import os
from argparse import ArgumentParser
from pathlib import Path

import numpy as np

HEADER_SIZE = 4
SAMPLE_SIZE = 1
NUM_SAMPLES = 100


def get_file_size(fname: str) -> int:
    return os.stat(fname).st_size


def main(args):
    samples = []

    with open(args.file_path, "rb") as fp:
        header = fp.read(HEADER_SIZE)
        seed = int.from_bytes(header, "big")
        rng = np.random.default_rng(seed)

        seeks = rng.integers(low=1, high=get_file_size(fp.name), size=NUM_SAMPLES, dtype=int)

        for s in seeks:
            fp.seek(s)
            samples.append(fp.read(SAMPLE_SIZE))

    checksum = hashlib.sha256(b"".join(samples)).hexdigest()

    print(checksum)


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("-f", "--file-path", type=Path, required=True)
    args = parser.parse_args()

    print(args.__dict__)

    main(args)
