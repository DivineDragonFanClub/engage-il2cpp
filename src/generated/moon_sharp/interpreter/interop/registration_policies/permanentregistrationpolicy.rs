
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/registration_policies/permanentregistrationpolicy/PermanentRegistrationPolicy.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.RegistrationPolicies",
    name = "PermanentRegistrationPolicy"
)]
#[parent(crate::system::object::Object)]
pub struct PermanentRegistrationPolicy {}

#[cfg(feature = "moon_sharp-interpreter-interop-registration_policies-permanentregistrationpolicy")]
#[::unity2::methods]
impl PermanentRegistrationPolicy {
    #[method(name = "HandleRegistration", args = 2)]
    pub fn handle_registration(
        self,
        new_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
        old_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "AllowTypeAutoRegistration", args = 1)]
    pub fn allow_type_auto_registration(self, r#type: ::unity2::SystemType) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-registration_policies-permanentregistrationpolicy")]
impl PermanentRegistrationPolicy {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PermanentRegistrationPolicy),
                ::core::stringify!(new),
            )
        });
        <Self as IPermanentRegistrationPolicyMethods>::ctor(this);
        this
    }
}
