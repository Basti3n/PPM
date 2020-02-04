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

#print("data" +str(lib.revertColor("/mnt/d/4_eme_annee/Rust/Rust/ppm/test.ppm", xsize, ysize, rgbmax, r, g, b)))
# print("data" +str(lib.grayColor('/mnt/d/ESGI/Rust/PPM/ppm/test', xsize, ysize, rgbmax, r, g, b)))
print("data" +str(lib.grayColor('test', xsize, ysize, rgbmax, r, g, b)))
xsize = -1
ysize = -1
rgbmax = -1
r = -1
g = -1
b = -1
print("data" +str(lib.revertColor('test', xsize, ysize, rgbmax, r, g, b)))



# print("xsize : " + str(xsize))
# print("ysize : " + str(ysize))
# print("rgbmax : " + str(rgbmax))
# print("r : " + str(r))
# print("g : " + str(g))
# print("b : " + str(b))

#lib.setFileName('/mnt/d/4_eme_annee/Rust/Rust/ppm/test.ppm')

print("done!")