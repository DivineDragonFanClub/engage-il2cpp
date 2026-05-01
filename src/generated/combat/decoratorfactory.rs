
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/decoratorfactory/DecoratorFactory.md")))]
#[::unity2::class(namespace = "Combat", name = "DecoratorFactory")]
#[parent(crate::system::object::Object)]
pub struct DecoratorFactory {}

#[cfg(feature = "combat-decoratorfactory")]
#[::unity2::methods]
impl DecoratorFactory {
    #[method(name = "CreateDecorators", args = 1)]
    pub fn create_decorators(
        arg: crate::combat::decoratorargs::DecoratorArgs,
    ) -> crate::combat::decorators::Decorators;
}
