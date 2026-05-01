
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/force/Force.md")))]
#[::unity2::class(namespace = "App", name = "Force")]
#[parent(crate::system::object::Object)]
pub struct Force {
    #[rename(name = "m_Head")]
    pub m_head: crate::app::unit::Unit,
    #[rename(name = "m_Tail")]
    pub m_tail: crate::app::unit::Unit,
    #[rename(name = "m_Type")]
    pub m_type: crate::app::force::Force_Type,
}

#[cfg(feature = "app-force")]
#[::unity2::methods]
impl Force {
    #[method(name = "Get", args = 1)]
    pub fn get(r#type: crate::app::force::Force_Type) -> crate::app::force::Force;

    #[method(name = "GetUnitFromEmpty", args = 0)]
    pub fn get_unit_from_empty() -> crate::app::unit::Unit;

    #[method(name = "GetColor", args = 1)]
    pub fn get_color(r#type: crate::app::force::Force_Type) -> crate::unity_engine::color::Color;

    #[method(name = "GetName", args = 1)]
    pub fn get_name(r#type: crate::app::force::Force_Type) -> ::unity2::Il2CppString;

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, r#type: crate::app::force::Force_Type) -> ();

    #[method(name = "JoinFirst", args = 1)]
    pub fn join_first(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "JoinLast", args = 1)]
    pub fn join_last(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, ins_unit: crate::app::unit::Unit, prev_unit: crate::app::unit::Unit) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Transfer", args = 2)]
    pub fn transfer(self, r#type: crate::app::force::Force_Type, is_last: bool) -> ();

    #[method(name = "TransferForSortie", args = 2)]
    pub fn transfer_for_sortie(self, r#type: crate::app::force::Force_Type, is_last: bool) -> ();

    #[method(name = "IsAllied", args = 1)]
    pub fn is_allied(self, r#type: crate::app::force::Force_Type) -> bool;

    #[method(name = "GetCount", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "GetIndex", args = 1)]
    pub fn get_index(self, target: crate::app::unit::Unit) -> i32;

    #[method(name = "GetHeroUnit", args = 0)]
    pub fn get_hero_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitFromPerson", args = 1)]
    pub fn get_unit_from_person(
        self,
        person: crate::app::persondata::PersonData,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetUnitFromPerson", args = 1)]
    pub fn get_unit_from_person_2(self, pid: ::unity2::Il2CppString) -> crate::app::unit::Unit;

    #[method(name = "GetUnitFromFace", args = 2)]
    pub fn get_unit_from_face(
        self,
        person: crate::app::persondata::PersonData,
        consider_relay: bool,
    ) -> crate::app::unit::Unit;

    #[method(name = "get_First", args = 0)]
    pub fn get_first(self) -> crate::app::unit::Unit;

    #[method(name = "set_First", args = 1)]
    pub fn set_first(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_Last", args = 0)]
    pub fn get_last(self) -> crate::app::unit::Unit;

    #[method(name = "set_Last", args = 1)]
    pub fn set_last(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_ForceType", args = 0)]
    pub fn get_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "get_ForceMask", args = 0)]
    pub fn get_force_mask(self) -> u32;

    #[method(name = "GetMask", args = 1)]
    pub fn get_mask(r#type: crate::app::force::Force_Type) -> u32;

    #[method(name = "GetMask", args = 2)]
    pub fn get_mask_2(
        type1: crate::app::force::Force_Type,
        type2: crate::app::force::Force_Type,
    ) -> u32;

    #[method(name = "GetMask", args = 3)]
    pub fn get_mask_3(
        type1: crate::app::force::Force_Type,
        type2: crate::app::force::Force_Type,
        type3: crate::app::force::Force_Type,
    ) -> u32;

    #[method(name = "GetMask", args = 4)]
    pub fn get_mask_4(
        type1: crate::app::force::Force_Type,
        type2: crate::app::force::Force_Type,
        type3: crate::app::force::Force_Type,
        type4: crate::app::force::Force_Type,
    ) -> u32;

    #[method(name = "GetMaskOnUsed", args = 0)]
    pub fn get_mask_on_used() -> u32;

    #[method(name = "GetMaskOnMap", args = 0)]
    pub fn get_mask_on_map() -> u32;

    #[method(name = "GetMaskOnSortie", args = 0)]
    pub fn get_mask_on_sortie() -> u32;

    #[method(name = "GetMaskOnChapterSave", args = 0)]
    pub fn get_mask_on_chapter_save() -> u32;

    #[method(name = "GetMaskSameForce", args = 1)]
    pub fn get_mask_same_force(r#type: crate::app::force::Force_Type) -> u32;

    #[method(name = "IsPlayer", args = 1)]
    pub fn is_player(r#type: crate::app::force::Force_Type) -> bool;

    #[method(name = "IsOnMap", args = 1)]
    pub fn is_on_map(r#type: crate::app::force::Force_Type) -> bool;

    #[method(name = "IsOnMap", args = 1)]
    pub fn is_on_map_2(unit: crate::app::unit::Unit) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-force")]
impl Force {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Force),
                ::core::stringify!(new),
            )
        });
        <Self as IForceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/force/Force_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Force_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Force_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Force.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Force_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Force_Type {
    pub fn player() -> Self {
        Self { value: 0 }
    }

    pub fn enemy() -> Self {
        Self { value: 1 }
    }

    pub fn ally() -> Self {
        Self { value: 2 }
    }

    pub fn absent() -> Self {
        Self { value: 3 }
    }

    pub fn dead() -> Self {
        Self { value: 4 }
    }

    pub fn lost() -> Self {
        Self { value: 5 }
    }

    pub fn temporary() -> Self {
        Self { value: 6 }
    }

    pub fn empty() -> Self {
        Self { value: 7 }
    }

    pub fn num() -> Self {
        Self { value: 8 }
    }

    pub fn f1st() -> Self {
        Self { value: 0 }
    }

    pub fn f2nd() -> Self {
        Self { value: 1 }
    }

    pub fn f3rd() -> Self {
        Self { value: 2 }
    }

    pub fn map_num() -> Self {
        Self { value: 3 }
    }

    pub fn used_num() -> Self {
        Self { value: 7 }
    }
}
