
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/expsetter/ExpSetter.md")))]
#[::unity2::class(namespace = "App", name = "ExpSetter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ExpSetter {
    #[rename(name = "InAnimWait")]
    pub in_anim_wait: f32,
    #[rename(name = "m_UnitWindow")]
    pub m_unit_window: crate::app::expsetter::ExpSetter_ExpWindow,
}

#[cfg(feature = "app-expsetter")]
#[::unity2::methods]
impl ExpSetter {
    #[method(name = "Setup", args = 2)]
    pub fn setup(self, unit: crate::app::unit::Unit, gain_exp: i32) -> ();

    #[method(name = "SetExp", args = 1)]
    pub fn set_exp(self, main: crate::app::unit::Unit) -> ();

    #[method(name = "SetAddExpMain", args = 1)]
    pub fn set_add_exp_main(self, exp: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-expsetter")]
impl ExpSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExpSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IExpSetterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/expsetter/ExpSetter_ExpWindow.md")))]
#[::unity2::class(namespace = "App", name = "ExpSetter.ExpWindow")]
#[parent(crate::system::object::Object)]
pub struct ExpSetter_ExpWindow {
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Name")]
    pub m_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RestExp")]
    pub m_rest_exp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GetExp")]
    pub m_get_exp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SpValue")]
    pub m_sp_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ExpValue")]
    pub m_exp_value: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LevelMax")]
    pub m_level_max: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Gauge")]
    pub m_gauge: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MaxColor")]
    pub m_max_color: crate::unity_engine::material::Material,
    #[rename(name = "m_OldLevel")]
    pub m_old_level: i32,
}

#[cfg(feature = "app-expsetter")]
#[::unity2::methods]
impl ExpSetter_ExpWindow {
    #[method(name = "SetupUnit", args = 2)]
    pub fn setup_unit(self, unit: crate::app::unit::Unit, gain_exp: i32) -> ();

    #[method(name = "UpdateUnit", args = 1)]
    pub fn update_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateAddExp", args = 1)]
    pub fn update_add_exp(self, exp: i32) -> ();

    #[method(name = "SetLevelMax", args = 0)]
    pub fn set_level_max(self) -> ();

    #[method(name = "IsLevelMax", args = 1)]
    pub fn is_level_max(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-expsetter")]
impl ExpSetter_ExpWindow {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExpSetter_ExpWindow),
                ::core::stringify!(new),
            )
        });
        <Self as IExpSetter_ExpWindowMethods>::ctor(this);
        this
    }
}
