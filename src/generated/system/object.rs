
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/object/Object.md")))]
#[::unity2::class(namespace = "System", name = "Object")]
pub struct Object {}

#[cfg(feature = "system-object")]
#[::unity2::methods]
impl Object {
    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 2)]
    pub fn equals_2(
        obj_a: crate::system::object::Object,
        obj_b: crate::system::object::Object,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "GetType", args = 0)]
    pub fn get_type(self) -> ::unity2::SystemType;

    #[method(name = "MemberwiseClone", args = 0)]
    pub fn memberwise_clone(self) -> crate::system::object::Object;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "InternalGetHashCode", args = 1)]
    pub fn internal_get_hash_code(o: crate::system::object::Object) -> i32;

    #[method(name = "FieldGetter", args = 3)]
    pub fn field_getter(
        self,
        type_name: ::unity2::Il2CppString,
        field_name: ::unity2::Il2CppString,
        val: ::unity2::IlInstance,
    ) -> ();

    #[method(name = "FieldSetter", args = 3)]
    pub fn field_setter(
        self,
        type_name: ::unity2::Il2CppString,
        field_name: ::unity2::Il2CppString,
        val: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "system-object")]
impl Object {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Object),
                ::core::stringify!(new),
            )
        });
        <Self as IObjectMethods>::ctor(this);
        this
    }
}
