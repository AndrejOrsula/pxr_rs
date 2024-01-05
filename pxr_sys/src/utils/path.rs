use std::{env, path::PathBuf};

#[inline(always)]
pub fn pxr_path() -> PathBuf {
    PathBuf::from(if let Ok(path) = env::var("OPENUSD_PATH") {
        path
    } else {
        env!("OPENUSD_PATH").to_string()
    })
}
