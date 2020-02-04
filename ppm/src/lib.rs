#![feature(test)]
extern crate test;

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
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[bench]
    fn bench_dummy(b: &mut Bencher) {
        b.iter(|| dummy());
    }

    #[bench]
    fn bench_gray(b: &mut Bencher) {
        // let path = "/mnt/d/4_eme_annee/Rust/Rust/ppm/test.ppm";
        // let mut xsize = 0;
        // let mut xsize_ptr = unsafe { &mut xsize as i32 };
        // let mut ysize = 0;
        // let mut ysize_ptr = unsafe { &mut ysize as i32 };
        // let mut rgbmax = 0;
        // let mut rgbmax_ptr = unsafe { &mut rgbmax as i32 };
        // let mut r = 0;
        // let mut r_ptr = unsafe { &mut r as i32 };
        // let mut g = 0;
        // let mut g_ptr = unsafe { &mut g as i32 };
        // let mut b = 0;
        // let mut b_ptr = unsafe { &mut b as i32 };
        let input_name: *const c_char = CString::new("/mnt/d/4_eme_annee/Rust/Rust/ppm/test.ppm").expect("CString::new failed").as_ptr();
        let mut xsizep:* mut c_int; 
        let mut ysizep:* mut c_int;
        let mut rgb_maxp:* mut c_int;
        let mut rp:* mut c_int;
        let mut gp:* mut c_int; 
        let mut bp:* mut c_int;
        
        xsizep = 0;
        ysizep = 0;
        rgb_maxp = 0;
        rp = 0;
        gp = 0;
        bp = 0;
        b.iter(|| {
            grayColor(input_name, xsizep, ysizep, rgb_maxp, rp, gp, bp);
            
            }
        ); 
            /*
            input_name: *const c_char, 
                        xsizep:* mut c_int, 
                        ysizep:* mut c_int, 
                        rgb_maxp:* mut c_int ,
                        rp:* mut c_int, 
                        gp:* mut c_int, 
                        bp:* mut c_int
        );*/
    }

    #[bench]
    fn bench_invert(b: &mut Bencher) {
        //b.iter(|| invertColor());
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/* STRUCTURES */
struct Pixel{
    r:*mut u8,
    g:*mut u8,
    b:*mut u8
}



//struct Pixel{
//    pixel:[Color;8]
//}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Image{
    height: * mut c_int,
    width: * mut c_int,
    rgbmax: * mut c_int,
    image:Vec<Pixel>
}

/*impl Image{
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
}*/

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
        println!("({:?} {:?} {:?})", *self.r, *self.g, *self.b)
    }
    unsafe fn invert(self, rgb_max : u8){
        *self.r = rgb_max - *self.r;
        *self.g = rgb_max - *self.g;
        *self.b = rgb_max - *self.b;
    }
    unsafe fn gray(self){
        let avg = (*self.r + *self.g + *self.b) /3;
        *self.r = avg;
        *self.g = avg;
        *self.b = avg;
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
fn readPPM(  filename: &String, 
            xsizep:* mut c_int, 
            ysizep:* mut c_int, 
            rgb_maxp:* mut c_int ,
            mut rp:* mut c_int, 
            mut gp:* mut c_int, 
            mut bp:* mut c_int,
            images: &mut Image
        )-> * mut c_int {

    /*                        
    let xsize:* mut c_int = xsizep ;
    let ysize:* mut c_int = ysizep ; */
    println!("{}",filename);
    unsafe { ppma_read(filename.as_ptr(), &images.height, &images.width,&images.rgbmax, &rp, &gp, &bp) };
    unsafe{
        for i in 0..images.height as u8
        {
            for j in 0..images.width as u8
            {
                //println!("pixel : R({:?}), G({:?}), B({:?})",*(rp as * mut u8),*(gp as * mut u8),*(bp as * mut u8));
                let x = Pixel::new(rp as * mut u8, gp as * mut u8, bp as * mut u8);
                images.image.push(x);
                rp = rp.offset(1);
                gp = gp.offset(1);
                bp = bp.offset(1);
                //println!("NIKE :({:?})",images.image.last());

            }
        }
    }
    //println!("{:0}{}{}{}{}{}",xsize, ysize, rgb_max, r, g, b);
    
    return rp;
}

//Print a image file (C)
#[no_mangle]
fn writePPM( input_name: &String, 
                r:&Vec<* mut c_int>, 
                g:&Vec<* mut c_int>, 
                b:&Vec<* mut c_int>,
                text: &Image
            )-> c_int {
    return unsafe { ppma_write(input_name.as_ptr(), text.height, text.width,&r[0], &g[0], &b[0]) };
}

#[no_mangle]
pub extern fn revertColor(input_name: *const c_char, 
                        xsizep:* mut c_int, 
                        ysizep:* mut c_int, 
                        rgb_maxp:* mut c_int ,
                        rp:* mut c_int, 
                        gp:* mut c_int, 
                        bp:* mut c_int){
    
    unsafe{
        let mut images = Image{
            height : xsizep,
            width : ysizep,
            image:Vec::new(),
            rgbmax:rgb_maxp
        };

        let filename = CStr::from_ptr(input_name).to_string_lossy().into_owned().to_string();
        readPPM(&filename,images.height,images.width,images.rgbmax,rp,gp,bp,&mut images);
        let filenamed = format!("{}-inverted.ppm", filename);
        println!(" ALLO {:?}",filenamed);
        
        let mut r:Vec<* mut c_int> = Vec::new(); 
        let mut g:Vec<* mut c_int> = Vec::new();
        let mut b:Vec<* mut c_int> = Vec::new();
        for i in &images.image{
            //println!("before : R({:?}), G({:?}), B({:?})",*i.red(),*i.green(),*i.blue());
            i.invert(images.rgbmax as u8);
            r.push(*i.red()  as * mut c_int);
            g.push(*i.green()  as * mut c_int);
            b.push(*i.blue()  as * mut c_int);
            //println!("after : R({:?}), G({:?}), B({:?})",r.last(), g.last(),b.last());
            //println!("");
        }
        writePPM(&filenamed,&r,&g,&b,&images);
    }
}

#[no_mangle]
pub extern fn grayColor(input_name: *const c_char, 
                        xsizep:* mut c_int, 
                        ysizep:* mut c_int, 
                        rgb_maxp:* mut c_int ,
                        rp:* mut c_int, 
                        gp:* mut c_int, 
                        bp:* mut c_int){
    
    unsafe{
        let mut images = Image{
            height : xsizep,
            width : ysizep,
            image:Vec::new(),
            rgbmax:rgb_maxp
        };

        let filename = CStr::from_ptr(input_name).to_string_lossy().into_owned().to_string();
        readPPM(&filename,images.height,images.width,images.rgbmax,rp,gp,bp,&mut images);
        let filenamed = format!("{}-gray.ppm", filename);
        println!(" ALLO {:?}",filenamed);
        
        let mut r:Vec<* mut c_int> = Vec::new(); 
        let mut g:Vec<* mut c_int> = Vec::new();
        let mut b:Vec<* mut c_int> = Vec::new();
        for i in &images.image{
            //println!("before : R({:?}), G({:?}), B({:?})",*i.red(),*i.green(),*i.blue());
            i.gray();
            r.push(*i.red()  as * mut c_int);
            g.push(*i.green()  as * mut c_int);
            b.push(*i.blue()  as * mut c_int);
            //println!("after : R({:?}), G({:?}), B({:?})",r.last(), g.last(),b.last());
            //println!("");
        }
        writePPM(&filenamed,&r,&g,&b,&images);
    }
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