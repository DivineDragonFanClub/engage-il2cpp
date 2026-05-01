
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotcolorchangemenu/MascotColorChangeMenu.md")))]
#[::unity2::class(namespace = "App", name = "MascotColorChangeMenu")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MascotColorChangeMenu {
    #[rename(name = "m_result")]
    pub m_result: crate::app::basicmenu::BasicMenu_Result,
    #[rename(name = "m_RootAnim")]
    pub m_root_anim: crate::unity_engine::animator::Animator,
    #[rename(name = "m_cursor")]
    pub m_cursor: crate::app::mascotcolorchangemenu::MascotColorChangeMenu_CursorTop,
}

#[cfg(feature = "app-mascotcolorchangemenu")]
#[::unity2::methods]
impl MascotColorChangeMenu {
    #[method(name = "get_Content", args = 0)]
    pub fn get_content(self) -> crate::app::mascotcolorchangecontent::MascotColorChangeContent;

    #[method(name = "set_Content", args = 1)]
    pub fn set_content(
        self,
        value: crate::app::mascotcolorchangecontent::MascotColorChangeContent,
    ) -> ();

    #[method(name = "get_ColorPalette", args = 0)]
    pub fn get_color_palette(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::mascotcolorchangeitemcontent::MascotColorChangeItemContent,
    >;

    #[method(name = "set_ColorPalette", args = 1)]
    pub fn set_color_palette(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::mascotcolorchangeitemcontent::MascotColorChangeItemContent,
        >,
    ) -> ();

    #[method(name = "SetSelectColor", args = 0)]
    pub fn set_select_color(self) -> ();

    #[method(name = "SetRadioCheckColorPalette", args = 0)]
    pub fn set_radio_check_color_palette(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Main", args = 0)]
    pub fn main(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlayCursorSE", args = 0)]
    pub fn play_cursor_se(self) -> ();

    #[method(name = "PlayDecideSE", args = 0)]
    pub fn play_decide_se(self) -> ();

    #[method(name = "PlayDecideBigSE", args = 0)]
    pub fn play_decide_big_se(self) -> ();

    #[method(name = "PlayCancelSE", args = 0)]
    pub fn play_cancel_se(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mascotcolorchangemenu")]
impl MascotColorChangeMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotColorChangeMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotColorChangeMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotcolorchangemenu/MascotColorChangeMenu_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MascotColorChangeMenu_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MascotColorChangeMenu_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MascotColorChangeMenu.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MascotColorChangeMenu_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MascotColorChangeMenu_Label {
    pub fn init() -> Self {
        Self { value: 0 }
    }

    pub fn main() -> Self {
        Self { value: 1 }
    }

    pub fn exit() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotcolorchangemenu/MascotColorChangeMenu_CursorTop.md")))]
#[::unity2::class(namespace = "App", name = "MascotColorChangeMenu.CursorTop")]
#[parent(crate::system::object::Object)]
pub struct MascotColorChangeMenu_CursorTop {
    #[rename(name = "XCount")]
    pub x_count: i32,
    #[rename(name = "YCount")]
    pub y_count: i32,
    #[rename(name = "MoveFrame")]
    pub move_frame: f32,
    #[rename(name = "m_cursorTop")]
    pub m_cursor_top: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_moveTick")]
    pub m_move_tick: f32,
}

#[cfg(feature = "app-mascotcolorchangemenu")]
#[::unity2::methods]
impl MascotColorChangeMenu_CursorTop {
    #[method(name = "get_SelectIndexX", args = 0)]
    pub fn get_select_index_x(self) -> i32;

    #[method(name = "get_SelectIndexY", args = 0)]
    pub fn get_select_index_y(self) -> i32;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, transform: crate::unity_engine::recttransform::RectTransform) -> ();

    #[method(name = "SetIndexInstant", args = 1)]
    pub fn set_index_instant(self, index: i32) -> ();

    #[method(name = "MoveCursor", args = 3)]
    pub fn move_cursor(self, move_x: i32, move_y: i32, is_trigger: bool) -> bool;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetPosition", args = 2)]
    pub fn set_position(self, x: f32, y: f32) -> ();

    #[method(name = "GetPositionX", args = 1)]
    pub fn get_position_x(self, select_index: i32) -> f32;

    #[method(name = "GetPositionY", args = 1)]
    pub fn get_position_y(self, select_index: i32) -> f32;
}

#[cfg(feature = "app-mascotcolorchangemenu")]
impl MascotColorChangeMenu_CursorTop {
    pub fn new(transform: crate::unity_engine::recttransform::RectTransform) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotColorChangeMenu_CursorTop),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotColorChangeMenu_CursorTopMethods>::ctor(this, transform);
        this
    }
}
