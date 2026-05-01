
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
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
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::app::yesnodialog::IYesNoDialog;
use crate::app::yesnodialog::YesNoDialog;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsequence/MyRoomSequence.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: myroomsequence :: MyRoomSequence >)]
pub struct MyRoomSequence {
    #[rename(name = "m_TopMenuResult")]
    pub m_top_menu_result: crate::app::myroomtopmenu::MyRoomTopMenu_MenuResult,
    #[rename(name = "m_RecallMenuResult")]
    pub m_recall_menu_result: crate::app::myroomrecallmenu::MyRoomRecallMenu_MenuResult,
    #[rename(name = "m_MenuItemResult")]
    pub m_menu_item_result: crate::app::basicmenu::BasicMenu_Result,
    #[static_field]
    #[rename(name = "m_relianceActiveMenu")]
    pub m_reliance_active_menu: crate::app::myroomsequence::MyRoomSequence_RelianceUnitMenuItem,
    #[rename(name = "messName")]
    pub mess_name: ::unity2::Il2CppString,
    #[rename(name = "demoName")]
    pub demo_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-myroomsequence")]
#[::unity2::methods]
impl MyRoomSequence {
    #[method(name = "ShowTitle", args = 0)]
    pub fn show_title() -> ();

    #[method(name = "HideTitle", args = 0)]
    pub fn hide_title() -> ();

    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "CreateTopMenu", args = 0)]
    pub fn create_top_menu(self) -> ();

    #[method(name = "CreateRecallMenu", args = 0)]
    pub fn create_recall_menu(self) -> ();

    #[method(name = "InitReliance", args = 0)]
    pub fn init_reliance(self) -> ();

    #[method(name = "MainReliance", args = 0)]
    pub fn main_reliance(self) -> ();

    #[method(name = "ExitReliance", args = 0)]
    pub fn exit_reliance(self) -> ();

    #[method(name = "InitWakeup", args = 0)]
    pub fn init_wakeup(self) -> ();

    #[method(name = "MainWakeup", args = 0)]
    pub fn main_wakeup(self) -> ();

    #[method(name = "ExitWakeup", args = 0)]
    pub fn exit_wakeup(self) -> ();

    #[method(name = "CreateSoundMenu", args = 0)]
    pub fn create_sound_menu(self) -> ();

    #[method(name = "CreateRelianceMenu", args = 0)]
    pub fn create_reliance_menu(self) -> ();

    #[method(name = "CreateWakeupMenu", args = 0)]
    pub fn create_wakeup_menu(self) -> ();

    #[method(name = "CreateMovieMenu", args = 0)]
    pub fn create_movie_menu(self) -> ();

    #[method(name = "CreateSetDifficulty", args = 0)]
    pub fn create_set_difficulty(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomsequence")]
impl MyRoomSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsequence/MyRoomSequence_RelianceListMenu.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomSequence.RelianceListMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MyRoomSequence_RelianceListMenu {}

#[cfg(feature = "app-myroomsequence")]
#[::unity2::methods]
impl MyRoomSequence_RelianceListMenu {
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

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-myroomsequence")]
impl MyRoomSequence_RelianceListMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        super_: crate::app::procinst::ProcInst,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSequence_RelianceListMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSequence_RelianceListMenuMethods>::ctor(this, menu_item_list, super_);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsequence/MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemNo.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MyRoomSequence.SleepConfirmDialog.ConfirmDialogItemNo"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemNo {}

#[cfg(feature = "app-myroomsequence")]
#[::unity2::methods]
impl MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemNo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomsequence")]
impl MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSequence_SleepConfirmDialog_ConfirmDialogItemNoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsequence/MyRoomSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn select() -> Self {
        Self { value: 1 }
    }

    pub fn reliance() -> Self {
        Self { value: 2 }
    }

    pub fn reliance_main() -> Self {
        Self { value: 3 }
    }

    pub fn wakeup_confirm() -> Self {
        Self { value: 4 }
    }

    pub fn wakeup_main() -> Self {
        Self { value: 5 }
    }

    pub fn wakeup_exit() -> Self {
        Self { value: 6 }
    }

    pub fn recall() -> Self {
        Self { value: 7 }
    }

    pub fn recall_reliance() -> Self {
        Self { value: 8 }
    }

    pub fn recall_wakeup() -> Self {
        Self { value: 9 }
    }

    pub fn recall_movie() -> Self {
        Self { value: 10 }
    }

    pub fn recall_music() -> Self {
        Self { value: 11 }
    }

    pub fn set_difficulty() -> Self {
        Self { value: 12 }
    }

    pub fn exit() -> Self {
        Self { value: 13 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsequence/MyRoomSequence_SleepConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomSequence.SleepConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct MyRoomSequence_SleepConfirmDialog {}

#[cfg(feature = "app-myroomsequence")]
#[::unity2::methods]
impl MyRoomSequence_SleepConfirmDialog {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-myroomsequence")]
impl MyRoomSequence_SleepConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSequence_SleepConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSequence_SleepConfirmDialogMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsequence/MyRoomSequence_RelianceUnitMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomSequence.RelianceUnitMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomSequence_RelianceUnitMenuItem {}

#[cfg(feature = "app-myroomsequence")]
#[::unity2::methods]
impl MyRoomSequence_RelianceUnitMenuItem {
    #[method(name = "get_Self", args = 0)]
    pub fn get_self(self) -> crate::app::persondata::PersonData;

    #[method(name = "get_Target", args = 0)]
    pub fn get_target(self) -> crate::app::persondata::PersonData;

    #[method(name = "get_CanLevelUp", args = 0)]
    pub fn get_can_level_up(self) -> bool;

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> crate::app::reliancedata::RelianceData_Level;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        self_: crate::app::persondata::PersonData,
        target: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-myroomsequence")]
impl MyRoomSequence_RelianceUnitMenuItem {
    pub fn new(
        self_: crate::app::persondata::PersonData,
        target: crate::app::persondata::PersonData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSequence_RelianceUnitMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSequence_RelianceUnitMenuItemMethods>::ctor(this, self_, target);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsequence/MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MyRoomSequence.SleepConfirmDialog.ConfirmDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemYes {}

#[cfg(feature = "app-myroomsequence")]
#[::unity2::methods]
impl MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemYes {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-myroomsequence")]
impl MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemYes {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSequence_SleepConfirmDialog_ConfirmDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSequence_SleepConfirmDialog_ConfirmDialogItemYesMethods>::ctor(this);
        this
    }
}
