use wasm_bindgen::prelude::*;
use qrcode_generator::QrCodeEcc;
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[wasm_bindgen]
pub fn generate_qr_code(data: &str, size: u32) -> String {
    let qr = qrcode_generator::to_svg_to_string(data, QrCodeEcc::Low, size as usize, None::<&str>).unwrap();
    let base64_encoded = STANDARD.encode(qr);
    format!("data:image/svg+xml;base64,{}", base64_encoded)
}
