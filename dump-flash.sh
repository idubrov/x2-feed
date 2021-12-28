#!/bin/sh -e

st-flash --format ihex read program.txt 0x8000000 65536
