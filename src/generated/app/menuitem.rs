
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/menuitem/MenuItem_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MenuItem_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MenuItem_State {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MenuItem.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MenuItem_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MenuItem_State {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn open() -> Self {
        Self { value: 1 }
    }

    pub fn close() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/menuitem/MenuItem_Align.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MenuItem_Align {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MenuItem_Align {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MenuItem.Align";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MenuItem_Align {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MenuItem_Align {
    pub fn left() -> Self {
        Self { value: 0 }
    }

    pub fn center() -> Self {
        Self { value: 1 }
    }

    pub fn right() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/menuitem/MenuItem_Result.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MenuItem_Result {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MenuItem_Result {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MenuItem.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MenuItem_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MenuItem_Result {
    pub fn pass() -> Self {
        Self { value: 0 }
    }

    pub fn do_nothing() -> Self {
        Self { value: 1 }
    }

    pub fn decide() -> Self {
        Self { value: 2 }
    }

    pub fn cancel() -> Self {
        Self { value: 3 }
    }

    pub fn close() -> Self {
        Self { value: 4 }
    }

    pub fn close_all() -> Self {
        Self { value: 5 }
    }

    pub fn rebuild() -> Self {
        Self { value: 6 }
    }

    pub fn parent_rebuild() -> Self {
        Self { value: 7 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/menuitem/MenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MenuItem")]
#[parent(crate::system::object::Object)]
pub struct MenuItem {
    #[rename(name = "m_Menu")]
    pub m_menu: crate::app::debugmenu::DebugMenu,
    #[rename(name = "m_State")]
    pub m_state: crate::app::menuitem::MenuItem_State,
    #[rename(name = "m_Bind")]
    pub m_bind: i32,
}

#[cfg(feature = "app-menuitem")]
#[::unity2::methods]
impl MenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetNameEnglish", args = 0)]
    pub fn get_name_english(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpEnglish", args = 0)]
    pub fn get_help_english(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpWidth", args = 0)]
    pub fn get_help_width(self) -> f32;

    #[method(name = "GetHelpHeight", args = 0)]
    pub fn get_help_height(self) -> f32;

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind(self) -> crate::app::menuitem::MenuItem_Kind;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "YCall", args = 0)]
    pub fn y_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "LCall", args = 0)]
    pub fn l_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "RCall", args = 0)]
    pub fn r_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "MinusCall", args = 0)]
    pub fn minus_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "LeftCall", args = 0)]
    pub fn left_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "RightCall", args = 0)]
    pub fn right_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = "IsSelectable", args = 0)]
    pub fn is_selectable(self) -> bool;

    #[method(name = "GetWidth", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "GetHeight", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "GetFontColor", args = 0)]
    pub fn get_font_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetBackColor", args = 0)]
    pub fn get_back_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth", args = 1)]
    pub fn get_column_width(self, i: i32) -> f32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetColumnWidth2", args = 0)]
    pub fn get_column_width2(self) -> f32;

    #[method(name = "GetColumnWidth3", args = 0)]
    pub fn get_column_width3(self) -> f32;

    #[method(name = "GetColumnWidth4", args = 0)]
    pub fn get_column_width4(self) -> f32;

    #[method(name = "GetColumnHeight", args = 1)]
    pub fn get_column_height(self, i: i32) -> f32;

    #[method(name = "GetColumnHeight0", args = 0)]
    pub fn get_column_height0(self) -> f32;

    #[method(name = "GetColumnHeight1", args = 0)]
    pub fn get_column_height1(self) -> f32;

    #[method(name = "GetColumnHeight2", args = 0)]
    pub fn get_column_height2(self) -> f32;

    #[method(name = "GetColumnHeight3", args = 0)]
    pub fn get_column_height3(self) -> f32;

    #[method(name = "GetColumnHeight4", args = 0)]
    pub fn get_column_height4(self) -> f32;

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName3", args = 0)]
    pub fn get_column_name3(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName4", args = 0)]
    pub fn get_column_name4(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnEnglish0", args = 0)]
    pub fn get_column_english0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnEnglish1", args = 0)]
    pub fn get_column_english1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnEnglish2", args = 0)]
    pub fn get_column_english2(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnEnglish3", args = 0)]
    pub fn get_column_english3(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnEnglish4", args = 0)]
    pub fn get_column_english4(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnAlign", args = 1)]
    pub fn get_column_align(self, i: i32) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "GetColumnAlign0", args = 0)]
    pub fn get_column_align0(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "GetColumnAlign1", args = 0)]
    pub fn get_column_align1(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "GetColumnAlign2", args = 0)]
    pub fn get_column_align2(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "GetColumnAlign3", args = 0)]
    pub fn get_column_align3(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "GetColumnAlign4", args = 0)]
    pub fn get_column_align4(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "GetColumnColor", args = 1)]
    pub fn get_column_color(self, i: i32) -> crate::unity_engine::color::Color;

    #[method(name = "GetColumnColor0", args = 0)]
    pub fn get_column_color0(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetColumnColor1", args = 0)]
    pub fn get_column_color1(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetColumnColor2", args = 0)]
    pub fn get_column_color2(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetColumnColor3", args = 0)]
    pub fn get_column_color3(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetColumnColor4", args = 0)]
    pub fn get_column_color4(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetDisableColor", args = 0)]
    pub fn get_disable_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetMarginWidth", args = 0)]
    pub fn get_margin_width(self) -> f32;

    #[method(name = "GetMarginHeight", args = 0)]
    pub fn get_margin_height(self) -> f32;

    #[method(name = "GetKey", args = 1)]
    pub fn get_key(self, index: i32) -> i32;

    #[method(name = "GetMenu", args = 0)]
    pub fn get_menu(self) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "GetTextWidth", args = 1)]
    pub fn get_text_width(self, i: i32) -> f32;

    #[method(name = "GetTextHeight", args = 1)]
    pub fn get_text_height(self, i: i32) -> f32;

    #[method(name = "SetMenu", args = 1)]
    pub fn set_menu(self, menu: crate::app::debugmenu::DebugMenu) -> ();

    #[method(name = "GetState", args = 0)]
    pub fn get_state(self) -> crate::app::menuitem::MenuItem_State;

    #[method(name = "SetState", args = 1)]
    pub fn set_state(self, state: crate::app::menuitem::MenuItem_State) -> ();

    #[method(name = "GetBind", args = 0)]
    pub fn get_bind(self) -> i32;

    #[method(name = "SetBind", args = 1)]
    pub fn set_bind(self, bind: i32) -> ();

    #[method(name = "GetRoot", args = 0)]
    pub fn get_root(self) -> crate::app::procinst::ProcInst;

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();

    #[method(name = "CreateMenuBind", args = 0)]
    pub fn create_menu_bind(self) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "GetLangName", args = 1)]
    pub fn get_lang_name(self, i: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetLangHelp", args = 0)]
    pub fn get_lang_help(self) -> ::unity2::Il2CppString;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(p: crate::app::menuitem::MenuItem) -> crate::app::debugmenu::DebugMenu;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-menuitem")]
impl MenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/menuitem/MenuItem_Kind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MenuItem_Kind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MenuItem_Kind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MenuItem.Kind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MenuItem_Kind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MenuItem_Kind {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn group_begin() -> Self {
        Self { value: 1 }
    }

    pub fn group_end() -> Self {
        Self { value: 2 }
    }
}
