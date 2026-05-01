
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dishresultdialogcontent/DishResultDialogContent.md")))]
#[::unity2::class(namespace = "App", name = "DishResultDialogContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DishResultDialogContent {
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_DishText")]
    pub m_dish_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GradeTitle")]
    pub m_grade_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GradeList")]
    pub m_grade_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_CondText")]
    pub m_cond_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_EnhanceObjectList")]
    pub m_enhance_object_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_NothingObject")]
    pub m_nothing_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_EnhanceBonus")]
    pub m_enhance_bonus: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BonusObjectList")]
    pub m_bonus_object_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_EveryoneText")]
    pub m_everyone_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-dishresultdialogcontent")]
#[::unity2::methods]
impl DishResultDialogContent {
    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsOpening", args = 0)]
    pub fn is_opening(self) -> bool;

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "SetDish", args = 1)]
    pub fn set_dish(self, dish: crate::app::dish::Dish) -> ();

    #[method(name = "SetGrade", args = 1)]
    pub fn set_grade(self, taste: crate::app::tastedata::TasteData) -> ();

    #[method(name = "SetEnhance", args = 3)]
    pub fn set_enhance(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        index: i32,
        value: i32,
    ) -> bool;

    #[method(name = "SetBonus", args = 3)]
    pub fn set_bonus(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        index: i32,
        value: i32,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dishresultdialogcontent")]
impl DishResultDialogContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DishResultDialogContent),
                ::core::stringify!(new),
            )
        });
        <Self as IDishResultDialogContentMethods>::ctor(this);
        this
    }
}
