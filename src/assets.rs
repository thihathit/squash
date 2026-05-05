use anyhow::Result;
use gpui::{AssetSource, SharedString};
use include_dir::{Dir, include_dir};
use std::borrow::Cow;

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/public");

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(ASSETS.get_file(path).map(|f| Cow::Borrowed(f.contents())))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(ASSETS
            .get_dir(path)
            .map(|dir| {
                dir.files()
                    .map(|f| f.path().to_string_lossy().into())
                    .collect()
            })
            .unwrap_or_default())
    }
}
