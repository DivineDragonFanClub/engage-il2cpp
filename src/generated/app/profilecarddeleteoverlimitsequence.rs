
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecarddeleteoverlimitsequence/ProfileCardDeleteOverLimitSequence.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardDeleteOverLimitSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ProfileCardDeleteOverLimitSequence {
    #[rename(name = "m_ProfileList")]
    pub m_profile_list: crate::app::profilelist::ProfileList,
    #[rename(name = "m_AlbumRoot")]
    pub m_album_root: crate::app::profilecardalbumroot::ProfileCardAlbumRoot,
    #[rename(name = "m_IsFadeOut")]
    pub m_is_fade_out: bool,
}

#[cfg(feature = "app-profilecarddeleteoverlimitsequence")]
#[::unity2::methods]
impl ProfileCardDeleteOverLimitSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, list: crate::app::profilelist::ProfileList) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "ReleaseResources", args = 0)]
    pub fn release_resources(self) -> ();

    #[method(name = "SetFade", args = 0)]
    pub fn set_fade(self) -> ();

    #[method(name = "RestoreFade", args = 0)]
    pub fn restore_fade(self) -> ();

    #[method(name = "MessageAtFirst", args = 0)]
    pub fn message_at_first(self) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create(self) -> ();

    #[method(name = "OpenList", args = 0)]
    pub fn open_list(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        list: crate::app::profilelist::ProfileList,
    ) -> ();
}

#[cfg(feature = "app-profilecarddeleteoverlimitsequence")]
impl ProfileCardDeleteOverLimitSequence {
    pub fn new(list: crate::app::profilelist::ProfileList) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardDeleteOverLimitSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardDeleteOverLimitSequenceMethods>::ctor(this, list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecarddeleteoverlimitsequence/ProfileCardDeleteOverLimitSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ProfileCardDeleteOverLimitSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ProfileCardDeleteOverLimitSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProfileCardDeleteOverLimitSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfileCardDeleteOverLimitSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ProfileCardDeleteOverLimitSequence_Label {
    pub fn list_menu() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}
