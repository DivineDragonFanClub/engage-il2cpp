
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/proportionparameters/ProportionParameters.md")))]
#[::unity2::class(namespace = "Combat", name = "ProportionParameters")]
#[parent(crate::system::object::Object)]
pub struct ProportionParameters {
    #[rename(name = "ScaleAll")]
    pub scale_all: f32,
    #[rename(name = "ScaleHead")]
    pub scale_head: f32,
    #[rename(name = "ScaleNeck")]
    pub scale_neck: f32,
    #[rename(name = "ScaleTorso")]
    pub scale_torso: f32,
    #[rename(name = "ScaleShoulders")]
    pub scale_shoulders: f32,
    #[rename(name = "ScaleArms")]
    pub scale_arms: f32,
    #[rename(name = "ScaleHands")]
    pub scale_hands: f32,
    #[rename(name = "ScaleLegs")]
    pub scale_legs: f32,
    #[rename(name = "ScaleFeet")]
    pub scale_feet: f32,
    #[rename(name = "VolumeArms")]
    pub volume_arms: f32,
    #[rename(name = "VolumeLegs")]
    pub volume_legs: f32,
    #[rename(name = "VolumeBust")]
    pub volume_bust: f32,
    #[rename(name = "VolumeAbdomen")]
    pub volume_abdomen: f32,
    #[rename(name = "VolumeTorso")]
    pub volume_torso: f32,
    #[rename(name = "HipJointHeight")]
    pub hip_joint_height: f32,
    #[rename(name = "AnkleHeight")]
    pub ankle_height: f32,
    #[rename(name = "targetNodes")]
    pub target_nodes: ::unity2::Array<crate::unity_engine::transform::Transform>,
    #[rename(name = "proportionScales")]
    pub proportion_scales: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "writeCount")]
    pub write_count: i32,
}

#[cfg(feature = "combat-proportionparameters")]
#[::unity2::methods]
impl ProportionParameters {
    #[method(name = "ImportLegScaleParamsFromModel", args = 1)]
    pub fn import_leg_scale_params_from_model(
        self,
        root: crate::unity_engine::transform::Transform,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, rhs: crate::combat::proportionparameters::ProportionParameters) -> ();

    #[method(name = "ResetToOne", args = 0)]
    pub fn reset_to_one(self) -> ();

    #[method(name = "GetClipboardFormedString", args = 0)]
    pub fn get_clipboard_formed_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Random", args = 0)]
    pub fn random(self) -> ();

    #[method(name = "get_IsValid", args = 0)]
    pub fn get_is_valid(self) -> bool;

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, rhs: crate::combat::proportionparameters::ProportionParameters) -> ();

    #[method(name = "ImportFromAssetResult", args = 1)]
    pub fn import_from_asset_result(self, r: crate::app::assettable::AssetTable_Result) -> ();

    #[method(name = "DumpToString", args = 0)]
    pub fn dump_to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "DumpToShortString", args = 0)]
    pub fn dump_to_short_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Calculate", args = 1)]
    pub fn calculate(self, j: crate::combat::characterjoint::CharacterJoint) -> ();

    #[method(name = "Flush", args = 0)]
    pub fn flush(self) -> ();
}

#[cfg(feature = "combat-proportionparameters")]
impl ProportionParameters {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProportionParameters),
                ::core::stringify!(new),
            )
        });
        <Self as IProportionParametersMethods>::ctor(this);
        this
    }

    pub fn new_2(rhs: crate::combat::proportionparameters::ProportionParameters) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProportionParameters),
                ::core::stringify!(new_2),
            )
        });
        <Self as IProportionParametersMethods>::ctor_2(this, rhs);
        this
    }
}
