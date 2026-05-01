
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayprofilesequence/RelayProfileSequence_DownloadSequence_DownloadYesNoDialog_YesItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RelayProfileSequence.DownloadSequence.DownloadYesNoDialog.YesItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct RelayProfileSequence_DownloadSequence_DownloadYesNoDialog_YesItem {
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::system::action::Action,
}

#[cfg(feature = "app-relayprofilesequence")]
#[::unity2::methods]
impl RelayProfileSequence_DownloadSequence_DownloadYesNoDialog_YesItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, decide_callback: crate::system::action::Action) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-relayprofilesequence")]
impl RelayProfileSequence_DownloadSequence_DownloadYesNoDialog_YesItem {
    pub fn new(decide_callback: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    RelayProfileSequence_DownloadSequence_DownloadYesNoDialog_YesItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayProfileSequence_DownloadSequence_DownloadYesNoDialog_YesItemMethods>::ctor(
            this,
            decide_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayprofilesequence/RelayProfileSequence_UploadSequence.md")))]
#[::unity2::class(namespace = "App", name = "RelayProfileSequence.UploadSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct RelayProfileSequence_UploadSequence {
    #[rename(name = "m_Profile")]
    pub m_profile: crate::app::profilecard::ProfileCard,
}

#[cfg(feature = "app-relayprofilesequence")]
#[::unity2::methods]
impl RelayProfileSequence_UploadSequence {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Sanitize", args = 0)]
    pub fn sanitize(self) -> ();

    #[method(name = "Upload", args = 0)]
    pub fn upload(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-relayprofilesequence")]
impl RelayProfileSequence_UploadSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayProfileSequence_UploadSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayProfileSequence_UploadSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayprofilesequence/RelayProfileSequence_DownloadSequence.md")))]
#[::unity2::class(namespace = "App", name = "RelayProfileSequence.DownloadSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct RelayProfileSequence_DownloadSequence {
    #[rename(name = "m_PrincipalId")]
    pub m_principal_id: u64,
    #[rename(name = "m_PlayerIds")]
    pub m_player_ids: crate::system::collections::generic::list_1::List_1<u64>,
    #[rename(name = "m_PlayerIdIndex")]
    pub m_player_id_index: i32,
    #[rename(name = "m_Profiles")]
    pub m_profiles:
        crate::system::collections::generic::list_1::List_1<crate::app::profilecard::ProfileCard>,
}

#[cfg(feature = "app-relayprofilesequence")]
#[::unity2::methods]
impl RelayProfileSequence_DownloadSequence {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        principal_id: u64,
        player_ids: crate::system::collections::generic::list_1::List_1<u64>,
    ) -> ();

    #[method(name = "Download", args = 0)]
    pub fn download(self) -> ();

    #[method(name = "Postdownload", args = 0)]
    pub fn postdownload(self) -> ();

    #[method(name = "NextPlayer", args = 0)]
    pub fn next_player(self) -> ();

    #[method(name = "Sanitize", args = 0)]
    pub fn sanitize(self) -> ();

    #[method(name = "ConfirmSave", args = 0)]
    pub fn confirm_save(self) -> ();

    #[method(name = "Save", args = 0)]
    pub fn save(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        principal_id: u64,
        player_ids: crate::system::collections::generic::list_1::List_1<u64>,
    ) -> ();
}

#[cfg(feature = "app-relayprofilesequence")]
impl RelayProfileSequence_DownloadSequence {
    pub fn new(
        principal_id: u64,
        player_ids: crate::system::collections::generic::list_1::List_1<u64>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayProfileSequence_DownloadSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayProfileSequence_DownloadSequenceMethods>::ctor(
            this,
            principal_id,
            player_ids,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayprofilesequence/RelayProfileSequence_DownloadSequence_DownloadYesNoDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RelayProfileSequence.DownloadSequence.DownloadYesNoDialog"
)]
#[parent(crate::system::object::Object)]
pub struct RelayProfileSequence_DownloadSequence_DownloadYesNoDialog {}

#[cfg(feature = "app-relayprofilesequence")]
#[::unity2::methods]
impl RelayProfileSequence_DownloadSequence_DownloadYesNoDialog {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        mess: ::unity2::Il2CppString,
        decide_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relayprofilesequence")]
impl RelayProfileSequence_DownloadSequence_DownloadYesNoDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayProfileSequence_DownloadSequence_DownloadYesNoDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayProfileSequence_DownloadSequence_DownloadYesNoDialogMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayprofilesequence/RelayProfileSequence.md")))]
#[::unity2::class(namespace = "App", name = "RelayProfileSequence")]
#[parent(crate::system::object::Object)]
pub struct RelayProfileSequence {}

#[cfg(feature = "app-relayprofilesequence")]
#[::unity2::methods]
impl RelayProfileSequence {
    #[method(name = "CreateBindUpload", args = 1)]
    pub fn create_bind_upload(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindDownload", args = 1)]
    pub fn create_bind_download(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetPrincipalId", args = 0)]
    pub fn get_principal_id() -> u64;

    #[method(name = "GetPlayerIds", args = 0)]
    pub fn get_player_ids() -> crate::system::collections::generic::list_1::List_1<u64>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relayprofilesequence")]
impl RelayProfileSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayProfileSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayProfileSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayprofilesequence/RelayProfileSequence_DownloadSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelayProfileSequence_DownloadSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelayProfileSequence_DownloadSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayProfileSequence.DownloadSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayProfileSequence_DownloadSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelayProfileSequence_DownloadSequence_Label {
    pub fn download() -> Self {
        Self { value: 0 }
    }

    pub fn sanitize() -> Self {
        Self { value: 1 }
    }

    pub fn save() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}
