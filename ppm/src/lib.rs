extern crate libc;
extern crate c_string;
use libc::{c_char, c_int, size_t};
use c_string::CStrBuf;
use std::ffi::{CString, CStr};

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::mem;

// TEST function
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// TEST function
#[no_mangle]
pub extern fn dummy() ->u8{
    return 42;
}

// TEST function
#[no_mangle]
pub extern fn max(a:u8, b:u8)->u8{
    return if a > b {a} else {b};
}

/* STRUCTURES */
struct Pixel{
    r:u8,
    g:u8,
    b:u8
}

struct Image{
    image:[Pixel;8]
}
/*
UNCOMMENT
impl Image{
    fn save(filename: Path, img: Image){
        let mut f = File::open(filename)?;
        let mu writer = BufWriter::new(&f);
        write!(&mut writer, "P3\n{} {}\n{}\n{}", img.height, img.width, 255, img.pex);
        Ok(())
    }
    fn invert(img: &Image){
        for color in &img.pex{
            color.invert();
        }
    }
}
*/

/* **************************
 *      PPMA C LIBRARY        *
 ****************************/
#[link(name = "ppma_io")]
extern "C" {
    fn ch_cap    (  ch:c_char ) -> c_char;
    fn ppma_read (  input_name: *const u8, 
                    xsize:&* mut c_int, 
                    ysize:&* mut c_int, 
                    rgb_max:&* mut c_int,
                    r:&* mut c_int, 
                    g:&* mut c_int, 
                    b:&* mut c_int
                );
    fn ppma_write ( file_out_name: CStrBuf,
                    xsize: * mut c_int, 
                    ysize: * mut c_int, 
                    r: * mut c_int,
                    g: * mut c_int, 
                    b: * mut c_int
                ) -> c_int;
}

//Read the image file (C)
#[no_mangle]
pub extern fn readPPM(  input_name: *const c_char, 
                        xsize:* mut c_int, 
                        ysize:* mut c_int, 
                        rgb_max:* mut c_int ,
                        r:* mut c_int, 
                        g:* mut c_int, 
                        b:* mut c_int
                    ) {
    let filename = unsafe {
        CStr::from_ptr(input_name).to_string_lossy().into_owned().to_string()
    };
    println!("{}",filename);
    unsafe { ppma_read(filename.as_ptr(), &xsize, &ysize,&rgb_max, &r, &g, &b) };
}

//Print a image file (C)
#[no_mangle]
pub extern fn writePPM( file_out_name: CStrBuf,
                        xsize: * mut c_int, 
                        ysize: * mut c_int, 
                        r: * mut c_int,
                        g: * mut c_int, 
                        b: * mut c_int
                    ) -> c_int {
    return unsafe { ppma_write(file_out_name, xsize, ysize, r, g, b) };
}

//Read a file (RUST)
#[no_mangle]
pub extern fn read_file(filename : String)-> io::Result<()>{
    println!("filename: {}", filename);
    // work with `name`
    //let mut f = BufReader::new(File::open(filename).expect("open failed")); {
    let mut file=File::open(filename).unwrap();
    let mut buf=[0u8;32];
    file.read(&mut buf).unwrap();
    println!("{:?}",buf);
    Ok(())
}

//TEST (RUST)
#[no_mangle]
pub extern fn setFileName(filenam: *const c_char) {
    let filename = unsafe {
        CStr::from_ptr(filenam).to_string_lossy().into_owned()
    };
    let m = read_file(filename);
    match m {
        Ok(v) => println!("working with version: {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
    }
    println!("Cake");

}
