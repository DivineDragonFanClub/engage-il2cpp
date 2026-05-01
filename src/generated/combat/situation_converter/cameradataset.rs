
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/cameradataset/CameraDataSet.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "CameraDataSet")]
#[parent(crate::system::object::Object)]
pub struct CameraDataSet {}

#[cfg(feature = "combat-situation_converter-cameradataset")]
#[::unity2::methods]
impl CameraDataSet {
    #[method(name = "get_Record", args = 0)]
    pub fn get_record(self) -> crate::combat::combatrecord::CombatRecord;

    #[method(name = "get_Switch", args = 0)]
    pub fn get_switch(self) -> crate::combat::cameraswitch::CameraSwitch;

    #[method(name = "get_PosData", args = 0)]
    pub fn get_pos_data(self) -> crate::combat::camerapositiondata::CameraPositionData;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        record: crate::combat::combatrecord::CombatRecord,
        swt: crate::combat::cameraswitch::CameraSwitch,
        pos_data: crate::combat::camerapositiondata::CameraPositionData,
    ) -> ();
}

#[cfg(feature = "combat-situation_converter-cameradataset")]
impl CameraDataSet {
    pub fn new(
        record: crate::combat::combatrecord::CombatRecord,
        swt: crate::combat::cameraswitch::CameraSwitch,
        pos_data: crate::combat::camerapositiondata::CameraPositionData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraDataSet),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraDataSetMethods>::ctor(this, record, swt, pos_data);
        this
    }
}
