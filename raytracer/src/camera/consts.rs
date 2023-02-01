/// Image resolution with width and height in number of pixels.
#[derive(Copy, Clone)]
pub struct ImageResolution {
    pub width: usize,
    pub height: usize,
}

/// HD resolution in 16:9 aspect ratio.
pub const HD: ImageResolution = ImageResolution {
    width: 1280,
    height: 720,
};

/// FullHD resolution in 16:9 aspect ratio.
pub const FULL_HD: ImageResolution = ImageResolution {
    width: 1920,
    height: 1080,
};

/// QHD resolution in 16:9 aspect ratio.
pub const QHD: ImageResolution = ImageResolution {
    width: 2560,
    height: 1440,
};

/// UHD resolution in 16:9 aspect ratio.
pub const UHD: ImageResolution = ImageResolution {
    width: 3840,
    height: 2160,
};
