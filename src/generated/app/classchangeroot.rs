
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchangeroot/ClassChangeRoot.md")))]
#[::unity2::class(namespace = "App", name = "ClassChangeRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ClassChangeRoot {
    #[rename(name = "m_MenuContent")]
    pub m_menu_content: crate::app::classchangejobmenucontent::ClassChangeJobMenuContent,
    #[rename(name = "m_UnitName")]
    pub m_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_UnitInfoBefore")]
    pub m_unit_info_before: crate::app::unitstatussetter::UnitStatusSetter,
    #[rename(name = "m_UnitInfoAfter")]
    pub m_unit_info_after: crate::app::unitstatussetter::UnitStatusSetter,
    #[rename(name = "m_WeaponIconList")]
    pub m_weapon_icon_list:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::ui::image::Image>,
}

#[cfg(feature = "app-classchangeroot")]
#[::unity2::methods]
impl ClassChangeRoot {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "InitUnitInfoAll", args = 0)]
    pub fn init_unit_info_all(self) -> ();

    #[method(name = "GetMenuContent", args = 0)]
    pub fn get_menu_content(
        self,
    ) -> crate::app::classchangejobmenucontent::ClassChangeJobMenuContent;

    #[method(name = "SetUnitInfoAfter", args = 2)]
    pub fn set_unit_info_after(
        self,
        before: crate::app::unit::Unit,
        after: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetUnitInfoBefore", args = 1)]
    pub fn set_unit_info_before(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-classchangeroot")]
impl ClassChangeRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChangeRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeRootMethods>::ctor(this);
        this
    }
}
