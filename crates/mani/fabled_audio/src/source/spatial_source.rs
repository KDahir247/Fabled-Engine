// Small wrapper over ambisonic::SoundController function calls

pub struct SpatialAmbisonicSource {
    source: ambisonic::SoundController,
}

impl SpatialAmbisonicSource {
    pub fn new(controller: ambisonic::SoundController) -> Self {
        Self { source: controller }
    }

    pub fn stop(&self) {
        self.source.stop();
    }

    pub fn pause(&self) {
        self.source.pause();
    }

    pub fn resume(&self) {
        self.source.resume();
    }

    pub fn set_position(&mut self, target_position: [f32; 3]) {
        // Handle getting all zero values for target position, which will result in NaN
        let target_position = [
            target_position[0] + target_position[0].signum() * f32::EPSILON,
            target_position[1] + target_position[1].signum() * f32::EPSILON,
            target_position[2] + target_position[2].signum() * f32::EPSILON,
        ];

        self.source.adjust_position(target_position);
    }

    pub fn set_doppler(&mut self, factor: f32) {
        self.source.set_doppler_factor(factor);
    }

    pub fn set_velocity(&mut self, target_velocity: [f32; 3]) {
        self.source.set_velocity(target_velocity);
    }
}
