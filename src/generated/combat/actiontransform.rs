
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actiontransform/ActionTransform.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionTransform")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionTransform {
    #[rename(name = "m_DoPlayer")]
    pub m_do_player: bool,
    #[rename(name = "m_DoEnemy")]
    pub m_do_enemy: bool,
    #[rename(name = "m_State")]
    pub m_state: crate::combat::actiontransform::ActionTransform_State,
}

#[cfg(feature = "combat-actiontransform")]
#[::unity2::methods]
impl ActionTransform {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, do_player: bool, do_enemy: bool) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Transform", args = 0)]
    pub fn transform(self) -> crate::system::collections::ienumerator::IEnumerator;
}

#[cfg(feature = "combat-actiontransform")]
impl ActionTransform {
    pub fn new(do_player: bool, do_enemy: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionTransform),
                ::core::stringify!(new),
            )
        });
        <Self as IActionTransformMethods>::ctor(this, do_player, do_enemy);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actiontransform/ActionTransform_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ActionTransform_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ActionTransform_State {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "ActionTransform.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ActionTransform_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ActionTransform_State {
    pub fn begin() -> Self {
        Self { value: 0 }
    }

    pub fn transform() -> Self {
        Self { value: 1 }
    }

    pub fn finish() -> Self {
        Self { value: 2 }
    }
}
