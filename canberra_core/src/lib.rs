#![recursion_limit = "10240"]

pub mod qml;
mod uuid;

pub use self::uuid::{
  uuid, Uuid
};