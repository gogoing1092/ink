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

use crate::storage::{
    KeyPtr,
    PushForward,
    StorageSize,
};
use core::cell::Cell;

/// The entry of a single cached value of a lazy storage data structure.
#[derive(Debug, Clone)]
pub struct Entry<T> {
    /// The value or `None` if the value has been removed.
    value: Option<T>,
    /// This is `true` if the `value` is dirty and needs to be synchronized
    /// with the underlying contract storage.
    state: Cell<EntryState>,
}

/// The state of the entry.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EntryState {
    /// The entry's value must be synchronized with the contract storage.
    Mutated,
    /// The entry's value preserved the value from the contract storage.
    Preserved,
}

impl<T> PushForward for Entry<T>
where
    T: PushForward + StorageSize,
{
    fn push_forward(&self, ptr: &mut KeyPtr) {
        // Reset the state because we just synced.
        self.state.set(EntryState::Preserved);
        // Since `self.value` is of type `Option` this will eventually
        // clear the underlying storage entry if `self.value` is `None`.
        self.value.push_forward(ptr);
    }
}

impl<T> Entry<T> {
    /// Creates a new entry with the value and state.
    pub fn new(value: Option<T>, state: EntryState) -> Self {
        Self {
            value,
            state: Cell::new(state),
        }
    }

    /// Returns `true` if the cached value of the entry has potentially been mutated.
    pub fn mutated(&self) -> bool {
        self.state.get() == EntryState::Mutated
    }

    /// Returns `true` if the cached value of the entry has potentially been mutated.
    pub fn is_mutated(&self) -> bool {
        self.state.get() == EntryState::Mutated
    }

    /// Returns a shared reference to the value of the entry.
    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Returns an exclusive reference to the value of the entry.
    ///
    /// # Note
    ///
    /// This changes the `mutate` state of the entry if the entry was occupied
    /// since the caller could potentially change the returned value.
    pub fn value_mut(&mut self) -> Option<&mut T> {
        self.state.set(
            if self.value.is_some() {
                EntryState::Mutated
            } else {
                EntryState::Preserved
            },
        );
        self.value.as_mut()
    }

    /// Takes the value from the entry and returns it.
    ///
    /// # Note
    ///
    /// This changes the `mutate` state of the entry if the entry was occupied.
    pub fn take_value(&mut self) -> Option<T> {
        self.state.set(
            if self.value.is_some() {
                EntryState::Mutated
            } else {
                EntryState::Preserved
            },
        );
        self.value.take()
    }

    /// Converts the entry into its value.
    pub fn into_value(self) -> Option<T> {
        self.value
    }

    /// Puts the new value into the entry and returns the old value.
    ///
    /// # Note
    ///
    /// This changes the `mutate` state of the entry to `true` as long as at
    /// least one of `old_value` and `new_value` is `Some`.
    pub fn put(&mut self, new_value: Option<T>) -> Option<T> {
        match new_value {
            Some(new_value) => {
                self.state.set(EntryState::Mutated);
                self.value.replace(new_value)
            }
            None => self.take_value(),
        }
    }
}