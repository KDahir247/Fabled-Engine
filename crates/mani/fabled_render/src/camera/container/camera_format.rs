#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CameraFormat {
    CustomFormat,
    // D1 NTSC,
    // Usage : Digital Movie
    // Aspect Ratio : 4:3
    // Resolution 720 x 486 (MPEG2)
    // Dots : 720
    // Lines : 486
    // Pixels : 349,920
    D1NTSC,
    // NTSC
    // Usage : Analog TV
    // Aspect Ratio : 4:3
    // Resolution : 440 x 486
    // Dots : 440
    // Lines : 486
    // Pixels : 213,840
    NTSC,
    // PAL
    // Usage : Analog TV
    // Aspect Ratio : 4:3
    // Resolution : 520 x 576
    // Dots : 520
    // Lines : 576
    // Pixels : 299,520
    PAL,
    // D1 PAL
    // Usage : Digital Movie
    // Aspect Ratio : 4:3
    // Resolution : 720 x 576
    // Dots : 720
    // Lines : 576
    // Pixels : 414,720
    D1PAL,
    // HD (HDTV)
    // Usage : Computer Monitors
    // Aspect Ratio : 16:9
    // Resolution : 1366 x 768
    // Width (px) : 1366
    // Height (px) : 768
    // Pixels : 1,049,088
    HD,
    // 480
    // Usage : Camera and Video format
    // Aspect Ratio : 4:3
    // Resolution : 640 x 480
    // Width (px) : 640
    // Height (px) : 480
    // Pixels : 307,200
    R640x480,

    // 200
    // Usage : VGA Computer Monitors
    // Aspect Ratio : 4:3
    // Resolution : 320 x 200
    // Width (px) : 320
    // Height (px) : 200
    // Pixels : 64,000
    R320x200,

    // 240
    // Usage : CRT Monitors
    // Aspect Ratio : 4:3
    // Resolution : 320x240
    // Width (px) : 320
    // Height (px) : 240
    // Pixels : 76,800
    R320x240,
    // 128
    // Usage : Embedded Monitors
    // Aspect Ratio : 1/1
    // Resolution : 128 x 128
    // Width (px) : 128
    // Height (px) : 128
    // Pixels : 16,384
    R128x128,
    // User Monitor configuration
    FullScreen,
}
