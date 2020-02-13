#![crate_name = "ppm"]

#![feature(test)]
extern crate test;

extern crate libc;
extern crate c_string;
use libc::{c_char, c_int};
use std::ffi::{CStr, CString};

use std::io::{self};
use std::io::prelude::*;
use std::fs::File;

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
        let input_name: *const c_char = CString::new("test").expect("CString::new failed").as_ptr();
        let xsizep:* mut c_int; 
        let ysizep:* mut c_int;
        let rgb_maxp:* mut c_int;
        let rp:* mut c_int;
        let gp:* mut c_int; 
        let bp:* mut c_int;
        xsizep = 0 as * mut c_int;
        ysizep = 0 as * mut c_int;
        rgb_maxp = 0 as * mut c_int;
        rp = 0 as * mut c_int;
        gp = 0 as * mut c_int;
        bp = 0 as * mut c_int;
        b.iter(|| gray_color_noc(input_name, xsizep, ysizep, rgb_maxp, rp, gp, bp)); 
    }

    #[bench]
    fn bench_revert(b: &mut Bencher) {
        let input_name: *const c_char = CString::new("test").expect("CString::new failed").as_ptr();
        let xsizep:* mut c_int; 
        let ysizep:* mut c_int;
        let rgb_maxp:* mut c_int;
        let rp:* mut c_int;
        let gp:* mut c_int; 
        let bp:* mut c_int;
        xsizep = 0 as * mut c_int;
        ysizep = 0 as * mut c_int;
        rgb_maxp = 0 as * mut c_int;
        rp = 0 as * mut c_int;
        gp = 0 as * mut c_int;
        bp = 0 as * mut c_int;
        b.iter(|| revert_color_noc(input_name, xsizep, ysizep, rgb_maxp, rp, gp, bp));
    }
}

/// Return 42
/// # Return
/// * 42
#[no_mangle]
pub extern fn dummy() -> u8{
    return 42;
}

/// Return max of two number
/// # Args
/// * a : u8 
/// * b : u8 
#[no_mangle]
pub extern fn max(a:u8, b:u8)->u8{
    return if a > b {a} else {b};
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Pixel struct
pub struct Pixel{
    /// Red
    r:*mut u8,
    /// Grean
    g:*mut u8,
    /// Blue
    b:*mut u8
}



#[derive(Clone, PartialEq, Eq, Debug)]
/// Image Struct
pub struct Image{
    /// Height
    height: * mut c_int,
    /// Width
    width: * mut c_int,
    /// RgbMax
    rgbmax: * mut c_int,
    /// Image
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
/// Implement Pixel
impl Pixel{
    /// Create a new Pixel
    pub fn new(red: *mut u8, green: *mut u8, blue: *mut u8) -> Pixel {
        Pixel {r: red, g: green, b: blue}
    }
    /// Return the red pixel
    pub fn red(&self) ->*mut u8 {
        self.r
    }
    /// Return the green pixel
    pub fn green(&self) -> *mut u8 {
        self.g
    }
    /// Return the blue pixel
    pub fn blue(&self) -> *mut u8 {
        self.b
    }

    /// Invert the current color of a pixel
    /// # Args
    /// * self : current pixel
    /// * rgb_max : maximun of rgb color
    pub unsafe fn invert(self, rgb_max : u8){
        *self.r = rgb_max - *self.r;
        *self.g = rgb_max - *self.g;
        *self.b = rgb_max - *self.b;
    }
    /// Invert the grayscale of a pixel
    /// # Args
    /// * self : current pixel
    pub unsafe fn gray(self){
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

/// Read the image file (C)

/// # Args : 
/// * filename: &String
/// * _xsizep:* mut c_int
/// * _ysizep:* mut c_int
/// * _rgb_maxp:* mut c_int 
/// * rp:* mut c_int
/// * gp:* mut c_int
/// * bp:* mut c_int
/// * images: &mut Image
#[no_mangle]
pub fn read_ppm(  filename: &String, 
            _xsizep:* mut c_int, 
            _ysizep:* mut c_int, 
            _rgb_maxp:* mut c_int ,
            mut rp:* mut c_int, 
            mut gp:* mut c_int, 
            mut bp:* mut c_int,
            images: &mut Image
        )-> * mut c_int {
    unsafe { ppma_read(filename.as_ptr(), &images.height, &images.width,&images.rgbmax, &rp, &gp, &bp) };
    unsafe{
        for _i in 0..images.height as u8
        {
            for _j in 0..images.width as u8
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

/// Print a image file (C)

/// # Args :
/// * input_name: &String 
/// * r:&Vec<* mut c_int> 
/// * g:&Vec<* mut c_int> 
/// * b:&Vec<* mut c_int>
/// * text: &Image
#[no_mangle]
pub fn write_ppm( input_name: &String, 
                r:&Vec<* mut c_int>, 
                g:&Vec<* mut c_int>, 
                b:&Vec<* mut c_int>,
                text: &Image
            )-> c_int {
    return unsafe { ppma_write(input_name.as_ptr(), text.height, text.width,&r[0], &g[0], &b[0]) };
}

#[no_mangle]
/// Revert color of an image

/// # Args : 
/// * input_name: *const c_char 
/// * xsizep:* mut c_int
/// * ysizep:* mut c_int 
/// * rgb_maxp:* mut c_int
/// * rp:* mut c_int 
/// * gp:* mut c_int 
/// * bp:* mut c_int
pub extern fn revert_color(input_name: *const c_char, 
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
        read_ppm(&filename,images.height,images.width,images.rgbmax,rp,gp,bp,&mut images);
        let filenamed = format!("{}-inverted", filename);
        // println!(" ALLO {:?}",filenamed);
        
        let mut r:Vec<* mut c_int> = Vec::new(); 
        let mut g:Vec<* mut c_int> = Vec::new();
        let mut b:Vec<* mut c_int> = Vec::new();
        for _i in &images.image{
            //println!("before : R({:?}), G({:?}), B({:?})",*i.red(),*i.green(),*i.blue());
            _i.invert(images.rgbmax as u8);
            r.push(*_i.red()  as * mut c_int);
            g.push(*_i.green()  as * mut c_int);
            b.push(*_i.blue()  as * mut c_int);
            //println!("after : R({:?}), G({:?}), B({:?})",r.last(), g.last(),b.last());
            //println!("");
        }
        write_ppm(&filenamed,&r,&g,&b,&images);
    }
}

#[no_mangle]
/// Create a gray scale of an image

/// # Args : 
/// * input_name: *const c_char 
/// * xsizep:* mut c_int
/// * ysizep:* mut c_int 
/// * rgb_maxp:* mut c_int
/// * rp:* mut c_int 
/// * gp:* mut c_int 
/// * bp:* mut c_int
pub extern fn gray_color(input_name: *const c_char, 
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
        read_ppm(&filename,images.height,images.width,images.rgbmax,rp,gp,bp,&mut images);
        let filenamed = format!("{}-gray", filename);
                println!(" ALLO {:?}",filenamed);

        let mut r:Vec<* mut c_int> = Vec::new(); 
        let mut g:Vec<* mut c_int> = Vec::new();
        let mut b:Vec<* mut c_int> = Vec::new();
        for _i in &images.image{
            //println!("before : R({:?}), G({:?}), B({:?})",*i.red(),*i.green(),*i.blue());
            _i.gray();
            r.push(*_i.red()  as * mut c_int);
            g.push(*_i.green()  as * mut c_int);
            b.push(*_i.blue()  as * mut c_int);
            //println!("after : R({:?}), G({:?}), B({:?})",r.last(), g.last(),b.last());
            //println!("");
        }
        write_ppm(&filenamed,&r,&g,&b,&images);
    }
}

//Read a file (RUST)
#[no_mangle]
extern fn read_file(filename : String)-> io::Result<()>{
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
extern fn setFileName(filenam: *const c_char) {
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



#[no_mangle]
fn read_ppm_noc(  _filename: &String, 
            _xsizep:* mut c_int, 
            _ysizep:* mut c_int, 
            _rgb_maxp:* mut c_int ,
            mut rp:* mut c_int, 
            mut gp:* mut c_int, 
            mut bp:* mut c_int,
            images: &mut Image
        )-> * mut c_int {

    /*                        
    let xsize:* mut c_int = xsizep ;
    let ysize:* mut c_int = ysizep ; */
    // println!("{}",filename);
    // unsafe { 
    //     ppma_read(filename.as_ptr(), &images.height, &images.width,&images.rgbmax, &rp, &gp, &bp) 
    // };
    unsafe{
        for _i in 0..images.height as u8
        {
            for _j in 0..images.width as u8
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


#[no_mangle]
fn write_ppm_noc( _input_name: &String, 
                _r:&Vec<* mut c_int>, 
                _g:&Vec<* mut c_int>, 
                _b:&Vec<* mut c_int>,
                _text: &Image
            ) {
}

#[no_mangle]
extern fn revert_color_noc(_input_name: *const c_char, 
                        _xsizep:* mut c_int, 
                        _ysizep:* mut c_int, 
                        _rgb_maxp:* mut c_int ,
                        _rp:* mut c_int, 
                        _gp:* mut c_int, 
                        _bp:* mut c_int){
    
    unsafe{
        let mut images = Image{
            height : _xsizep,
            width : _ysizep,
            image:Vec::new(),
            rgbmax:_rgb_maxp
        };

        let _filename = CStr::from_ptr(_input_name).to_string_lossy().into_owned().to_string();
        read_ppm_noc(&_filename,images.height,images.width,images.rgbmax,_rp,_gp,_bp,&mut images);
        let filenamed = format!("{}-inverted", _filename);
        
        let mut r:Vec<* mut c_int> = Vec::new(); 
        let mut g:Vec<* mut c_int> = Vec::new();
        let mut b:Vec<* mut c_int> = Vec::new();
        for _i in &images.image{
            //println!("before : R({:?}), G({:?}), B({:?})",*i.red(),*i.green(),*i.blue());
            _i.invert(images.rgbmax as u8);
            r.push(*_i.red()  as * mut c_int);
            g.push(*_i.green()  as * mut c_int);
            b.push(*_i.blue()  as * mut c_int);
            //println!("after : R({:?}), G({:?}), B({:?})",r.last(), g.last(),b.last());
            //println!("");
        }
        write_ppm_noc(&filenamed,&r,&g,&b,&images);
    }
}


#[no_mangle]
extern fn gray_color_noc(_input_name: *const c_char, 
                        _xsizep:* mut c_int, 
                        _ysizep:* mut c_int, 
                        _rgb_maxp:* mut c_int ,
                        _rp:* mut c_int, 
                        _gp:* mut c_int, 
                        _bp:* mut c_int){
    
    unsafe{
        let mut images = Image{
            height : _xsizep,
            width : _ysizep,
            image:Vec::new(),
            rgbmax:_rgb_maxp
        };

        let _filename = CStr::from_ptr(_input_name).to_string_lossy().into_owned().to_string();
        read_ppm_noc(&_filename,images.height,images.width,images.rgbmax,_rp,_gp,_bp,&mut images);
        let filenamed = format!("{}-gray", _filename);
        
        let mut r:Vec<* mut c_int> = Vec::new(); 
        let mut g:Vec<* mut c_int> = Vec::new();
        let mut b:Vec<* mut c_int> = Vec::new();
        for _i in &images.image{
            //println!("before : R({:?}), G({:?}), B({:?})",*i.red(),*i.green(),*i.blue());
            _i.gray();
            r.push(*_i.red()  as * mut c_int);
            g.push(*_i.green()  as * mut c_int);
            b.push(*_i.blue()  as * mut c_int);
            //println!("after : R({:?}), G({:?}), B({:?})",r.last(), g.last(),b.last());
            //println!("");
        }
        write_ppm_noc(&filenamed,&r,&g,&b,&images);
    }
}