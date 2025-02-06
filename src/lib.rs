use wasm_bindgen::prelude::*;
use qrcode::QrCode;
use image::Luma;

#[wasm_bindgen]
pub fn generate_qr_code(data: &str, size: u32) -> Vec<u8> {
    let code = QrCode::new(data.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>()
        .quiet_zone(false)
        .module_dimensions(size, size)
        .build();
    
    let mut buffer = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png).unwrap();
    buffer
}
