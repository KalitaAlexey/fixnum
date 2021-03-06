use core::result::Result;

use parity_scale_codec::{Compact, CompactAs, Decode, Encode, EncodeLike, Error, Input, Output};
use static_assertions::{assert_eq_align, assert_eq_size};

use crate::FixedPoint;

impl<I, P> From<Compact<Self>> for FixedPoint<I, P> {
    fn from(value: Compact<Self>) -> Self {
        value.0
    }
}

macro_rules! impl_codec {
    ($layout:ty, $representation:ty) => {
        impl<P> EncodeLike for FixedPoint<$layout, P> {}

        impl<P> Encode for FixedPoint<$layout, P> {
            fn encode_to<O: Output>(&self, destination: &mut O) {
                destination.push(self.encode_as());
            }
        }

        impl<P> Decode for FixedPoint<$layout, P> {
            fn decode<In: Input>(input: &mut In) -> Result<Self, Error> {
                let raw = <$representation as Decode>::decode(input)
                    .map_err(|_| "Error decoding FixedPoint value")?;
                Ok(FixedPoint::decode_from(raw))
            }
        }

        impl<P> CompactAs for FixedPoint<$layout, P> {
            type As = $representation;

            fn encode_as(&self) -> &Self::As {
                assert_eq_size!($layout, $representation);
                assert_eq_align!($layout, $representation);
                // Representative type has the same size and memory layout so this cast is actually
                // safe.
                // TODO: Related issue: https://github.com/paritytech/parity-scale-codec/issues/205
                unsafe { &*(self.as_bits() as *const $layout as *const $representation) }
            }

            fn decode_from(value: Self::As) -> Self {
                Self::from_bits(value as $layout)
            }
        }
    };
}

impl_codec!(i16, u16);
impl_codec!(i32, u32);
impl_codec!(i64, u64);
#[cfg(feature = "i128")]
impl_codec!(i128, u128);
