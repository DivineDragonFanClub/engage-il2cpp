
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_AreaString.md")))]
#[::unity2::class(namespace = "App", name = "HubSequence.AreaString")]
#[parent(crate::system::object::Object)]
pub struct HubSequence_AreaString {
    #[rename(name = "m_value")]
    pub m_value: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_lastValue")]
    pub m_last_value: ::unity2::Il2CppString,
}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_AreaString {
    #[method(name = "Push", args = 1)]
    pub fn push(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Pop", args = 1)]
    pub fn pop(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_AreaString {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_AreaString),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_AreaStringMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubSequence_LastChapterSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubSequence_LastChapterSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubSequence.LastChapterSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubSequence_LastChapterSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubSequence_LastChapterSequence_Label {
    pub fn menu() -> Self {
        Self { value: 0 }
    }

    pub fn go_to() -> Self {
        Self { value: 1 }
    }

    pub fn ranking() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterMenu_GoToMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "HubSequence.LastChapterMenu.GoToMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubSequence_LastChapterMenu_GoToMenuItem {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterMenu_GoToMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterMenu_GoToMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_LastChapterMenu_GoToMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_LastChapterMenu_GoToMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_ConfirmEndRollDialogItemYes.md")))]
#[::unity2::class(namespace = "App", name = "HubSequence.ConfirmEndRollDialogItemYes")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct HubSequence_ConfirmEndRollDialogItemYes {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_ConfirmEndRollDialogItemYes {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_ConfirmEndRollDialogItemYes {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_ConfirmEndRollDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_ConfirmEndRollDialogItemYesMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "HubSequence.LastChapterMenu.RankingMenuItem.ConfirmDialog.YesMenuItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_YesMenuItem {
    #[rename(name = "m_ResultFunc")]
    pub m_result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_YesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_YesMenuItem {
    pub fn new(
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_YesMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IHubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_YesMenuItemMethods > :: ctor (this , result_func) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "HubSequence.LastChapterMenu.RankingMenuItem.ConfirmDialog"
)]
#[parent(crate::system::object::Object)]
pub struct HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterSequence_ConfirmGotoLastDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App",
    name = "HubSequence.LastChapterSequence.ConfirmGotoLastDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct HubSequence_LastChapterSequence_ConfirmGotoLastDialogItemYes {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterSequence_ConfirmGotoLastDialogItemYes {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterSequence_ConfirmGotoLastDialogItemYes {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_LastChapterSequence_ConfirmGotoLastDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_LastChapterSequence_ConfirmGotoLastDialogItemYesMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubSequence.LastChapterMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct HubSequence_LastChapterMenu {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterMenu {
    #[method(name = "get_CurrentMenuSelect", args = 0)]
    pub fn get_current_menu_select() -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        super_: crate::app::procinst::ProcInst,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        super_: crate::app::procinst::ProcInst,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_LastChapterMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_LastChapterMenuMethods>::ctor(this, menu_item_list, super_);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterSequence_ConfirmDialogItemNo.md")))]
#[::unity2::class(
    namespace = "App",
    name = "HubSequence.LastChapterSequence.ConfirmDialogItemNo"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct HubSequence_LastChapterSequence_ConfirmDialogItemNo {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterSequence_ConfirmDialogItemNo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterSequence_ConfirmDialogItemNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_LastChapterSequence_ConfirmDialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_LastChapterSequence_ConfirmDialogItemNoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubSequence_Label {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn init() -> Self {
        Self { value: 1 }
    }

    pub fn reload() -> Self {
        Self { value: 2 }
    }

    pub fn main() -> Self {
        Self { value: 3 }
    }

    pub fn talk() -> Self {
        Self { value: 4 }
    }

    pub fn script() -> Self {
        Self { value: 5 }
    }

    pub fn fast_travel() -> Self {
        Self { value: 6 }
    }

    pub fn mascot() -> Self {
        Self { value: 7 }
    }

    pub fn arena() -> Self {
        Self { value: 8 }
    }

    pub fn pedestal() -> Self {
        Self { value: 9 }
    }

    pub fn dragon_ride() -> Self {
        Self { value: 10 }
    }

    pub fn fishing() -> Self {
        Self { value: 11 }
    }

    pub fn muscle_exercise() -> Self {
        Self { value: 12 }
    }

    pub fn ring_cleaning() -> Self {
        Self { value: 13 }
    }

    pub fn fortune_telling() -> Self {
        Self { value: 14 }
    }

    pub fn signboard() -> Self {
        Self { value: 15 }
    }

    pub fn jukebox() -> Self {
        Self { value: 16 }
    }

    pub fn rest_place() -> Self {
        Self { value: 17 }
    }

    pub fn chest() -> Self {
        Self { value: 18 }
    }

    pub fn amiibo() -> Self {
        Self { value: 19 }
    }

    pub fn cape_tower() -> Self {
        Self { value: 20 }
    }

    pub fn animal() -> Self {
        Self { value: 21 }
    }

    pub fn photo() -> Self {
        Self { value: 22 }
    }

    pub fn well() -> Self {
        Self { value: 23 }
    }

    pub fn pool() -> Self {
        Self { value: 24 }
    }

    pub fn fruit() -> Self {
        Self { value: 25 }
    }

    pub fn horse() -> Self {
        Self { value: 26 }
    }

    pub fn end_roll() -> Self {
        Self { value: 27 }
    }

    pub fn flea_market() -> Self {
        Self { value: 28 }
    }

    pub fn call_script() -> Self {
        Self { value: 29 }
    }

    pub fn kizuna_exit() -> Self {
        Self { value: 30 }
    }

    pub fn last_chapter() -> Self {
        Self { value: 31 }
    }

    pub fn exit() -> Self {
        Self { value: 32 }
    }

    pub fn end() -> Self {
        Self { value: 33 }
    }

    pub fn tail() -> Self {
        Self { value: 34 }
    }

    pub fn save_data_load() -> Self {
        Self { value: 35 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_NoMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "HubSequence.LastChapterMenu.RankingMenuItem.ConfirmDialog.NoMenuItem"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_NoMenuItem {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_NoMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_NoMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    HubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_NoMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IHubSequence_LastChapterMenu_RankingMenuItem_ConfirmDialog_NoMenuItemMethods > :: ctor (this ,) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterRankingMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "HubSequence.LastChapterRankingMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubSequence_LastChapterRankingMenuItem {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterRankingMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterRankingMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_LastChapterRankingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_LastChapterRankingMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterMenu_RankingMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "HubSequence.LastChapterMenu.RankingMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubSequence_LastChapterMenu_RankingMenuItem {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterMenu_RankingMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterMenu_RankingMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_LastChapterMenu_RankingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_LastChapterMenu_RankingMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterSequence.md")))]
#[::unity2::class(namespace = "App", name = "HubSequence.LastChapterSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: hubsequence :: HubSequence_LastChapterSequence >)]
pub struct HubSequence_LastChapterSequence {
    #[static_field]
    #[rename(name = "isGotoLastConfirm")]
    pub is_goto_last_confirm: bool,
}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterSequence {
    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CreateMenu", args = 0)]
    pub fn create_menu(self) -> ();

    #[method(name = "CallLastChapterDemo", args = 0)]
    pub fn call_last_chapter_demo(self) -> ();

    #[method(name = "CreateConfirmDialog", args = 0)]
    pub fn create_confirm_dialog(self) -> ();

    #[method(name = "CallLastChapter", args = 0)]
    pub fn call_last_chapter(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_LastChapterSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_LastChapterSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence.md")))]
#[::unity2::class(namespace = "App", name = "HubSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: hubsequence :: HubSequence >)]
pub struct HubSequence {
    #[rename(name = "m_ScriptFuncName")]
    pub m_script_func_name: ::unity2::Il2CppString,
    #[rename(name = "m_FastTravelID")]
    pub m_fast_travel_id: ::unity2::Il2CppString,
    #[rename(name = "m_TalkAccess")]
    pub m_talk_access: crate::app::hubaccess::HubAccess,
    #[rename(name = "m_IsBackgroundBind")]
    pub m_is_background_bind: bool,
    #[rename(name = "m_IsKeyHelp")]
    pub m_is_key_help: bool,
    #[rename(name = "m_IsCave")]
    pub m_is_cave: bool,
    #[rename(name = "m_SceneName")]
    pub m_scene_name: ::unity2::Il2CppString,
    #[rename(name = "m_StartName")]
    pub m_start_name: ::unity2::Il2CppString,
    #[rename(name = "m_HubRoot")]
    pub m_hub_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_HubEnv")]
    pub m_hub_env: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_HubLocatorGroup")]
    pub m_hub_locator_group: crate::app::hublocatorgroup::HubLocatorGroup,
    #[rename(name = "m_HubPlayerController")]
    pub m_hub_player_controller: crate::app::hubplayercontroller::HubPlayerController,
    #[rename(name = "m_HubCamera")]
    pub m_hub_camera: crate::app::hubcamera::HubCamera,
    #[rename(name = "m_ButtonNavi")]
    pub m_button_navi: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DragonRideNode")]
    pub m_dragon_ride_node: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DragonRideEnv")]
    pub m_dragon_ride_env: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FishingNode")]
    pub m_fishing_node: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Hologram")]
    pub m_hologram: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_HubEffect")]
    pub m_hub_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LensFlare")]
    pub m_lens_flare: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MyRoomAccObject")]
    pub m_my_room_acc_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MyRoomAcc")]
    pub m_my_room_acc: crate::app::resourcehandle_2::ResourceHandle_2,
    #[rename(name = "m_IsShutdown")]
    pub m_is_shutdown: bool,
    #[rename(name = "m_numPieceOfBond")]
    pub m_num_piece_of_bond: i32,
    #[rename(name = "EndRollDisableList")]
    pub end_roll_disable_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "ArgConfirmMid")]
    pub arg_confirm_mid: ::unity2::Il2CppString,
    #[rename(name = "ArgScriptName")]
    pub arg_script_name: ::unity2::Il2CppString,
    #[rename(name = "SolanelBgmList")]
    pub solanel_bgm_list: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence {
    #[method(name = "get_LoadingMode", args = 0)]
    pub fn get_loading_mode(self) -> crate::app::loadingmanager::LoadingManager_Modes;

    #[method(name = "get_Camera", args = 0)]
    pub fn get_camera(self) -> crate::app::hubcamera::HubCamera;

    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "get_Root", args = 0)]
    pub fn get_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_LocatorGroup", args = 0)]
    pub fn get_locator_group(self) -> crate::app::hublocatorgroup::HubLocatorGroup;

    #[method(name = "get_MainCamera", args = 0)]
    pub fn get_main_camera(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_MainCamera", args = 1)]
    pub fn set_main_camera(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_MiniMap", args = 0)]
    pub fn get_mini_map(self) -> crate::app::hubminimap::HubMiniMap;

    #[method(name = "set_MiniMap", args = 1)]
    pub fn set_mini_map(self, value: crate::app::hubminimap::HubMiniMap) -> ();

    #[method(name = "get_EnvRoot", args = 0)]
    pub fn get_env_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_NowArea", args = 0)]
    pub fn get_now_area(self) -> crate::app::hubsequence::HubSequence_AreaString;

    #[method(name = "set_NowArea", args = 1)]
    pub fn set_now_area(self, value: crate::app::hubsequence::HubSequence_AreaString) -> ();

    #[method(name = "get_FastTravel", args = 0)]
    pub fn get_fast_travel(self) -> crate::app::hubfasttravel::HubFastTravel;

    #[method(name = "set_FastTravel", args = 1)]
    pub fn set_fast_travel(self, value: crate::app::hubfasttravel::HubFastTravel) -> ();

    #[method(name = "get_ForceNonstopBGM", args = 0)]
    pub fn get_force_nonstop_bgm(self) -> bool;

    #[method(name = "set_ForceNonstopBGM", args = 1)]
    pub fn set_force_nonstop_bgm(self, value: bool) -> ();

    #[method(name = "get_AccessData", args = 0)]
    pub fn get_access_data(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::hubaccessmanager::HubAccessManager,
    >;

    #[method(name = "set_AccessData", args = 1)]
    pub fn set_access_data(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::hubaccessmanager::HubAccessManager,
        >,
    ) -> ();

    #[method(name = "get_CurrentAccessData", args = 0)]
    pub fn get_current_access_data(self) -> crate::app::hubaccessmanager::HubAccessManager;

    #[method(name = "get_SolanelAccessData", args = 0)]
    pub fn get_solanel_access_data(self) -> crate::app::hubaccessmanager::HubAccessManager;

    #[method(name = "get_IsEnterEvent", args = 0)]
    pub fn get_is_enter_event(self) -> bool;

    #[method(name = "set_IsEnterEvent", args = 1)]
    pub fn set_is_enter_event(self, value: bool) -> ();

    #[method(name = "get_IsMain", args = 0)]
    pub fn get_is_main(self) -> bool;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "IsResourceLoading", args = 0)]
    pub fn is_resource_loading(self) -> bool;

    #[method(name = "InitAfterLoadScene", args = 0)]
    pub fn init_after_load_scene(self) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "Terminate", args = 0)]
    pub fn terminate(self) -> ();

    #[method(name = "LoadHubScene", args = 0)]
    pub fn load_hub_scene(self) -> ();

    #[method(name = "SetupDispos", args = 2)]
    pub fn setup_dispos(
        self,
        scene_name: ::unity2::Il2CppString,
        timezone_type: crate::app::hubutil::HubUtil_TimezoneType,
    ) -> ();

    #[method(name = "ResetDispos", args = 0)]
    pub fn reset_dispos(self) -> ();

    #[method(name = "SetupDispos", args = 0)]
    pub fn setup_dispos_2(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "SetupEnv", args = 0)]
    pub fn setup_env(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "SetupPlayer", args = 0)]
    pub fn setup_player(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "OpeningEvent", args = 0)]
    pub fn opening_event(self) -> ();

    #[method(name = "MapOpeningEvent", args = 0)]
    pub fn map_opening_event(self) -> ();

    #[method(name = "PrepareEvent", args = 0)]
    pub fn prepare_event(self) -> ();

    #[method(name = "KizunaExit", args = 0)]
    pub fn kizuna_exit(self) -> ();

    #[method(name = "KizunaExitTutorial", args = 0)]
    pub fn kizuna_exit_tutorial(self) -> ();

    #[method(name = "MapEndingEvent", args = 0)]
    pub fn map_ending_event(self) -> ();

    #[method(name = "EndingEvent", args = 0)]
    pub fn ending_event(self) -> ();

    #[method(name = "SetupAccess", args = 0)]
    pub fn setup_access(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "LoadAPlusGift", args = 1)]
    pub fn load_a_plus_gift(self, aplus_name: ::unity2::Il2CppString) -> ();

    #[method(name = "InitAfterLoadChara", args = 0)]
    pub fn init_after_load_chara(self) -> ();

    #[method(name = "ArrangePlayerStartPosition", args = 0)]
    pub fn arrange_player_start_position(self) -> ();

    #[method(name = "LoadMinimap", args = 0)]
    pub fn load_minimap(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "CleanupUnitImpl", args = 0)]
    pub fn cleanup_unit_impl(self) -> ();

    #[method(name = "CallShop", args = 2)]
    pub fn call_shop(
        self,
        shop_type: ::unity2::Il2CppString,
        access: crate::app::hubaccess::HubAccess,
    ) -> ();

    #[method(name = "CleanupUnit", args = 0)]
    pub fn cleanup_unit() -> ();

    #[method(name = "CreateDragonRideSequence", args = 0)]
    pub fn create_dragon_ride_sequence(self) -> ();

    #[method(name = "CreateFishingSequence", args = 0)]
    pub fn create_fishing_sequence(self) -> ();

    #[method(name = "CreateMuscleExerciseSequence", args = 0)]
    pub fn create_muscle_exercise_sequence(self) -> ();

    #[method(name = "CreateFortuneTelling", args = 0)]
    pub fn create_fortune_telling(self) -> ();

    #[method(name = "EntrySignboard", args = 0)]
    pub fn entry_signboard(self) -> ();

    #[method(name = "CreateSignboard", args = 0)]
    pub fn create_signboard(self) -> ();

    #[method(name = "ExitSignboard", args = 0)]
    pub fn exit_signboard(self) -> ();

    #[method(name = "CreateRestPlace", args = 0)]
    pub fn create_rest_place(self) -> ();

    #[method(name = "CreateChest", args = 0)]
    pub fn create_chest(self) -> ();

    #[method(name = "ConfirmEndRoll", args = 0)]
    pub fn confirm_end_roll(self) -> ();

    #[method(name = "EnterEndRoll", args = 0)]
    pub fn enter_end_roll(self) -> ();

    #[method(name = "CreateEndRoll", args = 0)]
    pub fn create_end_roll(self) -> ();

    #[method(name = "ExitEndRoll", args = 0)]
    pub fn exit_end_roll(self) -> ();

    #[method(name = "CallFleaMarketUnmanned", args = 0)]
    pub fn call_flea_market_unmanned(self) -> ();

    #[method(name = "CreateFleaMarket", args = 0)]
    pub fn create_flea_market(self) -> ();

    #[method(name = "CreateJukebox", args = 0)]
    pub fn create_jukebox(self) -> ();

    #[method(name = "OpenCheckAnimal", args = 0)]
    pub fn open_check_animal(self) -> ();

    #[method(name = "CreateAnimal", args = 0)]
    pub fn create_animal(self) -> ();

    #[method(name = "CreatePhotoMenu", args = 0)]
    pub fn create_photo_menu(self) -> ();

    #[method(name = "CreateWellMenu", args = 0)]
    pub fn create_well_menu(self) -> ();

    #[method(name = "CreatePool", args = 0)]
    pub fn create_pool(self) -> ();

    #[method(name = "CreateFruit", args = 0)]
    pub fn create_fruit(self) -> ();

    #[method(name = "CreateHorse", args = 0)]
    pub fn create_horse(self) -> ();

    #[method(name = "EntryAmiibo", args = 0)]
    pub fn entry_amiibo(self) -> ();

    #[method(name = "CreateAmiibo", args = 0)]
    pub fn create_amiibo(self) -> ();

    #[method(name = "ExitAmiibo", args = 0)]
    pub fn exit_amiibo(self) -> ();

    #[method(name = "OpenCheckCapeTower", args = 0)]
    pub fn open_check_cape_tower(self) -> ();

    #[method(name = "OpenCapeDoor", args = 0)]
    pub fn open_cape_door(self) -> ();

    #[method(name = "CreateCapeTower", args = 0)]
    pub fn create_cape_tower(self) -> ();

    #[method(name = "CloseCapeTower", args = 0)]
    pub fn close_cape_tower(self) -> ();

    #[method(name = "FinalCapeTower", args = 0)]
    pub fn final_cape_tower(self) -> ();

    #[method(name = "CallScript", args = 2)]
    pub fn call_script(
        self,
        confirm_mid: ::unity2::Il2CppString,
        script_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "EntryCallScript", args = 0)]
    pub fn entry_call_script(self) -> ();

    #[method(name = "MainCallScript", args = 0)]
    pub fn main_call_script(self) -> ();

    #[method(name = "ExitCallScript", args = 0)]
    pub fn exit_call_script(self) -> ();

    #[method(name = "EntryKizunaExit", args = 0)]
    pub fn entry_kizuna_exit(self) -> ();

    #[method(name = "MainKizunaExit", args = 0)]
    pub fn main_kizuna_exit(self) -> ();

    #[method(name = "ExitKizunaExit", args = 0)]
    pub fn exit_kizuna_exit(self) -> ();

    #[method(name = "GetSunPosition", args = 0)]
    pub fn get_sun_position() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "EnableDragonRideGroup", args = 0)]
    pub fn enable_dragon_ride_group(self) -> ();

    #[method(name = "DisableDragonRideGroup", args = 0)]
    pub fn disable_dragon_ride_group(self) -> ();

    #[method(name = "EnableFishingGroup", args = 0)]
    pub fn enable_fishing_group(self) -> ();

    #[method(name = "DisableFishingGroup", args = 0)]
    pub fn disable_fishing_group(self) -> ();

    #[method(name = "DisableGroup", args = 0)]
    pub fn disable_group(self) -> ();

    #[method(name = "EnableGroup", args = 0)]
    pub fn enable_group(self) -> ();

    #[method(name = "DisableRoot", args = 0)]
    pub fn disable_root(self) -> ();

    #[method(name = "EnableRoot", args = 0)]
    pub fn enable_root(self) -> ();

    #[method(name = "LoadScript", args = 0)]
    pub fn load_script(self) -> ();

    #[method(name = "UnloadScript", args = 0)]
    pub fn unload_script(self) -> ();

    #[method(name = "EntryScript", args = 0)]
    pub fn entry_script(self) -> ();

    #[method(name = "StartScript", args = 0)]
    pub fn start_script(self) -> ();

    #[method(name = "ExitScript", args = 0)]
    pub fn exit_script(self) -> ();

    #[method(name = "StartBind", args = 2)]
    pub fn start_bind(
        self,
        target_0: crate::app::procinst::ProcInst,
        event_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "StartEvent", args = 2)]
    pub fn start_event(
        self,
        script_name: ::unity2::Il2CppString,
        access: crate::app::hubaccess::HubAccess,
    ) -> ();

    #[method(name = "AccessMascot", args = 0)]
    pub fn access_mascot(self) -> ();

    #[method(name = "EntryArena", args = 0)]
    pub fn entry_arena(self) -> ();

    #[method(name = "StartArena", args = 0)]
    pub fn start_arena(self) -> ();

    #[method(name = "ExitArena", args = 0)]
    pub fn exit_arena(self) -> ();

    #[method(name = "StartPedestal", args = 0)]
    pub fn start_pedestal(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "AccessPedestal", args = 0)]
    pub fn access_pedestal(self) -> ();

    #[method(name = "EndPedestal", args = 0)]
    pub fn end_pedestal(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "NextMap", args = 2)]
    pub fn next_map(scene_name: ::unity2::Il2CppString, start_name: ::unity2::Il2CppString) -> ();

    #[method(name = "NextChapter", args = 1)]
    pub fn next_chapter(overwrite_chapter: ::unity2::Il2CppString) -> ();

    #[method(name = "NextGmap", args = 0)]
    pub fn next_gmap() -> ();

    #[method(name = "IsExit", args = 0)]
    pub fn is_exit() -> bool;

    #[method(name = "Exit", args = 0)]
    pub fn exit() -> ();

    #[method(name = "Exit", args = 1)]
    pub fn exit_2(label: crate::app::mainsequence::MainSequence_Label) -> ();

    #[method(name = "CanMoveGmap", args = 0)]
    pub fn can_move_gmap() -> bool;

    #[method(name = "GetPlayerController", args = 0)]
    pub fn get_player_controller() -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "SaveDataLoad", args = 0)]
    pub fn save_data_load(self) -> ();

    #[method(name = "SaveDataLoadResult", args = 0)]
    pub fn save_data_load_result(self) -> ();

    #[method(name = "SaveDataRelease", args = 0)]
    pub fn save_data_release(self) -> ();

    #[method(name = "SaveDataNormalize", args = 0)]
    pub fn save_data_normalize(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "GetCurrentKeyHelpName", args = 0)]
    pub fn get_current_key_help_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Persistent", args = 0)]
    pub fn persistent(self) -> ();

    #[method(name = "SetPlayerAnim", args = 1)]
    pub fn set_player_anim(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "StartFastTravel", args = 0)]
    pub fn start_fast_travel(self) -> ();

    #[method(name = "CallFastTravel", args = 0)]
    pub fn call_fast_travel(self) -> ();

    #[method(name = "EndFastTravel", args = 0)]
    pub fn end_fast_travel(self) -> ();

    #[method(name = "RunFastTravel", args = 1)]
    pub fn run_fast_travel(self, id: ::unity2::Il2CppString) -> ();

    #[method(name = "StartAccessAnimal", args = 1)]
    pub fn start_access_animal(self, access: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "StartTalk", args = 1)]
    pub fn start_talk(self, access: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "Talk", args = 0)]
    pub fn talk(self) -> ();

    #[method(name = "BeginSilentEnv", args = 0)]
    pub fn begin_silent_env(self) -> ();

    #[method(name = "EndSilentEnv", args = 0)]
    pub fn end_silent_env(self) -> ();

    #[method(name = "GameCleaerEvent", args = 0)]
    pub fn game_cleaer_event(self) -> ();

    #[method(name = "GiftGet", args = 2)]
    pub fn gift_get(
        self,
        reward_id: ::unity2::Il2CppString,
        message_id: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "TryGiftEvent", args = 3)]
    pub fn try_gift_event(
        self,
        flag_name: ::unity2::Il2CppString,
        reward_id: ::unity2::Il2CppString,
        message_id: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "DlcGift0Event", args = 0)]
    pub fn dlc_gift0_event(self) -> ();

    #[method(name = "Patch0GiftEvent", args = 0)]
    pub fn patch0_gift_event(self) -> ();

    #[method(name = "DlcGift1Event", args = 0)]
    pub fn dlc_gift1_event(self) -> ();

    #[method(name = "Patch3GiftEvent", args = 0)]
    pub fn patch3_gift_event(self) -> ();

    #[method(name = "EvilCleaerEvent", args = 0)]
    pub fn evil_cleaer_event(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ContentsEvent", args = 0)]
    pub fn contents_event(self) -> ();

    #[method(name = "RelayTicketEvent", args = 0)]
    pub fn relay_ticket_event(self) -> ();

    #[method(name = "SkillOpenEvent", args = 0)]
    pub fn skill_open_event(self) -> ();

    #[method(name = "EnterEvent", args = 0)]
    pub fn enter_event(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "PostEnterEvent", args = 0)]
    pub fn post_enter_event(self) -> ();

    #[method(name = "PlayBgm", args = 1)]
    pub fn play_bgm(self, fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "PauseBgm", args = 1)]
    pub fn pause_bgm(self, fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "GetBgmEventName", args = 0)]
    pub fn get_bgm_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "TryGetCurrentBgmHandle", args = 0)]
    pub fn try_get_current_bgm_handle(self) -> crate::app::gamesound::GameSound_Handle;

    #[method(name = "PlayBgm_Kizuna", args = 1)]
    pub fn play_bgm_kizuna(
        self,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "PlayBgm_Hub", args = 1)]
    pub fn play_bgm_hub(
        self,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "StopBgm", args = 1)]
    pub fn stop_bgm(self, play_movie: bool) -> ();

    #[method(name = "StopBgm_Kizuna", args = 0)]
    pub fn stop_bgm_kizuna(self) -> ();

    #[method(name = "StopBgm_Hub", args = 0)]
    pub fn stop_bgm_hub(self) -> ();

    #[method(name = "PlayerBeginAccess", args = 0)]
    pub fn player_begin_access(self) -> ();

    #[method(name = "PlayerEndAccess", args = 0)]
    pub fn player_end_access(self) -> ();

    #[method(name = "IsExistInstance", args = 0)]
    pub fn is_exist_instance() -> bool;

    #[method(name = "GetCharaLocatorsRoot", args = 0)]
    pub fn get_chara_locators_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "InitSceneInfo", args = 0)]
    pub fn init_scene_info(self) -> ();

    #[method(name = "Report", args = 0)]
    pub fn report(self) -> ();

    #[method(name = "CreateSolanelBind", args = 1)]
    pub fn create_solanel_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateKizunaBind", args = 1)]
    pub fn create_kizuna_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_LastChapterGoToMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "HubSequence.LastChapterGoToMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubSequence_LastChapterGoToMenuItem {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_LastChapterGoToMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_LastChapterGoToMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_LastChapterGoToMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_LastChapterGoToMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsequence/HubSequence_ConfirmEndRollDialogItemNo.md")))]
#[::unity2::class(namespace = "App", name = "HubSequence.ConfirmEndRollDialogItemNo")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct HubSequence_ConfirmEndRollDialogItemNo {}

#[cfg(feature = "app-hubsequence")]
#[::unity2::methods]
impl HubSequence_ConfirmEndRollDialogItemNo {
    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubsequence")]
impl HubSequence_ConfirmEndRollDialogItemNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSequence_ConfirmEndRollDialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSequence_ConfirmEndRollDialogItemNoMethods>::ctor(this);
        this
    }
}
