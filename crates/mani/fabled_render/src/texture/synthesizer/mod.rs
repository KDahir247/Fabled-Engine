mod synthesize;

use crate::{ColorType, Extent3d, Texture};
use image::EncodableLayout;
use texture_synthesis::session::ProgressUpdate;

pub struct ProgressStat {
    pub current: usize,
    pub total: usize,
}

impl From<texture_synthesis::session::ProgressStat> for ProgressStat {
    fn from(prog_stat: texture_synthesis::session::ProgressStat) -> Self {
        Self {
            current: prog_stat.current,
            total: prog_stat.total,
        }
    }
}

pub struct TextureSynProgress {
    pub image: Texture,
    pub total: ProgressStat,
    pub stage: ProgressStat,
}

impl texture_synthesis::session::GeneratorProgress for TextureSynProgress {
    fn update(&mut self, info: ProgressUpdate<'_>) {
        self.image = Texture {
            data: info.image.as_bytes().to_vec(),
            size: Extent3d {
                width: info.image.width(),
                height: info.image.height(),
                depth_or_array_layers: 1,
            },
            sample_count: 0,
            mip_level: 1,
            color_type: ColorType::Rgba8, // RGBA since info.image is of type rgba-image
            rows_per_image: info.image.width() * 4,
        };

        self.total = info.total.into();
        self.stage = info.stage.into();
    }
}

#[cfg(test)]
mod data_alignment_test {
    use crate::texture::synthesizer::synthesize::SessionBuilder;
    use crate::texture::synthesizer::{ProgressStat, TextureSynProgress};

    #[test]
    fn data_alignment() {
        let progress_stat = std::mem::size_of::<ProgressStat>();
        assert_eq!(progress_stat & (progress_stat - 1), 0);

        let texture_syn_progress = std::mem::size_of::<TextureSynProgress>();
        assert_eq!(texture_syn_progress & (texture_syn_progress - 1), 0);

        let session_builder = std::mem::size_of::<SessionBuilder>();
        assert_eq!(session_builder & (session_builder - 1), 0);
    }
}
