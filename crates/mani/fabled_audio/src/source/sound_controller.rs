// todo test script going to remove

pub struct SpatialStandardSource {
    controller: rodio::SpatialSink,
}


impl SpatialStandardSource {
    pub fn new(controller: rodio::SpatialSink) -> Self {
        Self { controller }
    }

    pub fn stop(&self) {
        self.controller.stop();
    }

    pub fn pause(&self) {
        self.controller.pause()
    }

    pub fn resume(&self) {
        self.controller.play();
    }

    pub fn set_position(&mut self, target_position: [f32; 3]) {
        self.controller.set_emitter_position(target_position);
    }

    pub fn set_left_ear_position(&mut self, left_ear_position: [f32; 3]) {
        self.controller.set_left_ear_position(left_ear_position);
    }

    pub fn set_right_ear_position(&mut self, right_ear_position: [f32; 3]) {
        self.controller.set_right_ear_position(right_ear_position);
    }
}
