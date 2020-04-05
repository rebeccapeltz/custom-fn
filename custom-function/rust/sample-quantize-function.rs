#[macro_use]
extern crate serde_derive;
extern crate exoquant;
extern crate serde;
extern crate serde_json;
extern crate byteorder;

use std::mem;
use byteorder::{ WriteBytesExt, BigEndian};
use exoquant::*;

#[derive(Deserialize, Debug)]
struct Metadata {
    context: Option<std::collections::HashMap<String, String>>,
    tags: Option<Vec<String>>,
    variables: Option<std::collections::HashMap<String, i32>>,
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, cap: usize) {
    unsafe  {
      let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn transform(width: u32, height: u32, image_ptr: *mut u8, meta_ptr: *mut u8, meta_size: usize) -> u32 {
  let size = (width * height * 4) as usize;
  let bytes = unsafe {Vec::from_raw_parts(image_ptr, size, size)};
  let meta_bytes = unsafe {Vec::from_raw_parts(meta_ptr, meta_size, meta_size)};
  let metadata: Metadata = serde_json::from_slice(&meta_bytes).expect("Failed to deserialize metadata json");
  host_trace(format!("{:?}", metadata));
  // let metadata = Metadata{context: None, tags: None, args: None};
  let (out_w, out_h, mut out_buffer) = quantize(width, height, &bytes, metadata);
  let mut dims = vec![];
  let _ = dims.write_u32::<BigEndian>(out_w);
  let _ = dims.write_u32::<BigEndian>(out_h);
  dims.append(&mut out_buffer);
  let out_buffer = dims;
  let out_ptr = out_buffer.as_ptr() as u32;
  mem::forget(out_buffer);
  out_ptr  
}

fn quantize(width: u32, height: u32, bytes: &Vec<u8>, metadata: Metadata) -> (u32, u32, Vec<u8>) {
  let colors = metadata.variables.and_then(|m| m.get("colors").map(|c| *c as usize)).unwrap_or(16);
  let image: Vec<Color> = bytes.chunks_exact(4).map(|c| Color::new(c[0], c[1], c[2], c[3])).collect();  
  let (palette, indexed_data) = exoquant::convert_to_indexed(&image, width as usize, colors, &optimizer::KMeans, &ditherer::FloydSteinberg::new());
  let out_bytes = indexed_data.into_iter().flat_map(|i| {
    let p = palette[i as usize];
    vec![p.r, p.g, p.b, p.a]
  }).collect();
  (width, height, out_bytes)
}

fn host_trace(x: String) {
  let buf = x.into_bytes();
  let length = buf.len();
  let ptr = buf.as_ptr();
  unsafe { trace(ptr as u32, length as u32) }
}

extern "C" {
  pub fn trace(x: u32, length: u32);
}


// pub blur(width: u32, height: u32, bytes: &Vec<u8>) -> u32 {
//   if let Some(ref img) = RgbaImage::from_raw(width, height, bytes) {
//     // let subimg = imageops::flip_vertical(img); //works
//     let subimg = imageops::blur(img, 2.5);
//     // let subimg = imageops::resize(img, 100, 100, FilterType::Nearest); // works
//     // let subimg = imageops::resize(img, 100, 100, FilterType::Lanczos3); // works
//     let out_w = subimg.width();
//     let out_h = subimg.height();
//     let mut out_buffer = subimg.into_raw();
//     let mut dims = vec![];
//     dims.write_u32::<BigEndian>(out_w);
//     dims.write_u32::<BigEndian>(out_h);
//     dims.append(&mut out_buffer);
//     let mut out_buffer = dims;
//     let out_ptr = out_buffer.as_ptr() as u32;
//     mem::forget(out_buffer);
//     out_ptr
//   } else {
//     0
//   }
// }