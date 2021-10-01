// You shouldn't have to directly call method from both Raw struct.
// todo should the audio effect start here?

pub struct RawClip<T>
where
    T: rodio::Source,
    T::Item: rodio::Sample, {
    data: T,
}

impl<T> AsRef<T> for RawClip<T>
where
    T: rodio::Source,
    T::Item: rodio::Sample,
{
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> RawClip<T>
where
    T: rodio::Source,
    T::Item: rodio::Sample,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn get(self) -> T {
        self.data
    }
}

pub struct RawAmbisonicClip<T>
where
    T: ambisonic::rodio::Source,
    T::Item: ambisonic::rodio::Sample, {
    data: T,
}

impl<T> AsRef<T> for RawAmbisonicClip<T>
where
    T: ambisonic::rodio::Source,
    T::Item: ambisonic::rodio::Sample,
{
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> RawAmbisonicClip<T>
where
    T: ambisonic::rodio::Source,
    T::Item: ambisonic::rodio::Sample,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn get(self) -> T {
        self.data
    }
}
