
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapTeleportSequence.md")))]
#[::unity2::class(namespace = "App", name = "GmapSequence.GmapTeleportSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GmapSequence_GmapTeleportSequence {
    #[rename(name = "m_Destination")]
    pub m_destination: crate::app::gmapspot::GmapSpot,
    #[rename(name = "m_IsClosed")]
    pub m_is_closed: bool,
}

#[cfg(feature = "app-gmapsequence")]
#[::unity2::methods]
impl GmapSequence_GmapTeleportSequence {
    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources() -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources() -> ();

    #[method(name = "StartTeleport", args = 0)]
    pub fn start_teleport(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "OpenMenu", args = 0)]
    pub fn open_menu(self) -> ();

    #[method(name = "Teleport", args = 0)]
    pub fn teleport(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "CloseMapAndTitleBar", args = 0)]
    pub fn close_map_and_title_bar(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapsequence")]
impl GmapSequence_GmapTeleportSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSequence_GmapTeleportSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSequence_GmapTeleportSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_EnterChapterSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSequence_EnterChapterSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSequence_EnterChapterSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSequence.EnterChapterSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSequence_EnterChapterSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSequence_EnterChapterSequence_Label {
    pub fn talk() -> Self {
        Self { value: 0 }
    }

    pub fn dialog() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapDisposeSequence.md")))]
#[::unity2::class(namespace = "App", name = "GmapSequence.GmapDisposeSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GmapSequence_GmapDisposeSequence {
    #[rename(name = "m_DisposSpot")]
    pub m_dispos_spot: crate::app::gmapspot::GmapSpot,
}

#[cfg(feature = "app-gmapsequence")]
#[::unity2::methods]
impl GmapSequence_GmapDisposeSequence {
    #[method(name = "CheckDispos", args = 0)]
    pub fn check_dispos(self) -> ();

    #[method(name = "DetermineDisposSpot", args = 0)]
    pub fn determine_dispos_spot(self) -> ();

    #[method(name = "CheckAppearDisposSpot", args = 0)]
    pub fn check_appear_dispos_spot(self) -> ();

    #[method(name = "DisposMoveCamera", args = 0)]
    pub fn dispos_move_camera(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "StartDisposEffect", args = 0)]
    pub fn start_dispos_effect(self) -> ();

    #[method(name = "WaitAppearDispos", args = 0)]
    pub fn wait_appear_dispos(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "SetReturnCameraPosition", args = 0)]
    pub fn set_return_camera_position(self) -> ();

    #[method(name = "StartAppearSkip", args = 0)]
    pub fn start_appear_skip(self) -> ();

    #[method(name = "EndAppearSkip", args = 0)]
    pub fn end_appear_skip(self) -> ();

    #[method(name = "WaitCameraMove", args = 0)]
    pub fn wait_camera_move(self) -> ();

    #[method(name = "DisposUpdateOfOtherMode", args = 0)]
    pub fn dispos_update_of_other_mode(self) -> ();

    #[method(name = "SetUpEncountInfo", args = 0)]
    pub fn set_up_encount_info(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetDescs", args = 1)]
    pub fn get_descs(
        p: crate::app::gmapsequence::GmapSequence_GmapDisposeSequence,
    ) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapsequence")]
impl GmapSequence_GmapDisposeSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSequence_GmapDisposeSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSequence_GmapDisposeSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapFreeCameraSequence_Dir.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSequence_GmapFreeCameraSequence_Dir {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSequence_GmapFreeCameraSequence_Dir {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSequence.GmapFreeCameraSequence.Dir";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSequence_GmapFreeCameraSequence_Dir {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSequence_GmapFreeCameraSequence_Dir {
    pub fn up() -> Self {
        Self { value: 0 }
    }

    pub fn down() -> Self {
        Self { value: 1 }
    }

    pub fn left() -> Self {
        Self { value: 2 }
    }

    pub fn right() -> Self {
        Self { value: 3 }
    }

    pub fn num() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapWholeMapSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSequence_GmapWholeMapSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSequence_GmapWholeMapSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSequence.GmapWholeMapSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSequence_GmapWholeMapSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSequence_GmapWholeMapSequence_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapWholeMapSequence.md")))]
#[::unity2::class(namespace = "App", name = "GmapSequence.GmapWholeMapSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GmapSequence_GmapWholeMapSequence {
    #[rename(name = "m_WholeMap")]
    pub m_whole_map: crate::app::gmapwholemapcontroller::GmapWholeMapController,
    #[rename(name = "m_GmapCamera")]
    pub m_gmap_camera: crate::app::gmapcamera::GmapCamera,
    #[rename(name = "m_MapInfo")]
    pub m_map_info: crate::app::gmapmapinfocontent::GmapMapInfoContent,
}

#[cfg(feature = "app-gmapsequence")]
#[::unity2::methods]
impl GmapSequence_GmapWholeMapSequence {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "WaitCameraMoveStrictly", args = 0)]
    pub fn wait_camera_move_strictly(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-gmapsequence")]
impl GmapSequence_GmapWholeMapSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSequence_GmapWholeMapSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSequence_GmapWholeMapSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapFreeCameraSequence_DirFlagField.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapSequence.GmapFreeCameraSequence.DirFlagField"
)]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: gmapsequence :: GmapSequence_GmapFreeCameraSequence_DirFlag >)]
pub struct GmapSequence_GmapFreeCameraSequence_DirFlagField {}

#[cfg(feature = "app-gmapsequence")]
#[::unity2::methods]
impl GmapSequence_GmapFreeCameraSequence_DirFlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        f: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlag,
    ) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(
        self,
        value: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlag,
    ) -> i32;

    #[method(name = "TestUp", args = 0)]
    pub fn test_up(self) -> bool;

    #[method(name = "TestDown", args = 0)]
    pub fn test_down(self) -> bool;

    #[method(name = "TestLeft", args = 0)]
    pub fn test_left(self) -> bool;

    #[method(name = "TestRight", args = 0)]
    pub fn test_right(self) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test(
        self,
        dir: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_Dir,
    ) -> bool;
}

#[cfg(feature = "app-gmapsequence")]
impl GmapSequence_GmapFreeCameraSequence_DirFlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSequence_GmapFreeCameraSequence_DirFlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSequence_GmapFreeCameraSequence_DirFlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlag) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSequence_GmapFreeCameraSequence_DirFlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGmapSequence_GmapFreeCameraSequence_DirFlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence.md")))]
#[::unity2::class(namespace = "App", name = "GmapSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: gmapsequence :: GmapSequence >)]
pub struct GmapSequence {
    #[rename(name = "m_NowSpot")]
    pub m_now_spot: crate::app::gmapspot::GmapSpot,
    #[rename(name = "m_ChangingSpot")]
    pub m_changing_spot: crate::app::gmapspot::GmapSpot,
    #[rename(name = "m_ChangingPath")]
    pub m_changing_path: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DisposSpot")]
    pub m_dispos_spot: crate::app::gmapspot::GmapSpot,
    #[rename(name = "m_GmapCamera")]
    pub m_gmap_camera: crate::app::gmapcamera::GmapCamera,
    #[rename(name = "m_PathController")]
    pub m_path_controller: crate::app::gmapcinemachinecontroller::GmapCinemachineController,
    #[rename(name = "m_VirtualSphere")]
    pub m_virtual_sphere: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WholeMap")]
    pub m_whole_map: crate::app::gmapwholemapcontroller::GmapWholeMapController,
    #[rename(name = "m_MapInfo")]
    pub m_map_info: crate::app::gmapmapinfocontent::GmapMapInfoContent,
    #[rename(name = "m_IsMoveForward")]
    pub m_is_move_forward: bool,
    #[rename(name = "m_NextSpot")]
    pub m_next_spot: crate::app::gmapspot::GmapSpot,
    #[rename(name = "m_PrevSpot")]
    pub m_prev_spot: crate::app::gmapspot::GmapSpot,
    #[rename(name = "m_Paths")]
    pub m_paths: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PathCollection")]
    pub m_path_collection: crate::app::gmap::gmappathcollection::GmapPathCollection,
    #[rename(name = "m_AutoMoveRoutes")]
    pub m_auto_move_routes:
        crate::system::collections::generic::list_1::List_1<crate::app::gmapspot::GmapSpot>,
    #[rename(name = "m_AppearedSpotList")]
    pub m_appeared_spot_list:
        crate::system::collections::generic::list_1::List_1<crate::app::gmapspot::GmapSpot>,
    #[rename(name = "m_IsAppearedEncount")]
    pub m_is_appeared_encount: bool,
    #[rename(name = "m_EffectParent")]
    pub m_effect_parent: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "AppearLineEffectPath")]
    pub appear_line_effect_path: ::unity2::Il2CppString,
    #[rename(name = "AppearSymbolEffectPath")]
    pub appear_symbol_effect_path: ::unity2::Il2CppString,
    #[rename(name = "AppearSpotEffectPath")]
    pub appear_spot_effect_path: ::unity2::Il2CppString,
    #[rename(name = "m_AppearLineEffect")]
    pub m_appear_line_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IsInitialized")]
    pub m_is_initialized: bool,
    #[rename(name = "m_IsSkipping")]
    pub m_is_skipping: bool,
    #[rename(name = "m_CinemaScope")]
    pub m_cinema_scope: crate::app::gmapcinemascope::GmapCinemaScope,
    #[rename(name = "m_DlcPathHandle")]
    pub m_dlc_path_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_DlcSpotHandle")]
    pub m_dlc_spot_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_DlcPath")]
    pub m_dlc_path: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DlcSpot")]
    pub m_dlc_spot: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "PathLengthMainToGod")]
    pub path_length_main_to_god: f32,
    #[static_field]
    #[rename(name = "PathLengthGodToMain")]
    pub path_length_god_to_main: f32,
    #[rename(name = "m_IsEncountAppearOfMain")]
    pub m_is_encount_appear_of_main: bool,
    #[rename(name = "m_IsEncountAppearOfGod")]
    pub m_is_encount_appear_of_god: bool,
    #[rename(name = "m_IsEncountAppearOfEvil")]
    pub m_is_encount_appear_of_evil: bool,
    #[rename(name = "m_G002SymbolPath")]
    pub m_g002_symbol_path: ::unity2::Il2CppString,
    #[rename(name = "m_G004SymbolPath")]
    pub m_g004_symbol_path: ::unity2::Il2CppString,
    #[rename(name = "m_G005SymbolPath")]
    pub m_g005_symbol_path: ::unity2::Il2CppString,
    #[rename(name = "m_G003SymbolPath")]
    pub m_g003_symbol_path: ::unity2::Il2CppString,
    #[rename(name = "m_G006SymbolPath")]
    pub m_g006_symbol_path: ::unity2::Il2CppString,
}

#[cfg(feature = "app-gmapsequence")]
#[::unity2::methods]
impl GmapSequence {
    #[method(name = "get_LoadingMode", args = 0)]
    pub fn get_loading_mode(self) -> crate::app::loadingmanager::LoadingManager_Modes;

    #[method(name = "get_NowSpot", args = 0)]
    pub fn get_now_spot(self) -> crate::app::gmapspot::GmapSpot;

    #[method(name = "set_NowSpot", args = 1)]
    pub fn set_now_spot(self, value: crate::app::gmapspot::GmapSpot) -> ();

    #[method(name = "get_GmapSpotIdFromData", args = 0)]
    pub fn get_gmap_spot_id_from_data(self) -> ::unity2::Il2CppString;

    #[method(name = "set_GmapSpotIdFromData", args = 1)]
    pub fn set_gmap_spot_id_from_data(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsMoveFast", args = 0)]
    pub fn get_is_move_fast() -> bool;

    #[method(name = "set_IsMoveFast", args = 1)]
    pub fn set_is_move_fast(value: bool) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "PersistentTick", args = 0)]
    pub fn persistent_tick(self) -> ();

    #[method(name = "BranchStart", args = 0)]
    pub fn branch_start(self) -> ();

    #[method(name = "LoadScript", args = 0)]
    pub fn load_script(self) -> ();

    #[method(name = "UnloadScript", args = 0)]
    pub fn unload_script(self) -> ();

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources(self) -> ();

    #[method(name = "Final", args = 0)]
    pub fn r#final(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "LoadActor", args = 0)]
    pub fn load_actor(self) -> ();

    #[method(name = "UnloadUnit", args = 0)]
    pub fn unload_unit(self) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "CreateObjectFromPrefab", args = 0)]
    pub fn create_object_from_prefab(self) -> ();

    #[method(name = "AttachSpotModels", args = 0)]
    pub fn attach_spot_models(self) -> ();

    #[method(name = "SetPlayerAndCamera", args = 0)]
    pub fn set_player_and_camera(self) -> ();

    #[method(name = "SetPlayerAndCameraMainFromGodDLC", args = 0)]
    pub fn set_player_and_camera_main_from_god_dlc(self) -> ();

    #[method(name = "SetPlayerAndCameraGodDLCFromMain", args = 0)]
    pub fn set_player_and_camera_god_dlc_from_main(self) -> ();

    #[method(name = "SetStateM017", args = 0)]
    pub fn set_state_m017(self) -> ();

    #[method(name = "CheckAppearPath", args = 0)]
    pub fn check_appear_path(self) -> ();

    #[method(name = "AppearPathTick", args = 0)]
    pub fn appear_path_tick(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "WaitCameraMove", args = 0)]
    pub fn wait_camera_move(self) -> ();

    #[method(name = "WaitCameraMoveStrictly", args = 0)]
    pub fn wait_camera_move_strictly(self) -> ();

    #[method(name = "SetPlayerSpot", args = 1)]
    pub fn set_player_spot(self, gmap_spot: crate::app::gmapspot::GmapSpot) -> ();

    #[method(name = "InitTick", args = 0)]
    pub fn init_tick(self) -> ();

    #[method(name = "GmapHelpEvent", args = 0)]
    pub fn gmap_help_event(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "MoveStart", args = 0)]
    pub fn move_start(self) -> ();

    #[method(name = "StartMoveNewestSpot", args = 0)]
    pub fn start_move_newest_spot(self) -> bool;

    #[method(name = "StartMoveSpotForAutoPlay", args = 0)]
    pub fn start_move_spot_for_auto_play(self) -> bool;

    #[method(name = "StartMoveTargetSpot", args = 1)]
    pub fn start_move_target_spot(self, target_spot: crate::app::gmapspot::GmapSpot) -> bool;

    #[method(name = "IsAutoMoveMode", args = 0)]
    pub fn is_auto_move_mode(self) -> bool;

    #[method(name = "TryStopAutoMode", args = 0)]
    pub fn try_stop_auto_mode(self) -> ();

    #[method(name = "EndAutoMode", args = 0)]
    pub fn end_auto_mode(self) -> ();

    #[method(name = "EnterChapter", args = 0)]
    pub fn enter_chapter(self) -> ();

    #[method(name = "EnterEncount", args = 0)]
    pub fn enter_encount(self) -> ();

    #[method(name = "EnterRecollection", args = 0)]
    pub fn enter_recollection(self) -> ();

    #[method(name = "InitMoving", args = 0)]
    pub fn init_moving(self) -> ();

    #[method(name = "Moving", args = 0)]
    pub fn moving(self) -> ();

    #[method(name = "EndMoving", args = 0)]
    pub fn end_moving(self) -> ();

    #[method(name = "GetMoveSpeed", args = 2)]
    pub fn get_move_speed(self, speed: f32, is_fast: bool) -> ();

    #[method(name = "AdjustPlayerUnit", args = 0)]
    pub fn adjust_player_unit(self) -> ();

    #[method(name = "AutoMoveNext", args = 0)]
    pub fn auto_move_next(self) -> ();

    #[method(name = "EnterFromDlcGod", args = 0)]
    pub fn enter_from_dlc_god(self) -> ();

    #[method(name = "MoveFromDlcGod", args = 0)]
    pub fn move_from_dlc_god(self) -> ();

    #[method(name = "MoveEndFromDlcGod", args = 0)]
    pub fn move_end_from_dlc_god(self) -> ();

    #[method(name = "MovingFade", args = 0)]
    pub fn moving_fade(self) -> ();

    #[method(name = "PlayBgm", args = 0)]
    pub fn play_bgm(self) -> ();

    #[method(name = "StopBgm", args = 0)]
    pub fn stop_bgm(self) -> ();

    #[method(name = "PauseBgm", args = 0)]
    pub fn pause_bgm(self) -> ();

    #[method(name = "ResumeBgm", args = 0)]
    pub fn resume_bgm(self) -> ();

    #[method(name = "KeyHelpHide", args = 0)]
    pub fn key_help_hide(self) -> ();

    #[method(name = "KeyHelpShow", args = 0)]
    pub fn key_help_show(self) -> ();

    #[method(name = "SaveDataLoad", args = 0)]
    pub fn save_data_load(self) -> ();

    #[method(name = "SaveDataLoadResult", args = 0)]
    pub fn save_data_load_result(self) -> ();

    #[method(name = "SaveDataRelease", args = 0)]
    pub fn save_data_release(self) -> ();

    #[method(name = "SaveDataNormalize", args = 0)]
    pub fn save_data_normalize(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "IsReverseStickL", args = 1)]
    pub fn is_reverse_stick_l(r#move: crate::unity_engine::vector3::Vector3) -> bool;

    #[method(name = "IsCheckDispos", args = 0)]
    pub fn is_check_dispos() -> bool;

    #[method(name = "ShowDialogOpenedM026Spot", args = 1)]
    pub fn show_dialog_opened_m026_spot(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "IsPlayAppearEncountSound", args = 0)]
    pub fn is_play_appear_encount_sound(self) -> bool;

    #[method(name = "CameraTrackingOn", args = 0)]
    pub fn camera_tracking_on(self) -> ();

    #[method(name = "CameraTrackingOff", args = 0)]
    pub fn camera_tracking_off(self) -> ();

    #[method(name = "SetUpEncountInfo", args = 0)]
    pub fn set_up_encount_info(self) -> ();

    #[method(name = "GmapMenuCreateBind", args = 1)]
    pub fn gmap_menu_create_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-gmapsequence")]
impl GmapSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapFreeCameraSequence_DirFlag.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSequence_GmapFreeCameraSequence_DirFlag {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSequence_GmapFreeCameraSequence_DirFlag {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSequence.GmapFreeCameraSequence.DirFlag";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSequence_GmapFreeCameraSequence_DirFlag {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSequence_GmapFreeCameraSequence_DirFlag {
    pub fn up() -> Self {
        Self { value: 1 }
    }

    pub fn left() -> Self {
        Self { value: 2 }
    }

    pub fn right() -> Self {
        Self { value: 4 }
    }

    pub fn down() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSequence_Label {
    pub fn init() -> Self {
        Self { value: 0 }
    }

    pub fn enter_from_other_gmap() -> Self {
        Self { value: 1 }
    }

    pub fn event() -> Self {
        Self { value: 2 }
    }

    pub fn check_spot() -> Self {
        Self { value: 3 }
    }

    pub fn appear_spot() -> Self {
        Self { value: 4 }
    }

    pub fn check_dispos() -> Self {
        Self { value: 5 }
    }

    pub fn check_dispos_debug() -> Self {
        Self { value: 6 }
    }

    pub fn tick() -> Self {
        Self { value: 7 }
    }

    pub fn tick_without_camara_wait() -> Self {
        Self { value: 8 }
    }

    pub fn r#move() -> Self {
        Self { value: 9 }
    }

    pub fn move_to_other_gmap() -> Self {
        Self { value: 10 }
    }

    pub fn save_data_load() -> Self {
        Self { value: 11 }
    }

    pub fn end() -> Self {
        Self { value: 12 }
    }

    pub fn tail() -> Self {
        Self { value: 13 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapDisposeSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSequence_GmapDisposeSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSequence_GmapDisposeSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSequence.GmapDisposeSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSequence_GmapDisposeSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSequence_GmapDisposeSequence_Label {
    pub fn check_dispos() -> Self {
        Self { value: 0 }
    }

    pub fn update_dispos() -> Self {
        Self { value: 1 }
    }

    pub fn appear_dispos() -> Self {
        Self { value: 2 }
    }

    pub fn appear_dispos_end() -> Self {
        Self { value: 3 }
    }

    pub fn appear_dispos_skip() -> Self {
        Self { value: 4 }
    }

    pub fn appear_dispos_skip_begin() -> Self {
        Self { value: 5 }
    }

    pub fn appear_dispos_skip_end() -> Self {
        Self { value: 6 }
    }

    pub fn end() -> Self {
        Self { value: 7 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapFreeCameraSequence.md")))]
#[::unity2::class(namespace = "App", name = "GmapSequence.GmapFreeCameraSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GmapSequence_GmapFreeCameraSequence {
    #[static_field]
    #[rename(name = "FreeCameraRootPath")]
    pub free_camera_root_path: ::unity2::Il2CppString,
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "MoveSpeedAngle")]
    pub move_speed_angle: f32,
    #[rename(name = "m_AngleX")]
    pub m_angle_x: f32,
    #[rename(name = "m_AngleZ")]
    pub m_angle_z: f32,
    #[rename(name = "m_StartPosition")]
    pub m_start_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_CachePosition")]
    pub m_cache_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_DisableFlag")]
    pub m_disable_flag: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlagField,
    #[rename(name = "m_AngleLimit")]
    pub m_angle_limit: ::unity2::Array<f32>,
    #[rename(name = "m_Camera")]
    pub m_camera: crate::app::gmapcamera::GmapCamera,
}

#[cfg(feature = "app-gmapsequence")]
#[::unity2::methods]
impl GmapSequence_GmapFreeCameraSequence {
    #[method(name = "LoadResorces", args = 0)]
    pub fn load_resorces() -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources() -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "WaitScroll", args = 0)]
    pub fn wait_scroll(self) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "TryLRMove", args = 1)]
    pub fn try_lr_move(self, move_l_stick_x: f32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "TryUDMove", args = 1)]
    pub fn try_ud_move(self, move_l_stick_y: f32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CalcLeftRightPos", args = 2)]
    pub fn calc_left_right_pos(
        pos: crate::unity_engine::vector3::Vector3,
        angle: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CalcUDPos", args = 2)]
    pub fn calc_ud_pos(
        pos: crate::unity_engine::vector3::Vector3,
        angle: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "InitAngleLimit", args = 1)]
    pub fn init_angle_limit(
        self,
        flags: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlagField,
    ) -> ();

    #[method(name = "ClampAngle", args = 2)]
    pub fn clamp_angle(
        self,
        angle: f32,
        dir: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_Dir,
    ) -> f32;

    #[method(name = "TryGetAngleLimit", args = 2)]
    pub fn try_get_angle_limit(
        self,
        dir: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_Dir,
        value: f32,
    ) -> bool;

    #[method(name = "SetAngleLimit", args = 3)]
    pub fn set_angle_limit(
        self,
        flag: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlagField,
        angle: f32,
        dir: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_Dir,
    ) -> ();

    #[method(name = "GetRecalcIgnoreFlag", args = 0)]
    pub fn get_recalc_ignore_flag(
        self,
    ) -> crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlagField;

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapsequence")]
impl GmapSequence_GmapFreeCameraSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSequence_GmapFreeCameraSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSequence_GmapFreeCameraSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapFreeCameraSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSequence_GmapFreeCameraSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSequence_GmapFreeCameraSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSequence.GmapFreeCameraSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSequence_GmapFreeCameraSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSequence_GmapFreeCameraSequence_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_EnterChapterSequence.md")))]
#[::unity2::class(namespace = "App", name = "GmapSequence.EnterChapterSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GmapSequence_EnterChapterSequence {
    #[static_field]
    #[rename(name = "TalkFlagNameM010")]
    pub talk_flag_name_m010: ::unity2::Il2CppString,
    #[rename(name = "m_NowSpot")]
    pub m_now_spot: crate::app::gmapspot::GmapSpot,
    #[rename(name = "m_Type")]
    pub m_type: crate::app::gmap::enterchapteryesnodialog::EnterChapterYesNoDialog_Type,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::system::action::Action,
    #[rename(name = "m_DecideEventHandler2")]
    pub m_decide_event_handler2: crate::system::action::Action,
}

#[cfg(feature = "app-gmapsequence")]
#[::unity2::methods]
impl GmapSequence_EnterChapterSequence {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        now_spot: crate::app::gmapspot::GmapSpot,
        r#type: crate::app::gmap::enterchapteryesnodialog::EnterChapterYesNoDialog_Type,
        decide_event_handler: crate::system::action::Action,
    ) -> ();

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "Talk", args = 0)]
    pub fn talk(self) -> ();

    #[method(name = "OpenDialog", args = 0)]
    pub fn open_dialog(self) -> ();

    #[method(name = "Final", args = 0)]
    pub fn r#final(self) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        now_spot: crate::app::gmapspot::GmapSpot,
        r#type: crate::app::gmap::enterchapteryesnodialog::EnterChapterYesNoDialog_Type,
        decide_event_handler: crate::system::action::Action,
    ) -> ();
}

#[cfg(feature = "app-gmapsequence")]
impl GmapSequence_EnterChapterSequence {
    pub fn new(
        now_spot: crate::app::gmapspot::GmapSpot,
        r#type: crate::app::gmap::enterchapteryesnodialog::EnterChapterYesNoDialog_Type,
        decide_event_handler: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSequence_EnterChapterSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSequence_EnterChapterSequenceMethods>::ctor(
            this,
            now_spot,
            r#type,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapTeleportSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSequence_GmapTeleportSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSequence_GmapTeleportSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSequence.GmapTeleportSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSequence_GmapTeleportSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSequence_GmapTeleportSequence_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapsequence/GmapSequence_GmapFreeCameraSequence_SelfDestroy.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapSequence.GmapFreeCameraSequence.SelfDestroy"
)]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct GmapSequence_GmapFreeCameraSequence_SelfDestroy {
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-gmapsequence")]
#[::unity2::methods]
impl GmapSequence_GmapFreeCameraSequence_SelfDestroy {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapsequence")]
impl GmapSequence_GmapFreeCameraSequence_SelfDestroy {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSequence_GmapFreeCameraSequence_SelfDestroy),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSequence_GmapFreeCameraSequence_SelfDestroyMethods>::ctor(this);
        this
    }
}
