// You shouldn't have to directly call method from both Raw struct.
// todo should the audio effect start here?

pub struct RawClip<T: rodio::Source<Item = f32>> {
    data: T,
}

impl<T> AsRef<T> for RawClip<T>
where
    T: rodio::Source<Item = f32>,
{
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> RawClip<T>
where
    T: rodio::Source<Item = f32>,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn get(self) -> T {
        self.data
    }
}

pub struct RawAmbisonicClip<T: ambisonic::rodio::Source<Item = f32>> {
    data: T,
}

impl<T> AsRef<T> for RawAmbisonicClip<T>
where
    T: ambisonic::rodio::Source<Item = f32>,
{
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> RawAmbisonicClip<T>
where
    T: ambisonic::rodio::Source<Item = f32>,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn get(self) -> T {
        self.data
    }
}
