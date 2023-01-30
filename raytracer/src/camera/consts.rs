#[derive(Copy, Clone)]
pub struct ImageResolution {
    pub width: usize,
    pub height: usize,
}

pub const HD: ImageResolution = ImageResolution {
    width: 1280,
    height: 720,
};

pub const FULL_HD: ImageResolution = ImageResolution {
    width: 1920,
    height: 1080,
};

pub const QHD: ImageResolution = ImageResolution {
    width: 2560,
    height: 1440,
};

pub const UHD: ImageResolution = ImageResolution {
    width: 3840,
    height: 2160,
};
