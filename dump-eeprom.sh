#!/bin/sh -e

st-flash --format ihex read eeprom.txt 0x800f800 2048
