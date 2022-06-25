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
        let [pos_x, pos_y, pos_z] = target_position;

        let sum = pos_x + pos_y + pos_z;

        if sum == 0.0 {
            let offset_x = pos_x + pos_x.signum() * f32::EPSILON;
            let offset_y = pos_y + pos_y.signum() * f32::EPSILON;
            let offset_z = pos_z + pos_z.signum() * f32::EPSILON;

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
