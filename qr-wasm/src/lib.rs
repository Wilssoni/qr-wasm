use wasm_bindgen::prelude::*;
use image;
use rqrr;

#[wasm_bindgen]
pub fn read_qr(buff:&mut [u8], height: usize, width: usize) -> String {
    let mut returnstring: String = "".to_string();
    let mut imgbuf = image::RgbaImage::new(width as u32,height as u32);
    for y in 0..(height as u32) {
        for x in 0..(width as u32) {
            let i: usize = (width * (y as usize) + (x as usize)) * 4;
            //console::log(&format!("{}", i));
            let r: u8 = buff[i];
            let g: u8 = buff[i+1];
            let b: u8 = buff[i+2];
            let a: u8 = 255;
            imgbuf.put_pixel(x,y, image::Rgba([r, g, b, a]));
        }
    }
    let img2 = image::DynamicImage::ImageRgba8(imgbuf);
    let mut img3 = rqrr::PreparedImage::prepare(img2.to_luma8());
    let grids = img3.detect_grids();
    if grids.len() > 0 {
        let (_meta, content) = grids[0].decode().unwrap();
        returnstring = content;
    }
    return returnstring;
}
