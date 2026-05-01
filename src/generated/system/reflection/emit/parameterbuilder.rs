
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/emit/parameterbuilder/ParameterBuilder.md")))]
#[::unity2::class(namespace = "System.Reflection.Emit", name = "ParameterBuilder")]
#[parent(crate::system::object::Object)]
pub struct ParameterBuilder {}
