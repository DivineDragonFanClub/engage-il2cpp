
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlerecordsetter/BattleRecordSetter.md")))]
#[::unity2::class(namespace = "App", name = "BattleRecordSetter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct BattleRecordSetter {
    #[rename(name = "InfoTitle")]
    pub info_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "Records")]
    pub records: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "RecordPrefab")]
    pub record_prefab: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ScrollTime")]
    pub m_scroll_time: f32,
    #[rename(name = "m_StartPos")]
    pub m_start_pos: f32,
    #[rename(name = "m_RecordLength")]
    pub m_record_length: f32,
    #[rename(name = "m_ScrolledTime")]
    pub m_scrolled_time: f32,
    #[rename(name = "m_RecordList")]
    pub m_record_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::recttransform::RectTransform,
    >,
    #[rename(name = "m_StartTime")]
    pub m_start_time: f64,
}

#[cfg(feature = "app-battlerecordsetter")]
#[::unity2::methods]
impl BattleRecordSetter {
    #[method(name = "get_ScrollTimePattern", args = 0)]
    pub fn get_scroll_time_pattern(self) -> i32;

    #[method(name = "set_ScrollTimePattern", args = 1)]
    pub fn set_scroll_time_pattern(self, value: i32) -> ();

    #[method(name = "get_IsFinished", args = 0)]
    pub fn get_is_finished(self) -> bool;

    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-battlerecordsetter")]
impl BattleRecordSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleRecordSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleRecordSetterMethods>::ctor(this);
        this
    }
}
