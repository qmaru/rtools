use qrcode::{EcLevel, QrCode, Version};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
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
    fn to_ecc(&self) -> EcLevel {
        match self {
            QrEccLevel::Low => EcLevel::L,
            QrEccLevel::Medium => EcLevel::M,
            QrEccLevel::Quartile => EcLevel::Q,
            QrEccLevel::High => EcLevel::H,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
/// QR code version
pub enum QrVersion {
    /// Micro QR version 1
    Micro1 = 1,
    /// Micro QR version 2
    Micro2 = 2,
    /// Micro QR version 3
    Micro3 = 3,
    /// Micro QR version 4
    Micro4 = 4,
    /// Normal QR version 1
    Normal1 = 10,
    /// Normal QR version 5
    Normal5 = 15,
    /// Normal QR version 10
    Normal10 = 20,
    /// Normal QR version 15
    Normal15 = 25,
    /// Normal QR version 20
    Normal20 = 30,
    /// Normal QR version 30
    Normal30 = 40,
    /// Normal QR version 40
    Normal40 = 50,
}

impl QrVersion {
    fn to_version(&self) -> Version {
        match self {
            QrVersion::Micro1 => Version::Micro(1),
            QrVersion::Micro2 => Version::Micro(2),
            QrVersion::Micro3 => Version::Micro(3),
            QrVersion::Micro4 => Version::Micro(4),
            QrVersion::Normal1 => Version::Normal(1),
            QrVersion::Normal5 => Version::Normal(5),
            QrVersion::Normal10 => Version::Normal(10),
            QrVersion::Normal15 => Version::Normal(15),
            QrVersion::Normal20 => Version::Normal(20),
            QrVersion::Normal30 => Version::Normal(30),
            QrVersion::Normal40 => Version::Normal(40),
        }
    }
}

#[wasm_bindgen]
/// `QRCode` QR code generator
pub struct QRCode {}

#[wasm_bindgen]
impl QRCode {
    /// `generate_manual` Generate a QR code with the specified data, version, and error correction level
    fn generate_manual(
        data: &str,
        version: QrVersion,
        ec_level: QrEccLevel,
    ) -> Result<QrCode, JsValue> {
        QrCode::with_version(data, version.to_version(), ec_level.to_ecc())
            .map_err(|e| JsValue::from_str(&format!("QR code error: {}", e)))
    }

    /// `generate_auto_version` Generate a QR code with the specified data and error correction level
    fn generate_auto_version(data: &str, ec_level: QrEccLevel) -> Result<QrCode, JsValue> {
        QrCode::with_error_correction_level(data, ec_level.to_ecc())
            .map_err(|e| JsValue::from_str(&format!("QR code error: {}", e)))
    }

    /// `to_raw_bytes` Convert QrCode to raw byte format
    ///
    /// ### Arguments
    /// * `code` - The QrCode object
    ///
    /// ### Returns
    /// Vec<u8> with format: [width, is_micro, module_data...]
    fn to_raw_bytes(code: &QrCode) -> Vec<u8> {
        let width = code.width() as u8;
        let is_micro = code.version().is_micro();
        let mut bytes = vec![width, if is_micro { 1 } else { 0 }];

        let total_modules = code.width() * code.width();
        let packed_size = (total_modules + 7) / 8;
        let mut packed_data = vec![0u8; packed_size];

        for y in 0..code.width() {
            for x in 0..code.width() {
                let idx = y * code.width() + x;
                if code[(x, y)] == qrcode::Color::Dark {
                    packed_data[idx / 8] |= 1 << (7 - (idx % 8));
                }
            }
        }
        bytes.extend(packed_data);
        bytes
    }

    /// `to_text_string` Convert QrCode to text string format
    ///
    /// ### Arguments
    /// * `code` - The QrCode object
    ///
    /// ### Returns
    /// String containing ASCII art representation of QR code
    fn to_text_string(code: &QrCode) -> String {
        code.render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build()
    }

    /// `raw` Generate raw QR code data with custom version and error correction level
    ///
    /// ### Arguments
    /// * `data` - The data to encode
    /// * `version` - QR code version (Micro1-Micro4 or Normal1-Normal40)
    /// * `ec_level` - Error correction level (Low, Medium, Quartile, High)
    ///
    /// ### Returns
    /// Vec<u8> with format: [width, is_micro, module_data...]
    pub fn raw(data: &str, version: QrVersion, ec_level: QrEccLevel) -> Result<Vec<u8>, JsValue> {
        let code: QrCode = Self::generate_manual(data, version, ec_level)?;
        Ok(Self::to_raw_bytes(&code))
    }

    /// `raw_auto` Generate raw QR code data with error correction level
    ///
    /// ### Arguments
    /// * `data` - The data to encode
    /// * `ec_level` - Error correction level (Low, Medium, Quartile, High)
    ///
    /// ### Returns
    /// Vec<u8> with format: [width, is_micro, module_data...]
    pub fn raw_auto(data: &str, ec_level: QrEccLevel) -> Result<Vec<u8>, JsValue> {
        let code = Self::generate_auto_version(data, ec_level)?;
        Ok(Self::to_raw_bytes(&code))
    }

    /// `raw_default` Generate raw QR code data with default settings
    ///
    /// ### Arguments
    /// * `data` - The data to encode
    ///
    /// ### Returns
    /// Vec<u8> with format: [width, is_micro, module_data...]
    pub fn raw_default(data: &str) -> Result<Vec<u8>, JsValue> {
        Self::raw_auto(data, QrEccLevel::Medium)
    }

    /// `text` Generate QR code to string format with custom version and error correction level
    ///
    /// ### Arguments
    /// * `data` - The data to encode
    /// * `version` - QR code version (Micro1-Micro4 or Normal1-Normal40)
    /// * `ec_level` - Error correction level (Low, Medium, Quartile, High)
    ///
    /// ### Returns
    /// String containing ASCII art representation of QR code (character-based rendering)
    pub fn text(data: &str, version: QrVersion, ec_level: QrEccLevel) -> Result<String, JsValue> {
        let code = Self::generate_manual(data, version, ec_level)?;
        Ok(Self::to_text_string(&code))
    }

    /// `text_auto` Generate QR code to string format with error correction level
    ///
    /// ### Arguments
    /// * `data` - The data to encode
    /// * `ec_level` - Error correction level (Low, Medium, Quartile, High)
    ///
    /// ### Returns
    /// String containing ASCII art representation of QR code (character-based rendering)
    pub fn text_auto(data: &str, ec_level: QrEccLevel) -> Result<String, JsValue> {
        let code = Self::generate_auto_version(data, ec_level)?;
        Ok(Self::to_text_string(&code))
    }

    /// `text_default` Generate QR code to string format with default settings
    ///
    /// ### Arguments
    /// * `data` - The data to encode
    ///
    /// ### Returns
    /// String containing ASCII art representation of QR code (character-based rendering)
    pub fn text_default(data: &str) -> Result<String, JsValue> {
        Self::text_auto(data, QrEccLevel::Medium)
    }
}

#[test]
fn qrcode_with_version_test() {
    let data = "hello, world!";

    let qr_raw = QRCode::raw_default(data).unwrap();
    println!(
        "QR raw default: width={}, is_micro={}",
        qr_raw[0], qr_raw[1]
    );
    assert!(qr_raw.len() > 2);

    let qr_text = QRCode::text_default(data).unwrap();
    println!("QR text default: {}", qr_text);
    assert!(qr_text.len() > 2);
}
