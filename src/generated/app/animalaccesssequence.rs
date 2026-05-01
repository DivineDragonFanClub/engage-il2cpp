
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalaccesssequence/AnimalAccessSequence.md")))]
#[::unity2::class(namespace = "App", name = "AnimalAccessSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: animalaccesssequence :: AnimalAccessSequence >)]
pub struct AnimalAccessSequence {
    #[rename(name = "m_AnimalCamera")]
    pub m_animal_camera: crate::app::animalaccesscamera::AnimalAccessCamera,
}

#[cfg(feature = "app-animalaccesssequence")]
#[::unity2::methods]
impl AnimalAccessSequence {
    #[method(name = "get_Access", args = 0)]
    pub fn get_access(self) -> crate::app::hubaccess::HubAccess;

    #[method(name = "set_Access", args = 1)]
    pub fn set_access(self, value: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "get_Animal", args = 0)]
    pub fn get_animal(self) -> crate::app::animaldata::AnimalData;

    #[method(name = "set_Animal", args = 1)]
    pub fn set_animal(self, value: crate::app::animaldata::AnimalData) -> ();

    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "get_Camera", args = 0)]
    pub fn get_camera(self) -> crate::app::hubcamera::HubCamera;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, access: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Main", args = 0)]
    pub fn main(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        access: crate::app::hubaccess::HubAccess,
    ) -> crate::app::procinst::ProcInst;
}

#[cfg(feature = "app-animalaccesssequence")]
impl AnimalAccessSequence {
    pub fn new(access: crate::app::hubaccess::HubAccess) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalAccessSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalAccessSequenceMethods>::ctor(this, access);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalaccesssequence/AnimalAccessSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AnimalAccessSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AnimalAccessSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AnimalAccessSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimalAccessSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AnimalAccessSequence_Label {
    pub fn main() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}
