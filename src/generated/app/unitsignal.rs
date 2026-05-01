
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitsignal/UnitSignal_Func.md")))]
#[::unity2::class(namespace = "App", name = "UnitSignal.Func")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitSignal_Func {}

#[cfg(feature = "app-unitsignal")]
#[::unity2::methods]
impl UnitSignal_Func {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, signal: crate::app::unitsignal::UnitSignal) -> ();
}

#[cfg(feature = "app-unitsignal")]
impl UnitSignal_Func {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSignal_Func),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSignal_FuncMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitsignal/UnitSignal.md")))]
#[::unity2::class(namespace = "App", name = "UnitSignal")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct UnitSignal {
    #[rename(name = "m_Model")]
    pub m_model: crate::app::unitmodel::UnitModel,
    #[rename(name = "m_HitCallback")]
    pub m_hit_callback: crate::app::unitsignal::UnitSignal_Func,
    #[rename(name = "m_ShootCallback")]
    pub m_shoot_callback: crate::app::unitsignal::UnitSignal_Func,
    #[rename(name = "m_AvoidCallback")]
    pub m_avoid_callback: crate::app::unitsignal::UnitSignal_Func,
    #[rename(name = "m_Arg")]
    pub m_arg: ::unity2::IlInstance,
}

#[cfg(feature = "app-unitsignal")]
#[::unity2::methods]
impl UnitSignal {
    #[method(name = "SetModel", args = 1)]
    pub fn set_model(
        self,
        model: crate::app::unitmodel::UnitModel,
    ) -> crate::app::unitsignal::UnitSignal;

    #[method(name = "SetHitCallback", args = 1)]
    pub fn set_hit_callback(
        self,
        func: crate::app::unitsignal::UnitSignal_Func,
    ) -> crate::app::unitsignal::UnitSignal;

    #[method(name = "SetShootCallback", args = 1)]
    pub fn set_shoot_callback(
        self,
        func: crate::app::unitsignal::UnitSignal_Func,
    ) -> crate::app::unitsignal::UnitSignal;

    #[method(name = "SetAvoidCallback", args = 1)]
    pub fn set_avoid_callback(
        self,
        func: crate::app::unitsignal::UnitSignal_Func,
    ) -> crate::app::unitsignal::UnitSignal;

    #[method(name = "SetUnitItem", args = 1)]
    pub fn set_unit_item(
        self,
        unit_item: crate::app::unititem::UnitItem,
    ) -> crate::app::unitsignal::UnitSignal;

    #[method(name = "ClearCallback", args = 0)]
    pub fn clear_callback(self) -> crate::app::unitsignal::UnitSignal;

    #[method(name = "PlayShoot", args = 1)]
    pub fn play_shoot(signal: crate::app::unitsignal::UnitSignal) -> ();

    #[method(name = "OnFootstep", args = 1)]
    pub fn on_footstep(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "OnFlapping", args = 0)]
    pub fn on_flapping(self) -> ();

    #[method(name = "羽ばたき", args = 0)]
    pub fn _unnamed(self) -> ();

    #[method(name = "魔法動作1", args = 0)]
    pub fn ____1(self) -> ();

    #[method(name = "魔法動作2", args = 0)]
    pub fn ____2(self) -> ();

    #[method(name = "魔法動作3", args = 0)]
    pub fn ____3(self) -> ();

    #[method(name = "汎用Object", args = 0)]
    pub fn __object(self) -> ();

    #[method(name = "Run速度", args = 0)]
    pub fn run__(self) -> ();

    #[method(name = "Vec3", args = 0)]
    pub fn vec3(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitsignal")]
impl UnitSignal {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSignal),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSignalMethods>::ctor(this);
        this
    }
}
