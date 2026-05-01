
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/registration_policies/iregistrationpolicy/IRegistrationPolicy.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.RegistrationPolicies",
    name = "IRegistrationPolicy"
)]
pub struct IRegistrationPolicy {}

#[cfg(feature = "moon_sharp-interpreter-interop-registration_policies-iregistrationpolicy")]
#[::unity2::methods]
impl IRegistrationPolicy {
    #[method(name = "HandleRegistration", args = 2)]
    pub fn handle_registration(
        self,
        new_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
        old_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "AllowTypeAutoRegistration", args = 1)]
    pub fn allow_type_auto_registration(self, r#type: ::unity2::SystemType) -> bool;
}
