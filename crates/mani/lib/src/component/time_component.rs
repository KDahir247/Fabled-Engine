pub struct Application {
    pub time: std::time::Instant,
}

pub struct Time {
    pub last_frame: f64,
    pub delta: f64,
    pub application: Application,
}
