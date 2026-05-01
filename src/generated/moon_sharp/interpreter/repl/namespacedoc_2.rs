
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/repl/namespacedoc_2/NamespaceDoc_2.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.REPL", name = "NamespaceDoc")]
#[parent(crate::system::object::Object)]
pub struct NamespaceDoc_2 {}
