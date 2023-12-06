use crate::non_max::{NonMaxU16, NonMaxU32, NonMaxU64, NonMaxU8, NonMaxUsize};
use core::{
    num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    ops::{Add, AddAssign},
};

macro_rules! impl_index {
    (@ $Index:ident, $NonMax:ident, $NonZero:ident, $Int:ty) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[repr(transparent)]
        pub struct $Index($NonMax);

        impl $Index {
            /// Creates an index.
            #[must_use]
            #[inline]
            pub const fn new(value: $Int) -> Self {
                match <$NonMax>::new(value) {
                    Some(inner) => Self(inner),
                    None => index_too_large(),
                }
            }

            /// Returns the value as a primitive type.
            #[inline]
            pub const fn get(self) -> $Int {
                self.0.get()
            }
        }

        impl From<usize> for $Index {
            #[inline]
            fn from(value: usize) -> Self {
                $Index::new(value.try_into().ok().unwrap_or_else(index_too_large))
            }
        }

        impl From<$Index> for usize {
            #[inline]
            fn from(value: $Index) -> Self {
                value.get().try_into().ok().unwrap_or_else(index_too_large)
            }
        }

        impl Add<$Int> for $Index {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $Int) -> Self::Output {
                // NOTE: Performing the addition on the underlying NonZero$Int which stores index + 1
                // directly eliminates `dec` and `inc` instructions.

                // NOTE: Performing the addition through `$Int::checked_add`, and unwrapping the result in
                // the same function, allows the compiler to not introduce two jumps (`jb` and `je`) and
                // only uses `jb`.
                Self($NonMax({
                    let Self($NonMax(lhs_inc)) = self;
                    let sum_inc = lhs_inc
                        .get()
                        .checked_add(rhs)
                        .unwrap_or_else(index_too_large);
                    // SAFETY: Adding `lhs_inc` (1..=MAX) and `rhs` (0..=MAX) will overflow, or result in
                    // `sum_inc` (1..=MAX). We panic in case of an overflow and so `sum_inc` (1..=MAX).
                    unsafe { $NonZero::new_unchecked(sum_inc) }
                }))
            }
        }

        forward_ref_binop! { impl Add, add for $Index, $Int }

        impl AddAssign<$Int> for $Index {
            #[inline]
            fn add_assign(&mut self, rhs: $Int) {
                *self = *self + rhs;
            }
        }

        forward_ref_op_assign! { impl AddAssign, add_assign for $Index, $Int }

        delegate_get_fmt! {
            (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Index
        }
    };
    ( $( $Index:ident, $NonMax:ident, $NonZero:ident, $Int:ty; )+ ) => {
        $(
            impl_index! { @ $Index, $NonMax, $NonZero, $Int }
        )*
    };
}

impl_index! {
    IndexU8, NonMaxU8, NonZeroU8, u8;
    IndexU16, NonMaxU16, NonZeroU16, u16;
    IndexU32, NonMaxU32, NonZeroU32, u32;
    IndexU64, NonMaxU64, NonZeroU64, u64;
    IndexUsize, NonMaxUsize, NonZeroUsize, usize;
}

#[cold]
const fn index_too_large<T>() -> T {
    panic!("index too large")
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_eq_size, const_assert, const_assert_eq};

    assert_eq_size!(Option<IndexU8>, u8);
    assert_eq_size!(Option<IndexU16>, u16);
    assert_eq_size!(Option<IndexU32>, u32);
    assert_eq_size!(Option<IndexU64>, u64);
    assert_eq_size!(Option<IndexUsize>, usize);

    #[allow(unused)]
    const fn non_max_u8(value: u8) -> NonMaxU8 {
        if let Some(value) = NonMaxU8::new(value) {
            value
        } else {
            panic!()
        }
    }

    macro_rules! test_index_new {
        ($n:expr) => {
            const_assert!({
                const RHS: NonMaxU8 = non_max_u8($n);
                matches!(IndexU8::new($n), IndexU8(RHS))
            });
        };
    }
    test_index_new!(0);
    test_index_new!(1);
    test_index_new!(u8::MAX - 1);

    macro_rules! test_index_get {
        ($n:expr) => {
            const_assert_eq!(IndexU8(non_max_u8($n)).get(), $n);
        };
    }
    test_index_get!(0);
    test_index_get!(1);
    test_index_get!(u8::MAX - 1);

    #[test]
    fn index_add_works() {
        assert_eq!(IndexU8::new(0) + 0, IndexU8::new(0));
        assert_eq!(IndexU8::new(0) + 1, IndexU8::new(1));
        assert_eq!(IndexU8::new(1) + 1, IndexU8::new(2));
        assert_eq!(IndexU8::new(0) + (u8::MAX - 1), IndexU8::new(u8::MAX - 1));
        assert_eq!(IndexU8::new(u8::MAX - 1) + 0, IndexU8::new(u8::MAX - 1));
    }

    #[test]
    #[should_panic(expected = "index too large")]
    fn index_add_to_max_panics() {
        _ = IndexU8::new(u8::MAX - 1) + 1;
    }

    #[test]
    #[should_panic(expected = "index too large")]
    fn index_add_past_max_panics() {
        _ = IndexU8::new(u8::MAX - 1) + 2;
    }
}
