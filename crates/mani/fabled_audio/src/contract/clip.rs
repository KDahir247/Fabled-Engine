pub trait Clip {
    fn to_buffer(&self) -> rodio::buffer::SamplesBuffer<i16>;

    fn to_ambisonic_buffer(
        &self,
    ) -> ambisonic::rodio::source::SamplesConverter<ambisonic::rodio::buffer::SamplesBuffer<i16>, f32>;
}
