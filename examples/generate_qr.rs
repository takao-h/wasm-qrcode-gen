use qr_wasm_lib::generate_qr_code;

fn main() {
    let qr_data = "https://www.example.com";
    let qr_size = 10; // ピクセル単位でのモジュールサイズ
    
    let png_data = generate_qr_code(qr_data, qr_size);
    
    // PNGデータをファイルに保存
    std::fs::write("qrcode.png", png_data).unwrap();
    
    println!("QR code generated and saved as qrcode.png");
}
