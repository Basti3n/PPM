extern crate libc;
extern crate c_string;
use libc::{c_char, c_int, size_t};
use c_string::CStrBuf;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern fn dummy() ->u8{
    return 42;
}

#[no_mangle]
pub extern fn max(a:u8, b:u8)->u8{
    return if a > b {a} else {b};
}

struct Pixel{
    r:u8,
    g:u8,
    b:u8
}

struct Image{
    image:[Pixel;24]
}


#[link(name = "ppma_io")]
extern "C" {
    fn ch_cap ( ch:c_char )->c_char;
    fn ppma_read (input_name: CStrBuf, xsize:* mut c_int, ysize:* mut c_int , rgb_max:* mut c_int ,
        r:* mut c_int, g:* mut c_int, b:* mut c_int);
    fn ppma_write (file_out_name: CStrBuf ,xsize: * mut c_int, ysize: * mut c_int, r: * mut c_int,
        g: * mut c_int, b: * mut c_int )->c_int;
}


#[no_mangle]
pub extern fn readPPM(input_name: CStrBuf, xsize:* mut c_int, ysize:* mut c_int , rgb_max:* mut c_int ,
    r:* mut c_int, g:* mut c_int, b:* mut c_int){
    unsafe { ppma_read(input_name, xsize, ysize,rgb_max, r, g, b) };
}

#[no_mangle]
pub extern fn writePPM(file_out_name: CStrBuf ,xsize: * mut c_int, ysize: * mut c_int, r: * mut c_int,
    g: * mut c_int, b: * mut c_int)->c_int{
    return unsafe { ppma_write(file_out_name, xsize, ysize, r, g, b) };
}