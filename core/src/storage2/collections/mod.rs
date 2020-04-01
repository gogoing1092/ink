// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

pub mod boxed;
pub mod hashmap;
pub mod smallvec;
pub mod vec;
pub mod stash;

/// Extends the lifetime 'a to the outliving lifetime 'b for the given reference.
///
/// # Note
///
/// This interface is a bit more constraint than a simple
/// [transmut](`core::mem::transmute`) and therefore preferred
/// for extending lifetimes only.
///
/// # Safety
///
/// This function is `unsafe` because lifetimes can be extended beyond the
/// lifetimes of the objects they are referencing and thus potentially create
/// dangling references if not used carefully.
pub(crate) unsafe fn extend_lifetime<'a, 'b: 'a, T>(reference: &'a mut T) -> &'b mut T {
    #[allow(unused_unsafe)]
    unsafe {
        core::mem::transmute::<&'a mut T, &'b mut T>(reference)
    }
}