
use crate::combat::situation_converter::baseconverter::BaseConverter;
use crate::combat::situation_converter::baseconverter::IBaseConverter;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/convertjobintro/ConvertJobIntro.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "ConvertJobIntro")]
#[parent(crate::combat::situation_converter::baseconverter::BaseConverter)]
pub struct ConvertJobIntro {
    #[rename(name = "m_State")]
    pub m_state: crate::combat::situation_converter::convertjobintro::ConvertJobIntro_State,
}

#[cfg(feature = "combat-situation_converter-convertjobintro")]
#[::unity2::methods]
impl ConvertJobIntro {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        situation: crate::combat::camerasituation::CameraSituation,
        arg: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;
}

#[cfg(feature = "combat-situation_converter-convertjobintro")]
impl ConvertJobIntro {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConvertJobIntro),
                ::core::stringify!(new),
            )
        });
        <Self as IConvertJobIntroMethods>::ctor(this, data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/convertjobintro/ConvertJobIntro_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ConvertJobIntro_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ConvertJobIntro_State {
    const NAMESPACE: &'static str = "Combat.SituationConverter";

    const NAME: &'static str = "ConvertJobIntro.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ConvertJobIntro_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ConvertJobIntro_State {
    pub fn before_critical() -> Self {
        Self { value: 0 }
    }

    pub fn critical_now() -> Self {
        Self { value: 1 }
    }

    pub fn after_critical() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}
