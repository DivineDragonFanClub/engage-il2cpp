
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::assembly::Assembly;
use crate::system::reflection::assembly::IAssembly;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/emit/assemblybuilder/AssemblyBuilder.md")))]
#[::unity2::class(namespace = "System.Reflection.Emit", name = "AssemblyBuilder")]
#[parent(crate::system::reflection::assembly::Assembly)]
pub struct AssemblyBuilder {}
