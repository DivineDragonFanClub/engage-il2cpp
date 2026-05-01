
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/keyhelpdata/KeyHelpData.md")))]
#[::unity2::class(namespace = "App", name = "KeyHelpData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: keyhelpdata :: KeyHelpData >)]
pub struct KeyHelpData {}

#[cfg(feature = "app-keyhelpdata")]
#[::unity2::methods]
impl KeyHelpData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_ButtonIndex", args = 0)]
    pub fn get_button_index(self) -> i8;

    #[method(name = "set_ButtonIndex", args = 1)]
    pub fn set_button_index(self, value: i8) -> ();

    #[method(name = "get_MID", args = 0)]
    pub fn get_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MID", args = 1)]
    pub fn set_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-keyhelpdata")]
impl KeyHelpData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KeyHelpData),
                ::core::stringify!(new),
            )
        });
        <Self as IKeyHelpDataMethods>::ctor(this);
        this
    }
}
