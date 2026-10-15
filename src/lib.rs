use zed_extension_api as zed;

struct VlangExtension;

impl zed::Extension for VlangExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // VLS publishes no standalone binaries, so there is nothing to
        // download: use the server the user installed and put on PATH.
        // `v ls --install` (from the V toolchain) is the way to get one.
        let command = worktree.which("vls").ok_or(
            "vls not found on PATH. Install it from https://github.com/vlang/vls \
             with the V toolchain (`v ls --install`) and make sure `vls` is on PATH.",
        )?;
        Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(VlangExtension);
