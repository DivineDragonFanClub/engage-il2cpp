
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubfasttravel/HubFastTravel.md")))]
#[::unity2::class(namespace = "App", name = "HubFastTravel")]
#[parent(crate::system::object::Object)]
pub struct HubFastTravel {
    #[rename(name = "m_location")]
    pub m_location: crate::system::collections::generic::list_1::List_1<
        crate::app::hubfasttravel::HubFastTravel_Location,
    >,
    #[rename(name = "m_currentAccessManager")]
    pub m_current_access_manager: crate::app::hubaccessmanager::HubAccessManager,
}

#[cfg(feature = "app-hubfasttravel")]
#[::unity2::methods]
impl HubFastTravel {
    #[method(name = "get_Length", args = 0)]
    pub fn get_length(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::app::hubfasttravel::HubFastTravel_Location;

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, access_manager: crate::app::hubaccessmanager::HubAccessManager) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubfasttravel")]
impl HubFastTravel {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubFastTravel),
                ::core::stringify!(new),
            )
        });
        <Self as IHubFastTravelMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubfasttravel/HubFastTravel_Location.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HubFastTravel_Location {
    pub area: ::unity2::Il2CppString,
    pub pid: ::unity2::Il2CppString,
    pub access: crate::app::hubaccessdata::HubAccessData,
}

impl ::unity2::ClassIdentity for HubFastTravel_Location {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubFastTravel.Location";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubFastTravel_Location {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
