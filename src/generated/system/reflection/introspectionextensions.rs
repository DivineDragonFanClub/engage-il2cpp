
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/introspectionextensions/IntrospectionExtensions.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "IntrospectionExtensions")]
#[parent(crate::system::object::Object)]
pub struct IntrospectionExtensions {}

#[cfg(feature = "system-reflection-introspectionextensions")]
#[::unity2::methods]
impl IntrospectionExtensions {
    #[method(name = "GetTypeInfo", args = 1)]
    pub fn get_type_info(
        r#type: ::unity2::SystemType,
    ) -> crate::system::reflection::typeinfo::TypeInfo;
}
