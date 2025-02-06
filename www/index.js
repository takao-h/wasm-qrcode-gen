import init, { generate_qr_code } from './pkg/qr_wasm_lib.js';

async function run() {
    await init();

    const qrData = "https://www.example.com";
    const qrSize = 10;

    const pngData = generate_qr_code(qrData, qrSize);
    
    // PNGデータをBlobに変換
    const blob = new Blob([pngData], { type: 'image/png' });
    
    // Blobから画像URLを作成
    const imageUrl = URL.createObjectURL(blob);
    
    // 画像を表示
    const img = document.createElement('img');
    img.src = imageUrl;
    document.body.appendChild(img);
}

run();
