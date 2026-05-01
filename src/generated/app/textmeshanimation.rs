
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/textmeshanimation/TextMeshAnimation_AnimationParams.md")))]
#[::unity2::class(namespace = "App", name = "TextMeshAnimation.AnimationParams")]
#[parent(crate::system::object::Object)]
pub struct TextMeshAnimation_AnimationParams {
    #[rename(name = "OffsetCurveX")]
    pub offset_curve_x: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "OffsetCurveY")]
    pub offset_curve_y: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "RotationCurve")]
    pub rotation_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "ScaleCurveX")]
    pub scale_curve_x: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "ScaleCurveY")]
    pub scale_curve_y: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "RedCurve")]
    pub red_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "GreenCurve")]
    pub green_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "BlueCurve")]
    pub blue_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "AlphaCurve")]
    pub alpha_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_PivotType")]
    pub m_pivot_type: crate::app::textmeshanimation::TextMeshAnimation_PivotType,
    #[rename(name = "m_DelayTime")]
    pub m_delay_time: f32,
    #[rename(name = "m_DelayTimeType")]
    pub m_delay_time_type: crate::app::textmeshanimation::TextMeshAnimation_DelayTimeType,
    #[rename(name = "m_EndType")]
    pub m_end_type: crate::app::textmeshanimation::TextMeshAnimation_EndType,
    #[rename(name = "m_StartFromRight")]
    pub m_start_from_right: bool,
    #[rename(name = "m_MaxIntervalTime")]
    pub m_max_interval_time: f32,
    #[rename(name = "m_EndTime")]
    pub m_end_time: f32,
}

#[cfg(feature = "app-textmeshanimation")]
#[::unity2::methods]
impl TextMeshAnimation_AnimationParams {
    #[method(name = "GetDelayTime", args = 1)]
    pub fn get_delay_time(self, character_count: i32) -> f32;

    #[method(name = "CalcEndTime", args = 0)]
    pub fn calc_end_time(self) -> ();

    #[method(name = "IsEnd", args = 1)]
    pub fn is_end(self, time: f32) -> bool;

    #[method(name = "SetWrapMode", args = 1)]
    pub fn set_wrap_mode(self, mode: crate::unity_engine::wrapmode::WrapMode) -> ();

    #[method(name = "ExpandAnimation", args = 1)]
    pub fn expand_animation(self, ratio: f32) -> ();

    #[method(name = "get_EndTime", args = 0)]
    pub fn get_end_time(self) -> f32;

    #[method(name = "GetAnimationList", args = 0)]
    pub fn get_animation_list(
        self,
    ) -> ::unity2::Array<crate::unity_engine::animationcurve::AnimationCurve>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-textmeshanimation")]
impl TextMeshAnimation_AnimationParams {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextMeshAnimation_AnimationParams),
                ::core::stringify!(new),
            )
        });
        <Self as ITextMeshAnimation_AnimationParamsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/textmeshanimation/TextMeshAnimation.md")))]
#[::unity2::class(namespace = "App", name = "TextMeshAnimation")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TextMeshAnimation {
    #[rename(name = "Params")]
    pub params: ::unity2::Array<crate::app::textmeshanimation::TextMeshAnimation_AnimationParams>,
    #[rename(name = "m_TextComponent")]
    pub m_text_component: crate::tm_pro::tmp_text::TMP_Text,
    #[rename(name = "m_IsAnimation")]
    pub m_is_animation: bool,
    #[rename(name = "m_IsFirstFrame")]
    pub m_is_first_frame: bool,
    #[rename(name = "m_NowAnimationIndex")]
    pub m_now_animation_index: i32,
    #[rename(name = "m_NowTime")]
    pub m_now_time: f32,
}

#[cfg(feature = "app-textmeshanimation")]
#[::unity2::methods]
impl TextMeshAnimation {
    #[method(name = "get_isUnscaledDeltaTime", args = 0)]
    pub fn get_is_unscaled_delta_time(self) -> bool;

    #[method(name = "set_isUnscaledDeltaTime", args = 1)]
    pub fn set_is_unscaled_delta_time(self, value: bool) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "AnimationText", args = 0)]
    pub fn animation_text(self) -> ();

    #[method(name = "SetText", args = 1)]
    pub fn set_text(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "Restart", args = 0)]
    pub fn restart(self) -> ();

    #[method(name = "StartAnimation", args = 0)]
    pub fn start_animation(self) -> ();

    #[method(name = "Skip", args = 0)]
    pub fn skip(self) -> ();

    #[method(name = "StartAnimation", args = 1)]
    pub fn start_animation_2(self, index: i32) -> ();

    #[method(name = "ToNextAnimation", args = 0)]
    pub fn to_next_animation(self) -> ();

    #[method(name = "SetAsUnscaledInChildren", args = 2)]
    pub fn set_as_unscaled_in_children(
        root: crate::unity_engine::transform::Transform,
        v: bool,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-textmeshanimation")]
impl TextMeshAnimation {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextMeshAnimation),
                ::core::stringify!(new),
            )
        });
        <Self as ITextMeshAnimationMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/textmeshanimation/TextMeshAnimation_PivotType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TextMeshAnimation_PivotType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TextMeshAnimation_PivotType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TextMeshAnimation.PivotType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TextMeshAnimation_PivotType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TextMeshAnimation_PivotType {
    pub fn center() -> Self {
        Self { value: 0 }
    }

    pub fn base_line() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/textmeshanimation/TextMeshAnimation_EndType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TextMeshAnimation_EndType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TextMeshAnimation_EndType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TextMeshAnimation.EndType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TextMeshAnimation_EndType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TextMeshAnimation_EndType {
    pub fn r#loop() -> Self {
        Self { value: 0 }
    }

    pub fn to_next() -> Self {
        Self { value: 1 }
    }

    pub fn wait() -> Self {
        Self { value: 2 }
    }

    pub fn hide() -> Self {
        Self { value: 3 }
    }

    pub fn delete() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/textmeshanimation/TextMeshAnimation_DelayTimeType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TextMeshAnimation_DelayTimeType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TextMeshAnimation_DelayTimeType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TextMeshAnimation.DelayTimeType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TextMeshAnimation_DelayTimeType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TextMeshAnimation_DelayTimeType {
    pub fn interval() -> Self {
        Self { value: 0 }
    }

    pub fn total() -> Self {
        Self { value: 1 }
    }
}
