
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/unitywebrequest/UnityWebRequest_Result.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnityWebRequest_Result {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnityWebRequest_Result {
    const NAMESPACE: &'static str = "UnityEngine.Networking";

    const NAME: &'static str = "UnityWebRequest.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnityWebRequest_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnityWebRequest_Result {
    pub fn in_progress() -> Self {
        Self { value: 0 }
    }

    pub fn success() -> Self {
        Self { value: 1 }
    }

    pub fn connection_error() -> Self {
        Self { value: 2 }
    }

    pub fn protocol_error() -> Self {
        Self { value: 3 }
    }

    pub fn data_processing_error() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/unitywebrequest/UnityWebRequest.md")))]
#[::unity2::class(namespace = "UnityEngine.Networking", name = "UnityWebRequest")]
#[parent(crate::system::object::Object)]
pub struct UnityWebRequest {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_DownloadHandler")]
    pub m_download_handler: crate::unity_engine::networking::downloadhandler::DownloadHandler,
    #[rename(name = "m_UploadHandler")]
    pub m_upload_handler: crate::unity_engine::networking::uploadhandler::UploadHandler,
    #[rename(name = "m_CertificateHandler")]
    pub m_certificate_handler:
        crate::unity_engine::networking::certificatehandler::CertificateHandler,
    #[static_field]
    #[rename(name = "kHttpVerbGET")]
    pub k_http_verb_get: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "kHttpVerbHEAD")]
    pub k_http_verb_head: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "kHttpVerbPOST")]
    pub k_http_verb_post: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "kHttpVerbPUT")]
    pub k_http_verb_put: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "kHttpVerbCREATE")]
    pub k_http_verb_create: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "kHttpVerbDELETE")]
    pub k_http_verb_delete: ::unity2::Il2CppString,
}

#[cfg(feature = "unity_engine-networking-unitywebrequest")]
#[::unity2::methods]
impl UnityWebRequest {
    #[method(name = "GetWebErrorString", args = 1)]
    pub fn get_web_error_string(
        err: crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetHTTPStatusString", args = 1)]
    pub fn get_http_status_string(response_code: i64) -> ::unity2::Il2CppString;

    #[method(name = "get_disposeCertificateHandlerOnDispose", args = 0)]
    pub fn get_dispose_certificate_handler_on_dispose(self) -> bool;

    #[method(name = "set_disposeCertificateHandlerOnDispose", args = 1)]
    pub fn set_dispose_certificate_handler_on_dispose(self, value: bool) -> ();

    #[method(name = "get_disposeDownloadHandlerOnDispose", args = 0)]
    pub fn get_dispose_download_handler_on_dispose(self) -> bool;

    #[method(name = "set_disposeDownloadHandlerOnDispose", args = 1)]
    pub fn set_dispose_download_handler_on_dispose(self, value: bool) -> ();

    #[method(name = "get_disposeUploadHandlerOnDispose", args = 0)]
    pub fn get_dispose_upload_handler_on_dispose(self) -> bool;

    #[method(name = "set_disposeUploadHandlerOnDispose", args = 1)]
    pub fn set_dispose_upload_handler_on_dispose(self, value: bool) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> ::unity2::IntPtr;

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "InternalDestroy", args = 0)]
    pub fn internal_destroy(self) -> ();

    #[method(name = "InternalSetDefaults", args = 0)]
    pub fn internal_set_defaults(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, url: ::unity2::Il2CppString, method: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_2(
        self,
        url: ::unity2::Il2CppString,
        method: ::unity2::Il2CppString,
        download_handler: crate::unity_engine::networking::downloadhandler::DownloadHandler,
        upload_handler: crate::unity_engine::networking::uploadhandler::UploadHandler,
    ) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "DisposeHandlers", args = 0)]
    pub fn dispose_handlers(self) -> ();

    #[method(name = "BeginWebRequest", args = 0)]
    pub fn begin_web_request(
        self,
    ) -> crate::unity_engine::networking::unitywebrequestasyncoperation::UnityWebRequestAsyncOperation;

    #[method(name = "SendWebRequest", args = 0)]
    pub fn send_web_request(
        self,
    ) -> crate::unity_engine::networking::unitywebrequestasyncoperation::UnityWebRequestAsyncOperation;

    #[method(name = "Abort", args = 0)]
    pub fn abort(self) -> ();

    #[method(name = "SetMethod", args = 1)]
    pub fn set_method(
        self,
        method_type : crate :: unity_engine :: networking :: unitywebrequest :: UnityWebRequest_UnityWebRequestMethod,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError;

    #[method(name = "InternalSetMethod", args = 1)]
    pub fn internal_set_method(
        self,
        method_type : crate :: unity_engine :: networking :: unitywebrequest :: UnityWebRequest_UnityWebRequestMethod,
    ) -> ();

    #[method(name = "SetCustomMethod", args = 1)]
    pub fn set_custom_method(
        self,
        custom_method_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError;

    #[method(name = "InternalSetCustomMethod", args = 1)]
    pub fn internal_set_custom_method(self, custom_method_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetMethod", args = 0)]
    pub fn get_method(
        self,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestMethod;

    #[method(name = "GetCustomMethod", args = 0)]
    pub fn get_custom_method(self) -> ::unity2::Il2CppString;

    #[method(name = "set_method", args = 1)]
    pub fn set_method_2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetError", args = 0)]
    pub fn get_error(
        self,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError;

    #[method(name = "get_url", args = 0)]
    pub fn get_url(self) -> ::unity2::Il2CppString;

    #[method(name = "set_url", args = 1)]
    pub fn set_url(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "InternalSetUrl", args = 1)]
    pub fn internal_set_url(self, url: ::unity2::Il2CppString) -> ();

    #[method(name = "get_responseCode", args = 0)]
    pub fn get_response_code(self) -> i64;

    #[method(name = "IsExecuting", args = 0)]
    pub fn is_executing(self) -> bool;

    #[method(name = "get_isModifiable", args = 0)]
    pub fn get_is_modifiable(self) -> bool;

    #[method(name = "get_isDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = "get_result", args = 0)]
    pub fn get_result(
        self,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_Result;

    #[method(name = "GetDownloadProgress", args = 0)]
    pub fn get_download_progress(self) -> f32;

    #[method(name = "get_downloadedBytes", args = 0)]
    pub fn get_downloaded_bytes(self) -> u64;

    #[method(name = "SetRedirectLimitFromScripting", args = 1)]
    pub fn set_redirect_limit_from_scripting(self, limit: i32) -> ();

    #[method(name = "set_redirectLimit", args = 1)]
    pub fn set_redirect_limit(self, value: i32) -> ();

    #[method(name = "SetChunked", args = 1)]
    pub fn set_chunked(
        self,
        chunked: bool,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError;

    #[method(name = "set_chunkedTransfer", args = 1)]
    pub fn set_chunked_transfer(self, value: bool) -> ();

    #[method(name = "InternalSetRequestHeader", args = 2)]
    pub fn internal_set_request_header(
        self,
        name: ::unity2::Il2CppString,
        value: ::unity2::Il2CppString,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError;

    #[method(name = "SetRequestHeader", args = 2)]
    pub fn set_request_header(
        self,
        name: ::unity2::Il2CppString,
        value: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetResponseHeader", args = 1)]
    pub fn get_response_header(self, name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetResponseHeaderKeys", args = 0)]
    pub fn get_response_header_keys(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetResponseHeaders", args = 0)]
    pub fn get_response_headers(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >;

    #[method(name = "SetUploadHandler", args = 1)]
    pub fn set_upload_handler(
        self,
        uh: crate::unity_engine::networking::uploadhandler::UploadHandler,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError;

    #[method(name = "get_uploadHandler", args = 0)]
    pub fn get_upload_handler(
        self,
    ) -> crate::unity_engine::networking::uploadhandler::UploadHandler;

    #[method(name = "SetDownloadHandler", args = 1)]
    pub fn set_download_handler(
        self,
        dh: crate::unity_engine::networking::downloadhandler::DownloadHandler,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError;

    #[method(name = "get_downloadHandler", args = 0)]
    pub fn get_download_handler(
        self,
    ) -> crate::unity_engine::networking::downloadhandler::DownloadHandler;

    #[method(name = "SetCertificateHandler", args = 1)]
    pub fn set_certificate_handler(
        self,
        ch: crate::unity_engine::networking::certificatehandler::CertificateHandler,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError;

    #[method(name = "get_certificateHandler", args = 0)]
    pub fn get_certificate_handler(
        self,
    ) -> crate::unity_engine::networking::certificatehandler::CertificateHandler;

    #[method(name = "SetTimeoutMsec", args = 1)]
    pub fn set_timeout_msec(
        self,
        timeout: i32,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_UnityWebRequestError;

    #[method(name = "set_timeout", args = 1)]
    pub fn set_timeout(self, value: i32) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(
        uri: ::unity2::Il2CppString,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest;

    #[method(name = "Post", args = 2)]
    pub fn post(
        uri: ::unity2::Il2CppString,
        form_data: crate::unity_engine::wwwform::WWWForm,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest;

    #[method(name = "SetupPost", args = 2)]
    pub fn setup_post(
        request: crate::unity_engine::networking::unitywebrequest::UnityWebRequest,
        form_data: crate::unity_engine::wwwform::WWWForm,
    ) -> ();
}

#[cfg(feature = "unity_engine-networking-unitywebrequest")]
impl UnityWebRequest {
    pub fn new(url: ::unity2::Il2CppString, method: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityWebRequest),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityWebRequestMethods>::ctor(this, url, method);
        this
    }

    pub fn new_2(
        url: ::unity2::Il2CppString,
        method: ::unity2::Il2CppString,
        download_handler: crate::unity_engine::networking::downloadhandler::DownloadHandler,
        upload_handler: crate::unity_engine::networking::uploadhandler::UploadHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityWebRequest),
                ::core::stringify!(new_2),
            )
        });
        <Self as IUnityWebRequestMethods>::ctor_2(
            this,
            url,
            method,
            download_handler,
            upload_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/unitywebrequest/UnityWebRequest_UnityWebRequestError.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnityWebRequest_UnityWebRequestError {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnityWebRequest_UnityWebRequestError {
    const NAMESPACE: &'static str = "UnityEngine.Networking";

    const NAME: &'static str = "UnityWebRequest.UnityWebRequestError";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnityWebRequest_UnityWebRequestError {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnityWebRequest_UnityWebRequestError {
    pub fn ok() -> Self {
        Self { value: 0 }
    }

    pub fn unknown() -> Self {
        Self { value: 1 }
    }

    pub fn sdk_error() -> Self {
        Self { value: 2 }
    }

    pub fn unsupported_protocol() -> Self {
        Self { value: 3 }
    }

    pub fn malformatted_url() -> Self {
        Self { value: 4 }
    }

    pub fn cannot_resolve_proxy() -> Self {
        Self { value: 5 }
    }

    pub fn cannot_resolve_host() -> Self {
        Self { value: 6 }
    }

    pub fn cannot_connect_to_host() -> Self {
        Self { value: 7 }
    }

    pub fn access_denied() -> Self {
        Self { value: 8 }
    }

    pub fn generic_http_error() -> Self {
        Self { value: 9 }
    }

    pub fn write_error() -> Self {
        Self { value: 10 }
    }

    pub fn read_error() -> Self {
        Self { value: 11 }
    }

    pub fn out_of_memory() -> Self {
        Self { value: 12 }
    }

    pub fn timeout() -> Self {
        Self { value: 13 }
    }

    pub fn http_post_error() -> Self {
        Self { value: 14 }
    }

    pub fn ssl_cannot_connect() -> Self {
        Self { value: 15 }
    }

    pub fn aborted() -> Self {
        Self { value: 16 }
    }

    pub fn too_many_redirects() -> Self {
        Self { value: 17 }
    }

    pub fn received_no_data() -> Self {
        Self { value: 18 }
    }

    pub fn ssl_not_supported() -> Self {
        Self { value: 19 }
    }

    pub fn failed_to_send_data() -> Self {
        Self { value: 20 }
    }

    pub fn failed_to_receive_data() -> Self {
        Self { value: 21 }
    }

    pub fn ssl_certificate_error() -> Self {
        Self { value: 22 }
    }

    pub fn ssl_cipher_not_available() -> Self {
        Self { value: 23 }
    }

    pub fn sslca_cert_error() -> Self {
        Self { value: 24 }
    }

    pub fn unrecognized_content_encoding() -> Self {
        Self { value: 25 }
    }

    pub fn login_failed() -> Self {
        Self { value: 26 }
    }

    pub fn ssl_shutdown_failed() -> Self {
        Self { value: 27 }
    }

    pub fn no_internet_connection() -> Self {
        Self { value: 28 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/unitywebrequest/UnityWebRequest_UnityWebRequestMethod.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnityWebRequest_UnityWebRequestMethod {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnityWebRequest_UnityWebRequestMethod {
    const NAMESPACE: &'static str = "UnityEngine.Networking";

    const NAME: &'static str = "UnityWebRequest.UnityWebRequestMethod";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnityWebRequest_UnityWebRequestMethod {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnityWebRequest_UnityWebRequestMethod {
    pub fn get() -> Self {
        Self { value: 0 }
    }

    pub fn post() -> Self {
        Self { value: 1 }
    }

    pub fn put() -> Self {
        Self { value: 2 }
    }

    pub fn head() -> Self {
        Self { value: 3 }
    }

    pub fn custom() -> Self {
        Self { value: 4 }
    }
}
