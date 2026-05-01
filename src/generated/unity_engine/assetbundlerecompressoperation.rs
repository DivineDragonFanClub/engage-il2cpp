
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::asyncoperation::AsyncOperation;
use crate::unity_engine::asyncoperation::IAsyncOperation;
use crate::unity_engine::yieldinstruction::IYieldInstruction;
use crate::unity_engine::yieldinstruction::YieldInstruction;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/assetbundlerecompressoperation/AssetBundleRecompressOperation.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AssetBundleRecompressOperation")]
#[parent(crate::unity_engine::asyncoperation::AsyncOperation)]
pub struct AssetBundleRecompressOperation {}
