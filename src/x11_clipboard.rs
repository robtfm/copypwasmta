// Copyright 2017 Avraham Weinstock
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::marker::PhantomData;
use std::time::Duration;

use x11_clipboard::Atom;
use x11_clipboard::{Atoms, Clipboard as X11Clipboard};

use async_trait::async_trait;

use crate::common::*;

pub trait Selection: Send {
    fn atom(atoms: &Atoms) -> Atom;
}

pub struct Primary;

impl Selection for Primary {
    fn atom(atoms: &Atoms) -> Atom {
        atoms.primary
    }
}

pub struct Clipboard;

impl Selection for Clipboard {
    fn atom(atoms: &Atoms) -> Atom {
        atoms.clipboard
    }
}

pub struct X11ClipboardContext<S = Clipboard>(X11Clipboard, PhantomData<S>)
where
    S: Selection;

impl<S> X11ClipboardContext<S>
where
    S: Selection,
{
    pub fn new() -> Result<X11ClipboardContext<S>> {
        Ok(X11ClipboardContext(X11Clipboard::new()?, PhantomData))
    }
}

#[async_trait]
impl<S> ClipboardProvider for X11ClipboardContext<S>
where
    S: Selection,
{
    async fn get_contents(&mut self) -> Result<String> {
        Ok(String::from_utf8(self.0.load(
            S::atom(&self.0.getter.atoms),
            self.0.getter.atoms.utf8_string,
            self.0.getter.atoms.property,
            Duration::from_secs(3),
        )?)?)
    }

    async fn set_contents(&mut self, data: String) -> Result<()> {
        Ok(self.0.store(S::atom(&self.0.setter.atoms), self.0.setter.atoms.utf8_string, data)?)
    }
}
