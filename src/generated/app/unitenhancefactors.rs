
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitenhancefactors/UnitEnhanceFactors.md")))]
#[::unity2::class(namespace = "App", name = "UnitEnhanceFactors")]
#[parent(crate::system::object::Object)]
pub struct UnitEnhanceFactors {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_HubValues")]
    pub m_hub_values: crate::app::unitenhancevalues::UnitEnhanceValues,
    #[rename(name = "m_FoodValues")]
    pub m_food_values: crate::app::unitenhancevalues::UnitEnhanceValues,
    #[rename(name = "m_ItemValues")]
    pub m_item_values: crate::app::unitenhancevalues::UnitEnhanceValues,
}

#[cfg(feature = "app-unitenhancefactors")]
#[::unity2::methods]
impl UnitEnhanceFactors {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, factors: crate::app::unitenhancefactors::UnitEnhanceFactors) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetHub", args = 1)]
    pub fn set_hub(self, values: crate::app::unitenhancevalues::UnitEnhanceValues) -> ();

    #[method(name = "SetHubValue", args = 2)]
    pub fn set_hub_value(
        self,
        index: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        value: i32,
    ) -> ();

    #[method(name = "SetHubValue", args = 2)]
    pub fn set_hub_value_2(self, index: i32, value: i32) -> ();

    #[method(name = "ClearHub", args = 0)]
    pub fn clear_hub(self) -> ();

    #[method(name = "SetFoodValues", args = 1)]
    pub fn set_food_values(self, values: crate::app::unitenhancevalues::UnitEnhanceValues) -> ();

    #[method(name = "SetFoodValue", args = 2)]
    pub fn set_food_value(
        self,
        index: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        value: i32,
    ) -> ();

    #[method(name = "SetFoodValue", args = 2)]
    pub fn set_food_value_2(self, index: i32, value: i32) -> ();

    #[method(name = "ClearFoodValues", args = 0)]
    pub fn clear_food_values(self) -> ();

    #[method(name = "SetItemValue", args = 1)]
    pub fn set_item_value(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "AddItemValue", args = 1)]
    pub fn add_item_value(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "SetItemValue", args = 2)]
    pub fn set_item_value_2(self, index: i32, value: i32) -> ();

    #[method(name = "ClearItemValue", args = 0)]
    pub fn clear_item_value(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_HubValues", args = 0)]
    pub fn get_hub_values(self) -> crate::app::unitenhancevalues::UnitEnhanceValues;

    #[method(name = "get_FoodValues", args = 0)]
    pub fn get_food_values(self) -> crate::app::unitenhancevalues::UnitEnhanceValues;

    #[method(name = "get_ItemValues", args = 0)]
    pub fn get_item_values(self) -> crate::app::unitenhancevalues::UnitEnhanceValues;
}

#[cfg(feature = "app-unitenhancefactors")]
impl UnitEnhanceFactors {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitEnhanceFactors),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitEnhanceFactorsMethods>::ctor(this);
        this
    }
}
