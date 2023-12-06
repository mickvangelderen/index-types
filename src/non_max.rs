use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

macro_rules! impl_nonmax {
    ( @ $NonMax: ident, $NonZero: ty, $Int: ty ) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[repr(transparent)]
        pub struct $NonMax(pub $NonZero);

        impl $NonMax {
            /// Creates a non-max if the given value is not MAX.
            #[must_use]
            #[inline]
            pub const fn new(value: $Int) -> Option<Self> {
                match <$NonZero>::new(value.wrapping_add(1)) {
                    Some(inner) => Some(Self(inner)),
                    None => None,
                }
            }

            /// Returns the value as a primitive type.
            #[inline]
            pub const fn get(self) -> $Int {
                self.0.get().wrapping_sub(1)
            }
        }

        delegate_get_fmt! {
            (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $NonMax
        }
    };
    ($($NonMax:ident, $NonZero:ident, $Int:ty ;)+) => {
        $(
            impl_nonmax! { @ $NonMax, $NonZero, $Int }
        )*
    };
}

impl_nonmax! {
    NonMaxU8, NonZeroU8, u8;
    NonMaxU16, NonZeroU16, u16;
    NonMaxU32, NonZeroU32, u32;
    NonMaxU64, NonZeroU64, u64;
    NonMaxUsize, NonZeroUsize, usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_eq_size, const_assert, const_assert_eq};

    assert_eq_size!(Option<NonMaxU8>, u8);
    assert_eq_size!(Option<NonMaxU16>, u16);
    assert_eq_size!(Option<NonMaxU32>, u32);
    assert_eq_size!(Option<NonMaxU64>, u64);
    assert_eq_size!(Option<NonMaxUsize>, usize);

    #[allow(unused)]
    const fn non_zero_u8(value: u8) -> NonZeroU8 {
        if let Some(value) = NonZeroU8::new(value) {
            value
        } else {
            panic!()
        }
    }

    macro_rules! test_non_max_new_some {
        ($n:expr) => {
            const_assert!({
                const RHS: NonZeroU8 = non_zero_u8($n + 1);
                matches!(NonMaxU8::new($n), Some(NonMaxU8(RHS)))
            });
        };
    }
    test_non_max_new_some!(0);
    test_non_max_new_some!(1);
    test_non_max_new_some!(u8::MAX - 1);
    const_assert!(NonMaxU8::new(u8::MAX).is_none());

    macro_rules! test_non_max_get {
        ($n:expr) => {
            const_assert_eq!(NonMaxU8(non_zero_u8($n + 1)).get(), $n);
        };
    }
    test_non_max_get!(0);
    test_non_max_get!(1);
    test_non_max_get!(u8::MAX - 1);
}
