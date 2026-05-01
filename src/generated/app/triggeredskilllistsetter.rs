
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/triggeredskilllistsetter/TriggeredSkillListSetter_ShowSkill.md")))]
#[::unity2::class(namespace = "App", name = "TriggeredSkillListSetter.ShowSkill")]
#[parent(crate::system::object::Object)]
pub struct TriggeredSkillListSetter_ShowSkill {
    #[rename(name = "m_Object")]
    pub m_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::skilldata::SkillData,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_Ani")]
    pub m_ani: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-triggeredskilllistsetter")]
#[::unity2::methods]
impl TriggeredSkillListSetter_ShowSkill {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, obj: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Show", args = 2)]
    pub fn show(self, data: crate::app::skilldata::SkillData, time: f32) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, data: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "IsEmpty", args = 0)]
    pub fn is_empty(self) -> bool;
}

#[cfg(feature = "app-triggeredskilllistsetter")]
impl TriggeredSkillListSetter_ShowSkill {
    pub fn new(obj: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TriggeredSkillListSetter_ShowSkill),
                ::core::stringify!(new),
            )
        });
        <Self as ITriggeredSkillListSetter_ShowSkillMethods>::ctor(this, obj);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/triggeredskilllistsetter/TriggeredSkillListSetter.md")))]
#[::unity2::class(namespace = "App", name = "TriggeredSkillListSetter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TriggeredSkillListSetter {
    #[rename(name = "m_ShowSkillList")]
    pub m_show_skill_list:
        ::unity2::Array<crate::app::triggeredskilllistsetter::TriggeredSkillListSetter_ShowSkill>,
}

#[cfg(feature = "app-triggeredskilllistsetter")]
#[::unity2::methods]
impl TriggeredSkillListSetter {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "AddShowSkill", args = 2)]
    pub fn add_show_skill(self, skill_data: crate::app::skilldata::SkillData, time: f32) -> ();

    #[method(name = "AddShowSkill", args = 1)]
    pub fn add_show_skill_2(self, skill_data: crate::app::skilldata::SkillData) -> ();

    #[method(name = "PushShowSkill", args = 1)]
    pub fn push_show_skill(self, skill_data: crate::app::skilldata::SkillData) -> ();

    #[method(name = "PopShowSkill", args = 1)]
    pub fn pop_show_skill(self, skill_data: crate::app::skilldata::SkillData) -> ();

    #[method(name = "HideSkillDisplay", args = 0)]
    pub fn hide_skill_display(self) -> ();

    #[method(name = "TryCreateList", args = 0)]
    pub fn try_create_list(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-triggeredskilllistsetter")]
impl TriggeredSkillListSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TriggeredSkillListSetter),
                ::core::stringify!(new),
            )
        });
        <Self as ITriggeredSkillListSetterMethods>::ctor(this);
        this
    }
}
