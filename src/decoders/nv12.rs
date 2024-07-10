use image::{ImageBuffer, Rgb};
use nokhwa_core::buffer::Buffer;
use nokhwa_core::decoder::{Decoder, IdemptDecoder, StaticDecoder};
use nokhwa_core::frame_format::SourceFrameFormat;

pub struct NV12Decoder {}

impl Decoder for NV12Decoder {
    const ALLOWED_FORMATS: &'static [SourceFrameFormat] = &[];
    type OutputPixels = Self::OutputPixels;
    type PixelContainer = Self::PixelContainer;
    type Error = ();

    fn decode(
        &mut self,
        buffer: Buffer,
    ) -> Result<ImageBuffer<Self::Pixel, Self::Container>, Self::Error> {
        todo!()
    }

    fn decode_buffer(&mut self, buffer: &mut [Pixel::Subpixel]) -> Result<(), Self::Error> {
        todo!()
    }

    fn predicted_size_of_frame(&mut self) -> Option<usize> {
        todo!()
    }
}

impl StaticDecoder for NV12Decoder {
    fn decode_static(
        buffer: Buffer,
    ) -> Result<ImageBuffer<Self::Pixel, Self::Container>, Self::Error> {
        todo!()
    }

    fn decode_static_to_buffer(buffer: &mut [Pixel::Subpixel]) -> Result<(), Self::Error> {
        todo!()
    }
}

impl IdemptDecoder for NV12Decoder {
    fn decode_nm(buffer: Buffer) -> Result<ImageBuffer<Self::Pixel, Self::Container>, Self::Error> {
        todo!()
    }

    fn decode_nm_to_buffer(
        &self,
        buffer: &mut [<<Self as Decoder>::OutputPixels as image::Pixel>::Subpixel],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
