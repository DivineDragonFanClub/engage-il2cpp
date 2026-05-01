
use crate::app::struct_object::basepiece::BasePiece;
use crate::app::struct_object::basepiece::IBasePiece;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/struct_object/baseitem/BaseItem.md")))]
#[::unity2::class(namespace = "App.StructObject", name = "BaseItem")]
#[parent(crate::app::struct_object::basepiece::BasePiece)]
pub struct BaseItem {}

#[cfg(feature = "app-struct_object-baseitem")]
#[::unity2::methods]
impl BaseItem {
    #[method(name = "get_DictionaryKey", args = 0)]
    pub fn get_dictionary_key(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DictionaryKey", args = 1)]
    pub fn set_dictionary_key(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "AddPiece", args = 1)]
    pub fn add_piece(self, obj: crate::system::object::Object) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-struct_object-baseitem")]
impl BaseItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseItem),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseItemMethods>::ctor(this);
        this
    }
}
