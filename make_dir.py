#!/usr/bin/env python3

import os
import sys

print(f"number: {sys.argv[1]}")
print(f"difficulty: {sys.argv[2]}") if len(sys.argv) == 3 else print("difficulty: all")

difficulty = ["a", "b", "c", "d"]


def create_file(path):
    if not os.path.exists(path):
        os.mkdir(path)

    file_name = path + "/main.py"
    if not os.path.exists(file_name):
        with open(file_name, "w"):
            # empty main.py
            pass


if 1 < len(sys.argv) and sys.argv[1].isdigit():
    if len(sys.argv) == 2:
        for p in difficulty:
            target = "abc" + sys.argv[1] + "_" + p
            create_file(target)
    elif len(sys.argv) == 3 and sys.argv[2] in difficulty:
        target = "abc" + sys.argv[1] + "_" + sys.argv[2]
        create_file(target)
else:
    print("Usage: python make_dir.py [number]")
    print("or:    python make_dir.py [number] [a|b|c|d]")
    print("e.g.   python make_dir.py 416")
    print("e.g.   python make_dir.py 416 a")
