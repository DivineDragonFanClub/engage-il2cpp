
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characternodeconstraint/CharacterNodeConstraint.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterNodeConstraint")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterNodeConstraint {
    #[rename(name = "state")]
    pub state: crate::combat::characternodeconstraint::CharacterNodeConstraint_State,
    #[rename(name = "SpecialPurposeCameraNodes")]
    pub special_purpose_camera_nodes: ::unity2::Array<crate::unity_engine::transform::Transform>,
    #[rename(name = "SpecialPurposeParticleNodes")]
    pub special_purpose_particle_nodes: ::unity2::Array<crate::unity_engine::transform::Transform>,
    #[rename(name = "specialPurposeParticleNodesOffset")]
    pub special_purpose_particle_nodes_offset:
        ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "_ctr")]
    pub ctr: crate::unity_engine::transform::Transform,
}

#[cfg(feature = "combat-characternodeconstraint")]
#[::unity2::methods]
impl CharacterNodeConstraint {
    #[method(name = "get_CTR", args = 0)]
    pub fn get_ctr(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "AllocOnce", args = 0)]
    pub fn alloc_once(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "SetNodeOffset", args = 2)]
    pub fn set_node_offset(
        self,
        node: crate::unity_engine::transform::Transform,
        offset: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characternodeconstraint")]
impl CharacterNodeConstraint {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterNodeConstraint),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterNodeConstraintMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characternodeconstraint/CharacterNodeConstraint_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CharacterNodeConstraint_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CharacterNodeConstraint_State {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "CharacterNodeConstraint.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CharacterNodeConstraint_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CharacterNodeConstraint_State {
    pub fn uninitialized() -> Self {
        Self { value: 0 }
    }

    pub fn no_use() -> Self {
        Self { value: 1 }
    }

    pub fn running() -> Self {
        Self { value: 2 }
    }
}
