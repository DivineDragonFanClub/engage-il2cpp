
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguihandlerpanel/DebugUIHandlerPanel.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.UI", name = "DebugUIHandlerPanel")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DebugUIHandlerPanel {
    #[rename(name = "nameLabel")]
    pub name_label: crate::unity_engine::ui::text::Text,
    #[rename(name = "scrollRect")]
    pub scroll_rect: crate::unity_engine::ui::scrollrect::ScrollRect,
    #[rename(name = "viewport")]
    pub viewport: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_ScrollTransform")]
    pub m_scroll_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_ContentTransform")]
    pub m_content_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_MaskTransform")]
    pub m_mask_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_Panel")]
    pub m_panel: crate::unity_engine::rendering::debugui::DebugUI_Panel,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerpanel")]
#[::unity2::methods]
impl DebugUIHandlerPanel {
    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "SetPanel", args = 1)]
    pub fn set_panel(self, panel: crate::unity_engine::rendering::debugui::DebugUI_Panel) -> ();

    #[method(name = "GetPanel", args = 0)]
    pub fn get_panel(self) -> crate::unity_engine::rendering::debugui::DebugUI_Panel;

    #[method(name = "ScrollTo", args = 1)]
    pub fn scroll_to(
        self,
        target: crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
    ) -> ();

    #[method(name = "GetYPosInScroll", args = 1)]
    pub fn get_y_pos_in_scroll(
        self,
        target: crate::unity_engine::recttransform::RectTransform,
    ) -> f32;

    #[method(name = "GetFirstItem", args = 0)]
    pub fn get_first_item(
        self,
    ) -> crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerpanel")]
impl DebugUIHandlerPanel {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIHandlerPanel),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIHandlerPanelMethods>::ctor(this);
        this
    }
}
