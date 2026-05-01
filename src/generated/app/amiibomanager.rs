
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibomanager/AmiiboManager_AmiiboInfo.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboManager.AmiiboInfo")]
#[parent(crate::system::object::Object)]
pub struct AmiiboManager_AmiiboInfo {}

#[cfg(feature = "app-amiibomanager")]
#[::unity2::methods]
impl AmiiboManager_AmiiboInfo {
    #[method(name = "GetCharacterId", args = 0)]
    pub fn get_character_id(self) -> i32;

    #[method(name = "GetCharacterBaseId", args = 0)]
    pub fn get_character_base_id(self) -> i32;

    #[method(name = "GetCharacterBaseUpperId", args = 0)]
    pub fn get_character_base_upper_id(self) -> u8;

    #[method(name = "GetCharacterBaseLowerId", args = 0)]
    pub fn get_character_base_lower_id(self) -> u8;

    #[method(name = "GetCharacterDetailId", args = 0)]
    pub fn get_character_detail_id(self) -> i32;

    #[method(name = "GetSeriesId", args = 0)]
    pub fn get_series_id(self) -> i32;

    #[method(name = "GetNumberingId", args = 0)]
    pub fn get_numbering_id(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-amiibomanager")]
impl AmiiboManager_AmiiboInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboManager_AmiiboInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboManager_AmiiboInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibomanager/AmiiboManager_Sequence.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AmiiboManager_Sequence {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AmiiboManager_Sequence {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AmiiboManager.Sequence";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AmiiboManager_Sequence {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AmiiboManager_Sequence {
    pub fn sequence_none() -> Self {
        Self { value: 0 }
    }

    pub fn sequence_init() -> Self {
        Self { value: 1 }
    }

    pub fn sequence_search() -> Self {
        Self { value: 2 }
    }

    pub fn sequence_active() -> Self {
        Self { value: 3 }
    }

    pub fn sequence_mount() -> Self {
        Self { value: 4 }
    }

    pub fn sequence_deactive() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibomanager/AmiiboManager.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboManager")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: amiibomanager :: AmiiboManager >)]
pub struct AmiiboManager {
    #[static_field]
    #[rename(name = "DeviceCountMax")]
    pub device_count_max: i32,
    #[rename(name = "m_DeviceCount")]
    pub m_device_count: i32,
    #[rename(name = "m_SelectDeviceIndex")]
    pub m_select_device_index: i32,
    #[rename(name = "m_Sequence")]
    pub m_sequence: crate::app::amiibomanager::AmiiboManager_Sequence,
    #[rename(name = "m_ActivateEventIndex")]
    pub m_activate_event_index: u32,
    #[rename(name = "m_DeactivateEventIndex")]
    pub m_deactivate_event_index: u32,
    #[rename(name = "m_AttachedEvent")]
    pub m_attached_event: bool,
}

#[cfg(feature = "app-amiibomanager")]
#[::unity2::methods]
impl AmiiboManager {
    #[method(name = "get_ReceiveCountMax", args = 0)]
    pub fn get_receive_count_max() -> i32;

    #[method(name = "set_ReceiveCountMax", args = 1)]
    pub fn set_receive_count_max(value: i32) -> ();

    #[method(name = "get_ReceiveFEAmiiboCountMax", args = 0)]
    pub fn get_receive_fe_amiibo_count_max() -> i32;

    #[method(name = "set_ReceiveFEAmiiboCountMax", args = 1)]
    pub fn set_receive_fe_amiibo_count_max(value: i32) -> ();

    #[method(name = "InitializeSystem", args = 0)]
    pub fn initialize_system(self) -> ();

    #[method(name = "FinalizeSystem", args = 0)]
    pub fn finalize_system(self) -> ();

    #[method(name = "DestroyEvent", args = 0)]
    pub fn destroy_event(self) -> ();

    #[method(name = "IsAvailabilityChanged", args = 0)]
    pub fn is_availability_changed(self) -> bool;

    #[method(name = "IsActivatedDevice", args = 0)]
    pub fn is_activated_device(self) -> bool;

    #[method(name = "IsDeactivatedDevice", args = 0)]
    pub fn is_deactivated_device(self) -> bool;

    #[method(name = "SelectDevice", args = 1)]
    pub fn select_device(self, index: i32) -> bool;

    #[method(name = "GetSelectDeviceIndex", args = 0)]
    pub fn get_select_device_index(self) -> i32;

    #[method(name = "GetDeviceCount", args = 0)]
    pub fn get_device_count(self) -> i32;

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-amiibomanager")]
impl AmiiboManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboManager),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboManagerMethods>::ctor(this);
        this
    }
}
