
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/charactercollision/CharacterCollision.md")))]
#[::unity2::class(namespace = "App", name = "CharacterCollision")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterCollision {
    #[rename(name = "m_Kinds")]
    pub m_kinds: crate::app::charactercollision::CharacterCollision_Kinds,
    #[rename(name = "m_Color")]
    pub m_color: crate::unity_engine::color::Color,
    #[rename(name = "m_Radius")]
    pub m_radius: f32,
    #[rename(name = "m_Result")]
    pub m_result: f32,
}

#[cfg(feature = "app-charactercollision")]
#[::unity2::methods]
impl CharacterCollision {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind(self) -> crate::app::charactercollision::CharacterCollision_Kinds;

    #[method(name = "GetRadius", args = 0)]
    pub fn get_radius(self) -> f32;

    #[method(name = "SetOffset", args = 2)]
    pub fn set_offset(self, move_offset: f32, size_offset: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-charactercollision")]
impl CharacterCollision {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterCollision),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterCollisionMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/charactercollision/CharacterCollision_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CharacterCollision_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CharacterCollision_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "CharacterCollision.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CharacterCollision_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CharacterCollision_Kinds {
    pub fn _unnamed() -> Self {
        Self { value: 0 }
    }
}
