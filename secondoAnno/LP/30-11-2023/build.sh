#!/bin/bash

gcc -c dict/*.c
gcc main.c treedict.o node.o -o main

rm *.o
