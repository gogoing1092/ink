// Copyright 2018-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Core abstractions for storage manipulation. (revision 2)

pub mod collections;
pub mod lazy;
mod pack;
mod traits;

#[doc(inline)]
pub use self::{
    collections::{
        boxed::Box,
        hashmap::HashMap,
        smallvec::SmallVec,
        stash::Stash,
        vec::Vec,
    },
    lazy::{
        Lazy,
        LazyArray,
        LazyArrayLength,
        LazyCell,
        LazyHashMap,
        LazyIndexMap,
    },
    pack::Pack,
    traits::{
        pull_single_cell,
        storage_footprint_u128,
        storage_footprint_u64,
        ClearAt,
        ClearForward,
        KeyPtr,
        PullAt,
        PullForward,
        PushAt,
        PushForward,
        SaturatingStorage,
        StorageFootprint,
        StorageFootprintOf,
    },
};
