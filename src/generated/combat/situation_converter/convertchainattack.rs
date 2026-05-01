
use crate::combat::situation_converter::baseconverter::BaseConverter;
use crate::combat::situation_converter::baseconverter::IBaseConverter;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/convertchainattack/ConvertChainAttack.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "ConvertChainAttack")]
#[parent(crate::combat::situation_converter::baseconverter::BaseConverter)]
pub struct ConvertChainAttack {
    #[rename(name = "m_LastChainAttackID")]
    pub m_last_chain_attack_id: i32,
    #[rename(name = "m_IsEnemyEngageAttack")]
    pub m_is_enemy_engage_attack: bool,
    #[rename(name = "m_IsChainBegun")]
    pub m_is_chain_begun: bool,
    #[rename(name = "m_IsAttacking")]
    pub m_is_attacking: bool,
    #[rename(name = "m_IsChainFinished")]
    pub m_is_chain_finished: bool,
}

#[cfg(feature = "combat-situation_converter-convertchainattack")]
#[::unity2::methods]
impl ConvertChainAttack {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        situation: crate::combat::camerasituation::CameraSituation,
        arg: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "AttackCam", args = 0)]
    pub fn attack_cam(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "DamageCam", args = 0)]
    pub fn damage_cam(self) -> crate::combat::cameraposition::CameraPosition;
}

#[cfg(feature = "combat-situation_converter-convertchainattack")]
impl ConvertChainAttack {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConvertChainAttack),
                ::core::stringify!(new),
            )
        });
        <Self as IConvertChainAttackMethods>::ctor(this, data);
        this
    }
}
