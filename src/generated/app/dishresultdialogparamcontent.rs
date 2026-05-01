
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dishresultdialogparamcontent/DishResultDialogParamContent.md")))]
#[::unity2::class(namespace = "App", name = "DishResultDialogParamContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DishResultDialogParamContent {
    #[rename(name = "m_TitleText")]
    pub m_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ValueObject")]
    pub m_value_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UpObject")]
    pub m_up_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DownObject")]
    pub m_down_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NothingObject")]
    pub m_nothing_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ValueText")]
    pub m_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-dishresultdialogparamcontent")]
#[::unity2::methods]
impl DishResultDialogParamContent {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "SetEnhance", args = 2)]
    pub fn set_enhance(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        value: i32,
    ) -> bool;

    #[method(name = "SetBonus", args = 2)]
    pub fn set_bonus(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        value: i32,
    ) -> bool;

    #[method(name = "SetTitle", args = 1)]
    pub fn set_title(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dishresultdialogparamcontent")]
impl DishResultDialogParamContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DishResultDialogParamContent),
                ::core::stringify!(new),
            )
        });
        <Self as IDishResultDialogParamContentMethods>::ctor(this);
        this
    }
}
