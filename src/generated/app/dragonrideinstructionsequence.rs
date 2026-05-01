
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideinstructionsequence/DragonRideInstructionSequence.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideInstructionSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct DragonRideInstructionSequence {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_InstractionObject")]
    pub m_instraction_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-dragonrideinstructionsequence")]
#[::unity2::methods]
impl DragonRideInstructionSequence {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource(self) -> ();

    #[method(name = "UnloadResource", args = 0)]
    pub fn unload_resource(self) -> ();

    #[method(name = "IsResourceLoading", args = 0)]
    pub fn is_resource_loading(self) -> bool;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "PlayOpenSE", args = 0)]
    pub fn play_open_se(self) -> ();

    #[method(name = "PlayCloseSE", args = 0)]
    pub fn play_close_se(self) -> ();

    #[method(name = "TickInstruction", args = 0)]
    pub fn tick_instruction(self) -> ();

    #[method(name = "ToggleOperation", args = 1)]
    pub fn toggle_operation(obj: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "UpdateOperationObject", args = 1)]
    pub fn update_operation_object(obj: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-dragonrideinstructionsequence")]
impl DragonRideInstructionSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideInstructionSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideInstructionSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideinstructionsequence/DragonRideInstructionSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DragonRideInstructionSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DragonRideInstructionSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DragonRideInstructionSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DragonRideInstructionSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DragonRideInstructionSequence_Label {
    pub fn load() -> Self {
        Self { value: 0 }
    }

    pub fn tick_main() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}
