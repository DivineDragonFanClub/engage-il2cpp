
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::ui::horizontalorverticallayoutgroup::HorizontalOrVerticalLayoutGroup;
use crate::unity_engine::ui::horizontalorverticallayoutgroup::IHorizontalOrVerticalLayoutGroup;
use crate::unity_engine::ui::layoutgroup::ILayoutGroup;
use crate::unity_engine::ui::layoutgroup::LayoutGroup;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/verticallayoutgroup/VerticalLayoutGroup.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "VerticalLayoutGroup")]
#[parent(crate::unity_engine::ui::horizontalorverticallayoutgroup::HorizontalOrVerticalLayoutGroup)]
pub struct VerticalLayoutGroup {}

#[cfg(feature = "unity_engine-ui-verticallayoutgroup")]
#[::unity2::methods]
impl VerticalLayoutGroup {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CalculateLayoutInputHorizontal", args = 0)]
    pub fn calculate_layout_input_horizontal(self) -> ();

    #[method(name = "CalculateLayoutInputVertical", args = 0)]
    pub fn calculate_layout_input_vertical(self) -> ();

    #[method(name = "SetLayoutHorizontal", args = 0)]
    pub fn set_layout_horizontal(self) -> ();

    #[method(name = "SetLayoutVertical", args = 0)]
    pub fn set_layout_vertical(self) -> ();
}

#[cfg(feature = "unity_engine-ui-verticallayoutgroup")]
impl VerticalLayoutGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VerticalLayoutGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IVerticalLayoutGroupMethods>::ctor(this);
        this
    }
}
