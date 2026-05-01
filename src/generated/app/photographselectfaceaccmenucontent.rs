
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectfaceaccmenucontent/PhotographSelectFaceAccMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectFaceAccMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct PhotographSelectFaceAccMenuContent {}

#[cfg(feature = "app-photographselectfaceaccmenucontent")]
#[::unity2::methods]
impl PhotographSelectFaceAccMenuContent {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        all_menu_content: crate::app::photographallmenucontent::PhotographAllMenuContent,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
    ) -> ();

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "CalcCursorMovedPosX", args = 1)]
    pub fn calc_cursor_moved_pos_x(self, menu_item_idx: i32) -> f32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_idx: i32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-photographselectfaceaccmenucontent")]
impl PhotographSelectFaceAccMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectFaceAccMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectFaceAccMenuContentMethods>::ctor(this);
        this
    }
}
