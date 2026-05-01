
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/uniticonviewercontent/UnitIconViewerContent.md")))]
#[::unity2::class(namespace = "App", name = "UnitIconViewerContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct UnitIconViewerContent {
    #[rename(name = "WEAPON_NAMES")]
    pub weapon_names: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "GOD_NAMES")]
    pub god_names: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_JobObject")]
    pub m_job_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_EmblemObject")]
    pub m_emblem_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NameDictionary")]
    pub m_name_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >,
}

#[cfg(feature = "app-uniticonviewercontent")]
#[::unity2::methods]
impl UnitIconViewerContent {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetUnitIcon", args = 3)]
    pub fn set_unit_icon(
        self,
        person: crate::app::persondata::PersonData,
        is_female: bool,
        item_kind: crate::app::itemdata::ItemData_Kinds,
    ) -> ();

    #[method(name = "SetEmblemIcon", args = 1)]
    pub fn set_emblem_icon(self, is_female: bool) -> ();

    #[method(name = "ChangeR", args = 1)]
    pub fn change_r(self, change_value: f32) -> ();

    #[method(name = "ChangeG", args = 1)]
    pub fn change_g(self, change_value: f32) -> ();

    #[method(name = "ChangeB", args = 1)]
    pub fn change_b(self, change_value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-uniticonviewercontent")]
impl UnitIconViewerContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitIconViewerContent),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitIconViewerContentMethods>::ctor(this);
        this
    }
}
