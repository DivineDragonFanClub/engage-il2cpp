
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/endrollsetter/EndRollSetter.md")))]
#[::unity2::class(namespace = "App", name = "EndRollSetter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct EndRollSetter {
    #[rename(name = "PictureSide0")]
    pub picture_side0: crate::unity_engine::ui::image::Image,
    #[rename(name = "PictureSide1")]
    pub picture_side1: crate::unity_engine::ui::image::Image,
    #[rename(name = "PictureLarge")]
    pub picture_large: crate::unity_engine::ui::image::Image,
    #[rename(name = "CreditRoot")]
    pub credit_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "PrefabTitlePosition")]
    pub prefab_title_position: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "PrefabTitleCompany")]
    pub prefab_title_company: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "Prefab1Lines")]
    pub prefab1_lines: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "Prefab2Lines")]
    pub prefab2_lines: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "Prefab3Lines")]
    pub prefab3_lines: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "PrefabVoiceActor")]
    pub prefab_voice_actor: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Pictures")]
    pub m_pictures: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_StartPos")]
    pub m_start_pos: f32,
    #[rename(name = "m_IllustFadeTime")]
    pub m_illust_fade_time: f32,
    #[rename(name = "m_ScrollTime")]
    pub m_scroll_time: f32,
    #[rename(name = "m_WaitTimeAfterStop")]
    pub m_wait_time_after_stop: f32,
    #[rename(name = "m_FadeOutTime")]
    pub m_fade_out_time: f32,
    #[rename(name = "m_LastPicWaitTime")]
    pub m_last_pic_wait_time: f32,
    #[rename(name = "m_LastPicFadeIn")]
    pub m_last_pic_fade_in: f32,
    #[rename(name = "m_LastPicDisplay")]
    pub m_last_pic_display: f32,
    #[rename(name = "m_LastPicFadeOut")]
    pub m_last_pic_fade_out: f32,
    #[rename(name = "m_ScrolledTime")]
    pub m_scrolled_time: f32,
    #[rename(name = "m_ScrollLength")]
    pub m_scroll_length: f32,
    #[rename(name = "m_LastObject")]
    pub m_last_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CreditList")]
    pub m_credit_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::recttransform::RectTransform,
    >,
    #[rename(name = "m_IllustList")]
    pub m_illust_list: crate::system::collections::generic::list_1::List_1<i32>,
    #[rename(name = "m_IllustFade")]
    pub m_illust_fade: f32,
    #[rename(name = "m_StartTime")]
    pub m_start_time: f64,
    #[rename(name = "m_Phase")]
    pub m_phase: i32,
    #[rename(name = "seqIllust")]
    pub seq_illust: ::unity2::Array<i32>,
}

#[cfg(feature = "app-endrollsetter")]
#[::unity2::methods]
impl EndRollSetter {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "SetupEndRoll", args = 0)]
    pub fn setup_end_roll(self) -> ();

    #[method(name = "SetupIllust", args = 0)]
    pub fn setup_illust(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Update1", args = 0)]
    pub fn update1(self) -> ();

    #[method(name = "Update2", args = 0)]
    pub fn update2(self) -> ();

    #[method(name = "IsFinished1", args = 0)]
    pub fn is_finished1(self) -> bool;

    #[method(name = "IsFinished2", args = 0)]
    pub fn is_finished2(self) -> bool;

    #[method(name = "DisplayLargePic", args = 0)]
    pub fn display_large_pic(self) -> ();

    #[method(name = "EndLargePic", args = 0)]
    pub fn end_large_pic(self) -> ();

    #[method(name = "WaitStartFadeOut", args = 2)]
    pub fn wait_start_fade_out(self, cur: i32, aft: i32) -> f32;

    #[method(name = "PreFadeOut", args = 2)]
    pub fn pre_fade_out(self, bef: i32, aft: i32) -> ();

    #[method(name = "FadeOut", args = 2)]
    pub fn fade_out(self, bef: i32, aft: i32) -> f32;

    #[method(name = "PreFadeIn", args = 2)]
    pub fn pre_fade_in(self, bef: i32, aft: i32) -> ();

    #[method(name = "FadeIn", args = 2)]
    pub fn fade_in(self, bef: i32, aft: i32) -> f32;

    #[method(name = "IsSequential", args = 1)]
    pub fn is_sequential(self, id: i32) -> bool;

    #[method(name = "SetPicture", args = 2)]
    pub fn set_picture(self, img: crate::unity_engine::ui::image::Image, num: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-endrollsetter")]
impl EndRollSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EndRollSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IEndRollSetterMethods>::ctor(this);
        this
    }
}
