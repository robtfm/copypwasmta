use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use async_trait::async_trait;

use crate::common::{ClipboardProvider, Result};

pub struct WasmClipboardContext;

impl WasmClipboardContext {
    pub fn new() -> Result<Self> {
        Ok(WasmClipboardContext)
    }
}

#[async_trait(?Send)]
impl ClipboardProvider for WasmClipboardContext {
    async fn get_contents(&mut self) -> Result<String> {
        let Some(window) = window() else {
            return Err("Should have a window in this context".into());
        };
        let navigator = window.navigator();
        let result = JsFuture::from(navigator.clipboard().read_text())
            .await
            .map_err(|jse| format!("{jse:?}"))?;
        Ok(result.as_string().unwrap_or_default())
    }

    async fn set_contents(&mut self, data: String) -> Result<()> {
        let Some(window) = window() else {
            return Err("Should have a window in this context".into());
        };
        let navigator = window.navigator();
        let promise = navigator.clipboard().write_text(&data);
        JsFuture::from(promise).await.map(|_| ()).map_err(|jse| format!("{jse:?}").into())
    }
}
