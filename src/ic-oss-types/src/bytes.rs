use candid::CandidType;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteArray;
use std::ops::Deref;

/// ByteN<N> is a wrapper around ByteArray<N> to provide CandidType implementation
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteN<const N: usize>(pub ByteArray<N>);

impl<const N: usize> ByteN<N> {
    pub fn from_hex(val: &str) -> Result<Self, String> {
        let data = hex::decode(val).map_err(|_| format!("failed to decode hex: {}", val))?;
        Self::try_from(data.as_slice())
    }
}

impl<const N: usize> Default for ByteN<N> {
    fn default() -> Self {
        Self(ByteArray::new([0u8; N]))
    }
}

impl<const N: usize> CandidType for ByteN<N> {
    fn _ty() -> candid::types::internal::Type {
        candid::types::internal::TypeInner::Vec(candid::types::internal::TypeInner::Nat8.into())
            .into()
    }
    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        serializer.serialize_blob(self.0.as_slice())
    }
}

impl<const N: usize> Deref for ByteN<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> AsRef<[u8; N]> for ByteN<N> {
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> From<[u8; N]> for ByteN<N> {
    fn from(val: [u8; N]) -> Self {
        Self(ByteArray::new(val))
    }
}

impl<const N: usize> TryFrom<&[u8]> for ByteN<N> {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != N {
            return Err(format!("expected {} bytes, got {}", N, value.len()));
        }
        let mut bytes = [0u8; N];
        bytes.copy_from_slice(value);
        Ok(Self(ByteArray::new(bytes)))
    }
}

impl<const N: usize> From<ByteArray<N>> for ByteN<N> {
    fn from(val: ByteArray<N>) -> Self {
        Self(val)
    }
}

impl<const N: usize> From<ByteN<N>> for ByteArray<N> {
    fn from(val: ByteN<N>) -> Self {
        val.0
    }
}
