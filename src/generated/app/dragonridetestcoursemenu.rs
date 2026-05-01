
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridetestcoursemenu/DragonRideTestCourseMenu.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideTestCourseMenu")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DragonRideTestCourseMenu {
    #[rename(name = "m_CameraComp")]
    pub m_camera_comp: crate::app::dragonridecamera::DragonRideCamera,
    #[rename(name = "m_PatternList")]
    pub m_pattern_list: crate::app::structarraylist_1::StructArrayList_1<
        crate::app::dragonridetargetpatterndata::DragonRideTargetPatternData,
    >,
    #[rename(name = "m_PatternCount")]
    pub m_pattern_count: i32,
}

#[cfg(feature = "app-dragonridetestcoursemenu")]
#[::unity2::methods]
impl DragonRideTestCourseMenu {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();
}

#[cfg(feature = "app-dragonridetestcoursemenu")]
impl DragonRideTestCourseMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideTestCourseMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideTestCourseMenuMethods>::ctor(this);
        this
    }
}
