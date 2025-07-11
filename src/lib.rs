// Copyright 2016 Avraham Weinstock
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
#![deny(clippy::all, clippy::if_not_else, clippy::enum_glob_use)]

mod common;
pub use crate::common::ClipboardProvider;

#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten"
    ))
))]
#[cfg(feature = "wayland")]
pub mod wayland_clipboard;
#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten"
    ))
))]
#[cfg(feature = "x11")]
pub mod x11_clipboard;

#[cfg(windows)]
pub mod windows_clipboard;

#[cfg(target_arch = "wasm32")]
pub mod wasm_clipboard;

#[cfg(target_os = "macos")]
pub mod osx_clipboard;

pub mod nop_clipboard;

#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten"
    ))
))]
#[cfg(feature = "x11")]
pub type ClipboardContext = x11_clipboard::X11ClipboardContext;
#[cfg(windows)]
pub type ClipboardContext = windows_clipboard::WindowsClipboardContext;
#[cfg(target_os = "macos")]
pub type ClipboardContext = osx_clipboard::OSXClipboardContext;
#[cfg(target_os = "android")]
pub type ClipboardContext = nop_clipboard::NopClipboardContext; // TODO: implement AndroidClipboardContext
#[cfg(target_os = "ios")]
pub type ClipboardContext = nop_clipboard::NopClipboardContext; // TODO: implement IOSClipboardContext
#[cfg(target_arch = "wasm32")]
pub type ClipboardContext = wasm_clipboard::WasmClipboardContext;
#[cfg(not(any(
    unix,
    windows,
    target_arch = "wasm32",
    target_os = "macos",
    target_os = "android",
    target_os = "ios",
    target_os = "emscripten"
)))]
pub type ClipboardContext = nop_clipboard::NopClipboardContext;
