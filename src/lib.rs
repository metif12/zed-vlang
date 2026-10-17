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

// VLS reports detail strings that often restate what the label already says
// (V's `field int` style). Only append it when it actually adds information,
// so completion rows stay short instead of repeating the type twice.
fn completion_detail(label: &str, detail: Option<&str>) -> Option<String> {
    let detail = detail?.trim();
    if detail.is_empty() || label.contains(detail) {
        return None;
    }
    Some(detail.to_string())
}

// Tint completion entries by kind so a wall of VLS results separates into
// types, functions and fields at a glance. Highlight names are restricted to
// captures Zed's themes actually define (the highlight capture table in Zed's
// language-extension docs), so nothing silently falls back to unstyled.
fn completion_highlight(kind: Option<zed::lsp::CompletionKind>) -> &'static str {
    use zed::lsp::CompletionKind as K;
    match kind {
        Some(K::Struct | K::Interface | K::Enum | K::TypeParameter | K::Class) => "type",
        Some(K::Function | K::Method) => "function",
        Some(K::Constructor) => "constructor",
        Some(K::Field | K::Property) => "property",
        Some(K::EnumMember | K::Constant) => "constant",
        Some(K::Variable) => "variable",
        Some(K::Keyword) => "keyword",
        Some(K::Snippet) => "text.literal",
        _ => "identifier",
    }
}

// Kept free of `LanguageServerId` so it can be tested: that type's
// constructor is private outside zed_extension_api.
fn completion_label(completion: zed::lsp::Completion) -> Option<zed::CodeLabel> {
    let label = completion.label;
    let mut spans = vec![zed::CodeLabelSpan::Literal(zed::CodeLabelSpanLiteral {
        text: label.clone(),
        highlight_name: Some(completion_highlight(completion.kind).to_string()),
    })];

    if let Some(detail) = completion_detail(&label, completion.detail.as_deref()) {
        spans.push(zed::CodeLabelSpan::Literal(zed::CodeLabelSpanLiteral {
            text: format!(" {detail}"),
            // The signature is context, not the name: keep it quieter than the
            // identifier itself.
            highlight_name: Some("type".to_string()),
        }));
    }

    // Filter on the identifier alone, so typing never matches the appended
    // signature.
    Some(zed::CodeLabel {
        code: label.clone(),
        spans,
        filter_range: (0..label.len()).into(),
    })
}

// VLS reads exactly two settings, both booleans, from the pushed
// `workspace/didChangeConfiguration`. Verified by reading the pinned fork's
// decoder (`resolve_workspace_settings` in its `handlers.v`) and by driving the
// live server: `{"settings":{"vls":{"inlayHints":false}}}` drops the inlay-hint
// count from 1 to 0.
//
// Two corrections to earlier documentation, both measured:
//   * There is no `coverageDirectory` setting. It is silently ignored.
//   * The JSON key is camelCase `inlayHints`, not the snake_case `inlay_hints`
//     used in V's struct field. The snake_case spelling is ignored.
//
// The shape to put under `language_servers.vls.settings` is therefore:
//
//   { "vls": { "inlayHints": false, "diagnostics": true } }
//
// A JSON Schema for this is deliberately NOT shipped: Zed's
// `language_server_workspace_configuration_schema` hook exists in the 0.8.0
// WIT but not in the published 0.7.0 Rust trait, so the extension cannot
// declare one until 0.8.0 ships. Until then users write these two keys by
// hand, as docs/index.md explains.
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

    // Pass the user's `language_servers.vls.settings` straight through, so
    // whatever they write is exactly what VLS receives. Zed pushes this on
    // every settings change, which is the only way to reach a server that
    // reads configuration by push rather than by pull — VLS does.
    //
    // This hook alone is what makes the settings reachable; the JSON Schema
    // that would give them a UI is not in the 0.7.0 trait (see the note above).
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        Ok(
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .ok()
                .and_then(|settings| settings.settings),
        )
    }

    // Zed calls this hook (and, since 0.7.0, its batch form
    // `labels_for_completions`, which wraps this method). It has been
    // available in every published API version, so this is not 0.8.0-only.
    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        completion_label(completion)
    }
}

zed::register_extension!(VlangExtension);

#[cfg(test)]
mod tests {
    use super::*;

    fn completion(
        label: &str,
        kind: Option<zed::lsp::CompletionKind>,
        detail: Option<&str>,
    ) -> zed::lsp::Completion {
        zed::lsp::Completion {
            label: label.to_string(),
            label_details: None,
            detail: detail.map(str::to_string),
            kind,
            insert_text_format: None,
        }
    }

    // The WIT-generated span/range types do not implement PartialEq, so pull
    // the fields out instead of comparing whole values.
    fn literal_text(span: &zed::CodeLabelSpan) -> (&str, Option<&str>) {
        match span {
            zed::CodeLabelSpan::Literal(l) => (l.text.as_str(), l.highlight_name.as_deref()),
            zed::CodeLabelSpan::CodeRange(_) => panic!("expected a literal span"),
        }
    }

    #[test]
    fn detail_is_dropped_when_it_only_restates_the_label() {
        assert_eq!(completion_detail("count", None), None);
        assert_eq!(completion_detail("count", Some("   ")), None);
        // VLS frequently repeats the label inside detail; showing both is noise.
        assert_eq!(completion_detail("count int", Some("count int")), None);
        assert_eq!(completion_detail("count", Some("int")), Some("int".into()));
        // Multi-word signatures still add information.
        assert_eq!(
            completion_detail("println", Some("(string, ...string) msg)")),
            Some("(string, ...string) msg)".into())
        );
    }

    #[test]
    fn kinds_map_to_defined_highlights() {
        use zed::lsp::CompletionKind as K;
        for (kind, want) in [
            (K::Struct, "type"),
            (K::Interface, "type"),
            (K::Enum, "type"),
            (K::Function, "function"),
            (K::Method, "function"),
            (K::Constructor, "constructor"),
            (K::Field, "property"),
            (K::Property, "property"),
            (K::EnumMember, "constant"),
            (K::Constant, "constant"),
            (K::Variable, "variable"),
            (K::Keyword, "keyword"),
            (K::Snippet, "text.literal"),
            // Anything else, and a missing kind, still gets a label.
            (K::File, "identifier"),
            (K::Operator, "identifier"),
        ] {
            let label = completion_label(completion("x", Some(kind), None)).expect("label");
            let (text, highlight) = literal_text(&label.spans[0]);
            assert_eq!(text, "x");
            assert_eq!(highlight, Some(want), "wrong highlight for {kind:?}");
        }
        assert_eq!(
            completion_highlight(None),
            "identifier",
            "VLS omitting a kind must not drop the entry"
        );
    }

    #[test]
    fn signature_is_appended_and_filtering_ignores_it() {
        let label = completion_label(completion("count", None, Some("int"))).expect("label");
        assert_eq!(label.code, "count");
        assert_eq!(label.spans.len(), 2);
        assert_eq!(literal_text(&label.spans[0]), ("count", Some("identifier")));
        assert_eq!(literal_text(&label.spans[1]), (" int", Some("type")));
        // 0..5 is the identifier only, so the appended " int" cannot be matched.
        assert_eq!(label.filter_range.start, 0);
        assert_eq!(label.filter_range.end, 5);
    }

    #[test]
    fn restated_detail_adds_no_second_span() {
        let label =
            completion_label(completion("count int", None, Some("count int"))).expect("label");
        assert_eq!(label.spans.len(), 1);
        // filter_range covers the full label as VLS sent it.
        assert_eq!(label.filter_range.end, 9);
    }
}
