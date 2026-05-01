
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguihandlercanvas/DebugUIHandlerCanvas.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.UI", name = "DebugUIHandlerCanvas")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DebugUIHandlerCanvas {
    #[rename(name = "m_DebugTreeState")]
    pub m_debug_tree_state: i32,
    #[rename(name = "m_PrefabsMap")]
    pub m_prefabs_map: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::SystemType,
        crate::unity_engine::transform::Transform,
    >,
    #[rename(name = "panelPrefab")]
    pub panel_prefab: crate::unity_engine::transform::Transform,
    #[rename(name = "prefabs")]
    pub prefabs: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::ui::debuguiprefabbundle::DebugUIPrefabBundle,
    >,
    #[rename(name = "m_UIPanels")]
    pub m_ui_panels: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::ui::debuguihandlerpanel::DebugUIHandlerPanel,
    >,
    #[rename(name = "m_SelectedPanel")]
    pub m_selected_panel: i32,
    #[rename(name = "m_SelectedWidget")]
    pub m_selected_widget:
        crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
    #[rename(name = "m_CurrentQueryPath")]
    pub m_current_query_path: ::unity2::Il2CppString,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlercanvas")]
#[::unity2::methods]
impl DebugUIHandlerCanvas {
    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "ResetAllHierarchy", args = 0)]
    pub fn reset_all_hierarchy(self) -> ();

    #[method(name = "Rebuild", args = 0)]
    pub fn rebuild(self) -> ();

    #[method(name = "Traverse", args = 3)]
    pub fn traverse(
        self,
        container: crate::unity_engine::rendering::debugui::DebugUI_IContainer,
        parent_transform: crate::unity_engine::transform::Transform,
        parent_ui_handler : crate :: unity_engine :: rendering :: ui :: debuguihandlerwidget :: DebugUIHandlerWidget,
    ) -> ();

    #[method(name = "GetWidgetFromPath", args = 1)]
    pub fn get_widget_from_path(
        self,
        query_path: ::unity2::Il2CppString,
    ) -> crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;

    #[method(name = "ActivatePanel", args = 2)]
    pub fn activate_panel(self, index: i32, try_and_keep_selection: bool) -> ();

    #[method(name = "ChangeSelection", args = 2)]
    pub fn change_selection(
        self,
        widget: crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
        from_next: bool,
    ) -> ();

    #[method(name = "SelectPreviousItem", args = 0)]
    pub fn select_previous_item(self) -> ();

    #[method(name = "SelectNextItem", args = 0)]
    pub fn select_next_item(self) -> ();

    #[method(name = "ChangeSelectionValue", args = 1)]
    pub fn change_selection_value(self, multiplier: f32) -> ();

    #[method(name = "ActivateSelection", args = 0)]
    pub fn activate_selection(self) -> ();

    #[method(name = "HandleInput", args = 0)]
    pub fn handle_input(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlercanvas")]
impl DebugUIHandlerCanvas {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIHandlerCanvas),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIHandlerCanvasMethods>::ctor(this);
        this
    }
}
