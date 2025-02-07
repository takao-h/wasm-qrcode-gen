use wasm_qrcode_gen::generate_qr_code;

fn main() {
    let qr_data = "https://www.example.com";
    let qr_size = 10;
    
    let svg_data = generate_qr_code(qr_data, qr_size);
    
    std::fs::write("qrcode.svg", svg_data).unwrap();
    
    println!("QR code generated and saved as qrcode.svg");
}
