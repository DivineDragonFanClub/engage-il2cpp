
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugmanager/DebugManager.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugManager")]
#[parent(crate::system::object::Object)]
pub struct DebugManager {
# [static_field] # [rename (name = "kEnableDebugBtn1")] pub k_enable_debug_btn1 : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kEnableDebugBtn2")] pub k_enable_debug_btn2 : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kDebugPreviousBtn")] pub k_debug_previous_btn : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kDebugNextBtn")] pub k_debug_next_btn : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kValidateBtn")] pub k_validate_btn : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kPersistentBtn")] pub k_persistent_btn : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kDPadVertical")] pub k_d_pad_vertical : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kDPadHorizontal")] pub k_d_pad_horizontal : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kMultiplierBtn")] pub k_multiplier_btn : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kResetBtn")] pub k_reset_btn : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kEnableDebug")] pub k_enable_debug : :: unity2 :: Il2CppString ,
# [rename (name = "m_DebugActions")] pub m_debug_actions : :: unity2 :: Array < crate :: unity_engine :: rendering :: debugactiondesc :: DebugActionDesc > ,
# [rename (name = "m_DebugActionStates")] pub m_debug_action_states : :: unity2 :: Array < crate :: unity_engine :: rendering :: debugactionstate :: DebugActionState > ,
# [rename (name = "m_ReadOnlyPanels")] pub m_read_only_panels : crate :: system :: collections :: object_model :: readonlycollection_1 :: ReadOnlyCollection_1 < crate :: unity_engine :: rendering :: debugui :: DebugUI_Panel > ,
# [rename (name = "m_Panels")] pub m_panels : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: rendering :: debugui :: DebugUI_Panel > ,
# [rename (name = "onDisplayRuntimeUIChanged")] pub on_display_runtime_ui_changed : crate :: system :: action_1 :: Action_1 < bool > ,
# [rename (name = "onSetDirty")] pub on_set_dirty : crate :: system :: action :: Action ,
# [rename (name = "resetData")] pub reset_data : crate :: system :: action :: Action ,
# [rename (name = "refreshEditorRequested")] pub refresh_editor_requested : bool ,
# [rename (name = "m_Root")] pub m_root : crate :: unity_engine :: gameobject :: GameObject ,
# [rename (name = "m_RootUICanvas")] pub m_root_ui_canvas : crate :: unity_engine :: rendering :: ui :: debuguihandlercanvas :: DebugUIHandlerCanvas ,
# [rename (name = "m_PersistentRoot")] pub m_persistent_root : crate :: unity_engine :: gameobject :: GameObject ,
# [rename (name = "m_RootUIPersistentCanvas")] pub m_root_ui_persistent_canvas : crate :: unity_engine :: rendering :: ui :: debuguihandlerpersistentcanvas :: DebugUIHandlerPersistentCanvas ,
# [rename (name = "m_EditorOpen")] pub m_editor_open : bool ,
}

#[cfg(feature = "unity_engine-rendering-debugmanager")]
#[::unity2::methods]
impl DebugManager {
    #[method(name = "RegisterActions", args = 0)]
    pub fn register_actions(self) -> ();

    #[method(name = "AddAction", args = 2)]
    pub fn add_action(
        self,
        action: crate::unity_engine::rendering::debugaction::DebugAction,
        desc: crate::unity_engine::rendering::debugactiondesc::DebugActionDesc,
    ) -> ();

    #[method(name = "SampleAction", args = 1)]
    pub fn sample_action(self, action_index: i32) -> ();

    #[method(name = "UpdateAction", args = 1)]
    pub fn update_action(self, action_index: i32) -> ();

    #[method(name = "UpdateActions", args = 0)]
    pub fn update_actions(self) -> ();

    #[method(name = "GetAction", args = 1)]
    pub fn get_action(
        self,
        action: crate::unity_engine::rendering::debugaction::DebugAction,
    ) -> f32;

    #[method(name = "RegisterInputs", args = 0)]
    pub fn register_inputs(self) -> ();

    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> crate::unity_engine::rendering::debugmanager::DebugManager;

    #[method(name = "UpdateReadOnlyCollection", args = 0)]
    pub fn update_read_only_collection(self) -> ();

    #[method(name = "get_panels", args = 0)]
    pub fn get_panels(
        self,
    ) -> crate::system::collections::object_model::readonlycollection_1::ReadOnlyCollection_1<
        crate::unity_engine::rendering::debugui::DebugUI_Panel,
    >;

    #[method(name = "add_onDisplayRuntimeUIChanged", args = 1)]
    pub fn add_on_display_runtime_ui_changed(
        self,
        value: crate::system::action_1::Action_1<bool>,
    ) -> ();

    #[method(name = "remove_onDisplayRuntimeUIChanged", args = 1)]
    pub fn remove_on_display_runtime_ui_changed(
        self,
        value: crate::system::action_1::Action_1<bool>,
    ) -> ();

    #[method(name = "add_onSetDirty", args = 1)]
    pub fn add_on_set_dirty(self, value: crate::system::action::Action) -> ();

    #[method(name = "remove_onSetDirty", args = 1)]
    pub fn remove_on_set_dirty(self, value: crate::system::action::Action) -> ();

    #[method(name = "add_resetData", args = 1)]
    pub fn add_reset_data(self, value: crate::system::action::Action) -> ();

    #[method(name = "remove_resetData", args = 1)]
    pub fn remove_reset_data(self, value: crate::system::action::Action) -> ();

    #[method(name = "get_displayEditorUI", args = 0)]
    pub fn get_display_editor_ui(self) -> bool;

    #[method(name = "ToggleEditorUI", args = 1)]
    pub fn toggle_editor_ui(self, open: bool) -> ();

    #[method(name = "get_displayRuntimeUI", args = 0)]
    pub fn get_display_runtime_ui(self) -> bool;

    #[method(name = "set_displayRuntimeUI", args = 1)]
    pub fn set_display_runtime_ui(self, value: bool) -> ();

    #[method(name = "get_displayPersistentRuntimeUI", args = 0)]
    pub fn get_display_persistent_runtime_ui(self) -> bool;

    #[method(name = "set_displayPersistentRuntimeUI", args = 1)]
    pub fn set_display_persistent_runtime_ui(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "RefreshEditor", args = 0)]
    pub fn refresh_editor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "ReDrawOnScreenDebug", args = 0)]
    pub fn re_draw_on_screen_debug(self) -> ();

    #[method(name = "RegisterData", args = 1)]
    pub fn register_data(self, data: crate::unity_engine::rendering::idebugdata::IDebugData) -> ();

    #[method(name = "UnregisterData", args = 1)]
    pub fn unregister_data(
        self,
        data: crate::unity_engine::rendering::idebugdata::IDebugData,
    ) -> ();

    #[method(name = "GetState", args = 0)]
    pub fn get_state(self) -> i32;

    #[method(name = "RegisterRootCanvas", args = 1)]
    pub fn register_root_canvas(
        self,
        root: crate::unity_engine::rendering::ui::debuguihandlercanvas::DebugUIHandlerCanvas,
    ) -> ();

    #[method(name = "ChangeSelection", args = 2)]
    pub fn change_selection(
        self,
        widget: crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
        from_next: bool,
    ) -> ();

    #[method(name = "CheckPersistentCanvas", args = 0)]
    pub fn check_persistent_canvas(self) -> ();

    #[method(name = "TogglePersistent", args = 1)]
    pub fn toggle_persistent(
        self,
        widget: crate::unity_engine::rendering::debugui::DebugUI_Widget,
    ) -> ();

    #[method(name = "OnPanelDirty", args = 1)]
    pub fn on_panel_dirty(
        self,
        panel: crate::unity_engine::rendering::debugui::DebugUI_Panel,
    ) -> ();

    #[method(name = "GetPanel", args = 4)]
    pub fn get_panel(
        self,
        display_name: ::unity2::Il2CppString,
        create_if_null: bool,
        group_index: i32,
        override_if_exist: bool,
    ) -> crate::unity_engine::rendering::debugui::DebugUI_Panel;

    #[method(name = "RemovePanel", args = 1)]
    pub fn remove_panel(self, display_name: ::unity2::Il2CppString) -> ();

    #[method(name = "RemovePanel", args = 1)]
    pub fn remove_panel_2(
        self,
        panel: crate::unity_engine::rendering::debugui::DebugUI_Panel,
    ) -> ();

    #[method(name = "GetItem", args = 1)]
    pub fn get_item(
        self,
        query_path: ::unity2::Il2CppString,
    ) -> crate::unity_engine::rendering::debugui::DebugUI_Widget;

    #[method(name = "GetItem", args = 2)]
    pub fn get_item_2(
        self,
        query_path: ::unity2::Il2CppString,
        container: crate::unity_engine::rendering::debugui::DebugUI_IContainer,
    ) -> crate::unity_engine::rendering::debugui::DebugUI_Widget;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-debugmanager")]
impl DebugManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugManager),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugManagerMethods>::ctor(this);
        this
    }
}
