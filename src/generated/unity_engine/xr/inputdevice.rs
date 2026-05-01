
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/xr/inputdevice/InputDevice.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct InputDevice {
    pub m_device_id: u64,
    pub m_initialized: bool,
}

impl ::unity2::ClassIdentity for InputDevice {
    const NAMESPACE: &'static str = "UnityEngine.XR";

    const NAME: &'static str = "InputDevice";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for InputDevice {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-xr-inputdevice")]
#[::unity2::methods(value)]
impl InputDevice {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, device_id: u64) -> ();

    #[method(name = "get_deviceId", args = 0)]
    pub fn get_device_id(self) -> u64;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::xr::inputdevice::InputDevice) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
