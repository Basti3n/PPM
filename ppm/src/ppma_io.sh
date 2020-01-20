#! /bin/bash

gcc -c -Wall -I  ppma_io.c ppma_io.h -o output.o
if [ $? -ne 0 ]; then
  echo "Compile error."
  exit
fi

echo "Normal end of execution."
