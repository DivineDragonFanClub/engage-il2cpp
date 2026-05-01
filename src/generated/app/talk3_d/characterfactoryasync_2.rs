
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/characterfactoryasync_2/CharacterFactoryAsync_2.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "CharacterFactoryAsync")]
#[parent(crate::system::object::Object)]
pub struct CharacterFactoryAsync_2 {}

#[cfg(feature = "app-talk3_d-characterfactoryasync_2")]
#[::unity2::methods]
impl CharacterFactoryAsync_2 {
    #[method(name = "CreateForTalk", args = 4)]
    pub fn create_for_talk(
        pid: ::unity2::Il2CppString,
        pid_for_create: ::unity2::Il2CppString,
        locator: crate::unity_engine::gameobject::GameObject,
        use_talk_controller: bool,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateForTalk", args = 3)]
    pub fn create_for_talk_2(
        unit: crate::app::unit::Unit,
        locator: crate::unity_engine::gameobject::GameObject,
        use_talk_controller: bool,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateForTalk", args = 3)]
    pub fn create_for_talk_3(
        god_unit: crate::app::godunit::GodUnit,
        locator: crate::unity_engine::gameobject::GameObject,
        use_talk_controller: bool,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateForUnitInfo", args = 2)]
    pub fn create_for_unit_info(
        unit: crate::app::unit::Unit,
        locator: crate::unity_engine::gameobject::GameObject,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateForUnitInfo", args = 2)]
    pub fn create_for_unit_info_2(
        god: crate::app::godunit::GodUnit,
        locator: crate::unity_engine::gameobject::GameObject,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateForUnitHub", args = 2)]
    pub fn create_for_unit_hub(
        unit: crate::app::unit::Unit,
        locator: crate::unity_engine::gameobject::GameObject,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateForUnitRelay", args = 4)]
    pub fn create_for_unit_relay(
        locator: crate::unity_engine::gameobject::GameObject,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        edit: crate::app::unitedit::UnitEdit,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateForRingCleaning", args = 2)]
    pub fn create_for_ring_cleaning(
        unit: crate::app::unit::Unit,
        locator: crate::unity_engine::gameobject::GameObject,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateForRingCleaning", args = 2)]
    pub fn create_for_ring_cleaning_2(
        god_unit: crate::app::godunit::GodUnit,
        locator: crate::unity_engine::gameobject::GameObject,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateCommon", args = 6)]
    pub fn create_common(
        result: crate::app::assettable::AssetTable_Result,
        pid: ::unity2::Il2CppString,
        locator: crate::unity_engine::gameobject::GameObject,
        use_talk_controller: bool,
        invisible: bool,
        is_engage: bool,
    ) -> crate::combat::character::Character;

    #[method(name = "SetupLookAt", args = 2)]
    pub fn setup_look_at(
        chara: crate::combat::character::Character,
        locator: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete(locator: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-characterfactoryasync_2")]
impl CharacterFactoryAsync_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterFactoryAsync_2),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterFactoryAsync_2Methods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/characterfactoryasync_2/CharacterFactoryAsync_UnitStatusScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct CharacterFactoryAsync_UnitStatusScope {
    pub m_unit: crate::app::unit::Unit,
    pub m_value: i64,
}

impl ::unity2::ClassIdentity for CharacterFactoryAsync_UnitStatusScope {
    const NAMESPACE: &'static str = "App.Talk3D";

    const NAME: &'static str = "CharacterFactoryAsync.UnitStatusScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CharacterFactoryAsync_UnitStatusScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-talk3_d-characterfactoryasync_2")]
#[::unity2::methods(value)]
impl CharacterFactoryAsync_UnitStatusScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/characterfactoryasync_2/CharacterFactoryAsync_onLoad.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "CharacterFactoryAsync.onLoad")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct CharacterFactoryAsync_onLoad {}

#[cfg(feature = "app-talk3_d-characterfactoryasync_2")]
#[::unity2::methods]
impl CharacterFactoryAsync_onLoad {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, chara: crate::combat::character::Character) -> ();
}

#[cfg(feature = "app-talk3_d-characterfactoryasync_2")]
impl CharacterFactoryAsync_onLoad {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterFactoryAsync_onLoad),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterFactoryAsync_onLoadMethods>::ctor(this, object, method);
        this
    }
}
