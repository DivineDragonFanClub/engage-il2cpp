
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlemath/BattleMath_Probability.md")))]
#[::unity2::class(namespace = "App", name = "BattleMath.Probability")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct BattleMath_Probability {}

#[cfg(feature = "app-battlemath")]
#[::unity2::methods]
impl BattleMath_Probability {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, ratio: i32) -> bool;
}

#[cfg(feature = "app-battlemath")]
impl BattleMath_Probability {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleMath_Probability),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleMath_ProbabilityMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlemath/BattleMath.md")))]
#[::unity2::class(namespace = "App", name = "BattleMath")]
#[parent(crate::system::object::Object)]
pub struct BattleMath {
    #[static_field]
    #[rename(name = "s_CurrentProbability100")]
    pub s_current_probability100: crate::app::battlemath::BattleMath_Probability,
    #[static_field]
    #[rename(name = "s_CurrentProbabilityHit")]
    pub s_current_probability_hit: crate::app::battlemath::BattleMath_Probability,
    #[static_field]
    #[rename(name = "s_Probability100")]
    pub s_probability100: crate::app::battlemath::BattleMath_Probability,
    #[static_field]
    #[rename(name = "s_ProbabilityHit")]
    pub s_probability_hit: crate::app::battlemath::BattleMath_Probability,
    #[static_field]
    #[rename(name = "s_ProbabilityTrue")]
    pub s_probability_true: crate::app::battlemath::BattleMath_Probability,
    #[static_field]
    #[rename(name = "s_ProbabilityFalse")]
    pub s_probability_false: crate::app::battlemath::BattleMath_Probability,
    #[static_field]
    #[rename(name = "s_RandomSeed")]
    pub s_random_seed:
        crate::system::collections::generic::stack_1::Stack_1<crate::app::randomseed::RandomSeed>,
    #[static_field]
    #[rename(name = "s_Simulationed")]
    pub s_simulationed: i32,
}

#[cfg(feature = "app-battlemath")]
#[::unity2::methods]
impl BattleMath {
    #[method(name = "_IsProbability100", args = 1)]
    pub fn is_probability100(ratio: i32) -> bool;

    #[method(name = "_IsProbabilityHit", args = 1)]
    pub fn is_probability_hit(ratio: i32) -> bool;

    #[method(name = "_IsProbabilityTrue", args = 1)]
    pub fn is_probability_true(ratio: i32) -> bool;

    #[method(name = "_IsProbabilityFalse", args = 1)]
    pub fn is_probability_false(ratio: i32) -> bool;

    #[method(name = "SetSimulation", args = 1)]
    pub fn set_simulation(enable: bool) -> ();

    #[method(name = "RandomCheck100", args = 1)]
    pub fn random_check100(percent: i32) -> bool;

    #[method(name = "PushRandomSeed", args = 0)]
    pub fn push_random_seed() -> ();

    #[method(name = "PopRandomSeed", args = 0)]
    pub fn pop_random_seed() -> ();

    #[method(name = "PushSimulation", args = 0)]
    pub fn push_simulation() -> ();

    #[method(name = "PopSimulation", args = 0)]
    pub fn pop_simulation() -> ();

    #[method(name = "RandomCheckHit", args = 1)]
    pub fn random_check_hit(ratio: i32) -> bool;

    #[method(name = "GetHitRatio10000", args = 1)]
    pub fn get_hit_ratio10000(ratio: i32) -> i32;

    #[method(name = "GetHitRealRatio", args = 1)]
    pub fn get_hit_real_ratio(ratio: i32) -> f32;

    #[method(name = "GetRandom", args = 1)]
    pub fn get_random(num: i32) -> i32;

    #[method(name = "GetRandom", args = 2)]
    pub fn get_random_2(min: i32, max: i32) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-battlemath")]
impl BattleMath {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleMath),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleMathMethods>::ctor(this);
        this
    }
}
