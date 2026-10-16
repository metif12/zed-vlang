use zed_extension_api as zed;

struct VlangExtension;

// Pinned VLS build — always preferred (this tag or above via deliberate,
// re-verified bumps): upstream master plus the tiered-diagnostics showcase
// and CI timeouts (vlang/vls PRs #548 and #549), built per platform as
// metif12/vls release assets. A `PATH` install is only a fallback for when
// the managed build is unreachable (offline, unsupported platform,
// declined download).
const VLS_REPO: &str = "metif12/vls";
const VLS_VERSION_TAG: &str = "vls-2";

struct VlsAsset {
    file: &'static str,
    binary: &'static str,
}

fn vls_asset() -> zed::Result<VlsAsset> {
    let (os, arch) = zed::current_platform();
    match (os, arch) {
        (zed::Os::Windows, zed::Architecture::X8664) => Ok(VlsAsset {
            file: "vls-windows-x86_64.zip",
            binary: "vls.exe",
        }),
        (zed::Os::Linux, zed::Architecture::X8664) => Ok(VlsAsset {
            file: "vls-linux-x86_64.zip",
            binary: "vls",
        }),
        (zed::Os::Mac, zed::Architecture::Aarch64) => Ok(VlsAsset {
            file: "vls-macos-aarch64.zip",
            binary: "vls",
        }),
        _ => Err("this platform ships no prebuilt VLS; install VLS with \
                  the V toolchain (`v ls --install`) and put it on PATH"
            .to_string()),
    }
}

fn managed_vls(language_server_id: &zed::LanguageServerId) -> zed::Result<String> {
    let asset = vls_asset()?;
    let dir = format!("vls-{VLS_VERSION_TAG}");
    let binary_path = format!("{dir}/{}", asset.binary);
    if std::fs::metadata(&binary_path).is_err() {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );
        let url = format!(
            "https://github.com/{VLS_REPO}/releases/download/{VLS_VERSION_TAG}/{}",
            asset.file
        );
        zed::download_file(&url, &dir, zed::DownloadedFileType::Zip)
            .map_err(|e| format!("download of {url} failed: {e}"))?;
        zed::make_file_executable(&binary_path)
            .map_err(|e| format!("cannot mark {binary_path} executable: {e}"))?;
    }
    Ok(binary_path)
}

impl zed::Extension for VlangExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // Our pinned build first: identical, verified server for every
        // user. Approve Zed's one-time download prompt when it appears.
        // A PATH install is only the fallback for when managed is
        // unreachable (its toolchain pairing still matters, since VLS
        // shells out to `v`); the managed failure is kept in the message.
        match managed_vls(language_server_id) {
            Ok(command) => Ok(zed::Command {
                command,
                args: vec![],
                env: Default::default(),
            }),
            Err(managed_err) => {
                let command = worktree.which("vls").ok_or(format!(
                    "managed VLS unavailable ({managed_err}), and no `vls` on \
                     PATH. Install VLS with the V toolchain (`v ls \
                     --install`) and make sure `vls` is on PATH."
                ))?;
                Ok(zed::Command {
                    command,
                    args: vec![],
                    env: Default::default(),
                })
            }
        }
    }
}

zed::register_extension!(VlangExtension);
