#![feature(exact_chunks)]

#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;
extern crate serde_json;
extern crate byteorder;

use std::mem;
use byteorder::{ WriteBytesExt, BigEndian};
use image::{RgbaImage, imageops};

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
  let (out_w, out_h, mut out_buffer) = blur(width, height, bytes, metadata);
  let mut dims = vec![];
  let _ = dims.write_u32::<BigEndian>(out_w);
  let _ = dims.write_u32::<BigEndian>(out_h);
  dims.append(&mut out_buffer);
  let out_buffer = dims;
  let out_ptr = out_buffer.as_ptr() as u32;
  mem::forget(out_buffer);
  out_ptr  
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


fn blur(width: u32, height: u32, bytes: Vec<u8>, _metadata: Metadata) -> (u32, u32, Vec<u8>) {
  let ref img = RgbaImage::from_raw(width, height, bytes).unwrap();
  let subimg = imageops::blur(img, 2.5);
  let out_w = subimg.width();
  let out_h = subimg.height();
  let out_buffer = subimg.into_raw();
  (out_w, out_h, out_buffer)
}