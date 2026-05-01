
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitaccessory/UnitAccessory.md")))]
#[::unity2::class(namespace = "App", name = "UnitAccessory")]
#[parent(crate::system::object::Object)]
pub struct UnitAccessory {
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-unitaccessory")]
#[::unity2::methods]
impl UnitAccessory {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, accessory: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        p: crate::app::unitaccessory::UnitAccessory,
    ) -> crate::app::accessorydata::AccessoryData;

    #[method(name = "New", args = 1)]
    pub fn new(self, accessory: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = "New", args = 1)]
    pub fn new_2(self, aid: ::unity2::Il2CppString) -> ();

    #[method(name = "New", args = 1)]
    pub fn new_3(self, index: i32) -> ();

    #[method(name = "New", args = 1)]
    pub fn new_4(self, unit_accessory: crate::app::unitaccessory::UnitAccessory) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "IsEmpty", args = 0)]
    pub fn is_empty(self) -> bool;

    #[method(name = "IsExist", args = 0)]
    pub fn is_exist(self) -> bool;

    #[method(name = "GetData", args = 0)]
    pub fn get_data(self) -> crate::app::accessorydata::AccessoryData;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();
}
