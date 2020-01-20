from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libppm.so")

print(lib.dummy())

print(lib.max(5,6))

print(lib.readPPM("/mnt/d/4_eme_annee/Rust/ppm/TeaPot.ppm", 512, 512, 255,255, 255, 255));

print("done!")