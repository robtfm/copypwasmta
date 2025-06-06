use wasm_bindgen_futures::spawn_local;
use web_sys::{window, console};

use crate::common::{ClipboardProvider, Result};

pub struct WasmClipboardContext;

impl WasmClipboardContext {
    pub fn new() -> Result<Self> {
        Ok(WasmClipboardContext)
    }
}

async fn write_text(data: String) {
    let Some(window) = window() else {
        console::error_1(&"should have a window in this context".into());
        return;
    };
    let navigator = window.navigator();
    let promise = navigator.clipboard().write_text(&data);
    if let Err(e) = wasm_bindgen_futures::JsFuture::from(promise).await {
        console::error_1(&format!("Failed to write to clipboard: {:?}", e).into());
    }
}

impl ClipboardProvider for WasmClipboardContext {
    fn get_contents(&mut self) -> Result<String> {
        println!(
            "Attempting to get the contents of the clipboard, which hasn't yet been implemented \
             on this platform."
        );
        Ok("".to_string())
    }

    fn set_contents(&mut self, data: String) -> Result<()> {
        spawn_local(write_text(data));
        Ok(())
    }
}
