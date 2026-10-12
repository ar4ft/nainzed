# Upstream review: v1.23.3

Previous upstream reference: `76659a55a8c10ed355a070f8764a0b1733e3c115`
Incoming stable release: [`v1.23.3`](https://github.com/zed-industries/zed/releases/tag/v1.23.3) (`96a1b1b5389efff473fc2306f7f8c690f2e1a417`)
[Upstream comparison](https://github.com/zed-industries/zed/compare/76659a55a8c10ed355a070f8764a0b1733e3c115...96a1b1b5389efff473fc2306f7f8c690f2e1a417)

**Manual approval required for this exact candidate, including a clean merge.**

A clean merge only means Git found no textual conflicts. It can introduce new workflows, defaults, startup hooks or other behavior. Every path below needs review; passing checks or an empty keyword scan never grants approval.

Fork baseline for the actual changes: `f1d6e60f135a8158959da72a228bfba4917b343f`.

**Behavior preservation: unverified.** 0 actual changed paths require review; 0 incoming automation paths were withheld. A maintainer must explicitly accept or remove the changes before approving.

## Merge state

The merge was aborted. This draft PR contains only this report; no upstream source has been integrated.

Conflicting paths:

- `Cargo.lock`
- `crates/auto_update_ui/Cargo.toml`
- `crates/auto_update_ui/src/auto_update_ui.rs`

## Incoming automation held out of the candidate

No automation was imported into the candidate. Review the incoming automation paths below even if the merge conflicted.

## Actual changes to Nain requiring approval

No application changes were integrated; approval is still required once a candidate is resolved.

## All incoming upstream changes and potential behavior impact

| Change | Path | Potential effect requiring review |
| --- | --- | --- |
| Modified | `.agents/skills/gpui-bench/SKILL.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `.config/typos.toml` | Other upstream change; effect on Nain needs manual review |
| Modified | `.github/actionlint.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/pull_request_template.md` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/danger.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/pr_issue_labeler.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/release.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/run_tests.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.rules` | Other upstream change; effect on Nain needs manual review |
| Modified | `CONTRIBUTING.md` | Other upstream change; effect on Nain needs manual review |
| Modified | `Cargo.lock` | Dependencies, compilation, packaging or release behavior |
| Modified | `Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `assets/keymaps/default-linux.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/default-macos.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/default-windows.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/vim.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/settings/default.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/acp_thread/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/acp_thread/src/acp_thread.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/acp_thread/src/connection.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/acp_thread/src/content.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/acp_thread/src/diff.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/acp_thread/src/diff/patch.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/acp_thread/src/submission.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/acp_thread/src/terminal.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/agent/src/agent.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tests/mod.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/thread.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tool_permissions.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/edit_file_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/evals/edit_file.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/evals/terminal_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/evals/write_file.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/write_file_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/acp.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/e2e_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_settings/src/agent_settings.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_configuration/manage_profiles_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_diff.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_panel.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_ui.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/buffer_codegen.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/message_queue.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/thread_search_bar.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/thread_view.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/entry_view_state.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/favorite_models.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/inline_assistant.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/inline_prompt_editor.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/language_model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/message_editor.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/terminal_codegen.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/terminal_inline_assistant.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/threads_archive_view.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui/agent_notification.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/ai_onboarding/src/edit_prediction_onboarding_content.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/ai_onboarding/src/plan_definitions.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/audio/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/audio/src/audio_pipeline/rodio_ext.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/bench_metrics/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/bench_metrics/LICENSE-APACHE` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/bench_metrics/src/bench_metrics.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/benchmarks/benches/display_map.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/benchmarks/benches/edit_file_tool.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/buffer_diff/src/buffer_diff.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/client/src/client.rs` | Privacy, networking, background work or updates |
| Modified | `crates/client/src/user.rs` | Privacy, networking, background work or updates |
| Modified | `crates/cloud_api_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/cloud_api_client/src/websocket/native.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/collab/src/rpc.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/editor_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/git_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/integration_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/context_server/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/copilot_chat/src/copilot_chat.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/copilot_chat/src/model.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/dap_adapters/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/dap_adapters/src/python.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/edit_prediction/benches/kept_rate.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/benches/ts_error_count.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/edit_prediction.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/edit_prediction_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/onboarding_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/zed_edit_prediction_delegate.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_ui/src/edit_prediction_button.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/editor/src/code_context_menus.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/config.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/block_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/fold_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/wrap_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/editor.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/editor_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/element.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/inlays/inlay_hints.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/input.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/items.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/selection.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/eval_cli/src/main.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/extension_suggest/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/extension_suggest/LICENSE-GPL` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/extension_suggest/src/extension_suggest.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/extensions_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/extensions_ui/src/extension_suggest.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/extensions_ui/src/extension_suggestions.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/extensions_ui/src/extensions_ui.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/fs/src/fs_watcher.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/git_ui/src/commit_view.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/git_ui/src/git_graph.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/git_ui/src/git_panel.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/git_ui/src/project_diff.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/google_ai/src/google_ai.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui/src/app.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/app/bench_context.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/app/headless_app_context.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/app/test_context.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/elements/text.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/gpui.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/key_dispatch.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/platform/test/platform.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/profiler/hang.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/gpui/src/profiler/hang/monitor.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/profiler/journal.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/queue.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/gpui/src/randomized_element_tree.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/gpui/src/seeds.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/test.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/text_system/line.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/text_system/line_layout.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/text_system/line_wrapper.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/window.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui/src/window/a11y.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_apple/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_apple/src/metal_renderer.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_linux/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_linux/src/linux/dispatcher.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/window.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11/window.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_macos/src/window.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_macros/src/bench.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_macros/src/gpui_macros.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_platform/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui_platform/benches/randomized_element_tree.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_platform/src/gpui_platform.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_web/src/events.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_web/src/window.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_wgpu/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_wgpu/src/cosmic_text_system.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_wgpu/src/gpui_wgpu.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_wgpu/src/wgpu_context.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_wgpu/src/wgpu_renderer.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_windows/src/clipboard.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/gpui_windows/src/platform.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/hang_telemetry/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/hang_telemetry/LICENSE-GPL` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/hang_telemetry/src/hang_telemetry.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/http_client/src/github.rs` | Privacy, networking, background work or updates |
| Modified | `crates/language/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language/benches/highlight_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/buffer.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/language_settings.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_extension/src/extension_lsp_adapter.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_model/src/fake_provider.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model/src/language_model.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model/src/registry.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/src/language_model_core.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/src/rate_limiter.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_models/src/provider/anthropic.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/anthropic/telemetry.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/anthropic_compatible.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/bedrock.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/cloud.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/copilot_chat.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/deepseek.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/google.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/llama_cpp.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/lmstudio.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/mistral.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/ollama.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/open_ai.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/open_ai_compatible.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/open_router.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/openai_subscribed.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/opencode.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/vercel_ai_gateway.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/x_ai.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/x_ai_subscribed.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models_cloud/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_models_cloud/src/language_models_cloud.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_tools/src/lsp_log_view.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_tools/src/lsp_log_view_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/livekit_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/markdown/src/markdown.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/multi_buffer/src/multi_buffer.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/multi_buffer/src/path_key.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/node_runtime/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/node_runtime/src/node_runtime.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/node_runtime/tests/system_node.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/open_ai/src/completion.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/openai_subscribed/src/openai_subscribed.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/outline_panel/src/outline_panel.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/platform_title_bar/src/platform_title_bar.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/project/src/git_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_command.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/inlay_hints.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/log_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/project.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/task_inventory.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/terminals.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/project_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/project_panel/src/project_panel.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/proto/proto/task.proto` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/proto/proto/zed.proto` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/proto/src/proto.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/recent_projects/src/remote_servers.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/remote/src/transport.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/remote_server/src/headless_project.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/src/remote_editing_tests.rs` | Privacy, networking, background work or updates |
| Modified | `crates/rope/benches/rope_benchmark.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/search/src/buffer_search.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/settings/src/vscode_import.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/agent.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/language.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/title_bar.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/page_data.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/settings_ui.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/sidebar/src/sidebar.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/sidebar/src/sidebar_tests.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/streaming_diff/benches/streaming_diff.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/tabular_data_preview/src/parser.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/performance_metrics_overlay.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/preview_view.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/render_table.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/row_identifiers.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/settings.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/table_cell.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/table_header.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/settings.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/table_data_engine.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `crates/tabular_data_preview/src/table_view.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/tabular_data_preview.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/tabular_data_preview/src/types/table_cell.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/terminal/src/alacritty.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/terminal/src/terminal.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/terminal_view/src/terminal_element.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/terminal_view/src/terminal_view.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/text/src/tests.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/text/src/text.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/theme/src/styles/colors.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/theme/src/theme.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/title_bar/src/application_menu.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/title_bar/src/title_bar_settings.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/ui/src/components/ai/thread_item.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/ui/src/components/button/button_like.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/ui/src/components/button/icon_button.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/ui/src/components/context_menu.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/ui/src/components/data_table.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/util/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/util/src/util.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/vim/src/normal/paste.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/vim/src/state.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/vim/src/test.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/which_key/src/pending_keystrokes_indicator.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/which_key/src/which_key.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/windows_resources/src/windows_resources.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/workspace/src/searchable.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/workspace.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/x_ai_subscribed/src/x_ai_subscribed.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `crates/zed/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/zed/src/reliability/hang_detection.rs` | Startup, UI, menus, window behavior or action registration |
| Deleted | `crates/zed/src/reliability/hang_detection/telemetry.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `docs/src/SUMMARY.md` | Other upstream change; effect on Nain needs manual review |
| Modified | `docs/src/account/plans-and-pricing.md` | Other upstream change; effect on Nain needs manual review |
| Modified | `docs/src/ai/edit-prediction.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/development.md` | Other upstream change; effect on Nain needs manual review |
| Added | `docs/src/development/ui-checklist.md` | Other upstream change; effect on Nain needs manual review |
| Modified | `docs/src/key-bindings.md` | Other upstream change; effect on Nain needs manual review |
| Modified | `docs/src/languages/json.md` | Other upstream change; effect on Nain needs manual review |
| Modified | `docs/src/outline-panel.md` | Other upstream change; effect on Nain needs manual review |
| Modified | `docs/src/reference/all-settings.md` | Other upstream change; effect on Nain needs manual review |
| Modified | `docs/src/visual-customization.md` | Other upstream change; effect on Nain needs manual review |
| Modified | `tooling/xtask/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `tooling/xtask/src/tasks/gpui.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `tooling/xtask/src/tasks/gpui/validation.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `tooling/xtask/src/tasks/gpui/validation/apache_licensed.rs` | Other upstream change; effect on Nain needs manual review |
| Added | `tooling/xtask/src/tasks/gpui/validation/unique_publish_names.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/danger.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/run_tests.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/runners.rs` | Other upstream change; effect on Nain needs manual review |
| Modified | `tooling/xtask/src/workspace.rs` | Other upstream change; effect on Nain needs manual review |

## AI, telemetry, startup and background-task candidates

- `.agents/skills/gpui-bench/SKILL.md`
- `.github/pull_request_template.md`
- `CONTRIBUTING.md`
- `Cargo.lock`
- `Cargo.toml`
- `assets/settings/default.json`
- `crates/acp_thread/src/acp_thread.rs`
- `crates/acp_thread/src/connection.rs`
- `crates/acp_thread/src/content.rs`
- `crates/acp_thread/src/diff.rs`
- `crates/acp_thread/src/submission.rs`
- `crates/acp_thread/src/terminal.rs`
- `crates/agent/src/agent.rs`
- `crates/agent/src/tests/mod.rs`
- `crates/agent/src/thread.rs`
- `crates/agent/src/tools/edit_file_tool.rs`
- `crates/agent/src/tools/evals/edit_file.rs`
- `crates/agent/src/tools/evals/terminal_tool.rs`
- `crates/agent/src/tools/evals/write_file.rs`
- `crates/agent/src/tools/write_file_tool.rs`
- `crates/agent_servers/src/acp.rs`
- `crates/agent_servers/src/e2e_tests.rs`
- `crates/agent_settings/src/agent_settings.rs`
- `crates/agent_ui/src/agent_model_selector.rs`
- `crates/agent_ui/src/agent_panel.rs`
- `crates/agent_ui/src/agent_ui.rs`
- `crates/agent_ui/src/buffer_codegen.rs`
- `crates/agent_ui/src/conversation_view.rs`
- `crates/agent_ui/src/conversation_view/thread_search_bar.rs`
- `crates/agent_ui/src/conversation_view/thread_view.rs`
- `crates/agent_ui/src/entry_view_state.rs`
- `crates/agent_ui/src/inline_assistant.rs`
- `crates/agent_ui/src/inline_prompt_editor.rs`
- `crates/agent_ui/src/language_model_selector.rs`
- `crates/agent_ui/src/message_editor.rs`
- `crates/agent_ui/src/terminal_codegen.rs`
- `crates/agent_ui/src/threads_archive_view.rs`
- `crates/bench_metrics/src/bench_metrics.rs`
- `crates/benchmarks/benches/edit_file_tool.rs`
- `crates/client/src/user.rs`
- `crates/cloud_api_client/Cargo.toml`
- `crates/cloud_api_client/src/websocket/native.rs`
- `crates/collab/tests/integration/git_tests.rs`
- `crates/collab/tests/integration/integration_tests.rs`
- `crates/context_server/Cargo.toml`
- `crates/copilot_chat/src/copilot_chat.rs`
- `crates/copilot_chat/src/model.rs`
- `crates/edit_prediction/src/edit_prediction.rs`
- `crates/edit_prediction/src/edit_prediction_tests.rs`
- `crates/edit_prediction/src/zed_edit_prediction_delegate.rs`
- `crates/edit_prediction_ui/src/edit_prediction_button.rs`
- `crates/editor/src/inlays/inlay_hints.rs`
- `crates/eval_cli/src/main.rs`
- `crates/extension_suggest/src/extension_suggest.rs`
- `crates/extensions_ui/src/extensions_ui.rs`
- `crates/git_ui/src/git_panel.rs`
- `crates/google_ai/src/google_ai.rs`
- `crates/gpui/src/app.rs`
- `crates/gpui/src/app/bench_context.rs`
- `crates/gpui/src/elements/text.rs`
- `crates/gpui/src/profiler/hang.rs`
- `crates/gpui/src/profiler/hang/monitor.rs`
- `crates/gpui/src/profiler/journal.rs`
- `crates/gpui/src/queue.rs`
- `crates/gpui/src/randomized_element_tree.rs`
- `crates/gpui_platform/benches/randomized_element_tree.rs`
- `crates/gpui_wgpu/src/wgpu_context.rs`
- `crates/gpui_wgpu/src/wgpu_renderer.rs`
- `crates/hang_telemetry/Cargo.toml`
- `crates/hang_telemetry/src/hang_telemetry.rs`
- `crates/language_model/src/language_model.rs`
- `crates/language_model_core/src/rate_limiter.rs`
- `crates/language_models/src/provider/anthropic.rs`
- `crates/language_models/src/provider/anthropic/telemetry.rs`
- `crates/language_models/src/provider/anthropic_compatible.rs`
- `crates/language_models/src/provider/bedrock.rs`
- `crates/language_models/src/provider/cloud.rs`
- `crates/language_models/src/provider/copilot_chat.rs`
- `crates/language_models/src/provider/deepseek.rs`
- `crates/language_models/src/provider/google.rs`
- `crates/language_models/src/provider/llama_cpp.rs`
- `crates/language_models/src/provider/lmstudio.rs`
- `crates/language_models/src/provider/mistral.rs`
- `crates/language_models/src/provider/ollama.rs`
- `crates/language_models/src/provider/open_ai.rs`
- `crates/language_models/src/provider/open_ai_compatible.rs`
- `crates/language_models/src/provider/open_router.rs`
- `crates/language_models/src/provider/openai_subscribed.rs`
- `crates/language_models/src/provider/opencode.rs`
- `crates/language_models/src/provider/vercel_ai_gateway.rs`
- `crates/language_models/src/provider/x_ai.rs`
- `crates/language_models/src/provider/x_ai_subscribed.rs`
- `crates/language_models_cloud/src/language_models_cloud.rs`
- `crates/language_tools/src/lsp_log_view_tests.rs`
- `crates/livekit_client/Cargo.toml`
- `crates/markdown/src/markdown.rs`
- `crates/openai_subscribed/src/openai_subscribed.rs`
- `crates/outline_panel/src/outline_panel.rs`
- `crates/project/src/git_store.rs`
- `crates/project/src/lsp_store/inlay_hints.rs`
- `crates/project/src/lsp_store/log_store.rs`
- `crates/project/src/terminals.rs`
- `crates/project_benchmarks/Cargo.toml`
- `crates/proto/src/proto.rs`
- `crates/remote/src/transport.rs`
- `crates/remote_server/src/remote_editing_tests.rs`
- `crates/settings_content/src/agent.rs`
- `crates/settings_ui/src/page_data.rs`
- `crates/sidebar/src/sidebar.rs`
- `crates/sidebar/src/sidebar_tests.rs`
- `crates/tabular_data_preview/src/parser.rs`
- `crates/tabular_data_preview/src/table_data_engine.rs`
- `crates/tabular_data_preview/src/table_view.rs`
- `crates/terminal/src/terminal.rs`
- `crates/terminal_view/src/terminal_element.rs`
- `crates/terminal_view/src/terminal_view.rs`
- `crates/theme/src/styles/colors.rs`
- `crates/ui/src/components/ai/thread_item.rs`
- `crates/ui/src/components/button/button_like.rs`
- `crates/ui/src/components/button/icon_button.rs`
- `crates/which_key/src/which_key.rs`
- `crates/x_ai_subscribed/src/x_ai_subscribed.rs`
- `crates/zed/Cargo.toml`
- `crates/zed/src/reliability/hang_detection.rs`
- `docs/src/account/plans-and-pricing.md`
- `docs/src/ai/edit-prediction.md`
- `docs/src/development.md`
- `docs/src/development/ui-checklist.md`

## Network-related additions

- `.github/pull_request_template.md`
- `CONTRIBUTING.md`
- `Cargo.lock`
- `crates/acp_thread/src/acp_thread.rs`
- `crates/acp_thread/src/content.rs`
- `crates/agent/src/tests/mod.rs`
- `crates/agent_ui/src/conversation_view.rs`
- `crates/cloud_api_client/Cargo.toml`
- `crates/cloud_api_client/src/websocket/native.rs`
- `crates/collab/tests/integration/integration_tests.rs`
- `crates/context_server/Cargo.toml`
- `crates/extension_suggest/src/extension_suggest.rs`
- `crates/extensions_ui/src/extensions_ui.rs`
- `crates/google_ai/src/google_ai.rs`
- `crates/gpui/src/profiler/journal.rs`
- `crates/language_models/src/provider/anthropic.rs`
- `crates/language_models/src/provider/bedrock.rs`
- `crates/language_models/src/provider/deepseek.rs`
- `crates/language_models/src/provider/google.rs`
- `crates/language_models/src/provider/llama_cpp.rs`
- `crates/language_models/src/provider/mistral.rs`
- `crates/language_models/src/provider/open_ai.rs`
- `crates/language_models/src/provider/open_router.rs`
- `crates/language_models/src/provider/opencode.rs`
- `crates/language_models/src/provider/vercel_ai_gateway.rs`
- `crates/language_models/src/provider/x_ai.rs`
- `crates/language_models_cloud/src/language_models_cloud.rs`
- `crates/markdown/src/markdown.rs`
- `crates/openai_subscribed/src/openai_subscribed.rs`
- `crates/project_benchmarks/Cargo.toml`
- `crates/remote/src/transport.rs`
- `crates/x_ai_subscribed/src/x_ai_subscribed.rs`
- `docs/src/ai/edit-prediction.md`
- `docs/src/development.md`
- `docs/src/development/ui-checklist.md`

## Required human review

- [ ] Review new startup hooks, background tasks, network endpoints, menus and command registrations.
- [ ] Confirm AI providers, agents and telemetry collectors remain disabled; review guard changes before refreshing hashes.
- [ ] Review Apple Silicon and Intel dependency audits, regression tests and development packages.
- [ ] Check Python kernel discovery and notebook open/save/data preservation.
- [ ] Review proxy-recorded startup traffic and benchmark artifacts; investigate unknown hosts or missing graphical checks.
- [ ] Confirm editor and remote-helper dependency trees still exclude call/audio, LiveKit and WebRTC implementations.
- [ ] Confirm signing remains manual and no upstream workflow can replace the fork with upstream binaries.
- [ ] Explicitly accept or remove every behavior change and document the decision in this PR; port any wanted upstream automation separately.
- [ ] After reviewing, manually run **Approve upstream candidate** on `main` with this PR number and its current full head SHA. Confirm the review checkbox. Approval expires on any new commit.
- [ ] Review the resulting Mac checks, then mark this draft ready and merge manually using a merge commit. No signed release is triggered by merging.
