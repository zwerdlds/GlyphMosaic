#![feature(test)]
pub mod commands;
pub mod document;
pub mod point;

pub mod prelude
{
    pub use crate::{
        document::{
            image::DocumentImage,
            serialization::*,
            Document,
        },
        point::GenericPoint,
    };
}
