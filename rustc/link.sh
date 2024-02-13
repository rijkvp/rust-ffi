#!/bin/sh
gcc -c -o sum.o sum.c
ar rcs libsum.a sum.o
rustc main.rs -L . -l sum
