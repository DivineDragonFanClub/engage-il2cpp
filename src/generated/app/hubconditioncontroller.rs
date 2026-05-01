
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubconditioncontroller/HubConditionController.md")))]
#[::unity2::class(namespace = "App", name = "HubConditionController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubConditionController {
    #[rename(name = "m_conditionType")]
    pub m_condition_type: crate::app::hubutil::HubUtil_ConditionType,
    #[rename(name = "m_controlType")]
    pub m_control_type: crate::app::hubconditioncontroller::HubConditionController_ControlType,
    #[rename(name = "m_conditionFlag")]
    pub m_condition_flag: bool,
    #[rename(name = "m_isUseTargets")]
    pub m_is_use_targets: bool,
    #[rename(name = "m_targets")]
    pub m_targets: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_lods")]
    pub m_lods: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "app-hubconditioncontroller")]
#[::unity2::methods]
impl HubConditionController {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(self, active: bool) -> ();

    #[method(name = "Apply", args = 0)]
    pub fn apply(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubconditioncontroller")]
impl HubConditionController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubConditionController),
                ::core::stringify!(new),
            )
        });
        <Self as IHubConditionControllerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubconditioncontroller/HubConditionController_ControlType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubConditionController_ControlType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubConditionController_ControlType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubConditionController.ControlType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubConditionController_ControlType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubConditionController_ControlType {
    pub fn show() -> Self {
        Self { value: 0 }
    }

    pub fn hide() -> Self {
        Self { value: 1 }
    }
}
