
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/random_2/Random_2.md")))]
#[::unity2::class(namespace = "App", name = "Random")]
#[parent(crate::system::object::Object)]
pub struct Random_2 {
    #[static_field]
    #[rename(name = "s_Rand")]
    pub s_rand: ::unity2::Array<crate::app::random_2::Random_2>,
    #[rename(name = "m_Seed")]
    pub m_seed: crate::app::randomseed::RandomSeed,
}

#[cfg(feature = "app-random_2")]
#[::unity2::methods]
impl Random_2 {
    #[method(name = "InitializeAll", args = 0)]
    pub fn initialize_all() -> ();

    #[method(name = "InitializeAll", args = 1)]
    pub fn initialize_all_2(v: u32) -> ();

    #[method(name = "GetInstance", args = 1)]
    pub fn get_instance(
        r#type: crate::app::random_2::Random_Type,
    ) -> crate::app::random_2::Random_2;

    #[method(name = "IsSave", args = 1)]
    pub fn is_save(r#type: crate::app::random_2::Random_Type) -> bool;

    #[method(name = "get_System", args = 0)]
    pub fn get_system() -> crate::app::random_2::Random_2;

    #[method(name = "get_Game", args = 0)]
    pub fn get_game() -> crate::app::random_2::Random_2;

    #[method(name = "get_Spot", args = 0)]
    pub fn get_spot() -> crate::app::random_2::Random_2;

    #[method(name = "get_Hub", args = 0)]
    pub fn get_hub() -> crate::app::random_2::Random_2;

    #[method(name = "get_HubItem", args = 0)]
    pub fn get_hub_item() -> crate::app::random_2::Random_2;

    #[method(name = "get_KillBonus", args = 0)]
    pub fn get_kill_bonus() -> crate::app::random_2::Random_2;

    #[method(name = "get_Combat", args = 0)]
    pub fn get_combat() -> crate::app::random_2::Random_2;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, seed: u32) -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "Initialize", args = 1)]
    pub fn initialize_2(self, v: u32) -> ();

    #[method(name = "Peek", args = 0)]
    pub fn peek(self) -> i32;

    #[method(name = "Spin", args = 1)]
    pub fn spin(self, count: i32) -> crate::app::random_2::Random_2;

    #[method(name = "GetValue", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "GetF01", args = 0)]
    pub fn get_f01(self) -> f32;

    #[method(name = "GetValue", args = 1)]
    pub fn get_value_2(self, num: i32) -> i32;

    #[method(name = "GetMinMax", args = 2)]
    pub fn get_min_max(self, min: i32, max: i32) -> i32;

    #[method(name = "GetMinMax", args = 2)]
    pub fn get_min_max_2(self, min: f32, max: f32) -> f32;

    #[method(name = "GetMaxMin", args = 2)]
    pub fn get_max_min(self, min: i32, max: i32) -> i32;

    #[method(name = "get_insideUnitSphere", args = 0)]
    pub fn get_inside_unit_sphere(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_insideUnitCircle", args = 0)]
    pub fn get_inside_unit_circle(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "IsProbability100", args = 1)]
    pub fn is_probability100(self, percent: f32) -> bool;

    #[method(name = "GetIndex", args = 1)]
    pub fn get_index(self, ratio_table: ::unity2::Array<i32>) -> i32;

    #[method(name = "get_Seed", args = 0)]
    pub fn get_seed(self) -> crate::app::randomseed::RandomSeed;

    #[method(name = "set_Seed", args = 1)]
    pub fn set_seed(self, value: crate::app::randomseed::RandomSeed) -> ();

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg(feature = "app-random_2")]
impl Random_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Random_2),
                ::core::stringify!(new),
            )
        });
        <Self as IRandom_2Methods>::ctor(this);
        this
    }

    pub fn new_2(seed: u32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Random_2),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRandom_2Methods>::ctor_2(this, seed);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/random_2/Random_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Random_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Random_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Random.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Random_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Random_Type {
    pub fn system() -> Self {
        Self { value: 0 }
    }

    pub fn game() -> Self {
        Self { value: 1 }
    }

    pub fn spot() -> Self {
        Self { value: 2 }
    }

    pub fn hub() -> Self {
        Self { value: 3 }
    }

    pub fn hub_item() -> Self {
        Self { value: 4 }
    }

    pub fn kill_bonus() -> Self {
        Self { value: 5 }
    }

    pub fn combat() -> Self {
        Self { value: 6 }
    }

    pub fn num() -> Self {
        Self { value: 7 }
    }
}
