
use crate::moon_sharp::interpreter::interop::registration_policies::defaultregistrationpolicy::DefaultRegistrationPolicy;
use crate::moon_sharp::interpreter::interop::registration_policies::defaultregistrationpolicy::IDefaultRegistrationPolicy;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/registration_policies/automaticregistrationpolicy/AutomaticRegistrationPolicy.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.RegistrationPolicies",
    name = "AutomaticRegistrationPolicy"
)]
# [parent (crate :: moon_sharp :: interpreter :: interop :: registration_policies :: defaultregistrationpolicy :: DefaultRegistrationPolicy)]
pub struct AutomaticRegistrationPolicy {}

#[cfg(feature = "moon_sharp-interpreter-interop-registration_policies-automaticregistrationpolicy")]
#[::unity2::methods]
impl AutomaticRegistrationPolicy {
    #[method(name = "AllowTypeAutoRegistration", args = 1)]
    pub fn allow_type_auto_registration(self, r#type: ::unity2::SystemType) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-registration_policies-automaticregistrationpolicy")]
impl AutomaticRegistrationPolicy {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AutomaticRegistrationPolicy),
                ::core::stringify!(new),
            )
        });
        <Self as IAutomaticRegistrationPolicyMethods>::ctor(this);
        this
    }
}
