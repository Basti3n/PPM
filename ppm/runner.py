import sys
from ctypes import cdll
import time
lib = cdll.LoadLibrary("target/debug/libppm.so")

print(lib.dummy())

print(lib.max(5,6))

xsize = -1
ysize = -1
rgbmax = -1
r = -1
g = -1
b = -1

arg = sys.argv
if len(arg)> 1 :
    if arg[1] == "gray":
        lib.gray_color('test', xsize, ysize, rgbmax, r, g, b)
    elif arg[1] == "revert":
        lib.revert_color('test', xsize, ysize, rgbmax, r, g, b)
else :
    lib.gray_color('test', xsize, ysize, rgbmax, r, g, b)

print("Your file has been edited !")
