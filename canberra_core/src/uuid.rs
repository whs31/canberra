#![allow(clippy::transmute_ptr_to_ref)]

use std::ops::{Deref, DerefMut};
use std::str::FromStr;
pub use uuid::uuid;

#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  Hash,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  serde::Serialize,
  serde::Deserialize,
  qmetaobject::QGadget,
)]
pub struct Uuid {
  inner: uuid::Uuid,
}

impl Deref for Uuid {
  type Target = uuid::Uuid;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl DerefMut for Uuid {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

impl From<uuid::Uuid> for Uuid {
  fn from(value: uuid::Uuid) -> Self {
    Self { inner: value }
  }
}

impl From<Uuid> for uuid::Uuid {
  fn from(value: Uuid) -> Self {
    value.inner
  }
}

impl FromStr for Uuid {
  type Err = uuid::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    uuid::Uuid::from_str(s).map(|inner| Self { inner })
  }
}

impl Uuid {
  pub const fn nil() -> Self {
    Self {
      inner: uuid::Uuid::nil(),
    }
  }

  pub const fn max() -> Self {
    Self {
      inner: uuid::Uuid::max(),
    }
  }

  pub const fn from_fields(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
    Self {
      inner: uuid::Uuid::from_fields(d1, d2, d3, d4),
    }
  }

  pub const fn from_fields_le(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
    Self {
      inner: uuid::Uuid::from_fields_le(d1, d2, d3, d4),
    }
  }

  pub const fn from_u128(v: u128) -> Self {
    Self {
      inner: uuid::Uuid::from_u128(v),
    }
  }

  pub const fn from_u128_le(v: u128) -> Self {
    Self {
      inner: uuid::Uuid::from_u128_le(v),
    }
  }

  pub const fn from_u64_pair(high_bits: u64, low_bits: u64) -> Self {
    Self {
      inner: uuid::Uuid::from_u64_pair(high_bits, low_bits),
    }
  }

  pub fn from_slice(b: &[u8]) -> Result<Self, uuid::Error> {
    uuid::Uuid::from_slice(b).map(|inner| Self { inner })
  }

  pub fn from_slice_le(b: &[u8]) -> Result<Self, uuid::Error> {
    uuid::Uuid::from_slice_le(b).map(|inner| Self { inner })
  }

  pub const fn from_bytes(bytes: uuid::Bytes) -> Self {
    Self {
      inner: uuid::Uuid::from_bytes(bytes),
    }
  }

  pub const fn from_bytes_le(bytes: uuid::Bytes) -> Self {
    Self {
      inner: uuid::Uuid::from_bytes_le(bytes),
    }
  }

  pub fn parse_str(input: &str) -> Result<Self, uuid::Error> {
    uuid::Uuid::parse_str(input).map(|inner| Self { inner })
  }

  pub fn try_parse(input: &str) -> Result<Self, uuid::Error> {
    uuid::Uuid::try_parse(input).map(|inner| Self { inner })
  }

  pub fn try_parse_ascii(input: &[u8]) -> Result<Self, uuid::Error> {
    uuid::Uuid::try_parse_ascii(input).map(|inner| Self { inner })
  }

  pub fn new_v4() -> Self {
    Self {
      inner: uuid::Uuid::new_v4(),
    }
  }
}
