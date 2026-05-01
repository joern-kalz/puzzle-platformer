use image::ImageReader;

pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub _height: u32,
}

impl Image {
    pub fn new_from_asset(data: &[u8], width: u32, height: u32) -> Image {
        let data = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode sprite sheet");

        // Convert to RGBA bytes
        let data = data.to_rgba8().into_raw();

        Image { data, width, _height: height }
    }
}