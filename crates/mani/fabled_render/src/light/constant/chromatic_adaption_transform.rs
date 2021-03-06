pub const XYZ_SCALING: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

pub const VON_KRIES: [f32; 9] = [
    0.3897, 0.6890, -0.0787, -0.2298, 1.1834, 0.0464, 0.0, 0.0, 1.0,
];

pub const VON_KRIES_TRANSPOSE: [f32; 9] = [
    0.3897, -0.2298, 0.0, 0.6890, 1.1834, 0.0, -0.0787, 0.0464, 1.0,
];

pub const VON_KRIES_INVERSE: [f32; 9] =
    [1.9102, -1.1121, 0.2019, 0.3710, 0.6291, 0.0, 0.0, 0.0, 1.0];

pub const VON_KRIES_TRANSPOSE_INVERSE: [f32; 9] =
    [1.9102, 0.3710, 0.0, -1.1121, 0.6291, 0.0, 0.2019, 0.0, 1.0];


pub const BRADFORD: [f32; 9] = [
    0.8951, 0.2664, -0.1614, -0.7502, 1.7135, 0.0367, 0.0389, -0.0685, 1.0296,
];

pub const BRADFORD_TRANSPOSE: [f32; 9] = [
    0.8951, -0.7502, 0.0389, 0.2664, 1.7136, -0.0685, -0.1614, 0.0367, 1.0296,
];

pub const BRADFORD_INVERSE: [f32; 9] = [
    0.9870, -0.1471, 0.1600, 0.4323, 0.5184, 0.0493, -0.0085, 0.0400, 0.9685,
];

pub const BRADFORD_TRANSPOSE_INVERSE: [f32; 9] = [
    0.9870, 0.4323, -0.0085, -0.1471, 0.5184, 0.0400, 0.1600, 0.0493, 0.9685,
];


pub const CIECAM02: [f32; 9] = [
    0.7328, 0.4296, -0.1624, -0.7036, 1.6975, 0.0061, 0.0030, 0.0136, 0.9834,
];

pub const CIECAM02_TRANSPOSE: [f32; 9] = [
    0.7328, -0.7036, 0.0030, 0.4296, 1.6975, 0.0136, -0.1624, 0.0061, 0.9834,
];

pub const CIECAM02_INVERSE: [f32; 9] = [
    1.0961, -0.2789, 0.1827, 0.4544, 0.4735, 0.0721, -0.0096, -0.0057, 1.0153,
];

pub const CIECAM02_TRANSPOSE_INVERSE: [f32; 9] = [
    1.0961, 0.4544, -0.0096, -0.2789, 0.4735, -0.0057, 0.1827, 0.0721, 1.0153,
];
