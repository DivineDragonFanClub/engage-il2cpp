
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_SaveSequence_EndConfirmDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSequenceEditor.SaveSequence.EndConfirmDialog"
)]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct MapSequenceEditor_SaveSequence_EndConfirmDialog {}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_SaveSequence_EndConfirmDialog {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        decide_callback: crate::system::action::Action,
    ) -> ();
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_SaveSequence_EndConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceEditor_SaveSequence_EndConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceEditor_SaveSequence_EndConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_SaveSequence_UploadConfirmDialog_DialogItemNo.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSequenceEditor.SaveSequence.UploadConfirmDialog.DialogItemNo"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct MapSequenceEditor_SaveSequence_UploadConfirmDialog_DialogItemNo {
    #[rename(name = "ACallback")]
    pub a_callback: crate::system::action::Action,
}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_SaveSequence_UploadConfirmDialog_DialogItemNo {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, action: crate::system::action::Action, text: ::unity2::Il2CppString) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_SaveSequence_UploadConfirmDialog_DialogItemNo {
    pub fn new(action: crate::system::action::Action, text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceEditor_SaveSequence_UploadConfirmDialog_DialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceEditor_SaveSequence_UploadConfirmDialog_DialogItemNoMethods>::ctor(
            this, action, text,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceEditor")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: mapsequenceeditor :: MapSequenceEditor >)]
pub struct MapSequenceEditor {
    #[rename(name = "m_ObjectList")]
    pub m_object_list: crate::system::collections::generic::list_1::List_1<
        crate::app::mapeditorobjectdata::MapEditorObjectData,
    >,
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_MapEditSelect")]
    pub m_map_edit_select: crate::app::versusmapeditcontent::VersusMapEditContent,
    #[static_field]
    #[rename(name = "PutEffectPath")]
    pub put_effect_path: ::unity2::Il2CppString,
    #[rename(name = "m_OldX")]
    pub m_old_x: i32,
    #[rename(name = "m_OldZ")]
    pub m_old_z: i32,
    #[rename(name = "m_IsSettable")]
    pub m_is_settable: bool,
    #[rename(name = "m_PrevRotate")]
    pub m_prev_rotate: i32,
    #[rename(name = "m_IsPlayableFailure")]
    pub m_is_playable_failure: bool,
}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor {
    #[method(name = "get_SelectObject", args = 0)]
    pub fn get_select_object(self) -> crate::app::mapeditorobjectdata::MapEditorObjectData;

    #[method(name = "get_IsDisableInfo", args = 0)]
    pub fn get_is_disable_info(self) -> bool;

    #[method(name = "set_IsDisableInfo", args = 1)]
    pub fn set_is_disable_info(self, value: bool) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources(self) -> ();

    #[method(name = "MainMenu", args = 0)]
    pub fn main_menu(self) -> ();

    #[method(name = "BeginFreeCursor", args = 0)]
    pub fn begin_free_cursor(self) -> ();

    #[method(name = "UpdateSettableStatus", args = 0)]
    pub fn update_settable_status(self) -> ();

    #[method(name = "TickFreeCursor", args = 0)]
    pub fn tick_free_cursor(self) -> ();

    #[method(name = "UpdateRotateObject", args = 0)]
    pub fn update_rotate_object(self) -> bool;

    #[method(name = "UpdateCursor", args = 0)]
    pub fn update_cursor(self) -> ();

    #[method(name = "CanPut", args = 0)]
    pub fn can_put(self) -> bool;

    #[method(name = "UpdateCursorColor", args = 0)]
    pub fn update_cursor_color(self) -> ();

    #[method(name = "SetActiveUnit", args = 1)]
    pub fn set_active_unit(self, target: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateInfoPanels", args = 0)]
    pub fn update_info_panels(self) -> ();

    #[method(name = "UpdateKeyHelp", args = 0)]
    pub fn update_key_help(self) -> ();

    #[method(name = "CursorSound", args = 0)]
    pub fn cursor_sound(self) -> ();

    #[method(name = "UpdateFailureStatus", args = 0)]
    pub fn update_failure_status(self) -> ();

    #[method(name = "PlayFailure", args = 0)]
    pub fn play_failure(self) -> ();

    #[method(name = "PlayRotateSE", args = 0)]
    pub fn play_rotate_se(self) -> ();

    #[method(name = "PlayUndoSE", args = 0)]
    pub fn play_undo_se(self) -> ();

    #[method(name = "GetPlayerUnitCursor", args = 0)]
    pub fn get_player_unit_cursor(self) -> crate::app::unit::Unit;

    #[method(name = "IsSortieCursor", args = 0)]
    pub fn is_sortie_cursor(self) -> bool;

    #[method(name = "IsSettableArea", args = 0)]
    pub fn is_settable_area(self) -> bool;

    #[method(name = "EndFreeCursor", args = 0)]
    pub fn end_free_cursor(self) -> ();

    #[method(name = "ShowKeyHelp", args = 0)]
    pub fn show_key_help(self) -> ();

    #[method(name = "HideKeyHelp", args = 0)]
    pub fn hide_key_help(self) -> ();

    #[method(name = "TerrainInfoUpdate", args = 0)]
    pub fn terrain_info_update(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "SetMapObject", args = 1)]
    pub fn set_map_object(
        self,
        obj_data: crate::app::mapeditorobjectdata::MapEditorObjectData,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "InitializeSelectObject", args = 0)]
    pub fn initialize_select_object(self) -> ();

    #[method(name = "TryPrevSelectObject", args = 1)]
    pub fn try_prev_select_object(self, is_trigger: bool) -> bool;

    #[method(name = "TryNextSelectObject", args = 1)]
    pub fn try_next_select_object(self, is_trigger: bool) -> bool;

    #[method(name = "UpdateInfo", args = 0)]
    pub fn update_info(self) -> ();

    #[method(name = "PlayBgm", args = 0)]
    pub fn play_bgm(self) -> ();

    #[method(name = "StopBgm", args = 0)]
    pub fn stop_bgm(self) -> ();

    #[method(name = "get_EndConfirmTitleMid", args = 0)]
    pub fn get_end_confirm_title_mid() -> ::unity2::Il2CppString;

    #[method(name = "get_EndConfirmNoSaveTitleMid", args = 0)]
    pub fn get_end_confirm_no_save_title_mid() -> ::unity2::Il2CppString;

    #[method(name = "get_EndConfirmYesMid", args = 0)]
    pub fn get_end_confirm_yes_mid() -> ::unity2::Il2CppString;

    #[method(name = "get_EndConfirmNoMid", args = 0)]
    pub fn get_end_confirm_no_mid() -> ::unity2::Il2CppString;

    #[method(name = "IsCasualField", args = 0)]
    pub fn is_casual_field() -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceEditor),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceEditorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSequenceEditor.SaveSequence.UploadOverwriteConfirmDialog"
)]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog {
    #[rename(name = "m_BCallback")]
    pub m_b_callback: crate::system::action::Action,
}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        action: crate::system::action::Action,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        cancel_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        action: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
            action,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceEditor_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceEditor_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceEditor.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceEditor_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceEditor_Label {
    pub fn main_menu() -> Self {
        Self { value: 0 }
    }

    pub fn free_cursor() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSequenceEditor.ClearObjectsSequence.ClearObjectsConfirmDialog"
)]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog {}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        a_callback: crate::system::action::Action,
    ) -> ();
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog_YesItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSequenceEditor.ClearObjectsSequence.ClearObjectsConfirmDialog.YesItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog_YesItem {
    #[rename(name = "m_ACallback")]
    pub m_a_callback: crate::system::action::Action,
}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog_YesItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, a_callback: crate::system::action::Action) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog_YesItem {
    pub fn new(a_callback: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog_YesItem
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IMapSequenceEditor_ClearObjectsSequence_ClearObjectsConfirmDialog_YesItemMethods > :: ctor (this , a_callback) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_SaveSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceEditor_SaveSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceEditor_SaveSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceEditor.SaveSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceEditor_SaveSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceEditor_SaveSequence_Label {
    pub fn save() -> Self {
        Self { value: 0 }
    }

    pub fn save_end() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_SaveSequence_UploadConfirmDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSequenceEditor.SaveSequence.UploadConfirmDialog"
)]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct MapSequenceEditor_SaveSequence_UploadConfirmDialog {
    #[rename(name = "m_CancelCallback")]
    pub m_cancel_callback: crate::system::action::Action,
}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_SaveSequence_UploadConfirmDialog {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        cancel_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        cancel_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_SaveSequence_UploadConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        cancel_callback: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceEditor_SaveSequence_UploadConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceEditor_SaveSequence_UploadConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
            cancel_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_SaveSequence.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceEditor.SaveSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapSequenceEditor_SaveSequence {
    #[static_field]
    #[rename(name = "s_IsEndEdit")]
    pub s_is_end_edit: bool,
    #[rename(name = "m_ParentMenu")]
    pub m_parent_menu: crate::app::basicmenu::BasicMenu,
}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_SaveSequence {
    #[method(name = "OpenThemeMenu", args = 0)]
    pub fn open_theme_menu(self) -> ();

    #[method(name = "SaveMap", args = 0)]
    pub fn save_map(self) -> ();

    #[method(name = "IsValidSortie", args = 0)]
    pub fn is_valid_sortie(self) -> bool;

    #[method(name = "IsValidAbsent", args = 0)]
    pub fn is_valid_absent(self) -> bool;

    #[method(name = "IsEditRuleCheck", args = 0)]
    pub fn is_edit_rule_check(self) -> bool;

    #[method(name = "OpenUploadDialog", args = 0)]
    pub fn open_upload_dialog(self) -> ();

    #[method(name = "UploadMap", args = 0)]
    pub fn upload_map(self) -> ();

    #[method(name = "EndConfirm", args = 0)]
    pub fn end_confirm(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_SaveSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceEditor_SaveSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceEditor_SaveSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog_DialogItemNo.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSequenceEditor.SaveSequence.UploadOverwriteConfirmDialog.DialogItemNo"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog_DialogItemNo {
    #[rename(name = "ACallback")]
    pub a_callback: crate::system::action::Action,
}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog_DialogItemNo {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, action: crate::system::action::Action) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog_DialogItemNo {
    pub fn new(action: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog_DialogItemNo
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IMapSequenceEditor_SaveSequence_UploadOverwriteConfirmDialog_DialogItemNoMethods > :: ctor (this , action) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_ClearObjectsSequence.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceEditor.ClearObjectsSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapSequenceEditor_ClearObjectsSequence {}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_ClearObjectsSequence {
    #[method(name = "ClearObjects", args = 0)]
    pub fn clear_objects(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_ClearObjectsSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceEditor_ClearObjectsSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceEditor_ClearObjectsSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceeditor/MapSequenceEditor_SaveSequence_EndConfirmDialog_ConfirmYesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSequenceEditor.SaveSequence.EndConfirmDialog.ConfirmYesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct MapSequenceEditor_SaveSequence_EndConfirmDialog_ConfirmYesDialogItem {
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::system::action::Action,
}

#[cfg(feature = "app-mapsequenceeditor")]
#[::unity2::methods]
impl MapSequenceEditor_SaveSequence_EndConfirmDialog_ConfirmYesDialogItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        text: ::unity2::Il2CppString,
        decide_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapsequenceeditor")]
impl MapSequenceEditor_SaveSequence_EndConfirmDialog_ConfirmYesDialogItem {
    pub fn new(
        text: ::unity2::Il2CppString,
        decide_callback: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MapSequenceEditor_SaveSequence_EndConfirmDialog_ConfirmYesDialogItem
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IMapSequenceEditor_SaveSequence_EndConfirmDialog_ConfirmYesDialogItemMethods > :: ctor (this , text , decide_callback) ;
        this
    }
}
