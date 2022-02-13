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

    pub fn set_position(&mut self, mut target_position: [f32; 3]) {
        let sum = target_position[0] + target_position[1] + target_position[2];

        if sum == 0.0 {
            let offset_x = target_position[0] + target_position[0].signum() * f32::EPSILON;
            let offset_y = target_position[1] + target_position[1].signum() * f32::EPSILON;
            let offset_z = target_position[2] + target_position[2].signum() * f32::EPSILON;

            target_position = [offset_x, offset_y, offset_z];
        }

        self.source.adjust_position(target_position);
    }

    pub fn set_doppler(&mut self, factor: f32) {
        self.source.set_doppler_factor(factor);
    }

    pub fn set_velocity(&mut self, target_velocity: [f32; 3]) {
        self.source.set_velocity(target_velocity);
    }
}
