use wasm_bindgen::prelude::*;
use image;
use rqrr;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const IMAGE_BYTES_LEN: usize = WIDTH * HEIGHT * 4;

static mut IMAGEBYTES:[u8;IMAGE_BYTES_LEN] = [0; IMAGE_BYTES_LEN];

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

pub mod console {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s:&str);
    }
}

#[no_mangle]
pub fn get_image_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = IMAGEBYTES.as_ptr();
    }

    return pointer;
}

#[wasm_bindgen]
pub fn image2link() {
    let mut imgbuf = image::RgbaImage::new(WIDTH as u32,HEIGHT as u32);
    unsafe {
        for y in 0..(HEIGHT as u32) {
            for x in 0..(WIDTH as u32) {
                let i: usize = (WIDTH * (y as usize) + (x as usize)) * 4;
                //console::log(&format!("{}", i));
                let r: u8 = IMAGEBYTES[i];
                let g: u8 = IMAGEBYTES[i+1];
                let b: u8 = IMAGEBYTES[i+2];
                let a: u8 = 255;
                imgbuf.put_pixel(x,y, image::Rgba([r, g, b, a]));
            }
        }
    }
    let img2 = image::DynamicImage::ImageRgba8(imgbuf);
    let mut img3 = rqrr::PreparedImage::prepare(img2.to_luma());
    let grids = img3.detect_grids();
    if grids.len() > 0 {
        let (meta, content) = grids[0].decode().unwrap();
        alert(&format!("{}!", content));
    }
}
