//! Copied from https://github.com/Seldom-SE/seldom_fn_plugin.
//! Allows using functions for defining plugins.

use bevy::prelude::*;

/// Implemented for `App` for the `fn_plugin` method
pub(crate) trait FnPluginExt {
    /// Runs `f` on `self`
    fn fn_plugin(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self;
}

impl FnPluginExt for App {
    fn fn_plugin(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self {
        f(self);
        self
    }
}