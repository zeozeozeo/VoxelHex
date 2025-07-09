use crate::boxtree::types::VoxelData;
use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::{Serialize, de::DeserializeOwned};

#[cfg(feature = "bytecode")]
use bendy::{decoding::FromBencode, encoding::ToBencode};

pub trait UnifiedVoxelData:
    Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static + UnifiedVoxelDataExt
{
}

#[cfg(all(feature = "bytecode", feature = "serde"))]
pub trait UnifiedVoxelDataExt: FromBencode + ToBencode + Serialize + DeserializeOwned {}

#[cfg(all(feature = "bytecode", not(feature = "serde")))]
pub trait UnifiedVoxelDataExt: FromBencode + ToBencode {}

#[cfg(all(not(feature = "bytecode"), feature = "serde"))]
pub trait UnifiedVoxelDataExt: Serialize + DeserializeOwned {}

#[cfg(all(not(feature = "bytecode"), not(feature = "serde")))]
pub trait UnifiedVoxelDataExt {}

#[cfg(all(feature = "bytecode", feature = "serde"))]
impl<T> UnifiedVoxelDataExt for T where T: FromBencode + ToBencode + Serialize + DeserializeOwned {}

#[cfg(all(feature = "bytecode", not(feature = "serde")))]
impl<T> UnifiedVoxelDataExt for T where T: FromBencode + ToBencode {}

#[cfg(all(not(feature = "bytecode"), feature = "serde"))]
impl<T> UnifiedVoxelDataExt for T where T: Serialize + DeserializeOwned {}

#[cfg(all(not(feature = "bytecode"), not(feature = "serde")))]
impl<T> UnifiedVoxelDataExt for T {}

impl<T> UnifiedVoxelData for T where
    T: Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static + UnifiedVoxelDataExt
{
}
