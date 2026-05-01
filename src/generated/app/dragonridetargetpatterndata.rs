
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridetargetpatterndata/DragonRideTargetPatternData.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideTargetPatternData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: dragonridetargetpatterndata :: DragonRideTargetPatternData >)]
pub struct DragonRideTargetPatternData {
    #[static_field]
    #[rename(name = "cPatternWidthMax")]
    pub c_pattern_width_max: i32,
    #[static_field]
    #[rename(name = "cPatternHeightMax")]
    pub c_pattern_height_max: i32,
}

#[cfg(feature = "app-dragonridetargetpatterndata")]
#[::unity2::methods]
impl DragonRideTargetPatternData {
    #[method(name = "get_Targets", args = 0)]
    pub fn get_targets(self) -> ::unity2::Array<i32>;

    #[method(name = "set_Targets", args = 1)]
    pub fn set_targets(self, value: ::unity2::Array<i32>) -> ();

    #[method(name = "get_Target1", args = 0)]
    pub fn get_target1(self) -> i32;

    #[method(name = "set_Target1", args = 1)]
    pub fn set_target1(self, value: i32) -> ();

    #[method(name = "get_Target2", args = 0)]
    pub fn get_target2(self) -> i32;

    #[method(name = "set_Target2", args = 1)]
    pub fn set_target2(self, value: i32) -> ();

    #[method(name = "get_Target3", args = 0)]
    pub fn get_target3(self) -> i32;

    #[method(name = "set_Target3", args = 1)]
    pub fn set_target3(self, value: i32) -> ();

    #[method(name = "get_Target4", args = 0)]
    pub fn get_target4(self) -> i32;

    #[method(name = "set_Target4", args = 1)]
    pub fn set_target4(self, value: i32) -> ();

    #[method(name = "get_Target5", args = 0)]
    pub fn get_target5(self) -> i32;

    #[method(name = "set_Target5", args = 1)]
    pub fn set_target5(self, value: i32) -> ();

    #[method(name = "get_Target6", args = 0)]
    pub fn get_target6(self) -> i32;

    #[method(name = "set_Target6", args = 1)]
    pub fn set_target6(self, value: i32) -> ();

    #[method(name = "get_Target7", args = 0)]
    pub fn get_target7(self) -> i32;

    #[method(name = "set_Target7", args = 1)]
    pub fn set_target7(self, value: i32) -> ();

    #[method(name = "get_Target8", args = 0)]
    pub fn get_target8(self) -> i32;

    #[method(name = "set_Target8", args = 1)]
    pub fn set_target8(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();
}

#[cfg(feature = "app-dragonridetargetpatterndata")]
impl DragonRideTargetPatternData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideTargetPatternData),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideTargetPatternDataMethods>::ctor(this);
        this
    }
}
