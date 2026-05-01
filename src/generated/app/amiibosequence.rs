
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibosequence/AmiiboSequence_ItemType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AmiiboSequence_ItemType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AmiiboSequence_ItemType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AmiiboSequence.ItemType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AmiiboSequence_ItemType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AmiiboSequence_ItemType {
    pub fn item() -> Self {
        Self { value: 0 }
    }

    pub fn bgm() -> Self {
        Self { value: 1 }
    }

    pub fn dress_ticket() -> Self {
        Self { value: 2 }
    }

    pub fn relay_ticket() -> Self {
        Self { value: 3 }
    }

    pub fn kizuna() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibosequence/AmiiboSequence.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: amiibosequence :: AmiiboSequence >)]
pub struct AmiiboSequence {
    #[rename(name = "m_RetryCount")]
    pub m_retry_count: i32,
    #[static_field]
    #[rename(name = "RetryCountMax")]
    pub retry_count_max: i32,
    #[rename(name = "m_GainItemList")]
    pub m_gain_item_list: crate::system::collections::generic::list_1::List_1<
        crate::app::amiibosequence::AmiiboSequence_GainItemData,
    >,
    #[rename(name = "m_GainItemIndex")]
    pub m_gain_item_index: i32,
    #[rename(name = "m_DebugAlreadyGetOnceItem")]
    pub m_debug_already_get_once_item: bool,
    #[static_field]
    #[rename(name = "ReesetTimeSeconds")]
    pub reeset_time_seconds: i64,
}

#[cfg(feature = "app-amiibosequence")]
#[::unity2::methods]
impl AmiiboSequence {
    #[method(name = "FinalizeSystem", args = 0)]
    pub fn finalize_system(self) -> ();

    #[method(name = "AmiiboReadingMenu", args = 0)]
    pub fn amiibo_reading_menu(self) -> ();

    #[method(name = "InitializeSystem", args = 0)]
    pub fn initialize_system(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "Read", args = 0)]
    pub fn read(self) -> ();

    #[method(name = "CheckPassedDay", args = 2)]
    pub fn check_passed_day(seconds: i32, old_seconds: i32) -> bool;

    #[method(name = "ReceiveRelayTicket", args = 1)]
    pub fn receive_relay_ticket(self, num: i32) -> ();

    #[method(name = "ReceiveFirstItem", args = 2)]
    pub fn receive_first_item(
        self,
        data: crate::app::amiibodata::AmiiboData,
        numbering_id: i32,
    ) -> ();

    #[method(name = "ReceiveOtherAmiiboItem", args = 3)]
    pub fn receive_other_amiibo_item(
        self,
        data: crate::app::amiibodata::AmiiboData,
        seed_offset: i32,
        numbering_id: i32,
    ) -> ();

    #[method(name = "Mount", args = 0)]
    pub fn mount(self) -> ();

    #[method(name = "GetItem", args = 0)]
    pub fn get_item(self) -> ();

    #[method(name = "AlreadyGetItem", args = 0)]
    pub fn already_get_item(self) -> ();

    #[method(name = "JumpToMount", args = 0)]
    pub fn jump_to_mount(self) -> ();

    #[method(name = "SearchAmiibo", args = 0)]
    pub fn search_amiibo(self) -> bool;

    #[method(name = "TagLostStandbyReady", args = 0)]
    pub fn tag_lost_standby_ready(self) -> ();

    #[method(name = "AmiiboGameUserDataReset", args = 0)]
    pub fn amiibo_game_user_data_reset() -> ();

    #[method(name = "CheckPassedDay", args = 0)]
    pub fn check_passed_day_2() -> bool;

    #[method(name = "GetCanReceiveCount", args = 0)]
    pub fn get_can_receive_count() -> i32;

    #[method(name = "GetReceiveCountResetTime", args = 0)]
    pub fn get_receive_count_reset_time() -> i32;

    #[method(name = "GetReceiveCountResetTimeString", args = 0)]
    pub fn get_receive_count_reset_time_string() -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "UnloadRes", args = 0)]
    pub fn unload_res(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-amiibosequence")]
impl AmiiboSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibosequence/AmiiboSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AmiiboSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AmiiboSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AmiiboSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AmiiboSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AmiiboSequence_Label {
    pub fn amiibo_reading_menu() -> Self {
        Self { value: 0 }
    }

    pub fn initialize() -> Self {
        Self { value: 1 }
    }

    pub fn setup_device() -> Self {
        Self { value: 2 }
    }

    pub fn read_amiibo() -> Self {
        Self { value: 3 }
    }

    pub fn mount() -> Self {
        Self { value: 4 }
    }

    pub fn get_item() -> Self {
        Self { value: 5 }
    }

    pub fn already_get_item() -> Self {
        Self { value: 6 }
    }

    pub fn tag_lost_standby_ready() -> Self {
        Self { value: 7 }
    }

    pub fn end() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibosequence/AmiiboSequence_TagData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AmiiboSequence_TagData {
    pub character_id_base: u32,
    pub character_id_detail: u8,
    pub numbering_id: u16,
    pub series_id: u8,
    pub nfp_type: u8,
    pub name_base: ::unity2::Il2CppString,
    pub name_detail: ::unity2::Il2CppString,
}

impl ::unity2::ClassIdentity for AmiiboSequence_TagData {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AmiiboSequence.TagData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AmiiboSequence_TagData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-amiibosequence")]
#[::unity2::methods(value)]
impl AmiiboSequence_TagData {
    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        character_id_base: u32,
        character_id_detail: u8,
        numbering_id: u16,
        series_id: u8,
        nfp_type: u8,
        name_base: ::unity2::Il2CppString,
        name_detail: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibosequence/AmiiboSequence_GainItemData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AmiiboSequence_GainItemData {
    pub name: ::unity2::Il2CppString,
    pub num: i32,
    pub r#type: crate::app::amiibosequence::AmiiboSequence_ItemType,
}

impl ::unity2::ClassIdentity for AmiiboSequence_GainItemData {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AmiiboSequence.GainItemData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AmiiboSequence_GainItemData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
