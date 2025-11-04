use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Error correction level for QR code
pub enum QrEccLevel {
    /// Low - 7% of codewords can be restored
    Low = 0,
    /// Medium - 15% of codewords can be restored
    Medium = 1,
    /// Quartile - 25% of codewords can be restored
    Quartile = 2,
    /// High - 30% of codewords can be restored
    High = 3,
}

impl QrEccLevel {
    fn to_qrcodegen_ecc(&self) -> QrCodeEcc {
        match self {
            QrEccLevel::Low => QrCodeEcc::Low,
            QrEccLevel::Medium => QrCodeEcc::Medium,
            QrEccLevel::Quartile => QrCodeEcc::Quartile,
            QrEccLevel::High => QrCodeEcc::High,
        }
    }
}

#[wasm_bindgen]
/// `QRCode` QR code generator
pub struct QRCode {
    qr: QrCode,
}

impl std::fmt::Debug for QRCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QRCode")
            .field("size", &self.qr.size())
            .finish()
    }
}

#[wasm_bindgen]
impl QRCode {
    /// Create a new QR code from data
    #[wasm_bindgen(constructor)]
    pub fn new(data: &str, ecc_level: u8) -> Result<QRCode, String> {
        let ecc = match ecc_level {
            0 => QrEccLevel::Low,
            1 => QrEccLevel::Medium,
            2 => QrEccLevel::Quartile,
            3 => QrEccLevel::High,
            _ => return Err("Invalid error correction level. Use 0-3.".to_string()),
        };

        let qr = QrCode::encode_text(data, ecc.to_qrcodegen_ecc())
            .map_err(|e| format!("Failed to encode QR code: {:?}", e))?;

        Ok(QRCode { qr })
    }

    /// Get the size of the QR code
    pub fn size(&self) -> i32 {
        self.qr.size()
    }

    /// Get module value at position (x, y)
    pub fn get_module(&self, x: i32, y: i32) -> bool {
        self.qr.get_module(x, y)
    }

    /// Generate SVG string
    pub fn to_svg(&self, border: u32) -> String {
        let size = self.qr.size() as i32;
        let svg_size = size + (border as i32 * 2);

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" stroke="none">"#,
            svg_size, svg_size
        ));
        svg.push_str("\n  <rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>\n  <path fill=\"#000000\" d=\"");

        for y in 0..size {
            for x in 0..size {
                if self.qr.get_module(x, y) {
                    let px = x + border as i32;
                    let py = y + border as i32;
                    svg.push_str(&format!("M{},{}h1v1h-1z", px, py));
                }
            }
        }

        svg.push_str("\"/>\n</svg>");
        svg
    }

    /// Generate QR code with default settings (static method)
    pub fn generate(data: &str) -> Result<String, String> {
        let qr = Self::new(data, 1)?;
        Ok(qr.to_svg(4))
    }
}

#[test]
fn qrcode_test() {
    let data = "https://github.com";

    // Test instance API
    let qr = QRCode::new(data, 1).unwrap();
    println!("{:?}", qr);

    let svg = qr.to_svg(4);
    assert!(svg.contains("<svg"));

    // Test static API
    let svg = QRCode::generate(data).unwrap();
    assert!(svg.contains("<svg"));
}
