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
pub extern fn dummy() -> u8{
    return 42;
}

// TEST function
#[no_mangle]
pub extern fn max(a:u8, b:u8)->u8{
    return if a > b {a} else {b};
}

#[derive(Debug, Clone, Copy)]
/* STRUCTURES */
struct Pixel{
    r:*mut u8,
    g:*mut u8,
    b:*mut u8
}

//struct Pixel{
//    pixel:[Color;8]
//}

struct Image{
    height: * mut c_int,
    width: * mut c_int,
    rgbmax: * mut c_int,
    image:Vec<Pixel>
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

impl Pixel{
    fn new(red: *mut u8, green: *mut u8, blue: *mut u8) -> Pixel {
        Pixel {r: red, g: green, b: blue}
    }
    fn red(&self) ->*mut u8 {
        self.r
    }
    fn green(&self) -> *mut u8 {
        self.g
    }
    fn blue(&self) -> *mut u8 {
        self.b
    }
    unsafe fn display(self) {
        println!("({:x} {:x} {:x})", *self.r, *self.g, *self.b)
    }
    unsafe fn invert(self, rgbmax : u8){
        *self.r = rgbmax - *self.r;
        *self.g = rgbmax - *self.g;
        *self.b = rgbmax - *self.b;
    }
}



/*static mut Images:Image = Image{
    height:0 as * mut c_int,
    width:0 as * mut c_int,
    image: Vec::new()   
};*/



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
    fn ppma_write ( file_out_name: *const u8,
                    xsize: * mut c_int, 
                    ysize: * mut c_int, 
                    r: &* mut c_int,
                    g: &* mut c_int, 
                    b: &* mut c_int
                ) -> c_int;
}

//Read the image file (C)
#[no_mangle]
pub extern fn readPPM(  input_name: *const c_char, 
                        xsizep:* mut c_int, 
                        ysizep:* mut c_int, 
                        rgb_maxp:* mut c_int ,
                        rp:* mut c_int, 
                        gp:* mut c_int, 
                        bp:* mut c_int
                    )-> * mut c_int {

    /*                        
    let xsize:* mut c_int = xsizep ;
    let ysize:* mut c_int = ysizep ; */
    let mut r:* mut c_int = rp; 
    let mut g:* mut c_int = gp;
    let mut b:* mut c_int = bp;
    /*unsafe{
        Images.width = ysizep;
        Images.height = xsizep;
        Images.image = Vec::new();
    }*/
    let mut images = Image{
        height : xsizep,
        width : ysizep,
        image:Vec::new(),
        rgbmax:rgb_maxp
    };

    let filename = unsafe {
        CStr::from_ptr(input_name).to_string_lossy().into_owned().to_string()
    };
    println!("{}",filename);
    unsafe { ppma_read(filename.as_ptr(), &images.height, &images.width,&images.rgbmax, &r, &g, &b) };
    unsafe{
        for i in 0..images.height as u8
        {
            for j in 0..images.width as u8
            {
                //println!("pixel : R({:?}), G({:?}), B({:?})",*(r as * mut u8),*(g as * mut u8),*(b as * mut u8));
                images.image.push(Pixel::new(r as * mut u8, g as * mut u8, b as * mut u8));
                r = r.offset(1);
                g = g.offset(1);
                b = b.offset(1);
                //println!("NIKE :({:?})",images.image.last());

            }
        }
    }
    //println!("{:0}{}{}{}{}{}",xsize, ysize, rgb_max, r, g, b);
    writePPM(filename,rp,gp,bp,images);
    return r;
}

//Print a image file (C)

fn writePPM( input_name: String, 
                        rp:* mut c_int, 
                        gp:* mut c_int, 
                        bp:* mut c_int,
                        text: Image
                    )-> c_int {

        let mut filename = format!("{}-inverted.ppm", input_name);
        println!(" ALLO {:?}",filename);
        let mut r:Vec<* mut c_int> = Vec::new(); 
        let mut g:Vec<* mut c_int> = Vec::new();
        let mut b:Vec<* mut c_int> = Vec::new();
        //println!("{:?}",text.rgbmax as u8);
        unsafe{
            for i in &text.image{
                //println!("before : R({:?}), G({:?}), B({:?})",*i.red(),*i.green(),*i.blue());
                i.invert(text.rgbmax as u8);
                r.push(*i.red()  as * mut c_int);
                g.push(*i.green()  as * mut c_int);
                b.push(*i.blue()  as * mut c_int);
                println!("after : R({:?}), G({:?}), B({:?})",*i.red(),*i.green(),*i.blue());
                println!("");
            }
        }
    return unsafe { ppma_write(filename.as_ptr(), text.height, text.width, &r[0], &g[0], &b[0]) };
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
