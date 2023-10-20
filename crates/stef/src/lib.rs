use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

pub use buf::{Buf, BufMut, Decode, Encode};

pub mod buf;
pub mod varint;

#[derive(Clone, Debug, PartialEq)]
pub struct NonZero<T>(T);

impl<T> NonZero<T> {
    /// ```
    /// let value = stef::NonZeroString::new("hello".to_owned()).unwrap();
    /// assert_eq!("hello", value.get());
    /// ```
    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for NonZero<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! non_zero_collection {
    ($name:ident $(< $($gens:tt),+ >)?) => {
        impl $(< $($gens),+ >)? NonZero<$name $(< $($gens),+ >)?> {
            /// Try to create a new non-zero instance, which will succeed if the given collection
            /// contains in fact some elements. Otherwise `None` is returned.
            pub fn new(value: $name $(< $($gens),+ >)?) -> Option<Self> {
                (!value.is_empty()).then_some(Self(value))
            }
        }
    };
}

non_zero_collection!(String);
non_zero_collection!(Vec<T>);
non_zero_collection!(HashMap<K, V>);
non_zero_collection!(HashSet<T>);

pub type NonZeroString = NonZero<String>;
pub type NonZeroBytes = NonZero<Vec<u8>>;
pub type NonZeroVec<T> = NonZero<Vec<T>>;
pub type NonZeroHashMap<K, V> = NonZero<HashMap<K, V>>;
pub type NonZeroHashSet<T> = NonZero<HashSet<T>>;
