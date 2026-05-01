
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eatunitsselectmenureliancecontent/EatUnitsSelectMenuRelianceContent.md")))]
#[::unity2::class(namespace = "App", name = "EatUnitsSelectMenuRelianceContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct EatUnitsSelectMenuRelianceContent {
    #[rename(name = "m_TogetherList")]
    pub m_together_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_RelianceList")]
    pub m_reliance_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "app-eatunitsselectmenureliancecontent")]
#[::unity2::methods]
impl EatUnitsSelectMenuRelianceContent {
    #[method(name = "UpdateList", args = 2)]
    pub fn update_list(
        self,
        selected_units: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
        now_cursor_unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "IsSameUnits", args = 2)]
    pub fn is_same_units(
        self,
        selected_units: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
        now_cursor_unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "SetUnitInfo", args = 2)]
    pub fn set_unit_info(
        self,
        together_list_item_object: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-eatunitsselectmenureliancecontent")]
impl EatUnitsSelectMenuRelianceContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EatUnitsSelectMenuRelianceContent),
                ::core::stringify!(new),
            )
        });
        <Self as IEatUnitsSelectMenuRelianceContentMethods>::ctor(this);
        this
    }
}
