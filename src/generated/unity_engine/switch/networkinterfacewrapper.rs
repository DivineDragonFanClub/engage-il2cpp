
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/networkinterfacewrapper/NetworkInterfaceWrapper_NetworkConnectedHandler.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Switch",
    name = "NetworkInterfaceWrapper.NetworkConnectedHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct NetworkInterfaceWrapper_NetworkConnectedHandler {}

#[cfg(feature = "unity_engine-switch-networkinterfacewrapper")]
#[::unity2::methods]
impl NetworkInterfaceWrapper_NetworkConnectedHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke (self ,) -> crate :: unity_engine :: switch :: networkinterfacewrapper :: NetworkInterfaceWrapper_NetworkConnectedResult ;
}

#[cfg(feature = "unity_engine-switch-networkinterfacewrapper")]
impl NetworkInterfaceWrapper_NetworkConnectedHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NetworkInterfaceWrapper_NetworkConnectedHandler),
                ::core::stringify!(new),
            )
        });
        <Self as INetworkInterfaceWrapper_NetworkConnectedHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/networkinterfacewrapper/NetworkInterfaceWrapper.md")))]
#[::unity2::class(namespace = "UnityEngine.Switch", name = "NetworkInterfaceWrapper")]
#[parent(crate::system::object::Object)]
pub struct NetworkInterfaceWrapper {
# [static_field] # [rename (name = "_networkConnected")] pub network_connected : crate :: unity_engine :: switch :: networkinterfacewrapper :: NetworkInterfaceWrapper_NetworkConnectedHandler ,
}

#[cfg(feature = "unity_engine-switch-networkinterfacewrapper")]
#[::unity2::methods]
impl NetworkInterfaceWrapper {
    #[method(name = "EnterNetworkConnecting", args = 1)]
    pub fn enter_network_connecting(is_local_network_mode: bool) -> bool;

    #[method(name = "EnterNetworkConnecting", args = 2)]
    pub fn enter_network_connecting_2(
        is_local_network_mode: bool,
        report_if_unavailable: bool,
    ) -> bool;

    #[method(name = "WeakEnterNetworkConnecting", args = 0)]
    pub fn weak_enter_network_connecting() -> bool;

    #[method(name = "LeaveNetworkConnecting", args = 0)]
    pub fn leave_network_connecting() -> ();

    #[method(name = "IsNetworkConnecting", args = 0)]
    pub fn is_network_connecting() -> bool;

    #[method(name = "WaitForNetworkConnecting", args = 0)]
    pub fn wait_for_network_connecting() -> ();

    #[method(name = "IsNetworkAccepted", args = 0)]
    pub fn is_network_accepted() -> bool;

    #[method(name = "IsNetworkFinished", args = 0)]
    pub fn is_network_finished() -> bool;

    #[method(name = "GetNetworkReferenceCount", args = 0)]
    pub fn get_network_reference_count() -> i32;

    #[method(name = "IsNetworkAvailable", args = 0)]
    pub fn is_network_available() -> bool;

    #[method(name = "IsNetworkConnectingOnBackground", args = 0)]
    pub fn is_network_connecting_on_background() -> bool;

    #[method(name = "SetNetworkConnectingOnBackground", args = 1)]
    pub fn set_network_connecting_on_background(is_background: bool) -> ();

    #[method(name = "SetNetworkConnectedEnabled", args = 1)]
    pub fn set_network_connected_enabled(is_enabled: bool) -> ();

    #[method(name = "InvokeNetworkConnected", args = 0)]
    pub fn invoke_network_connected() -> i32;

    #[method(name = "add_networkConnected", args = 1)]
    pub fn add_network_connected(
        value : crate :: unity_engine :: switch :: networkinterfacewrapper :: NetworkInterfaceWrapper_NetworkConnectedHandler,
    ) -> ();

    #[method(name = "remove_networkConnected", args = 1)]
    pub fn remove_network_connected(
        value : crate :: unity_engine :: switch :: networkinterfacewrapper :: NetworkInterfaceWrapper_NetworkConnectedHandler,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/networkinterfacewrapper/NetworkInterfaceWrapper_NetworkConnectedResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NetworkInterfaceWrapper_NetworkConnectedResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NetworkInterfaceWrapper_NetworkConnectedResult {
    const NAMESPACE: &'static str = "UnityEngine.Switch";

    const NAME: &'static str = "NetworkInterfaceWrapper.NetworkConnectedResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NetworkInterfaceWrapper_NetworkConnectedResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NetworkInterfaceWrapper_NetworkConnectedResult {
    pub fn pending() -> Self {
        Self { value: -1 }
    }

    pub fn denied() -> Self {
        Self { value: 0 }
    }

    pub fn accepted() -> Self {
        Self { value: 1 }
    }
}
