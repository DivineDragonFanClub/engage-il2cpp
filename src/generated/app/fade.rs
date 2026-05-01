
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdescuser::IProcDescUser;
use crate::app::procdescuser::ProcDescUser;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fade/Fade_ProcDescWait.md")))]
#[::unity2::class(namespace = "App", name = "Fade.ProcDescWait")]
#[parent(crate::app::fade::Fade_ProcDescFade)]
pub struct Fade_ProcDescWait {}

#[cfg(feature = "app-fade")]
#[::unity2::methods]
impl Fade_ProcDescWait {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, layer: crate::app::fade::Fade_Layer) -> ();

    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        inst: crate::app::procinst::ProcInst,
    ) -> crate::app::procdesc::ProcDesc_Result;
}

#[cfg(feature = "app-fade")]
impl Fade_ProcDescWait {
    pub fn new(layer: crate::app::fade::Fade_Layer) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Fade_ProcDescWait),
                ::core::stringify!(new),
            )
        });
        <Self as IFade_ProcDescWaitMethods>::ctor(this, layer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fade/Fade_ProcDescFade.md")))]
#[::unity2::class(namespace = "App", name = "Fade.ProcDescFade")]
#[parent(crate::app::procdescuser::ProcDescUser)]
pub struct Fade_ProcDescFade {
    #[rename(name = "m_Layer")]
    pub m_layer: crate::app::fade::Fade_Layer,
}

#[cfg(feature = "app-fade")]
#[::unity2::methods]
impl Fade_ProcDescFade {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, layer: crate::app::fade::Fade_Layer) -> ();

    #[method(name = "GetLayer", args = 0)]
    pub fn get_layer(self) -> crate::app::fade::Fade_Layer;
}

#[cfg(feature = "app-fade")]
impl Fade_ProcDescFade {
    pub fn new(layer: crate::app::fade::Fade_Layer) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Fade_ProcDescFade),
                ::core::stringify!(new),
            )
        });
        <Self as IFade_ProcDescFadeMethods>::ctor(this, layer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fade/Fade_LayerScope.md")))]
#[::unity2::class(namespace = "App", name = "Fade.LayerScope")]
#[parent(crate::system::object::Object)]
pub struct Fade_LayerScope {
    #[rename(name = "m_Layer")]
    pub m_layer: crate::app::fade::Fade_Layer,
}

#[cfg(feature = "app-fade")]
#[::unity2::methods]
impl Fade_LayerScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, layer: crate::app::fade::Fade_Layer) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "app-fade")]
impl Fade_LayerScope {
    pub fn new(layer: crate::app::fade::Fade_Layer) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Fade_LayerScope),
                ::core::stringify!(new),
            )
        });
        <Self as IFade_LayerScopeMethods>::ctor(this, layer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fade/Fade_Layer.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Fade_Layer {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Fade_Layer {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Fade.Layer";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Fade_Layer {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Fade_Layer {
    pub fn system() -> Self {
        Self { value: 0 }
    }

    pub fn skip() -> Self {
        Self { value: 1 }
    }

    pub fn talk() -> Self {
        Self { value: 2 }
    }

    pub fn combat() -> Self {
        Self { value: 3 }
    }

    pub fn current() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fade/Fade_ProcDescInOut.md")))]
#[::unity2::class(namespace = "App", name = "Fade.ProcDescInOut")]
#[parent(crate::app::fade::Fade_ProcDescFade)]
pub struct Fade_ProcDescInOut {
    #[rename(name = "m_Color")]
    pub m_color: crate::unity_engine::color::Color,
    #[rename(name = "m_Duration")]
    pub m_duration: f32,
    #[rename(name = "m_IsIn")]
    pub m_is_in: bool,
}

#[cfg(feature = "app-fade")]
#[::unity2::methods]
impl Fade_ProcDescInOut {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        layer: crate::app::fade::Fade_Layer,
        color: crate::unity_engine::color::Color,
        duration: f32,
        is_in: bool,
    ) -> ();

    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        inst: crate::app::procinst::ProcInst,
    ) -> crate::app::procdesc::ProcDesc_Result;
}

#[cfg(feature = "app-fade")]
impl Fade_ProcDescInOut {
    pub fn new(
        layer: crate::app::fade::Fade_Layer,
        color: crate::unity_engine::color::Color,
        duration: f32,
        is_in: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Fade_ProcDescInOut),
                ::core::stringify!(new),
            )
        });
        <Self as IFade_ProcDescInOutMethods>::ctor(this, layer, color, duration, is_in);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fade/Fade_ProcFadeWait.md")))]
#[::unity2::class(namespace = "App", name = "Fade.ProcFadeWait")]
#[parent(crate::app::procinst::ProcInst)]
pub struct Fade_ProcFadeWait {
    #[rename(name = "m_Layer")]
    pub m_layer: crate::app::fade::Fade_Layer,
}

#[cfg(feature = "app-fade")]
#[::unity2::methods]
impl Fade_ProcFadeWait {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, layer: crate::app::fade::Fade_Layer) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();
}

#[cfg(feature = "app-fade")]
impl Fade_ProcFadeWait {
    pub fn new(layer: crate::app::fade::Fade_Layer) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Fade_ProcFadeWait),
                ::core::stringify!(new),
            )
        });
        <Self as IFade_ProcFadeWaitMethods>::ctor(this, layer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fade/Fade.md")))]
#[::unity2::class(namespace = "App", name = "Fade")]
#[parent(crate::system::object::Object)]
pub struct Fade {
    #[static_field]
    #[rename(name = "Num")]
    pub num: i32,
    #[static_field]
    #[rename(name = "VeryFast")]
    pub very_fast: f32,
    #[static_field]
    #[rename(name = "Fast")]
    pub fast: f32,
    #[static_field]
    #[rename(name = "Normal")]
    pub normal: f32,
    #[static_field]
    #[rename(name = "Slow")]
    pub slow: f32,
    #[static_field]
    #[rename(name = "VerySlow")]
    pub very_slow: f32,
    #[static_field]
    #[rename(name = "s_Proc")]
    pub s_proc: crate::app::fade::Fade_ProcFade,
    #[static_field]
    #[rename(name = "s_Colors")]
    pub s_colors: ::unity2::Array<crate::unity_engine::color::Color>,
    #[static_field]
    #[rename(name = "s_Layer")]
    pub s_layer: crate::app::fade::Fade_Layer,
    #[static_field]
    #[rename(name = "s_Statck")]
    pub s_statck:
        crate::system::collections::generic::stack_1::Stack_1<crate::app::fade::Fade_Layer>,
}

#[cfg(feature = "app-fade")]
#[::unity2::methods]
impl Fade {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "GetImageColor", args = 1)]
    pub fn get_image_color(
        layer: crate::app::fade::Fade_Layer,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "SetImageColor", args = 2)]
    pub fn set_image_color(
        layer: crate::app::fade::Fade_Layer,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "FadeIn", args = 1)]
    pub fn fade_in(duration: f32) -> ();

    #[method(name = "FadeOut", args = 1)]
    pub fn fade_out(duration: f32) -> ();

    #[method(name = "FadeOut", args = 2)]
    pub fn fade_out_2(color: crate::unity_engine::color::Color, duration: f32) -> ();

    #[method(name = "FadeIn", args = 2)]
    pub fn fade_in_2(layer: crate::app::fade::Fade_Layer, duration: f32) -> ();

    #[method(name = "FadeOut", args = 2)]
    pub fn fade_out_3(layer: crate::app::fade::Fade_Layer, duration: f32) -> ();

    #[method(name = "FadeOut", args = 3)]
    pub fn fade_out_4(
        layer: crate::app::fade::Fade_Layer,
        color: crate::unity_engine::color::Color,
        duration: f32,
    ) -> ();

    #[method(name = "PushLayer", args = 1)]
    pub fn push_layer(layer: crate::app::fade::Fade_Layer) -> ();

    #[method(name = "PopLayer", args = 0)]
    pub fn pop_layer() -> ();

    #[method(name = "IsActive", args = 0)]
    pub fn is_active() -> bool;

    #[method(name = "IsActive", args = 1)]
    pub fn is_active_2(layer: crate::app::fade::Fade_Layer) -> bool;

    #[method(name = "IsFadeOut", args = 0)]
    pub fn is_fade_out() -> bool;

    #[method(name = "IsFadeOut", args = 1)]
    pub fn is_fade_out_2(layer: crate::app::fade::Fade_Layer) -> bool;

    #[method(name = "GetAlpha", args = 0)]
    pub fn get_alpha() -> f32;

    #[method(name = "WaitBind", args = 2)]
    pub fn wait_bind(
        super_: crate::app::procinst::ProcInst,
        layer: crate::app::fade::Fade_Layer,
    ) -> bool;

    #[method(name = "BlackIn", args = 2)]
    pub fn black_in(
        duration: f32,
        layer: crate::app::fade::Fade_Layer,
    ) -> crate::app::procdesc::ProcDesc;

    #[method(name = "BlackOut", args = 2)]
    pub fn black_out(
        duration: f32,
        layer: crate::app::fade::Fade_Layer,
    ) -> crate::app::procdesc::ProcDesc;

    #[method(name = "WhiteIn", args = 2)]
    pub fn white_in(
        duration: f32,
        layer: crate::app::fade::Fade_Layer,
    ) -> crate::app::procdesc::ProcDesc;

    #[method(name = "WhiteOut", args = 2)]
    pub fn white_out(
        duration: f32,
        layer: crate::app::fade::Fade_Layer,
    ) -> crate::app::procdesc::ProcDesc;

    #[method(name = "FadeIn", args = 3)]
    pub fn fade_in_3(
        color: crate::unity_engine::color::Color,
        duration: f32,
        layer: crate::app::fade::Fade_Layer,
    ) -> crate::app::procdesc::ProcDesc;

    #[method(name = "FadeOut", args = 3)]
    pub fn fade_out_5(
        color: crate::unity_engine::color::Color,
        duration: f32,
        layer: crate::app::fade::Fade_Layer,
    ) -> crate::app::procdesc::ProcDesc;

    #[method(name = "FadeWait", args = 1)]
    pub fn fade_wait(layer: crate::app::fade::Fade_Layer) -> crate::app::procdesc::ProcDesc;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-fade")]
impl Fade {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Fade),
                ::core::stringify!(new),
            )
        });
        <Self as IFadeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fade/Fade_ProcFade.md")))]
#[::unity2::class(namespace = "App", name = "Fade.ProcFade")]
#[parent(crate::app::procinst::ProcInst)]
pub struct Fade_ProcFade {
    #[rename(name = "m_FadeLayer")]
    pub m_fade_layer: ::unity2::Array<crate::app::fade::Fade_FadeLayer>,
}

#[cfg(feature = "app-fade")]
#[::unity2::methods]
impl Fade_ProcFade {
    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::fade::Fade_FadeLayer;

    #[method(name = "Create", args = 2)]
    pub fn create(to_color: crate::unity_engine::color::Color, duration: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "IsActive", args = 1)]
    pub fn is_active(self, layer: crate::app::fade::Fade_Layer) -> bool;
}

#[cfg(feature = "app-fade")]
impl Fade_ProcFade {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Fade_ProcFade),
                ::core::stringify!(new),
            )
        });
        <Self as IFade_ProcFadeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fade/Fade_FadeLayer.md")))]
#[::unity2::class(namespace = "App", name = "Fade.FadeLayer")]
#[parent(crate::system::object::Object)]
pub struct Fade_FadeLayer {
    #[rename(name = "m_FromColor")]
    pub m_from_color: crate::unity_engine::color::Color,
    #[rename(name = "m_ToColor")]
    pub m_to_color: crate::unity_engine::color::Color,
    #[rename(name = "m_Duration")]
    pub m_duration: f32,
    #[rename(name = "m_Elapsed")]
    pub m_elapsed: f32,
}

#[cfg(feature = "app-fade")]
#[::unity2::methods]
impl Fade_FadeLayer {
    #[method(name = "Exec", args = 2)]
    pub fn exec(self, to_color: crate::unity_engine::color::Color, duration: f32) -> ();

    #[method(name = "Tick", args = 1)]
    pub fn tick(self, delta_time: f32) -> bool;

    #[method(name = "GetColor", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "get_ToColor", args = 0)]
    pub fn get_to_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "IsAlive", args = 0)]
    pub fn is_alive(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-fade")]
impl Fade_FadeLayer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Fade_FadeLayer),
                ::core::stringify!(new),
            )
        });
        <Self as IFade_FadeLayerMethods>::ctor(this);
        this
    }
}
