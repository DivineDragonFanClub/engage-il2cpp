
use crate::app::savedatahandle::ISaveDataHandle;
use crate::app::savedatahandle::SaveDataHandle;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/savedatareadhandle/SaveDataReadHandle.md")))]
#[::unity2::class(namespace = "App", name = "SaveDataReadHandle")]
#[parent(crate::app::savedatahandle::SaveDataHandle)]
pub struct SaveDataReadHandle {}

#[cfg(feature = "app-savedatareadhandle")]
#[::unity2::methods]
impl SaveDataReadHandle {
    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> ::unity2::Array<u8>;

    #[method(name = "set_Data", args = 1)]
    pub fn set_data(self, value: ::unity2::Array<u8>) -> ();

    #[method(name = "get_ReadSize", args = 0)]
    pub fn get_read_size(self) -> i64;

    #[method(name = "set_ReadSize", args = 1)]
    pub fn set_read_size(self, value: i64) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-savedatareadhandle")]
impl SaveDataReadHandle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SaveDataReadHandle),
                ::core::stringify!(new),
            )
        });
        <Self as ISaveDataReadHandleMethods>::ctor(this);
        this
    }
}
