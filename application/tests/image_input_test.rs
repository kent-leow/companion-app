use std::fs;
use tempfile::TempDir;

#[test]
fn image_file_to_base64() {
    let dir = TempDir::new().unwrap();
    let img_path = dir.path().join("test.png");
    // Write a minimal 1x1 PNG (67 bytes)
    let png_bytes: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
        0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41,
        0x54, 0x08, 0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00,
        0x00, 0x00, 0x02, 0x00, 0x01, 0xE2, 0x21, 0xBC,
        0x33, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E,
        0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    fs::write(&img_path, &png_bytes).unwrap();

    let data = fs::read(&img_path).unwrap();
    let b64 = base64_encode(&data);
    let data_uri = format!("data:image/png;base64,{}", b64);

    assert!(data_uri.starts_with("data:image/png;base64,"));
    assert!(!b64.is_empty());
}

#[test]
fn multimodal_message_format() {
    let data_uri = "data:image/png;base64,iVBORw0KGgo=";
    let msg = serde_json::json!({
        "role": "user",
        "content": [
            {"type": "text", "text": "describe this image"},
            {"type": "image_url", "image_url": {"url": data_uri}}
        ]
    });

    assert_eq!(msg["content"][0]["type"], "text");
    assert_eq!(msg["content"][1]["type"], "image_url");
    assert!(msg["content"][1]["image_url"]["url"]
        .as_str()
        .unwrap()
        .starts_with("data:image/png;base64,"));
}

#[test]
fn detect_image_extension() {
    let extensions = vec!["png", "jpg", "jpeg", "gif", "webp"];
    for ext in extensions {
        let path = format!("screenshot.{}", ext);
        let is_image = path.ends_with(".png")
            || path.ends_with(".jpg")
            || path.ends_with(".jpeg")
            || path.ends_with(".gif")
            || path.ends_with(".webp");
        assert!(is_image, "should detect {} as image", ext);
    }
}

fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}
