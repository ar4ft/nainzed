# Reviewed upstream snapshot: v1.31.0

Previous upstream reference: `76659a55a8c10ed355a070f8764a0b1733e3c115`
Incoming upstream main snapshot for PR #3: `58fea3d33581871a2dbf94344261bea918ec6c65` (Zed package version `1.31.0`).
[Upstream comparison](https://github.com/zed-industries/zed/compare/76659a55a8c10ed355a070f8764a0b1733e3c115...58fea3d33581871a2dbf94344261bea918ec6c65)

**Manual approval required for this exact candidate, including a clean merge.**

A clean merge only means Git found no textual conflicts. It can introduce new workflows, defaults, startup hooks or other behavior. Every path below needs review; passing checks or an empty keyword scan never grants approval.

Fork baseline for the actual changes: `6f9d938c2770cd87622c8dfe48b27bfe801618ee`.

**Local no-AI integration review completed.** All 1433 actual changed paths are listed below, and 23 incoming automation paths were withheld. Broader behavior review, Apple-platform validation and the repository's explicit human approval remain required.

## Merge state

The branch contains a **resolved, unapproved** merge of PR #3. All 46 merge conflicts were reviewed. Incoming `.github/` changes were restored to the fork baseline so new or changed upstream automation cannot become active automatically. Manual approval remains pending.

## Incoming automation held out of the candidate

| Change | Path | Potential effect requiring review |
| --- | --- | --- |
| Modified | `.github/actionlint.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/actions/check_style/action.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/actions/run_tests/action.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/actions/run_tests_windows/action.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/pull_request_template.md` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/after_release.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/community_pr_board.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/community_pr_board_refresh.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/community_pr_cleanup.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/danger.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/deploy_collab.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/workflows/docs_suggestions.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/workflows/hotfix-review-monitor.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/pr_issue_labeler.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/release.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/release_nightly.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/run_bundling.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/run_tests.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/slack_notify_community_automation_failure.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/workflows/stale-pr-reminder.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/triage_queue_board.yml` | GitHub automation, permissions, checks or publication |
| Added | `.github/workflows/welcome_first_contribution.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/wezel.yml` | GitHub automation, permissions, checks or publication |

## Actual changes to Ned requiring approval

| Change | Path | Potential effect requiring review |
| --- | --- | --- |
| Modified | `.cargo/bundle-config.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `.cargo/config.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `.config/typos.toml` | Other upstream change; effect on Ned needs manual review |
| Deleted | `.factory/prompts/crash/fix.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `.factory/prompts/crash/investigate.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `.factory/prompts/crash/link-issues.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `.gitignore` | Other upstream change; effect on Ned needs manual review |
| Deleted | `.rules` | Other upstream change; effect on Ned needs manual review |
| Modified | `.wezel/experiments/binary-size/experiment.toml` | Other upstream change; effect on Ned needs manual review |
| Type changed | `AGENTS.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `CLAUDE.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `Cargo.lock` | Dependencies, compilation, packaging or release behavior |
| Modified | `Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `Dockerfile-collab` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `GEMINI.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `assets/icons/attach.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/book_copy.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/database_zap.svg` | Application assets, themes or branding |
| Added | `assets/icons/delete.svg` | Application assets, themes or branding |
| Added | `assets/icons/fast_forward_ultrafast.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/indicator.svg` | Application assets, themes or branding |
| Added | `assets/icons/square_r.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/text_snippet.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/tool_copy.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/tool_notification.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/triangle_right.svg` | Application assets, themes or branding |
| Modified | `assets/keymaps/default-linux.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/default-macos.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/default-windows.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/linux/jetbrains.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/macos/jetbrains.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/vim.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/no-ai-source-guards.json` | Application assets, themes or branding |
| Modified | `assets/settings/default.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `corgi.toml` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/acp_thread.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/auth_methods.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/commands.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/compaction.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/config_options.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/connection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/content.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/diff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/diff/patch.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/elicitation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/mention.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/notices.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/prompt_capabilities.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/subagents.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/acp_thread/src/submission.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/terminal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_tools/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/acp_tools/src/acp_tools.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/action_log/src/action_log.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/agent/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/agent/src/agent.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/db.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent/src/global_skills.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/outline.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/sandboxing.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/templates.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/templates/system_prompt.hbs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tests/mod.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tests/test_tools.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/thread.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/thread_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tool_permissions.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/apply_code_action_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/ask_user_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/context_server_registry.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/copy_path_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/create_directory_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/create_thread_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/delete_path_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/diagnostics_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/edit_file_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/edit_session.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/edit_session/streaming_fuzzy_matcher.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/evals/edit_file.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/evals/terminal_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/evals/write_file.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/fetch_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/find_path_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/find_references_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/get_code_actions_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/go_to_definition_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/grep_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/list_agents_and_models_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/list_directory_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/move_path_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/read_file_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/rename_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/skill_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/spawn_agent_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/symbol_locator.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/terminal_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/tool_permissions.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/web_search_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/write_file_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/agent_servers/src/acp.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/auth_status.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/acp/debug_log.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/runtime.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/session_updates.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/terminal_updates.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/acp/transport.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/v1.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/v2.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/agent_servers.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/custom.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/e2e_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_settings/src/agent_settings.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_skills/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/agent_skills/agent_skills.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_skills/global_skill_paths.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_configuration/manage_profiles_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_connection_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_diff.rs` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `crates/agent_ui/src/agent_model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_panel.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_registry_ui.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_ui.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/buffer_codegen.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/completion_provider.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/config_options.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/context_server_configuration.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/elicitation.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/message_queue.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui/src/conversation_view/subagent_card.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/thread_search_bar.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/thread_view.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/draft_prompt_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/entry_view_state.rs` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `crates/agent_ui/src/favorite_models.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/inline_prompt_editor.rs` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `crates/agent_ui/src/language_model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/mention_set.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/message_editor.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/mode_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui/src/model_search.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/profile_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/terminal_thread_metadata_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/test_support.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/thread_import.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/thread_metadata_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/thread_worktree_archive.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/threads_archive_view.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui/agent_notification.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui/mention_crease.rs` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `crates/agent_ui/src/ui/model_selector_components.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui/session_notice.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui/terminal_tool_header.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/agent_ui_benchmarks/LICENSE-GPL` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui_benchmarks/README.md` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui_benchmarks/benches/model_search.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/ai_onboarding/src/edit_prediction_onboarding_content.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/ai_onboarding/src/plan_definitions.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/anthropic/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/anthropic/src/anthropic.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/anthropic/src/completion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/askpass/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/askpass/src/askpass.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/audio/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/audio/src/audio.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/audio/src/audio_pipeline.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/audio/src/audio_pipeline/rodio_ext.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/auto_update_helper/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/auto_update_helper/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/bedrock/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/bedrock/src/bedrock.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/bedrock/src/models.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/bench_metrics/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/benchmarks/benches/editor_render.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/breadcrumbs/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/buffer_diff/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/buffer_diff/src/buffer_diff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/call/src/call_impl/room.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/call_hierarchy/src/call_hierarchy.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/channel/src/channel_buffer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/channel/src/channel_store.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cli/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/cli/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/cli/src/cli.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cli/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/client/src/client.rs` | Privacy, networking, background work or updates |
| Modified | `crates/client/src/llm_token.rs` | Privacy, networking, background work or updates |
| Modified | `crates/client/src/telemetry.rs` | Privacy, networking, background work or updates |
| Modified | `crates/client/src/user.rs` | Privacy, networking, background work or updates |
| Modified | `crates/client/src/zed_urls.rs` | Privacy, networking, background work or updates |
| Modified | `crates/clock/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/clock/src/system_clock.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cloud_api_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/cloud_api_types/src/cloud_api_types.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cloud_api_types/src/websocket_protocol.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cloud_llm_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/cloud_llm_client/src/cloud_llm_client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/codestral/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/codestral/src/codestral.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/collab/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/collab/src/api/events.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/api/extensions.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/ids.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/queries/buffers.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/queries/extensions.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/queries/projects.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/queries/rooms.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/lib.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/rpc.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/rpc/connection_pool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/channel_buffer_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/collab_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/db_tests/extension_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/editor_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/following_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/git_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/integration_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/notification_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/random_project_collaboration_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/remote_editing_collaboration_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/test_server.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/collab_ui/src/collab_panel.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab_ui/src/collab_panel/channel_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/command_palette/src/command_palette.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/component/src/component.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/component/src/component_layout.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/src/client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/src/context_server.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/src/listener.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/src/transport/stdio_transport.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/src/types.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/copilot/src/copilot.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/copilot/src/copilot_edit_prediction_delegate.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/copilot_chat/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/copilot_chat/src/copilot_chat.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/copilot_chat/src/model.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/crashes/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/credentials_provider/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/credentials_provider/src/credentials_provider.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/credentials_provider/src/test_support.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/dap/src/adapters.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap/src/dap.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap/src/debugger_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap/src/transport.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap_adapters/src/codelldb.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap_adapters/src/go.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap_adapters/src/javascript.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap_adapters/src/python.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/db/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/debugger_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/debugger_ui/src/debugger_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/dropdown_menus.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/new_process_modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/session/running.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/session/running/memory_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/session/running/stack_frame_list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/session/running/variable_list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/tests/debugger_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/debugger_ui/src/tests/hover_values.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/tests/inline_values.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/deepseek/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/dev_container/src/devcontainer_api.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dev_container/src/devcontainer_manifest.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dev_container/src/lib.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dev_container/src/oci.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/diagnostics/src/diagnostics.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/docs_preprocessor/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/docs_preprocessor/src/ai_discovery.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/docs_preprocessor/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/edit_prediction/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction/src/edit_prediction.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/edit_prediction_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/example_spec.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/fim.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/mercury.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/onboarding_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/open_ai_compatible.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/zed_edit_prediction_delegate.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction_cli/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction_cli/src/anthropic_client.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/filter_languages.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/format_prompt.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/load_project.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/main.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/openai_client.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/predict.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/qa.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/reorder_patch.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/repair.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/score.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/synthesize.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/word_diff.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_context/src/assemble_excerpts.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_context/src/edit_prediction_context.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_context/src/fake_definition_lsp.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_metrics/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction_metrics/src/kept_rate.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_metrics/src/main.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_metrics/src/patch_metrics.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction_ui/src/edit_prediction_button.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_ui/src/rate_prediction_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/editor/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/editor/src/actions.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/bracket_colorization.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/code_completion_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/code_context_menus.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/code_lens.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/completions.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/config.rs` | Editing, files, LSP, completion or search behavior |
| Added | `crates/editor/src/debugger_hover.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/diagnostics.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/block_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/fold_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/inlay_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/tab_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/document_colors.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/document_links.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/document_symbols.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/edit_prediction.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/editor/src/edit_prediction_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/editor/src/editor.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/editor_settings.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/editor_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/element.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/element/header.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/element/mouse.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/emmet_ext.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/fold.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/folding_ranges.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/git.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/git/blame.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/highlight_matching_bracket.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/hover_links.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/hover_popover.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/indent_guides.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/inlays.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/inlays/inlay_hints.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/inline_input.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/input.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/items.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/movement.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/navigation.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/persistence.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/rewrap.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/rust_analyzer_ext.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/scroll.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/scroll/autoscroll.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/selection.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/selections_collection.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/signature_help.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/split.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/split_editor_view.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/test.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/test/editor_lsp_test_context.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/test/editor_test_context.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/editor_benchmarks/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/etw_tracing/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/etw_tracing/etw_tracing.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/eval_cli/Dockerfile` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/eval_cli/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/eval_cli/zed_eval/source.py` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/explorer_command_injector/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension/src/capabilities/download_file_capability.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/capabilities/process_exec_capability.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/extension.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/extension_builder.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/extension_host_proxy.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/extension_manifest.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/extension/src/extension_resources.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_api/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension_api/wit/since_v0.8.0/extension.wit` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_cli/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension_cli/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension_host/src/extension_host.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/extension_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/extension_store_test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/headless_host.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/wasm_host.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/wasm_host/wit/since_v0_1_0.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/wasm_host/wit/since_v0_8_0.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_suggest/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/extension_suggest/LICENSE-APACHE` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/extension_suggest/LICENSE-GPL` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_suggest/src/extension_suggest.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extensions_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extensions_ui/src/components.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extensions_ui/src/components/extension_card.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extensions_ui/src/extension_suggestions.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extensions_ui/src/extensions_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/feature_flags_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/feedback/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/feedback/src/feedback.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/file_content/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/file_content/LICENSE-GPL` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/file_content/src/file_content.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/file_finder/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/file_finder/src/file_finder.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/file_finder/src/file_finder_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/file_icons/src/file_icons.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/fs/src/fake_git_repo.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/src/fs.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/src/fs_watcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/src/fs_watcher/diagnostics.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/tests/integration/fake_git_repo_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/tests/integration/fs_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/fs_benchmarks/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy/src/paths.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy/src/strings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy_nucleo/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/fuzzy_nucleo/benches/match_benchmark.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy_nucleo/src/fuzzy_nucleo.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy_nucleo/src/paths.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy_nucleo/src/strings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/git/src/blame.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/git/src/checkpoint.gitignore` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git/src/git.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git/src/repository.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git/src/status.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_hosting_providers/src/providers.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_hosting_providers/src/providers/azure.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_hosting_providers/src/providers/forgejo.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_hosting_providers/src/providers/gerrit.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_hosting_providers/src/providers/gitiles.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_hosting_providers/src/settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/git_ui/src/blame_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/branch_diff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/branch_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/clone.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/commit_context_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/commit_modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/commit_tooltip.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/commit_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/conflict_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/diff_multibuffer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_graph.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_ui/src/git_panel/branch_mode.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_panel_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_ui/src/git_timestamp.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/multi_diff_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/project_diff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/remote_output.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/repository_selector.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/stash_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/text_diff_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/created_worktrees.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/git_ui_core.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/notifications.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_ui_core/src/worktree_labels.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/worktree_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/worktree_service.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/github_releases/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/github_releases/LICENSE-APACHE` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/github_releases/src/github_releases.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/github_releases/src/releases.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/go_to_line/src/go_to_line.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/google_ai/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/google_ai/src/completion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/google_ai/src/google_ai.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui/benches/focus_cleanup.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui/examples/README.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/image/image.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/image_gallery.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/input.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/examples/native_view.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/examples/native_view_appkit_field.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/painting.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/paths_bench.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/examples/svg_animation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/system_notifications.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/testing.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/examples/toggle_headless.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/view_example/example_editor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/window_positioning.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/action.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/async_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/bench_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/entity_map.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/headless_app_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/test_app.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/test_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/visual_test_context.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/bench.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/bounds_tree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/colors.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/element.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/anchored.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/animation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/canvas.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/container_query.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/deferred.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/div.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/image_cache.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/img.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/surface.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/svg.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/text.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/uniform_list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/executor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/geometry.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/gestures.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/gpui.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/hot_reload.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/input.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/interactive.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/key_dispatch.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/keymap.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/keymap/binding.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/keymap/context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/path_builder.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/path_winding.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/keystroke.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/platform/platform_info.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/test/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/test/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/test/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/visual_test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/profiler.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/profiler/hang.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/profiler/hang/monitor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/profiler/journal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/queue.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/randomized_element_tree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/scene.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/style.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/styled.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/subscription.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/svg_renderer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/tab_stop.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/taffy.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system/font_features.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system/line.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system/line_layout.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system/line_wrapper.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/window.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/window/native_view.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/tests/eq_hash.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_apple/build.rs` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui_apple/src/dispatcher.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/display_link.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/src/gpui_apple.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/keychain.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/src/metal_atlas.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/src/metal_renderer.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/open_type.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/process_info.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/src/shaders.metal` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/text_system.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui_ios/LICENSE-APACHE` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/examples/ios.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/examples/ios/app/Info.plist` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/examples/ios/build-simulator.sh` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/gpui_ios.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/application.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/display.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/events.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/text_input.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/window.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/tests/text_input.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_linux/src/gpui_linux.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/dispatcher.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_linux/src/linux/display_connection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/headless.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/gpui_linux/src/linux/headless/client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/headless/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_linux/src/linux/power.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/clipboard.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/serial.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11/client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11/clipboard.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/xdg_desktop_portal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/gpui_macos/src/dispatcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/display_link.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/events.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/gpui_macos.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/keyboard.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/gpui_macos/src/open_type.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/pasteboard.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/system_notifications.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/gpui_macos/src/text_system.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/window.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_macos/src/window/native_views.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/window_appearance.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_macros/src/derive_inspector_reflection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macros/src/property_test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_platform/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui_platform/benches/atlas_upload.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_platform/benches/inspector_render.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_platform/benches/primitives.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_platform/benches/randomized_element_tree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_platform/src/gpui_platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_shared_string/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_shared_string/gpui_shared_string.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_tokio/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_util/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_util/src/lib.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_web/examples/hello_web/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui_web/examples/hello_web/http_client_test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/examples/hello_web/index.html` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/examples/hello_web/test_canvas_fallback.mjs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_web/examples/hello_web/test_http_client.mjs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/examples/hello_web/test_queue.mjs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/dispatcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/events.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/http_client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/viewport.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_wgpu/src/cosmic_text_system.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/shaders.wgsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/shaders_storage.wgsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/shaders_subpixel.wgsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/shaders_webgl.wgsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/wgpu_atlas.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/wgpu_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/wgpu_renderer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_windows/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_windows/src/clipboard.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/gpui_windows/src/color_text_raster.hlsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/direct_manipulation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/direct_write.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/directx_renderer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/events.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/shaders.hlsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/vsync.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/bash/config.toml` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/c/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/cpp/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/brackets.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/config.toml` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/overrides.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/redactions.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/go/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/go/runnables.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/javascript/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/rust/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/tsx/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/tsx/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/typescript/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/typescript/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/hang_telemetry/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/hang_telemetry/src/hang_telemetry.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/html_to_markdown/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/html_to_markdown/src/html_element.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/http_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/http_client/src/github.rs` | Privacy, networking, background work or updates |
| Deleted | `crates/http_client/src/github_download.rs` | Privacy, networking, background work or updates |
| Modified | `crates/http_client/src/http_client.rs` | Privacy, networking, background work or updates |
| Modified | `crates/http_client_tls/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/http_proxy/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/http_proxy/src/allowlist.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/http_proxy/src/pinned_host.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/http_proxy/src/proxy/connection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/icons/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/icons/src/icons.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/image_viewer/src/image_info.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/image_viewer/src/image_viewer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/input_latency_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/input_latency_ui/src/input_latency_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/inspector_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/inspector_ui/src/div_inspector.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/install_cli/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/json_schema_store/src/json_schema_store.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/keymap_editor/src/action_completion_provider.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/keymap_editor/src/keymap_editor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/keymap_editor/src/ui_components/keystroke_input.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/language/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/language/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language/src/available_languages.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/buffer.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/buffer/row_chunk.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/buffer_tests.rs` | Editing, files, LSP, completion or search behavior |
| Deleted | `crates/language/src/file_content.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/language.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/language_registry.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/language_settings.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/modeline.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/outline.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/proto.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/runnable.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/syntax_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/syntax_map/syntax_map_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/text_diff.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_core/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_core/src/highlight_cache.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_core/src/language_config.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_core/src/toolchain.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_extension/src/extension_lsp_adapter.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_extension/src/language_extension.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_model/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_model/src/api_key.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model/src/fake_provider.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model/src/language_model.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_model_core/src/chat_completion.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/src/language_model_core.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/src/provider.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/src/request.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/language_model_selector/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/language_model_selector/LICENSE-GPL` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/language_model_selector/src/favorite_models.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/language_model_selector/src/language_model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/language_model_selector/src/model_selector_components.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_models/src/language_models.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/anthropic.rs` | AI or collaboration surface; verify it stays unavailable |
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
| Modified | `crates/language_models/src/provider/opencode.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/vercel_ai_gateway.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/src/provider/x_ai.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models_cloud/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_models_cloud/src/language_models_cloud.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_selector/src/language_selector.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_tools/src/key_context_view.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_tools/src/lsp_button.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_tools/src/lsp_log_view_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/languages/src/c.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/cpp.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/eslint.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/go.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/json.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/lib.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/python.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/rust.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/typescript.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/vtsls.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/livekit_api/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/livekit_api/src/livekit_api.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/livekit_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/livekit_client/src/livekit_client/playback.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/livekit_client/src/record.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/lmstudio/src/lmstudio.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/lsp/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/lsp/src/input_handler.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/lsp/src/lsp.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/lsp_command_selector/src/lsp_command_selector.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/lsp_locations/src/lsp_locations.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/markdown/src/html/html_minifier.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown/src/markdown.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown/src/mermaid.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown/src/parser.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown_preview/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/markdown_preview/src/markdown_preview_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/menu/src/menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/mermaid_render/src/mermaid_render.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/postprocess/accent_colors.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/postprocess/accent_colors/class_diagram.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/postprocess/accent_colors/mindmap.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/postprocess/inject_css.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/render.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/tests/check_invalid_attrs.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/migrator/src/migrations.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/migrator/src/migrations/m_2026_09_29/settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/migrator/src/migrator.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/miniprofiler_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/miniprofiler_ui/src/miniprofiler_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mistral/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/mistral/src/mistral.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/multi_buffer/src/anchor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/multi_buffer/src/multi_buffer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/multi_buffer/src/multi_buffer_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/multi_buffer/src/path_key.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/net/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/node_runtime/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/node_runtime/src/node_runtime.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/node_runtime/tests/system_node.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/notifications/src/notification_store.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/notifications/src/status_toast.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/oauth_callback_server/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/ollama/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/ollama/src/ollama.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/onboarding/src/onboarding.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/onboarding/src/theme_preview.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/open_ai/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/open_ai/src/chat_completion_transport_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_ai/src/completion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_ai/src/open_ai.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_ai/src/responses.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_path_prompt/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/open_path_prompt/src/file_finder_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_path_prompt/src/open_path_prompt.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_path_prompt/src/open_path_prompt_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_router/src/open_router.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/openai_subscribed/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/openai_subscribed/src/openai_subscribed.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/opencode/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/opencode/src/catalog.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/opencode/src/completion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/opencode/src/opencode.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/outline/src/outline.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/outline_panel/src/outline_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/path/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/path/src/abs_path.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/path/src/path.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/path/src/path_component.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/path/src/rel_path.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/path/src/windows_prefix.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/paths/src/paths.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/picker/src/head.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/picker/src/picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/picker_preview/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/picker_preview/src/picker_preview.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/platform_title_bar/src/platform_title_bar.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/platform_title_bar/src/platforms/platform_linux.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/platform_title_bar/src/system_window_tabs.rs` | Startup, UI, menus, window behavior or action registration |
| Added | `crates/platform_title_bar/src/title_bar_settings.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/prettier/src/prettier.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/project/src/agent_registry_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/project/src/agent_server_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/project/src/bookmark_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/buffer_store.rs` | Editing, files, LSP, completion or search behavior |
| Deleted | `crates/project/src/connection_manager.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/context_server_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/breakpoint_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/dap_command.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/dap_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/locators/go.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/session.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/environment.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/git_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/git_store/diff_buffer_list.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/git_store/pending_op.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/image_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_command.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_command/signature_help.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/document_symbols.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/dynamic_registration.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/inlay_hints.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/lsp_ext_command.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/semantic_tokens.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/manifest_tree/server_tree.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/project.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/project_search.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/project_settings.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/search.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/task_inventory.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/terminals.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/trusted_worktrees.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/worktree_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/bookmark_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/color_extractor.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/context_server_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/debugger.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/dynamic_registration.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/git_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/image_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/lsp_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/project_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/search.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/task_inventory.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/trusted_worktrees.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/project_bookmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/project_bookmarks/LICENSE-GPL` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/project_bookmarks/src/project_bookmarks.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_panel/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/project_panel/benches/sorting.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_panel/src/project_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_panel/src/project_panel_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_panel/src/undo.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_symbols/src/project_symbols.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/prompt_store/src/prompt_store.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/prompt_store/src/prompts.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/proto/proto/ai.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/call.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/git.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/lsp.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/worktree.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/zed.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/src/error.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/src/proto.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proxy_handshake/src/proxy_handshake.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/recent_projects/src/disconnected_overlay.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/recent_projects.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/remote_connections.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/remote_servers.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/remote_servers/filter.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/sidebar_recent_projects.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/wsl_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/refineable/derive_refineable/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/remote/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/remote/src/command.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/remote.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/remote_client.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/remote_identity.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport/docker.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport/mock.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport/ssh.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport/wsl.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_connection/src/remote_connection.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/remote_server/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/remote_server/src/headless_project.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/src/main.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/src/remote_editing_tests.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/src/server.rs` | Privacy, networking, background work or updates |
| Deleted | `crates/remote_server/src/windows.rs` | Privacy, networking, background work or updates |
| Added | `crates/remote_server/tests/command.rs` | Privacy, networking, background work or updates |
| Modified | `crates/repl/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/repl/src/components/kernel_options.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/mod.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/native_kernel.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/remote_kernels.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/ssh_kernel.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/wsl_kernel.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/notebook/cell.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/notebook/notebook_ui.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/outputs.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/outputs/table.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/outputs/user_error.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/repl.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/repl_editor.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/repl_sessions_ui.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/session.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/reqwest_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/rope/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/rope/src/chunk.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/rope/src/rope.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/rpc/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/rpc/src/conn.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/rpc/src/message_stream.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/rpc/src/peer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/sandbox/README.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/sandbox/src/bwrap_test_helper.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/linux_bubblewrap.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/macos_seatbelt.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/sandbox.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/util.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/util/canonical_path.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/util/host_filesystem_location.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox/src/windows_psec.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/sandbox/src/windows_wsl.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/sandbox/src/wsl_sandbox_test_helper.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/sandbox_linux/DESIGN.md` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/LICENSE-GPL` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/examples/end_to_end.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/examples/run.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/broker.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/dns.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/helper.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/net.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/policy.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/process.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/protocol.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/resolve.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/sandbox_linux.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/seccomp.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/supervisor.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/sys.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/scheduler/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/scheduler/src/executor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/scheduler/src/scheduler.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/scheduler/src/test_scheduler.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/scheduler/src/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/schema_generator/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/search/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/search/src/buffer_search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/project_search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/search_bar.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/text_finder.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/text_finder/delegate.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/text_finder/render.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/settings/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/settings/src/editable_setting_control.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/editorconfig_store.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/granted_write_path.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/keymap_file.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/settings.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/settings_store.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/vscode_import.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/settings_content/src/agent.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/editor.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/extension.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/language.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/language_model.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/merge_from.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/project.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/settings_content.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/terminal.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/theme.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/title_bar.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/workspace.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_json/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/settings_json/src/settings_json.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/settings_profile_selector/src/settings_profile_selector.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/components/dropdown.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/components/font_picker.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/pages/audio_test_window.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/settings_ui.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/shell_command_parser/src/shell_command_parser.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sidebar/src/sidebar.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sidebar/src/sidebar_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sidebar/src/thread_switcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/snippet_provider/src/lib.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/snippet_provider/src/registry.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/snippets_ui/src/snippets_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sqlez/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/sqlez/src/bindable.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sqlez/src/connection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sqlez/src/migrations.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sqlez_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/sum_tree/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/sum_tree/src/sum_tree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/svg_preview/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/svg_preview/src/svg_preview_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tab_switcher/src/tab_switcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tab_switcher/src/tab_switcher_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/tabular_data_preview/src/renderer/preview_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/render_table.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/table_header.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/table_data_engine.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/table_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/tabular_data_preview.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/types/coordinates.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/task/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/task/src/static_source.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/task/src/task.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tasks_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/tasks_ui/src/modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tasks_ui/src/tasks_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/telemetry_events/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/telemetry_events/src/telemetry_events.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/alacritty.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/alacritty/hyperlinks.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/mappings/keys.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/mappings/mouse.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/terminal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal_view/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/terminal_view/src/persistence.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal_view/src/terminal_element.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal_view/src/terminal_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal_view/src/terminal_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/text/src/anchor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/src/network.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/src/selection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/src/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/src/text.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/theme/src/default_colors.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/src/icon_theme.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/src/registry.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/src/scale.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/src/styles/colors.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme_extension/src/theme_extension.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme_importer/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/theme_importer/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme_settings/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/theme_settings/src/settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme_settings/src/theme_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/time_format/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/title_bar/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/title_bar/src/application_menu.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/title_bar/src/collab.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/title_bar/src/title_bar.rs` | Startup, UI, menus, window behavior or action registration |
| Deleted | `crates/title_bar/src/title_bar_settings.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/toolchain_selector/src/toolchain_selector.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/ui/src/components.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/ai.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/ai/ai_setting_item.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/ui/src/components/ai/skills_illustration.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/ai/thread_item.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/banner.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/button_like.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/button_link.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/copy_button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/icon_button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/split_button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/toggle_button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/callout.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/chip.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/ui/src/components/choice_card.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/collab/collab_notification.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/ui/src/components/context_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/ui/src/components/context_menu/preview.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/data_table/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/diff_stat.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/dropdown_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/icon.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/image.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/indent_guides.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/keybinding.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/label/label.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/label/loading_label.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/label/spinner_label.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/list/list_bullet_item.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/notification/announcement_toast.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/popover_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/project_empty_state.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/redistributable_columns.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/right_click_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/scrollbar.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/sticky_items.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/tooltip.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/tree_view_item.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/styles/animation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/traits/animation_ext.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/traits/toggleable.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/traits/transformable.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/utils.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/utils/format_distance.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/ui/src/utils/retained_focus.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/utils/with_rem_size.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui_input/src/input_field.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/util/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/util/src/fs.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/util/src/libc_type.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/markdown.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/path_list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/paths.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/process.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/shell.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/shell_builder.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/shell_env.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/util.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/util_macros/src/util_macros.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/vim/src/command.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/digraph.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/helix.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/helix/paste.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/helix/surround.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/motion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal/increment.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal/paste.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal/repeat.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal/search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/object.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/state.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/surrounds.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/test/neovim_backed_test_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/test/neovim_connection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/test/vim_test_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/vim.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/visual.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/vim/test_data/test_replace_repetition_operators.json` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/vim/test_data/test_star_hash_under_cursor.json` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/vim/test_data/test_visual_block_end_of_document_wrapped_eof.json` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/test_data/test_visual_star_hash.json` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/watch/src/watch.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/web_search/src/web_search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/which_key/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/which_key/src/pending_bindings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/which_key/src/pending_keystrokes_indicator.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/which_key/src/which_key.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/which_key/src/which_key_modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/workspace/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/workspace/src/dock.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/focus_follows_mouse.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/invalid_item_view.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/item.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/modal_layer.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/multi_workspace.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/multi_workspace_tests.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/notifications.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/pane.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/pane_group.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/path_link.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/persistence.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/persistence/model.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/security_modal.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/status_bar.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/tasks.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/theme_preview.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/toast_layer.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/toolbar.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/welcome.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/workspace.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/workspace_error.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/workspace_settings.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/worktree/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/worktree/src/worktree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/worktree/tests/integration/worktree_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/worktree_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/x_ai/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/x_ai_subscribed/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/x_ai_subscribed/src/x_ai_subscribed.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zed/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/zed/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/zed/resources/snap/snapcraft.yaml.in` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/main.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/visual_test_runner.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/watcher_debug.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed/mac_only_instance.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed/open_listener.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed/quick_action_bar/repl_menu.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed/windows_only_instance.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed_actions/src/lib.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zed_credentials_provider/src/zed_credentials_provider.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zeta_prompt/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/zeta_prompt/src/hashed_regions.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zeta_prompt/src/multi_region.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zeta_prompt/src/udiff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zeta_prompt/src/zeta_prompt.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zlog/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/zlog/src/filter.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zlog/src/zlog.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ztracing/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/ztracing_macro/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `docs/.prettierignore` | Other upstream change; effect on Ned needs manual review |
| Deleted | `docs/.rules` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/AGENTS.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/README.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/book.toml` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/SUMMARY.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/account/plans-and-pricing.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/account/zed-hosted-models.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/agent-panel.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/agent-settings.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/edit-prediction.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/external-agents.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/parallel-agents.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/sandboxing.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/skills.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/tool-permissions.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/tools.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/use-a-gateway.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/use-an-existing-subscription.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/completions.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/configuring-languages.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/debugger.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/development/macos.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/development/release-notes.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/extensions/developing-extensions.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/extensions/languages.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/extensions/publishing/prerequisites.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/finding-navigating.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/git.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/helix.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/installation.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/key-bindings.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/css.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/emmet.md` | Other upstream change; effect on Ned needs manual review |
| Added | `docs/src/languages/env.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/gleam.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/html.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/javascript.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/python.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/rego.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/roc.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/sh.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/typescript.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/linux.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/macos.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/project-panel.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/reference/all-settings.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/reference/cli.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/remote-development.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/repl.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/tab-switcher.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/vim.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/visual-customization.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/windows-and-projects.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/worktree-trust.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/theme/analytics.js` | Other upstream change; effect on Ned needs manual review |
| Deleted | `docs/theme/c15t@2.0.0-rc.3.js` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/theme/plugins.js` | Other upstream change; effect on Ned needs manual review |
| Modified | `extensions/README.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `flake.lock` | Other upstream change; effect on Ned needs manual review |
| Modified | `legal/terms.md` | Other upstream change; effect on Ned needs manual review |
| Added | `legal/zed-product-terms.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/build.nix` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/dev/flake.lock` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/dev/flake.nix` | Other upstream change; effect on Ned needs manual review |
| Deleted | `nix/livekit-libwebrtc/0001-shared-libraries.patch` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/README.md` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/chromium-129-rust.patch` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/libwebrtc.version` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/mkSystemLibraries.nix` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/package.nix` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/sources.json` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/update.sh` | AI or collaboration surface; verify it stays unavailable |
| Modified | `nix/modules/devshells.nix` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/modules/packages.nix` | Other upstream change; effect on Ned needs manual review |
| Added | `nix/tests/sandboxing/README.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `nix/tests/sandboxing/bwrap_smoke_test.sh` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/tests/sandboxing/default.nix` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/tests/sandboxing/helper.nix` | Other upstream change; effect on Ned needs manual review |
| Modified | `rust-toolchain.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/analyze_highlights.py` | Dependencies, compilation, packaging or release behavior |
| Added | `script/bench-agent-model-search` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/bundle-freebsd` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/bundle-linux` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/bundle-mac` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/bundle-windows.ps1` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/danger/dangerfile.ts` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/docs-strip-preview-callouts` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/docs-suggest` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/docs-suggest-publish` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/generate-terms-rtf` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github-community-pr-board.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github-pr-cleanup.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github-track-duplicate-bot-effectiveness.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github-triage-queue-board.py` | Dependencies, compilation, packaging or release behavior |
| Added | `script/github-welcome-first-contribution.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github_helpers.py` | Dependencies, compilation, packaging or release behavior |
| Added | `script/gpui-bench-compare` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/linux` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/prompts` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/randomized-test-ci` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/randomized-test-minimize` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/remote-server` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/run-background-agent-mvp-local` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/snap-build` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/snap-try` | Dependencies, compilation, packaging or release behavior |
| Added | `script/snowflake_helpers.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/terms/terms.rtf` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/test-docs-suggest-batch` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/test-wsl-sandbox.ps1` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/triage_watcher.jl` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/update_top_ranking_issues/uv.lock` | Dependencies, compilation, packaging or release behavior |
| Modified | `tooling/compliance/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `tooling/compliance/src/git.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/compliance/src/github.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `tooling/crates_io_client/LICENSE-APACHE` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/crates_io_client.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/real_crates_io_client.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/simulated_crates_io_client.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/types.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/perf/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `tooling/xtask/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `tooling/xtask/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/gpui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/gpui/publish_plan.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/xtask/src/tasks/gpui/registry.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/package_conformity.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/sandbox_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/after_release.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/deploy_collab.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `tooling/xtask/src/tasks/workflows/release.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/release_nightly.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/run_bundling.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/run_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/runners.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/steps.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/vars.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `tooling/xtask/src/tasks/wsl_sandbox_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/workspace.rs` | Other upstream change; effect on Ned needs manual review |

## All incoming upstream changes and potential behavior impact

| Change | Path | Potential effect requiring review |
| --- | --- | --- |
| Modified | `.agents/skills/gpui-bench/SKILL.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `.cargo/bundle-config.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `.cargo/config.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `.config/typos.toml` | Other upstream change; effect on Ned needs manual review |
| Deleted | `.factory/prompts/crash/fix.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `.factory/prompts/crash/investigate.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `.factory/prompts/crash/link-issues.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `.github/actionlint.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/actions/check_style/action.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/actions/run_tests/action.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/actions/run_tests_windows/action.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/pull_request_template.md` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/after_release.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/community_pr_board.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/community_pr_board_refresh.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/community_pr_cleanup.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/danger.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/deploy_collab.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/workflows/docs_suggestions.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/workflows/hotfix-review-monitor.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/pr_issue_labeler.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/release.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/release_nightly.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/run_bundling.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/run_tests.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/slack_notify_community_automation_failure.yml` | GitHub automation, permissions, checks or publication |
| Deleted | `.github/workflows/stale-pr-reminder.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/triage_queue_board.yml` | GitHub automation, permissions, checks or publication |
| Added | `.github/workflows/welcome_first_contribution.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.github/workflows/wezel.yml` | GitHub automation, permissions, checks or publication |
| Modified | `.gitignore` | Other upstream change; effect on Ned needs manual review |
| Deleted | `.rules` | Other upstream change; effect on Ned needs manual review |
| Modified | `.wezel/experiments/binary-size/experiment.toml` | Other upstream change; effect on Ned needs manual review |
| Type changed | `AGENTS.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `CLAUDE.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `CONTRIBUTING.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `Cargo.lock` | Dependencies, compilation, packaging or release behavior |
| Modified | `Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `Dockerfile-collab` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `GEMINI.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `assets/icons/attach.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/book_copy.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/database_zap.svg` | Application assets, themes or branding |
| Added | `assets/icons/delete.svg` | Application assets, themes or branding |
| Added | `assets/icons/fast_forward_ultrafast.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/indicator.svg` | Application assets, themes or branding |
| Added | `assets/icons/square_r.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/text_snippet.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/tool_copy.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/tool_notification.svg` | Application assets, themes or branding |
| Deleted | `assets/icons/triangle_right.svg` | Application assets, themes or branding |
| Modified | `assets/keymaps/default-linux.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/default-macos.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/default-windows.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/linux/jetbrains.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/macos/jetbrains.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/keymaps/vim.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `assets/settings/default.json` | Defaults, settings, shortcuts or feature availability |
| Modified | `corgi.toml` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/acp_thread/src/acp_thread.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/auth_methods.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/commands.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/compaction.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/config_options.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/connection.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/content.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/diff.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/diff/patch.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/elicitation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/mention.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/notices.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/prompt_capabilities.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/acp_thread/src/subagents.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/acp_thread/src/submission.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_thread/src/terminal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/acp_tools/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/acp_tools/src/acp_tools.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/action_log/src/action_log.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/agent/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/agent/src/agent.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/db.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent/src/global_skills.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/outline.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/sandboxing.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/templates.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/templates/system_prompt.hbs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tests/mod.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tests/test_tools.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/thread.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/thread_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tool_permissions.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/apply_code_action_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/ask_user_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/context_server_registry.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/copy_path_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/create_directory_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/create_thread_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/delete_path_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/diagnostics_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/edit_file_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/edit_session.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/edit_session/streaming_fuzzy_matcher.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/evals/edit_file.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/evals/terminal_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/evals/write_file.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/fetch_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/find_path_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/find_references_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/get_code_actions_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/go_to_definition_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/grep_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/list_agents_and_models_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/list_directory_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/move_path_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/read_file_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/rename_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/skill_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/spawn_agent_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/symbol_locator.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/terminal_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/tool_permissions.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/web_search_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent/src/tools/write_file_tool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/agent_servers/src/acp.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/auth_status.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/acp/debug_log.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/runtime.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/session_updates.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/terminal_updates.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/acp/transport.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/v1.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_servers/src/acp/v2.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/agent_servers.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/custom.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_servers/src/e2e_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_settings/src/agent_settings.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_skills/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/agent_skills/agent_skills.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_skills/global_skill_paths.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_configuration/manage_profiles_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_connection_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_diff.rs` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `crates/agent_ui/src/agent_model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_panel.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_registry_ui.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/agent_ui.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/buffer_codegen.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/completion_provider.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/config_options.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/context_server_configuration.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/elicitation.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/message_queue.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui/src/conversation_view/subagent_card.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/thread_search_bar.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/conversation_view/thread_view.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/draft_prompt_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/entry_view_state.rs` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `crates/agent_ui/src/favorite_models.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/inline_assistant.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/inline_prompt_editor.rs` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `crates/agent_ui/src/language_model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/mention_set.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/message_editor.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/mode_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui/src/model_search.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/profile_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/terminal_codegen.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/terminal_inline_assistant.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/terminal_thread_metadata_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/test_support.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/thread_import.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/thread_metadata_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/thread_worktree_archive.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/threads_archive_view.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui/agent_notification.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui/mention_crease.rs` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `crates/agent_ui/src/ui/model_selector_components.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui/session_notice.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/agent_ui/src/ui/terminal_tool_header.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/agent_ui_benchmarks/LICENSE-GPL` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui_benchmarks/README.md` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/agent_ui_benchmarks/benches/model_search.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/ai_onboarding/src/edit_prediction_onboarding_content.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/ai_onboarding/src/plan_definitions.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/anthropic/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/anthropic/src/anthropic.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/anthropic/src/completion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/askpass/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/askpass/src/askpass.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/audio/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/audio/src/audio.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/audio/src/audio_pipeline.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/audio/src/audio_pipeline/rodio_ext.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/auto_update_helper/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/auto_update_helper/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/bedrock/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/bedrock/src/bedrock.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/bedrock/src/models.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/bench_metrics/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/bench_metrics/LICENSE-APACHE` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/bench_metrics/src/bench_metrics.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/benchmarks/benches/display_map.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/benchmarks/benches/edit_file_tool.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/benchmarks/benches/editor_render.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/breadcrumbs/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/buffer_diff/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/buffer_diff/src/buffer_diff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/call/src/call_impl/room.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/call_hierarchy/src/call_hierarchy.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/channel/src/channel_buffer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/channel/src/channel_store.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cli/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/cli/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/cli/src/cli.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cli/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/client/src/client.rs` | Privacy, networking, background work or updates |
| Modified | `crates/client/src/llm_token.rs` | Privacy, networking, background work or updates |
| Modified | `crates/client/src/telemetry.rs` | Privacy, networking, background work or updates |
| Modified | `crates/client/src/user.rs` | Privacy, networking, background work or updates |
| Modified | `crates/client/src/zed_urls.rs` | Privacy, networking, background work or updates |
| Modified | `crates/clock/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/clock/src/system_clock.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cloud_api_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/cloud_api_client/src/websocket/native.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cloud_api_types/src/cloud_api_types.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cloud_api_types/src/websocket_protocol.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/cloud_llm_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/cloud_llm_client/src/cloud_llm_client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/codestral/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/codestral/src/codestral.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/collab/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/collab/src/api/events.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/api/extensions.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/ids.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/queries/buffers.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/queries/extensions.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/queries/projects.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/db/queries/rooms.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/lib.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/rpc.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/src/rpc/connection_pool.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/channel_buffer_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/collab_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/db_tests/extension_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/editor_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/following_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/git_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/integration_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/notification_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/random_project_collaboration_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/remote_editing_collaboration_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab/tests/integration/test_server.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/collab_ui/src/collab_panel.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/collab_ui/src/collab_panel/channel_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/command_palette/src/command_palette.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/component/src/component.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/component/src/component_layout.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/context_server/src/client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/src/context_server.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/src/listener.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/src/transport/stdio_transport.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/context_server/src/types.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/copilot/src/copilot.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/copilot/src/copilot_edit_prediction_delegate.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/copilot_chat/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/copilot_chat/src/copilot_chat.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/copilot_chat/src/model.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/crashes/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/credentials_provider/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/credentials_provider/src/credentials_provider.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/credentials_provider/src/test_support.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/dap/src/adapters.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap/src/dap.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap/src/debugger_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap/src/transport.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap_adapters/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/dap_adapters/src/codelldb.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap_adapters/src/go.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap_adapters/src/javascript.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dap_adapters/src/python.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/db/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/debugger_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/debugger_ui/src/debugger_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/dropdown_menus.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/new_process_modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/session/running.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/session/running/memory_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/session/running/stack_frame_list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/session/running/variable_list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/tests/debugger_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/debugger_ui/src/tests/hover_values.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/debugger_ui/src/tests/inline_values.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/deepseek/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/dev_container/src/devcontainer_api.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dev_container/src/devcontainer_manifest.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dev_container/src/lib.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/dev_container/src/oci.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/diagnostics/src/diagnostics.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/docs_preprocessor/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/docs_preprocessor/src/ai_discovery.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/docs_preprocessor/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/edit_prediction/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction/benches/kept_rate.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/benches/ts_error_count.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/edit_prediction.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/edit_prediction_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/example_spec.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/fim.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/mercury.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/onboarding_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/open_ai_compatible.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction/src/zed_edit_prediction_delegate.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction_cli/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction_cli/src/anthropic_client.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/filter_languages.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/format_prompt.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/load_project.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/main.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/openai_client.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/predict.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/qa.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/reorder_patch.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/repair.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/score.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/synthesize.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_cli/src/word_diff.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_context/src/assemble_excerpts.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_context/src/edit_prediction_context.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_context/src/fake_definition_lsp.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_metrics/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction_metrics/src/kept_rate.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_metrics/src/main.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_metrics/src/patch_metrics.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/edit_prediction_ui/src/edit_prediction_button.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/edit_prediction_ui/src/rate_prediction_modal.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/editor/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/editor/src/actions.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/bracket_colorization.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/code_completion_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/code_context_menus.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/code_lens.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/completions.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/config.rs` | Editing, files, LSP, completion or search behavior |
| Added | `crates/editor/src/debugger_hover.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/diagnostics.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/block_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/fold_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/inlay_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/tab_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/display_map/wrap_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/document_colors.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/document_links.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/document_symbols.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/edit_prediction.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/editor/src/edit_prediction_tests.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/editor/src/editor.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/editor_settings.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/editor_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/element.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/element/header.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/element/mouse.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/emmet_ext.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/fold.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/folding_ranges.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/git.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/git/blame.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/highlight_matching_bracket.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/hover_links.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/hover_popover.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/indent_guides.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/inlays.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/inlays/inlay_hints.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/inline_input.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/input.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/items.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/movement.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/navigation.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/persistence.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/rewrap.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/rust_analyzer_ext.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/scroll.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/scroll/autoscroll.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/selection.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/selections_collection.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/signature_help.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/split.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/split_editor_view.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/test.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/test/editor_lsp_test_context.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor/src/test/editor_test_context.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/editor_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/editor_benchmarks/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/etw_tracing/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/etw_tracing/etw_tracing.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/eval_cli/Dockerfile` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/eval_cli/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/eval_cli/zed_eval/source.py` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/explorer_command_injector/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension/src/capabilities/download_file_capability.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/capabilities/process_exec_capability.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/extension.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/extension_builder.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/extension_host_proxy.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension/src/extension_manifest.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/extension/src/extension_resources.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_api/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension_api/wit/since_v0.8.0/extension.wit` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_cli/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension_cli/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extension_host/src/extension_host.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/extension_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/extension_store_test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/headless_host.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/wasm_host.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/wasm_host/wit/since_v0_1_0.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extension_host/src/wasm_host/wit/since_v0_8_0.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/extension_suggest/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/extension_suggest/LICENSE-APACHE` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/extension_suggest/src/extension_suggest.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extensions_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/extensions_ui/src/components.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extensions_ui/src/components/extension_card.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/extensions_ui/src/extension_suggest.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/extensions_ui/src/extension_suggestions.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/extensions_ui/src/extensions_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/feature_flags_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/feedback/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/feedback/src/feedback.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/file_content/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/file_content/LICENSE-GPL` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/file_content/src/file_content.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/file_finder/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/file_finder/src/file_finder.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/file_finder/src/file_finder_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/file_icons/src/file_icons.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/fs/src/fake_git_repo.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/src/fs.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/src/fs_watcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/src/fs_watcher/diagnostics.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/tests/integration/fake_git_repo_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs/tests/integration/fs_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fs_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/fs_benchmarks/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy/src/paths.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy/src/strings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy_nucleo/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/fuzzy_nucleo/benches/match_benchmark.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy_nucleo/src/fuzzy_nucleo.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy_nucleo/src/paths.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/fuzzy_nucleo/src/strings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/git/src/blame.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/git/src/checkpoint.gitignore` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git/src/git.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git/src/repository.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git/src/status.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_hosting_providers/src/providers.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_hosting_providers/src/providers/azure.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_hosting_providers/src/providers/forgejo.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_hosting_providers/src/providers/gerrit.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_hosting_providers/src/providers/gitiles.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_hosting_providers/src/settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/git_ui/src/blame_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/branch_diff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/branch_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/clone.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/commit_context_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/commit_modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/commit_tooltip.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/commit_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/conflict_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/diff_multibuffer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_graph.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_ui/src/git_panel/branch_mode.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_panel_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_ui/src/git_timestamp.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/git_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/multi_diff_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/project_diff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/remote_output.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/repository_selector.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/stash_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui/src/text_diff_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/created_worktrees.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/git_ui_core.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/notifications.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/git_ui_core/src/worktree_labels.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/worktree_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/git_ui_core/src/worktree_service.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/github_releases/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/github_releases/LICENSE-APACHE` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/github_releases/src/github_releases.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/github_releases/src/releases.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/go_to_line/src/go_to_line.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/google_ai/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/google_ai/src/completion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/google_ai/src/google_ai.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui/benches/focus_cleanup.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui/examples/README.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/image/image.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/image_gallery.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/input.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/examples/native_view.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/examples/native_view_appkit_field.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/painting.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/paths_bench.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/examples/svg_animation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/system_notifications.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/testing.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/examples/toggle_headless.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/view_example/example_editor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/examples/window_positioning.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/action.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/async_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/bench_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/entity_map.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/headless_app_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/test_app.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/test_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/app/visual_test_context.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/bench.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/bounds_tree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/colors.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/element.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/anchored.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/animation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/canvas.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/container_query.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/deferred.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/div.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/image_cache.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/img.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/surface.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/svg.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/text.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/elements/uniform_list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/executor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/geometry.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/gestures.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/gpui.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/hot_reload.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/input.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/interactive.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/key_dispatch.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/keymap.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/keymap/binding.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/keymap/context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/path_builder.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/path_winding.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/keystroke.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/platform/platform_info.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/test/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/test/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/test/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/platform/visual_test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/profiler.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/profiler/hang.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/profiler/hang/monitor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/profiler/journal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/queue.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/randomized_element_tree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/scene.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/seeds.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/style.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/styled.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/subscription.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/svg_renderer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/tab_stop.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/taffy.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system/font_features.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system/line.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system/line_layout.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/text_system/line_wrapper.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui/src/window/a11y.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/src/window/native_view.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui/tests/eq_hash.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_apple/build.rs` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui_apple/src/dispatcher.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/display_link.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/src/gpui_apple.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/keychain.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/src/metal_atlas.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/src/metal_renderer.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/open_type.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/process_info.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_apple/src/shaders.metal` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_apple/src/text_system.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui_ios/LICENSE-APACHE` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/examples/ios.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/examples/ios/app/Info.plist` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/examples/ios/build-simulator.sh` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/gpui_ios.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/application.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/display.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/events.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/text_input.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/src/ios/window.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_ios/tests/text_input.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_linux/src/gpui_linux.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/dispatcher.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_linux/src/linux/display_connection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/headless.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/gpui_linux/src/linux/headless/client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/headless/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_linux/src/linux/power.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/clipboard.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/serial.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/wayland/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11/client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11/clipboard.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/x11/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_linux/src/linux/xdg_desktop_portal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/gpui_macos/src/dispatcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/display_link.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/events.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/gpui_macos.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/keyboard.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/gpui_macos/src/open_type.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/pasteboard.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/system_notifications.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/gpui_macos/src/text_system.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/window.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_macos/src/window/native_views.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macos/src/window_appearance.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_macros/src/bench.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macros/src/derive_inspector_reflection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macros/src/gpui_macros.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_macros/src/property_test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_platform/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui_platform/benches/atlas_upload.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_platform/benches/inspector_render.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_platform/benches/primitives.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_platform/benches/randomized_element_tree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_platform/src/gpui_platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_shared_string/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_shared_string/gpui_shared_string.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_tokio/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_util/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_util/src/lib.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_web/examples/hello_web/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/gpui_web/examples/hello_web/http_client_test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/examples/hello_web/index.html` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/examples/hello_web/test_canvas_fallback.mjs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/gpui_web/examples/hello_web/test_http_client.mjs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/examples/hello_web/test_queue.mjs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/dispatcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/events.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/http_client.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/viewport.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_web/src/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_wgpu/src/cosmic_text_system.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/gpui_wgpu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/shaders.wgsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/shaders_storage.wgsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/shaders_subpixel.wgsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/shaders_webgl.wgsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/wgpu_atlas.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/wgpu_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_wgpu/src/wgpu_renderer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_windows/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/gpui_windows/src/clipboard.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/gpui_windows/src/color_text_raster.hlsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/direct_manipulation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/direct_write.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/directx_renderer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/display.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/events.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/platform.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/shaders.hlsl` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/vsync.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/gpui_windows/src/window.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/bash/config.toml` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/c/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/cpp/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/brackets.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/config.toml` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/overrides.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/grammars/src/env/redactions.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/go/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/go/runnables.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/javascript/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/rust/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/tsx/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/tsx/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/typescript/highlights.scm` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/grammars/src/typescript/injections.scm` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/hang_telemetry/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/hang_telemetry/LICENSE-GPL` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/hang_telemetry/src/hang_telemetry.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/html_to_markdown/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/html_to_markdown/src/html_element.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/http_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/http_client/src/github.rs` | Privacy, networking, background work or updates |
| Deleted | `crates/http_client/src/github_download.rs` | Privacy, networking, background work or updates |
| Modified | `crates/http_client/src/http_client.rs` | Privacy, networking, background work or updates |
| Modified | `crates/http_client_tls/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/http_proxy/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/http_proxy/src/allowlist.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/http_proxy/src/pinned_host.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/http_proxy/src/proxy/connection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/icons/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/icons/src/icons.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/image_viewer/src/image_info.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/image_viewer/src/image_viewer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/input_latency_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/input_latency_ui/src/input_latency_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/inspector_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/inspector_ui/src/div_inspector.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/install_cli/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/json_schema_store/src/json_schema_store.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/keymap_editor/src/action_completion_provider.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/keymap_editor/src/keymap_editor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/keymap_editor/src/ui_components/keystroke_input.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/language/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language/benches/highlight_map.rs` | Editing, files, LSP, completion or search behavior |
| Deleted | `crates/language/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language/src/available_languages.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/buffer.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/buffer/row_chunk.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/buffer_tests.rs` | Editing, files, LSP, completion or search behavior |
| Deleted | `crates/language/src/file_content.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/language.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/language_registry.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/language_settings.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/modeline.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/outline.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/proto.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/runnable.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/syntax_map.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/syntax_map/syntax_map_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language/src/text_diff.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_core/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_core/src/highlight_cache.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_core/src/language_config.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_core/src/toolchain.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_extension/src/extension_lsp_adapter.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_extension/src/language_extension.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_model/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_model/src/api_key.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model/src/fake_provider.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model/src/language_model.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model/src/registry.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_model_core/src/chat_completion.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/src/language_model_core.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/src/provider.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/src/rate_limiter.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_model_core/src/request.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/language_model_selector/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/language_model_selector/LICENSE-GPL` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/language_model_selector/src/favorite_models.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/language_model_selector/src/language_model_selector.rs` | AI or collaboration surface; verify it stays unavailable |
| Added | `crates/language_model_selector/src/model_selector_components.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/language_models/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/language_models/src/language_models.rs` | AI or collaboration surface; verify it stays unavailable |
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
| Modified | `crates/language_selector/src/language_selector.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_tools/src/key_context_view.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_tools/src/lsp_button.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_tools/src/lsp_log_view.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/language_tools/src/lsp_log_view_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/languages/src/c.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/cpp.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/eslint.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/go.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/json.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/lib.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/python.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/rust.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/typescript.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/languages/src/vtsls.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/livekit_api/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/livekit_api/src/livekit_api.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/livekit_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/livekit_client/src/livekit_client/playback.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/livekit_client/src/record.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/lmstudio/src/lmstudio.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/lsp/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/lsp/src/input_handler.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/lsp/src/lsp.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/lsp_command_selector/src/lsp_command_selector.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/lsp_locations/src/lsp_locations.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/markdown/src/html/html_minifier.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown/src/markdown.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown/src/mermaid.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown/src/parser.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/markdown_preview/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/markdown_preview/src/markdown_preview_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/menu/src/menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/mermaid_render/src/mermaid_render.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/postprocess/accent_colors.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/postprocess/accent_colors/class_diagram.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/postprocess/accent_colors/mindmap.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/postprocess/inject_css.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/src/render.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mermaid_render/tests/check_invalid_attrs.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/migrator/src/migrations.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/migrator/src/migrations/m_2026_09_29/settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/migrator/src/migrator.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/miniprofiler_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/miniprofiler_ui/src/miniprofiler_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/mistral/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/mistral/src/mistral.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/multi_buffer/src/anchor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/multi_buffer/src/multi_buffer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/multi_buffer/src/multi_buffer_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/multi_buffer/src/path_key.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/net/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/node_runtime/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/node_runtime/src/node_runtime.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/node_runtime/tests/system_node.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/notifications/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/notifications/src/notification_store.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/notifications/src/status_toast.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/oauth_callback_server/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/ollama/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/ollama/src/ollama.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/onboarding/src/onboarding.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/onboarding/src/theme_preview.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/open_ai/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/open_ai/src/chat_completion_transport_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_ai/src/completion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_ai/src/open_ai.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_ai/src/responses.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_path_prompt/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/open_path_prompt/src/file_finder_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_path_prompt/src/open_path_prompt.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_path_prompt/src/open_path_prompt_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/open_router/src/open_router.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/openai_subscribed/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/openai_subscribed/src/openai_subscribed.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/opencode/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/opencode/src/catalog.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/opencode/src/completion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/opencode/src/opencode.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/outline/src/outline.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/outline_panel/src/outline_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/path/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/path/src/abs_path.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/path/src/path.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/path/src/path_component.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/path/src/rel_path.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/path/src/windows_prefix.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/paths/src/paths.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/picker/src/head.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/picker/src/picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/picker_preview/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/picker_preview/src/picker_preview.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/platform_title_bar/src/platform_title_bar.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/platform_title_bar/src/platforms/platform_linux.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/platform_title_bar/src/system_window_tabs.rs` | Startup, UI, menus, window behavior or action registration |
| Added | `crates/platform_title_bar/src/title_bar_settings.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/prettier/src/prettier.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/project/src/agent_registry_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/project/src/agent_server_store.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/project/src/bookmark_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/buffer_store.rs` | Editing, files, LSP, completion or search behavior |
| Deleted | `crates/project/src/connection_manager.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/context_server_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/breakpoint_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/dap_command.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/dap_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/locators/go.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/debugger/session.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/environment.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/git_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/git_store/diff_buffer_list.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/git_store/pending_op.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/image_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_command.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_command/signature_help.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/document_symbols.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/dynamic_registration.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/inlay_hints.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/log_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/lsp_ext_command.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/lsp_store/semantic_tokens.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/manifest_tree/server_tree.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/project.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/project_search.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/project_settings.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/search.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/task_inventory.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/terminals.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/trusted_worktrees.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/src/worktree_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/bookmark_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/color_extractor.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/context_server_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/debugger.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/dynamic_registration.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/git_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/image_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/lsp_store.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/project_tests.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/search.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/task_inventory.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project/tests/integration/trusted_worktrees.rs` | Editing, files, LSP, completion or search behavior |
| Modified | `crates/project_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/project_bookmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/project_bookmarks/LICENSE-GPL` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/project_bookmarks/src/project_bookmarks.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_panel/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/project_panel/benches/sorting.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_panel/src/project_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_panel/src/project_panel_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_panel/src/undo.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/project_symbols/src/project_symbols.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/prompt_store/src/prompt_store.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/prompt_store/src/prompts.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/proto/proto/ai.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/call.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/git.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/lsp.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/task.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/worktree.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/proto/zed.proto` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/src/error.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proto/src/proto.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/proxy_handshake/src/proxy_handshake.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/recent_projects/src/disconnected_overlay.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/recent_projects.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/remote_connections.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/remote_servers.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/remote_servers/filter.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/sidebar_recent_projects.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/recent_projects/src/wsl_picker.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/refineable/derive_refineable/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/remote/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/remote/src/command.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/remote.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/remote_client.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/remote_identity.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport/docker.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport/mock.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport/ssh.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote/src/transport/wsl.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_connection/src/remote_connection.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/remote_server/build.rs` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/remote_server/src/global_skills.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/src/headless_project.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/src/main.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/src/remote_editing_tests.rs` | Privacy, networking, background work or updates |
| Modified | `crates/remote_server/src/server.rs` | Privacy, networking, background work or updates |
| Deleted | `crates/remote_server/src/windows.rs` | Privacy, networking, background work or updates |
| Added | `crates/remote_server/tests/command.rs` | Privacy, networking, background work or updates |
| Modified | `crates/repl/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/repl/src/components/kernel_options.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/mod.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/native_kernel.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/remote_kernels.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/ssh_kernel.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/kernels/wsl_kernel.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/notebook/cell.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/notebook/notebook_ui.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/outputs.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/outputs/table.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/outputs/user_error.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/repl.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/repl_editor.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/repl_sessions_ui.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/repl/src/session.rs` | Notebook data, execution, kernels or completion |
| Modified | `crates/reqwest_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/rope/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/rope/benches/rope_benchmark.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/rope/src/chunk.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/rope/src/rope.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/rpc/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/rpc/src/conn.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/rpc/src/message_stream.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/rpc/src/peer.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/sandbox/README.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/sandbox/src/bwrap_test_helper.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/linux_bubblewrap.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/macos_seatbelt.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/sandbox.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/util.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/util/canonical_path.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sandbox/src/util/host_filesystem_location.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox/src/windows_psec.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/sandbox/src/windows_wsl.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `crates/sandbox/src/wsl_sandbox_test_helper.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `crates/sandbox_linux/DESIGN.md` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/LICENSE-GPL` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/examples/end_to_end.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/examples/run.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/broker.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/dns.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/helper.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/net.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/policy.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/process.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/protocol.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/resolve.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/sandbox_linux.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/seccomp.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/supervisor.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/sandbox_linux/src/sys.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/scheduler/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/scheduler/src/executor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/scheduler/src/scheduler.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/scheduler/src/test_scheduler.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/scheduler/src/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/schema_generator/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/search/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/search/src/buffer_search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/project_search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/search_bar.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/text_finder.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/text_finder/delegate.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/search/src/text_finder/render.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/settings/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/settings/src/editable_setting_control.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/editorconfig_store.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/granted_write_path.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/keymap_file.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/settings.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/settings_store.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings/src/vscode_import.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/settings_content/src/agent.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/editor.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/extension.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/language.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/language_model.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/merge_from.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/project.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/settings_content.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/terminal.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/theme.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/title_bar.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_content/src/workspace.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_json/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/settings_json/src/settings_json.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/settings_profile_selector/src/settings_profile_selector.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/settings_ui/src/components/dropdown.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/components/font_picker.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/page_data.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/pages.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/pages/audio_test_window.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/pages/edit_prediction_provider_setup.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/pages/external_agents_page.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/pages/sandbox_settings.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/pages/skill_creator.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/pages/skills_setup.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/settings_ui/src/settings_ui.rs` | Defaults, settings, shortcuts or feature availability |
| Modified | `crates/shell_command_parser/src/shell_command_parser.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sidebar/src/sidebar.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sidebar/src/sidebar_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sidebar/src/thread_switcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/snippet_provider/src/lib.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/snippet_provider/src/registry.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/snippets_ui/src/snippets_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sqlez/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/sqlez/src/bindable.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sqlez/src/connection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sqlez/src/migrations.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sqlez_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/streaming_diff/benches/streaming_diff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/sum_tree/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/sum_tree/src/sum_tree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/svg_preview/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/svg_preview/src/svg_preview_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tab_switcher/src/tab_switcher.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tab_switcher/src/tab_switcher_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/tabular_data_preview/src/parser.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/performance_metrics_overlay.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/preview_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/render_table.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/row_identifiers.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/table_cell.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/renderer/table_header.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/table_data_engine.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/tabular_data_preview/src/table_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/tabular_data_preview.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/types/coordinates.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tabular_data_preview/src/types/table_cell.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/task/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `crates/task/src/static_source.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/task/src/task.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tasks_ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/tasks_ui/src/modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/tasks_ui/src/tasks_ui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/telemetry_events/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/telemetry_events/src/telemetry_events.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/alacritty.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/alacritty/hyperlinks.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/mappings/keys.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/mappings/mouse.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal/src/terminal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal_view/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/terminal_view/src/persistence.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal_view/src/terminal_element.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal_view/src/terminal_panel.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/terminal_view/src/terminal_view.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/text/src/anchor.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/src/network.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/src/selection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/src/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/text/src/text.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/theme/src/default_colors.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/src/icon_theme.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/src/registry.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/src/scale.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/src/styles/colors.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme/src/theme.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme_extension/src/theme_extension.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme_importer/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/theme_importer/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme_settings/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/theme_settings/src/settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/theme_settings/src/theme_settings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/time_format/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/title_bar/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/title_bar/src/application_menu.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/title_bar/src/collab.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/title_bar/src/title_bar.rs` | Startup, UI, menus, window behavior or action registration |
| Deleted | `crates/title_bar/src/title_bar_settings.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/toolchain_selector/src/toolchain_selector.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/ui/src/components.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/ai/ai_setting_item.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/ai/thread_item.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/banner.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/button_like.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/button_link.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/copy_button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/icon_button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/split_button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/button/toggle_button.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/callout.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/chip.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/ui/src/components/choice_card.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/collab/collab_notification.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `crates/ui/src/components/context_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/ui/src/components/context_menu/preview.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/data_table.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/data_table/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/diff_stat.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/dropdown_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/icon.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/image.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/indent_guides.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/keybinding.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/label/label.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/label/loading_label.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/label/spinner_label.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/list/list_bullet_item.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/notification/announcement_toast.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/popover_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/project_empty_state.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/redistributable_columns.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/right_click_menu.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/scrollbar.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/sticky_items.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/tooltip.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/components/tree_view_item.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/styles/animation.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/traits/animation_ext.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/traits/toggleable.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/traits/transformable.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/utils.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/utils/format_distance.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/ui/src/utils/retained_focus.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui/src/utils/with_rem_size.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui_input/src/input_field.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ui_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/util/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/util/src/fs.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/util/src/libc_type.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/markdown.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/path_list.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/paths.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/process.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/shell.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/shell_builder.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/shell_env.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util/src/util.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/util_macros/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/util_macros/src/util_macros.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/vim/src/command.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/digraph.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/helix.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/helix/paste.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/helix/surround.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/motion.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal/increment.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal/paste.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal/repeat.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/normal/search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/object.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/state.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/surrounds.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/test.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/test/neovim_backed_test_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/test/neovim_connection.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/test/vim_test_context.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/vim.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/src/visual.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/vim/test_data/test_replace_repetition_operators.json` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/vim/test_data/test_star_hash_under_cursor.json` | Other upstream change; effect on Ned needs manual review |
| Added | `crates/vim/test_data/test_visual_block_end_of_document_wrapped_eof.json` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/vim/test_data/test_visual_star_hash.json` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/watch/src/watch.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/web_search/src/web_search.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/which_key/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/which_key/src/pending_bindings.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/which_key/src/pending_keystrokes_indicator.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/which_key/src/which_key.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/which_key/src/which_key_modal.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/windows_resources/src/windows_resources.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/workspace/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/workspace/src/dock.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/focus_follows_mouse.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/invalid_item_view.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/item.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/modal_layer.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/multi_workspace.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/multi_workspace_tests.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/notifications.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/pane.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/pane_group.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/path_link.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/persistence.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/persistence/model.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/searchable.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/security_modal.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/status_bar.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/tasks.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/theme_preview.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/toast_layer.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/toolbar.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/welcome.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/workspace.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/workspace_error.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/workspace/src/workspace_settings.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/worktree/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/worktree/src/worktree.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/worktree/tests/integration/worktree_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/worktree_benchmarks/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/x_ai/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/x_ai_subscribed/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/x_ai_subscribed/src/x_ai_subscribed.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zed/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/zed/RELEASE_CHANNEL` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/build.rs` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/zed/resources/snap/snapcraft.yaml.in` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/main.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/reliability/hang_detection.rs` | Startup, UI, menus, window behavior or action registration |
| Deleted | `crates/zed/src/reliability/hang_detection/telemetry.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/visual_test_runner.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/watcher_debug.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed/mac_only_instance.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed/open_listener.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed/quick_action_bar/repl_menu.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed/telemetry_log.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed/src/zed/windows_only_instance.rs` | Startup, UI, menus, window behavior or action registration |
| Modified | `crates/zed_actions/src/lib.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zed_credentials_provider/src/zed_credentials_provider.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zeta_prompt/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/zeta_prompt/src/hashed_regions.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zeta_prompt/src/multi_region.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zeta_prompt/src/udiff.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zeta_prompt/src/zeta_prompt.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zlog/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/zlog/src/filter.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/zlog/src/zlog.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `crates/ztracing/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `crates/ztracing_macro/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `docs/.prettierignore` | Other upstream change; effect on Ned needs manual review |
| Deleted | `docs/.rules` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/AGENTS.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/README.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/book.toml` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/SUMMARY.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/account/plans-and-pricing.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/account/zed-hosted-models.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/agent-panel.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/agent-settings.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/edit-prediction.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/external-agents.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/parallel-agents.md` | AI or collaboration surface; verify it stays unavailable |
| Modified | `docs/src/ai/sandboxing.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/skills.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/tool-permissions.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/tools.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/use-a-gateway.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/ai/use-an-existing-subscription.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/completions.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/configuring-languages.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/debugger.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/development.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/development/macos.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/development/release-notes.md` | Other upstream change; effect on Ned needs manual review |
| Added | `docs/src/development/ui-checklist.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/extensions/developing-extensions.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/extensions/languages.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/extensions/publishing/prerequisites.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/finding-navigating.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/git.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/helix.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/installation.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/key-bindings.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/css.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/emmet.md` | Other upstream change; effect on Ned needs manual review |
| Added | `docs/src/languages/env.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/gleam.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/html.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/javascript.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/json.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/python.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/rego.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/roc.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/sh.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/languages/typescript.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/linux.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/macos.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/outline-panel.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/project-panel.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/reference/all-settings.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/reference/cli.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/remote-development.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/repl.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/tab-switcher.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/vim.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/visual-customization.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/windows-and-projects.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/src/worktree-trust.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/theme/analytics.js` | Other upstream change; effect on Ned needs manual review |
| Deleted | `docs/theme/c15t@2.0.0-rc.3.js` | Other upstream change; effect on Ned needs manual review |
| Modified | `docs/theme/plugins.js` | Other upstream change; effect on Ned needs manual review |
| Modified | `extensions/README.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `flake.lock` | Other upstream change; effect on Ned needs manual review |
| Modified | `legal/privacy-policy.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `legal/terms.md` | Other upstream change; effect on Ned needs manual review |
| Added | `legal/zed-product-terms.md` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/build.nix` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/dev/flake.lock` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/dev/flake.nix` | Other upstream change; effect on Ned needs manual review |
| Deleted | `nix/livekit-libwebrtc/0001-shared-libraries.patch` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/README.md` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/chromium-129-rust.patch` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/libwebrtc.version` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/mkSystemLibraries.nix` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/package.nix` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/sources.json` | AI or collaboration surface; verify it stays unavailable |
| Deleted | `nix/livekit-libwebrtc/update.sh` | AI or collaboration surface; verify it stays unavailable |
| Modified | `nix/modules/devshells.nix` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/modules/packages.nix` | Other upstream change; effect on Ned needs manual review |
| Added | `nix/tests/sandboxing/README.md` | Other upstream change; effect on Ned needs manual review |
| Deleted | `nix/tests/sandboxing/bwrap_smoke_test.sh` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/tests/sandboxing/default.nix` | Other upstream change; effect on Ned needs manual review |
| Modified | `nix/tests/sandboxing/helper.nix` | Other upstream change; effect on Ned needs manual review |
| Modified | `rust-toolchain.toml` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/analyze_highlights.py` | Dependencies, compilation, packaging or release behavior |
| Added | `script/bench-agent-model-search` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/bundle-freebsd` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/bundle-linux` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/bundle-mac` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/bundle-windows.ps1` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/danger/dangerfile.ts` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/docs-strip-preview-callouts` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/docs-suggest` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/docs-suggest-publish` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/generate-terms-rtf` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github-community-pr-board.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github-pr-cleanup.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github-track-duplicate-bot-effectiveness.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github-triage-queue-board.py` | Dependencies, compilation, packaging or release behavior |
| Added | `script/github-welcome-first-contribution.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/github_helpers.py` | Dependencies, compilation, packaging or release behavior |
| Added | `script/gpui-bench-compare` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/linux` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/prompts` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/randomized-test-ci` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/randomized-test-minimize` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/remote-server` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/run-background-agent-mvp-local` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/snap-build` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/snap-try` | Dependencies, compilation, packaging or release behavior |
| Added | `script/snowflake_helpers.py` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/terms/terms.rtf` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/test-docs-suggest-batch` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/test-wsl-sandbox.ps1` | Dependencies, compilation, packaging or release behavior |
| Deleted | `script/triage_watcher.jl` | Dependencies, compilation, packaging or release behavior |
| Modified | `script/update_top_ranking_issues/uv.lock` | Dependencies, compilation, packaging or release behavior |
| Modified | `tooling/compliance/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `tooling/compliance/src/git.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/compliance/src/github.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Added | `tooling/crates_io_client/LICENSE-APACHE` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/crates_io_client.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/real_crates_io_client.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/simulated_crates_io_client.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/tests.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/crates_io_client/src/types.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/perf/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `tooling/xtask/Cargo.toml` | Dependencies, compilation, packaging or release behavior |
| Modified | `tooling/xtask/src/main.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/gpui.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/gpui/publish_plan.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/xtask/src/tasks/gpui/registry.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/xtask/src/tasks/gpui/validation.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/xtask/src/tasks/gpui/validation/apache_licensed.rs` | Other upstream change; effect on Ned needs manual review |
| Added | `tooling/xtask/src/tasks/gpui/validation/unique_publish_names.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/package_conformity.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/sandbox_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/after_release.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/danger.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/deploy_collab.rs` | AI or collaboration surface; verify it stays unavailable |
| Modified | `tooling/xtask/src/tasks/workflows/release.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/release_nightly.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/run_bundling.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/run_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/runners.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/steps.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/tasks/workflows/vars.rs` | Other upstream change; effect on Ned needs manual review |
| Deleted | `tooling/xtask/src/tasks/wsl_sandbox_tests.rs` | Other upstream change; effect on Ned needs manual review |
| Modified | `tooling/xtask/src/workspace.rs` | Other upstream change; effect on Ned needs manual review |

## AI, telemetry, startup and background-task candidates

- `.agents/skills/gpui-bench/SKILL.md`
- `.github/pull_request_template.md`
- `.github/workflows/deploy_collab.yml`
- `.gitignore`
- `AGENTS.md`
- `CONTRIBUTING.md`
- `Cargo.lock`
- `Cargo.toml`
- `assets/icons/delete.svg`
- `assets/icons/fast_forward_ultrafast.svg`
- `assets/icons/square_r.svg`
- `assets/keymaps/default-linux.json`
- `assets/keymaps/default-macos.json`
- `assets/keymaps/default-windows.json`
- `assets/keymaps/vim.json`
- `assets/settings/default.json`
- `corgi.toml`
- `crates/acp_thread/src/acp_thread.rs`
- `crates/acp_thread/src/auth_methods.rs`
- `crates/acp_thread/src/commands.rs`
- `crates/acp_thread/src/compaction.rs`
- `crates/acp_thread/src/config_options.rs`
- `crates/acp_thread/src/connection.rs`
- `crates/acp_thread/src/content.rs`
- `crates/acp_thread/src/diff.rs`
- `crates/acp_thread/src/elicitation.rs`
- `crates/acp_thread/src/mention.rs`
- `crates/acp_thread/src/notices.rs`
- `crates/acp_thread/src/prompt_capabilities.rs`
- `crates/acp_thread/src/subagents.rs`
- `crates/acp_thread/src/submission.rs`
- `crates/acp_thread/src/terminal.rs`
- `crates/acp_tools/src/acp_tools.rs`
- `crates/agent/Cargo.toml`
- `crates/agent/src/agent.rs`
- `crates/agent/src/db.rs`
- `crates/agent/src/global_skills.rs`
- `crates/agent/src/tests/mod.rs`
- `crates/agent/src/tests/test_tools.rs`
- `crates/agent/src/thread.rs`
- `crates/agent/src/thread_store.rs`
- `crates/agent/src/tool_permissions.rs`
- `crates/agent/src/tools/apply_code_action_tool.rs`
- `crates/agent/src/tools/ask_user_tool.rs`
- `crates/agent/src/tools/context_server_registry.rs`
- `crates/agent/src/tools/copy_path_tool.rs`
- `crates/agent/src/tools/create_directory_tool.rs`
- `crates/agent/src/tools/create_thread_tool.rs`
- `crates/agent/src/tools/delete_path_tool.rs`
- `crates/agent/src/tools/diagnostics_tool.rs`
- `crates/agent/src/tools/edit_file_tool.rs`
- `crates/agent/src/tools/edit_session.rs`
- `crates/agent/src/tools/evals/edit_file.rs`
- `crates/agent/src/tools/evals/terminal_tool.rs`
- `crates/agent/src/tools/evals/write_file.rs`
- `crates/agent/src/tools/fetch_tool.rs`
- `crates/agent/src/tools/find_path_tool.rs`
- `crates/agent/src/tools/find_references_tool.rs`
- `crates/agent/src/tools/get_code_actions_tool.rs`
- `crates/agent/src/tools/go_to_definition_tool.rs`
- `crates/agent/src/tools/grep_tool.rs`
- `crates/agent/src/tools/list_agents_and_models_tool.rs`
- `crates/agent/src/tools/list_directory_tool.rs`
- `crates/agent/src/tools/move_path_tool.rs`
- `crates/agent/src/tools/read_file_tool.rs`
- `crates/agent/src/tools/rename_tool.rs`
- `crates/agent/src/tools/skill_tool.rs`
- `crates/agent/src/tools/spawn_agent_tool.rs`
- `crates/agent/src/tools/terminal_tool.rs`
- `crates/agent/src/tools/tool_permissions.rs`
- `crates/agent/src/tools/web_search_tool.rs`
- `crates/agent/src/tools/write_file_tool.rs`
- `crates/agent_servers/src/acp.rs`
- `crates/agent_servers/src/acp/auth_status.rs`
- `crates/agent_servers/src/acp/debug_log.rs`
- `crates/agent_servers/src/acp/runtime.rs`
- `crates/agent_servers/src/acp/session_updates.rs`
- `crates/agent_servers/src/acp/terminal_updates.rs`
- `crates/agent_servers/src/acp/transport.rs`
- `crates/agent_servers/src/acp/v1.rs`
- `crates/agent_servers/src/acp/v2.rs`
- `crates/agent_servers/src/agent_servers.rs`
- `crates/agent_servers/src/custom.rs`
- `crates/agent_servers/src/e2e_tests.rs`
- `crates/agent_settings/src/agent_settings.rs`
- `crates/agent_skills/agent_skills.rs`
- `crates/agent_skills/global_skill_paths.rs`
- `crates/agent_ui/Cargo.toml`
- `crates/agent_ui/src/agent_configuration/manage_profiles_modal.rs`
- `crates/agent_ui/src/agent_connection_store.rs`
- `crates/agent_ui/src/agent_diff.rs`
- `crates/agent_ui/src/agent_panel.rs`
- `crates/agent_ui/src/agent_ui.rs`
- `crates/agent_ui/src/buffer_codegen.rs`
- `crates/agent_ui/src/completion_provider.rs`
- `crates/agent_ui/src/config_options.rs`
- `crates/agent_ui/src/context_server_configuration.rs`
- `crates/agent_ui/src/conversation_view.rs`
- `crates/agent_ui/src/conversation_view/elicitation.rs`
- `crates/agent_ui/src/conversation_view/subagent_card.rs`
- `crates/agent_ui/src/conversation_view/thread_search_bar.rs`
- `crates/agent_ui/src/conversation_view/thread_view.rs`
- `crates/agent_ui/src/draft_prompt_store.rs`
- `crates/agent_ui/src/entry_view_state.rs`
- `crates/agent_ui/src/inline_assistant.rs`
- `crates/agent_ui/src/inline_prompt_editor.rs`
- `crates/agent_ui/src/mention_set.rs`
- `crates/agent_ui/src/message_editor.rs`
- `crates/agent_ui/src/mode_selector.rs`
- `crates/agent_ui/src/model_search.rs`
- `crates/agent_ui/src/model_selector.rs`
- `crates/agent_ui/src/profile_selector.rs`
- `crates/agent_ui/src/terminal_codegen.rs`
- `crates/agent_ui/src/test_support.rs`
- `crates/agent_ui/src/thread_import.rs`
- `crates/agent_ui/src/thread_metadata_store.rs`
- `crates/agent_ui/src/threads_archive_view.rs`
- `crates/agent_ui/src/ui.rs`
- `crates/agent_ui/src/ui/mention_crease.rs`
- `crates/agent_ui/src/ui/session_notice.rs`
- `crates/agent_ui_benchmarks/Cargo.toml`
- `crates/agent_ui_benchmarks/README.md`
- `crates/agent_ui_benchmarks/benches/model_search.rs`
- `crates/anthropic/src/anthropic.rs`
- `crates/anthropic/src/completion.rs`
- `crates/audio/src/audio_pipeline.rs`
- `crates/bedrock/src/bedrock.rs`
- `crates/bedrock/src/models.rs`
- `crates/bench_metrics/src/bench_metrics.rs`
- `crates/benchmarks/benches/edit_file_tool.rs`
- `crates/cli/src/cli.rs`
- `crates/client/src/telemetry.rs`
- `crates/client/src/user.rs`
- `crates/cloud_api_client/Cargo.toml`
- `crates/cloud_api_client/src/websocket/native.rs`
- `crates/collab/Cargo.toml`
- `crates/collab/tests/integration/git_tests.rs`
- `crates/collab/tests/integration/integration_tests.rs`
- `crates/collab_ui/src/collab_panel.rs`
- `crates/context_server/Cargo.toml`
- `crates/context_server/src/transport/stdio_transport.rs`
- `crates/copilot/src/copilot.rs`
- `crates/copilot_chat/Cargo.toml`
- `crates/copilot_chat/src/copilot_chat.rs`
- `crates/copilot_chat/src/model.rs`
- `crates/dap/src/adapters.rs`
- `crates/dap_adapters/src/python.rs`
- `crates/debugger_ui/src/tests/debugger_panel.rs`
- `crates/debugger_ui/src/tests/hover_values.rs`
- `crates/dev_container/src/lib.rs`
- `crates/dev_container/src/oci.rs`
- `crates/edit_prediction/Cargo.toml`
- `crates/edit_prediction/src/edit_prediction.rs`
- `crates/edit_prediction/src/edit_prediction_tests.rs`
- `crates/edit_prediction/src/fim.rs`
- `crates/edit_prediction/src/zed_edit_prediction_delegate.rs`
- `crates/edit_prediction_cli/Cargo.toml`
- `crates/edit_prediction_cli/src/format_prompt.rs`
- `crates/edit_prediction_metrics/Cargo.toml`
- `crates/edit_prediction_metrics/src/main.rs`
- `crates/edit_prediction_ui/Cargo.toml`
- `crates/edit_prediction_ui/src/edit_prediction_button.rs`
- `crates/editor/src/code_lens.rs`
- `crates/editor/src/debugger_hover.rs`
- `crates/editor/src/diagnostics.rs`
- `crates/editor/src/display_map.rs`
- `crates/editor/src/display_map/tab_map.rs`
- `crates/editor/src/document_colors.rs`
- `crates/editor/src/document_links.rs`
- `crates/editor/src/document_symbols.rs`
- `crates/editor/src/editor.rs`
- `crates/editor/src/editor_tests.rs`
- `crates/editor/src/element.rs`
- `crates/editor/src/element/header.rs`
- `crates/editor/src/folding_ranges.rs`
- `crates/editor/src/hover_links.rs`
- `crates/editor/src/hover_popover.rs`
- `crates/editor/src/inlays/inlay_hints.rs`
- `crates/editor/src/rust_analyzer_ext.rs`
- `crates/eval_cli/src/main.rs`
- `crates/extension_host/src/extension_store_test.rs`
- `crates/extension_suggest/src/extension_suggest.rs`
- `crates/extensions_ui/src/components/extension_card.rs`
- `crates/extensions_ui/src/extensions_ui.rs`
- `crates/feedback/src/feedback.rs`
- `crates/file_finder/src/file_finder.rs`
- `crates/file_finder/src/file_finder_tests.rs`
- `crates/fs/src/fs.rs`
- `crates/fs/src/fs_watcher.rs`
- `crates/fs/tests/integration/fake_git_repo_tests.rs`
- `crates/fs/tests/integration/fs_tests.rs`
- `crates/fuzzy_nucleo/src/paths.rs`
- `crates/git/src/repository.rs`
- `crates/git_hosting_providers/src/providers/forgejo.rs`
- `crates/git_hosting_providers/src/providers/gerrit.rs`
- `crates/git_hosting_providers/src/providers/gitiles.rs`
- `crates/git_ui/Cargo.toml`
- `crates/git_ui/src/branch_picker.rs`
- `crates/git_ui/src/commit_view.rs`
- `crates/git_ui/src/git_graph.rs`
- `crates/git_ui/src/git_panel.rs`
- `crates/git_ui/src/git_panel/branch_mode.rs`
- `crates/git_ui/src/git_ui.rs`
- `crates/git_ui/src/remote_output.rs`
- `crates/git_ui_core/src/notifications.rs`
- `crates/git_ui_core/src/worktree_picker.rs`
- `crates/git_ui_core/src/worktree_service.rs`
- `crates/github_releases/Cargo.toml`
- `crates/github_releases/src/github_releases.rs`
- `crates/github_releases/src/releases.rs`
- `crates/google_ai/Cargo.toml`
- `crates/google_ai/src/completion.rs`
- `crates/google_ai/src/google_ai.rs`
- `crates/gpui/benches/focus_cleanup.rs`
- `crates/gpui/examples/image/image.rs`
- `crates/gpui/examples/image_gallery.rs`
- `crates/gpui/examples/native_view.rs`
- `crates/gpui/examples/native_view_appkit_field.rs`
- `crates/gpui/examples/painting.rs`
- `crates/gpui/examples/paths_bench.rs`
- `crates/gpui/examples/svg_animation.rs`
- `crates/gpui/examples/toggle_headless.rs`
- `crates/gpui/src/app.rs`
- `crates/gpui/src/app/async_context.rs`
- `crates/gpui/src/app/bench_context.rs`
- `crates/gpui/src/app/test_context.rs`
- `crates/gpui/src/app/visual_test_context.rs`
- `crates/gpui/src/elements/animation.rs`
- `crates/gpui/src/elements/svg.rs`
- `crates/gpui/src/elements/text.rs`
- `crates/gpui/src/executor.rs`
- `crates/gpui/src/gpui.rs`
- `crates/gpui/src/hot_reload.rs`
- `crates/gpui/src/keymap.rs`
- `crates/gpui/src/platform.rs`
- `crates/gpui/src/platform/test/platform.rs`
- `crates/gpui/src/platform/test/window.rs`
- `crates/gpui/src/profiler/hang.rs`
- `crates/gpui/src/profiler/hang/monitor.rs`
- `crates/gpui/src/profiler/journal.rs`
- `crates/gpui/src/queue.rs`
- `crates/gpui/src/randomized_element_tree.rs`
- `crates/gpui/src/scene.rs`
- `crates/gpui/src/svg_renderer.rs`
- `crates/gpui/src/text_system/line.rs`
- `crates/gpui/src/window.rs`
- `crates/gpui/src/window/native_view.rs`
- `crates/gpui_apple/src/display_link.rs`
- `crates/gpui_apple/src/gpui_apple.rs`
- `crates/gpui_apple/src/metal_renderer.rs`
- `crates/gpui_apple/src/process_info.rs`
- `crates/gpui_apple/src/shaders.metal`
- `crates/gpui_ios/examples/ios/app/Info.plist`
- `crates/gpui_ios/examples/ios/build-simulator.sh`
- `crates/gpui_ios/src/gpui_ios.rs`
- `crates/gpui_ios/src/ios/application.rs`
- `crates/gpui_ios/src/ios/platform.rs`
- `crates/gpui_ios/src/ios/window.rs`
- `crates/gpui_linux/src/linux/display_connection.rs`
- `crates/gpui_linux/src/linux/platform.rs`
- `crates/gpui_linux/src/linux/power.rs`
- `crates/gpui_linux/src/linux/wayland/client.rs`
- `crates/gpui_linux/src/linux/wayland/window.rs`
- `crates/gpui_linux/src/linux/x11/client.rs`
- `crates/gpui_linux/src/linux/x11/clipboard.rs`
- `crates/gpui_linux/src/linux/x11/window.rs`
- `crates/gpui_linux/src/linux/xdg_desktop_portal.rs`
- `crates/gpui_macos/src/events.rs`
- `crates/gpui_macos/src/pasteboard.rs`
- `crates/gpui_macos/src/platform.rs`
- `crates/gpui_macos/src/system_notifications.rs`
- `crates/gpui_macos/src/window.rs`
- `crates/gpui_macos/src/window/native_views.rs`
- `crates/gpui_platform/benches/primitives.rs`
- `crates/gpui_platform/benches/randomized_element_tree.rs`
- `crates/gpui_web/examples/hello_web/Cargo.toml`
- `crates/gpui_web/examples/hello_web/http_client_test.rs`
- `crates/gpui_web/examples/hello_web/index.html`
- `crates/gpui_web/examples/hello_web/test_http_client.mjs`
- `crates/gpui_web/src/events.rs`
- `crates/gpui_web/src/http_client.rs`
- `crates/gpui_web/src/platform.rs`
- `crates/gpui_web/src/window.rs`
- `crates/gpui_wgpu/Cargo.toml`
- `crates/gpui_wgpu/src/cosmic_text_system.rs`
- `crates/gpui_wgpu/src/shaders.wgsl`
- `crates/gpui_wgpu/src/shaders_webgl.wgsl`
- `crates/gpui_wgpu/src/wgpu_context.rs`
- `crates/gpui_wgpu/src/wgpu_renderer.rs`
- `crates/gpui_windows/Cargo.toml`
- `crates/gpui_windows/src/directx_renderer.rs`
- `crates/gpui_windows/src/events.rs`
- `crates/gpui_windows/src/shaders.hlsl`
- `crates/hang_telemetry/Cargo.toml`
- `crates/hang_telemetry/src/hang_telemetry.rs`
- `crates/input_latency_ui/src/input_latency_ui.rs`
- `crates/keymap_editor/src/keymap_editor.rs`
- `crates/language/src/buffer.rs`
- `crates/language/src/buffer_tests.rs`
- `crates/language/src/language.rs`
- `crates/language/src/language_settings.rs`
- `crates/language/src/syntax_map/syntax_map_tests.rs`
- `crates/language_model/src/api_key.rs`
- `crates/language_model/src/language_model.rs`
- `crates/language_model/src/registry.rs`
- `crates/language_model_core/src/provider.rs`
- `crates/language_model_core/src/rate_limiter.rs`
- `crates/language_model_core/src/request.rs`
- `crates/language_model_selector/Cargo.toml`
- `crates/language_model_selector/src/language_model_selector.rs`
- `crates/language_model_selector/src/model_selector_components.rs`
- `crates/language_models/src/language_models.rs`
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
- `crates/languages/src/c.rs`
- `crates/languages/src/eslint.rs`
- `crates/languages/src/lib.rs`
- `crates/languages/src/python.rs`
- `crates/languages/src/rust.rs`
- `crates/livekit_api/Cargo.toml`
- `crates/livekit_client/Cargo.toml`
- `crates/lsp/src/lsp.rs`
- `crates/markdown/src/markdown.rs`
- `crates/markdown/src/mermaid.rs`
- `crates/markdown/src/parser.rs`
- `crates/mermaid_render/src/render.rs`
- `crates/migrator/src/migrations.rs`
- `crates/migrator/src/migrations/m_2026_09_29/settings.rs`
- `crates/migrator/src/migrator.rs`
- `crates/multi_buffer/src/multi_buffer.rs`
- `crates/multi_buffer/src/multi_buffer_tests.rs`
- `crates/node_runtime/src/node_runtime.rs`
- `crates/node_runtime/tests/system_node.rs`
- `crates/open_ai/src/chat_completion_transport_tests.rs`
- `crates/open_ai/src/completion.rs`
- `crates/open_ai/src/open_ai.rs`
- `crates/open_ai/src/responses.rs`
- `crates/openai_subscribed/src/openai_subscribed.rs`
- `crates/opencode/Cargo.toml`
- `crates/opencode/src/catalog.rs`
- `crates/opencode/src/completion.rs`
- `crates/opencode/src/opencode.rs`
- `crates/outline_panel/src/outline_panel.rs`
- `crates/path/src/windows_prefix.rs`
- `crates/picker_preview/src/picker_preview.rs`
- `crates/project/Cargo.toml`
- `crates/project/src/agent_registry_store.rs`
- `crates/project/src/agent_server_store.rs`
- `crates/project/src/buffer_store.rs`
- `crates/project/src/debugger/session.rs`
- `crates/project/src/git_store.rs`
- `crates/project/src/lsp_command.rs`
- `crates/project/src/lsp_store.rs`
- `crates/project/src/lsp_store/inlay_hints.rs`
- `crates/project/src/lsp_store/log_store.rs`
- `crates/project/src/project.rs`
- `crates/project/src/project_settings.rs`
- `crates/project/src/terminals.rs`
- `crates/project/tests/integration/debugger.rs`
- `crates/project/tests/integration/git_store.rs`
- `crates/project/tests/integration/image_store.rs`
- `crates/project/tests/integration/project_tests.rs`
- `crates/project_benchmarks/Cargo.toml`
- `crates/project_bookmarks/src/project_bookmarks.rs`
- `crates/project_panel/src/project_panel_tests.rs`
- `crates/proto/src/proto.rs`
- `crates/recent_projects/src/remote_connections.rs`
- `crates/recent_projects/src/remote_servers.rs`
- `crates/remote/src/command.rs`
- `crates/remote/src/remote_client.rs`
- `crates/remote/src/transport.rs`
- `crates/remote/src/transport/docker.rs`
- `crates/remote/src/transport/mock.rs`
- `crates/remote/src/transport/ssh.rs`
- `crates/remote/src/transport/wsl.rs`
- `crates/remote_server/Cargo.toml`
- `crates/remote_server/src/global_skills.rs`
- `crates/remote_server/src/headless_project.rs`
- `crates/remote_server/src/remote_editing_tests.rs`
- `crates/remote_server/src/server.rs`
- `crates/remote_server/tests/command.rs`
- `crates/repl/src/notebook/cell.rs`
- `crates/sandbox/Cargo.toml`
- `crates/sandbox/src/sandbox.rs`
- `crates/sandbox/src/windows_psec.rs`
- `crates/sandbox_linux/DESIGN.md`
- `crates/sandbox_linux/examples/end_to_end.rs`
- `crates/sandbox_linux/examples/run.rs`
- `crates/sandbox_linux/src/broker.rs`
- `crates/sandbox_linux/src/dns.rs`
- `crates/sandbox_linux/src/helper.rs`
- `crates/sandbox_linux/src/net.rs`
- `crates/sandbox_linux/src/policy.rs`
- `crates/sandbox_linux/src/sandbox_linux.rs`
- `crates/sandbox_linux/src/supervisor.rs`
- `crates/scheduler/src/executor.rs`
- `crates/scheduler/src/scheduler.rs`
- `crates/scheduler/src/tests.rs`
- `crates/settings/src/keymap_file.rs`
- `crates/settings/src/settings.rs`
- `crates/settings/src/settings_store.rs`
- `crates/settings/src/vscode_import.rs`
- `crates/settings_content/src/agent.rs`
- `crates/settings_content/src/language.rs`
- `crates/settings_content/src/project.rs`
- `crates/settings_content/src/settings_content.rs`
- `crates/settings_content/src/theme.rs`
- `crates/settings_ui/src/page_data.rs`
- `crates/settings_ui/src/pages/skill_creator.rs`
- `crates/settings_ui/src/pages/skills_setup.rs`
- `crates/settings_ui/src/settings_ui.rs`
- `crates/sidebar/src/sidebar.rs`
- `crates/sidebar/src/sidebar_tests.rs`
- `crates/svg_preview/src/svg_preview_view.rs`
- `crates/tabular_data_preview/src/parser.rs`
- `crates/tabular_data_preview/src/table_data_engine.rs`
- `crates/tabular_data_preview/src/table_view.rs`
- `crates/tasks_ui/src/modal.rs`
- `crates/tasks_ui/src/tasks_ui.rs`
- `crates/terminal/src/terminal.rs`
- `crates/terminal_view/src/terminal_element.rs`
- `crates/terminal_view/src/terminal_view.rs`
- `crates/theme/src/styles/colors.rs`
- `crates/ui/src/components/ai/thread_item.rs`
- `crates/ui/src/components/button/button_like.rs`
- `crates/ui/src/components/button/button_link.rs`
- `crates/ui/src/components/button/icon_button.rs`
- `crates/ui/src/components/choice_card.rs`
- `crates/ui/src/components/scrollbar.rs`
- `crates/util/src/process.rs`
- `crates/util/src/shell.rs`
- `crates/util/src/shell_env.rs`
- `crates/vim/src/normal/search.rs`
- `crates/which_key/src/pending_bindings.rs`
- `crates/which_key/src/which_key.rs`
- `crates/workspace/src/multi_workspace.rs`
- `crates/workspace/src/multi_workspace_tests.rs`
- `crates/workspace/src/notifications.rs`
- `crates/workspace/src/security_modal.rs`
- `crates/workspace/src/workspace.rs`
- `crates/worktree/src/worktree.rs`
- `crates/worktree/tests/integration/worktree_tests.rs`
- `crates/x_ai_subscribed/src/x_ai_subscribed.rs`
- `crates/zed/Cargo.toml`
- `crates/zed/src/main.rs`
- `crates/zed/src/reliability/hang_detection.rs`
- `crates/zed/src/visual_test_runner.rs`
- `crates/zed/src/watcher_debug.rs`
- `crates/zed/src/zed.rs`
- `crates/zed/src/zed/open_listener.rs`
- `crates/zed/src/zed/windows_only_instance.rs`
- `docs/AGENTS.md`
- `docs/src/account/plans-and-pricing.md`
- `docs/src/account/zed-hosted-models.md`
- `docs/src/ai/agent-panel.md`
- `docs/src/ai/agent-settings.md`
- `docs/src/ai/edit-prediction.md`
- `docs/src/ai/external-agents.md`
- `docs/src/ai/parallel-agents.md`
- `docs/src/ai/skills.md`
- `docs/src/ai/tool-permissions.md`
- `docs/src/ai/tools.md`
- `docs/src/ai/use-a-gateway.md`
- `docs/src/ai/use-an-existing-subscription.md`
- `docs/src/completions.md`
- `docs/src/development.md`
- `docs/src/development/ui-checklist.md`
- `docs/src/extensions/languages.md`
- `docs/src/extensions/publishing/prerequisites.md`
- `docs/src/git.md`
- `docs/src/installation.md`
- `docs/src/languages.md`
- `docs/src/languages/emmet.md`
- `docs/src/languages/env.md`
- `docs/src/languages/gleam.md`
- `docs/src/languages/python.md`
- `docs/src/languages/rego.md`
- `docs/src/languages/roc.md`
- `docs/src/linux.md`
- `docs/src/reference/all-settings.md`
- `docs/src/repl.md`
- `docs/src/vim.md`
- `docs/src/visual-customization.md`
- `extensions/README.md`
- `legal/privacy-policy.md`
- `legal/terms.md`
- `legal/zed-product-terms.md`
- `nix/build.nix`
- `script/bench-agent-model-search`
- `script/bundle-linux`
- `script/bundle-mac`
- `script/danger/dangerfile.ts`
- `script/github-community-pr-board.py`
- `script/github-pr-cleanup.py`
- `script/github-welcome-first-contribution.py`
- `script/linux`
- `script/snowflake_helpers.py`
- `script/terms/terms.rtf`
- `script/update_top_ranking_issues/uv.lock`
- `tooling/crates_io_client/Cargo.toml`
- `tooling/crates_io_client/src/crates_io_client.rs`
- `tooling/crates_io_client/src/real_crates_io_client.rs`
- `tooling/crates_io_client/src/simulated_crates_io_client.rs`
- `tooling/crates_io_client/src/tests.rs`
- `tooling/xtask/Cargo.toml`
- `tooling/xtask/src/tasks/gpui.rs`
- `tooling/xtask/src/tasks/workflows/deploy_collab.rs`

## Network-related additions

- `.github/pull_request_template.md`
- `CONTRIBUTING.md`
- `Cargo.lock`
- `Cargo.toml`
- `assets/icons/delete.svg`
- `assets/icons/fast_forward_ultrafast.svg`
- `assets/icons/square_r.svg`
- `assets/keymaps/vim.json`
- `corgi.toml`
- `crates/acp_thread/src/acp_thread.rs`
- `crates/acp_thread/src/content.rs`
- `crates/acp_thread/src/elicitation.rs`
- `crates/acp_thread/src/submission.rs`
- `crates/agent/src/agent.rs`
- `crates/agent/src/tests/mod.rs`
- `crates/agent/src/thread.rs`
- `crates/agent_servers/src/acp.rs`
- `crates/agent_servers/src/acp/auth_status.rs`
- `crates/agent_servers/src/acp/runtime.rs`
- `crates/agent_servers/src/acp/v1.rs`
- `crates/agent_servers/src/acp/v2.rs`
- `crates/agent_ui/src/agent_connection_store.rs`
- `crates/agent_ui/src/agent_panel.rs`
- `crates/agent_ui/src/conversation_view.rs`
- `crates/agent_ui/src/conversation_view/thread_view.rs`
- `crates/agent_ui/src/mention_set.rs`
- `crates/agent_ui/src/message_editor.rs`
- `crates/anthropic/src/anthropic.rs`
- `crates/bedrock/src/models.rs`
- `crates/client/src/user.rs`
- `crates/cloud_api_client/Cargo.toml`
- `crates/cloud_api_client/src/websocket/native.rs`
- `crates/collab/Cargo.toml`
- `crates/collab/tests/integration/git_tests.rs`
- `crates/collab/tests/integration/integration_tests.rs`
- `crates/context_server/Cargo.toml`
- `crates/copilot_chat/Cargo.toml`
- `crates/copilot_chat/src/copilot_chat.rs`
- `crates/dap/src/adapters.rs`
- `crates/dev_container/src/lib.rs`
- `crates/dev_container/src/oci.rs`
- `crates/edit_prediction/src/edit_prediction_tests.rs`
- `crates/edit_prediction_ui/src/edit_prediction_button.rs`
- `crates/editor/src/editor_tests.rs`
- `crates/editor/src/element.rs`
- `crates/extension_suggest/src/extension_suggest.rs`
- `crates/extensions_ui/src/extensions_ui.rs`
- `crates/file_finder/src/file_finder_tests.rs`
- `crates/git/src/repository.rs`
- `crates/git_hosting_providers/src/providers/forgejo.rs`
- `crates/git_hosting_providers/src/providers/gerrit.rs`
- `crates/git_hosting_providers/src/providers/gitiles.rs`
- `crates/git_ui/src/git_panel.rs`
- `crates/git_ui/src/remote_output.rs`
- `crates/git_ui_core/src/notifications.rs`
- `crates/github_releases/Cargo.toml`
- `crates/github_releases/src/github_releases.rs`
- `crates/github_releases/src/releases.rs`
- `crates/google_ai/Cargo.toml`
- `crates/google_ai/src/google_ai.rs`
- `crates/gpui/examples/image/image.rs`
- `crates/gpui/examples/image_gallery.rs`
- `crates/gpui/examples/native_view.rs`
- `crates/gpui/examples/svg_animation.rs`
- `crates/gpui/src/app/visual_test_context.rs`
- `crates/gpui/src/elements/svg.rs`
- `crates/gpui/src/gpui.rs`
- `crates/gpui/src/hot_reload.rs`
- `crates/gpui/src/profiler/journal.rs`
- `crates/gpui/src/svg_renderer.rs`
- `crates/gpui_apple/src/display_link.rs`
- `crates/gpui_apple/src/metal_renderer.rs`
- `crates/gpui_apple/src/process_info.rs`
- `crates/gpui_apple/src/shaders.metal`
- `crates/gpui_ios/examples/ios/app/Info.plist`
- `crates/gpui_ios/src/ios/application.rs`
- `crates/gpui_ios/src/ios/platform.rs`
- `crates/gpui_linux/src/linux/platform.rs`
- `crates/gpui_linux/src/linux/power.rs`
- `crates/gpui_linux/src/linux/wayland/client.rs`
- `crates/gpui_linux/src/linux/wayland/window.rs`
- `crates/gpui_linux/src/linux/x11/client.rs`
- `crates/gpui_linux/src/linux/x11/clipboard.rs`
- `crates/gpui_macos/src/events.rs`
- `crates/gpui_macos/src/pasteboard.rs`
- `crates/gpui_macos/src/platform.rs`
- `crates/gpui_web/examples/hello_web/Cargo.toml`
- `crates/gpui_web/examples/hello_web/http_client_test.rs`
- `crates/gpui_web/examples/hello_web/index.html`
- `crates/gpui_web/examples/hello_web/test_http_client.mjs`
- `crates/gpui_wgpu/Cargo.toml`
- `crates/gpui_wgpu/src/shaders.wgsl`
- `crates/gpui_windows/src/shaders.hlsl`
- `crates/language_model_core/src/provider.rs`
- `crates/language_models/src/provider/anthropic.rs`
- `crates/language_models/src/provider/bedrock.rs`
- `crates/language_models/src/provider/deepseek.rs`
- `crates/language_models/src/provider/google.rs`
- `crates/language_models/src/provider/llama_cpp.rs`
- `crates/language_models/src/provider/lmstudio.rs`
- `crates/language_models/src/provider/mistral.rs`
- `crates/language_models/src/provider/ollama.rs`
- `crates/language_models/src/provider/open_ai.rs`
- `crates/language_models/src/provider/open_router.rs`
- `crates/language_models/src/provider/opencode.rs`
- `crates/language_models/src/provider/vercel_ai_gateway.rs`
- `crates/language_models/src/provider/x_ai.rs`
- `crates/language_models_cloud/src/language_models_cloud.rs`
- `crates/languages/src/c.rs`
- `crates/languages/src/eslint.rs`
- `crates/languages/src/python.rs`
- `crates/languages/src/rust.rs`
- `crates/livekit_api/Cargo.toml`
- `crates/markdown/src/markdown.rs`
- `crates/markdown/src/mermaid.rs`
- `crates/markdown/src/parser.rs`
- `crates/migrator/src/migrations/m_2026_09_29/settings.rs`
- `crates/migrator/src/migrator.rs`
- `crates/node_runtime/tests/system_node.rs`
- `crates/open_ai/src/chat_completion_transport_tests.rs`
- `crates/open_ai/src/responses.rs`
- `crates/openai_subscribed/src/openai_subscribed.rs`
- `crates/opencode/Cargo.toml`
- `crates/opencode/src/catalog.rs`
- `crates/opencode/src/completion.rs`
- `crates/path/src/windows_prefix.rs`
- `crates/project/Cargo.toml`
- `crates/project/src/agent_server_store.rs`
- `crates/project/src/git_store.rs`
- `crates/project/src/terminals.rs`
- `crates/project/tests/integration/image_store.rs`
- `crates/project/tests/integration/project_tests.rs`
- `crates/project_benchmarks/Cargo.toml`
- `crates/project_panel/src/project_panel_tests.rs`
- `crates/recent_projects/src/remote_connections.rs`
- `crates/remote/src/remote_client.rs`
- `crates/remote/src/transport.rs`
- `crates/remote/src/transport/mock.rs`
- `crates/remote/src/transport/ssh.rs`
- `crates/remote_server/src/remote_editing_tests.rs`
- `crates/remote_server/src/server.rs`
- `crates/sandbox/src/sandbox.rs`
- `crates/sandbox/src/windows_psec.rs`
- `crates/sandbox_linux/DESIGN.md`
- `crates/sandbox_linux/examples/end_to_end.rs`
- `crates/sandbox_linux/examples/run.rs`
- `crates/sandbox_linux/src/broker.rs`
- `crates/sandbox_linux/src/dns.rs`
- `crates/sandbox_linux/src/net.rs`
- `crates/sandbox_linux/src/policy.rs`
- `crates/settings_ui/src/settings_ui.rs`
- `crates/ui/src/components/button/button_link.rs`
- `crates/util/src/shell_env.rs`
- `crates/x_ai_subscribed/src/x_ai_subscribed.rs`
- `crates/zed/Cargo.toml`
- `crates/zed/src/zed.rs`
- `crates/zed/src/zed/windows_only_instance.rs`
- `docs/src/account/zed-hosted-models.md`
- `docs/src/ai/edit-prediction.md`
- `docs/src/ai/use-a-gateway.md`
- `docs/src/ai/use-an-existing-subscription.md`
- `docs/src/development.md`
- `docs/src/development/ui-checklist.md`
- `docs/src/extensions/languages.md`
- `docs/src/extensions/publishing/prerequisites.md`
- `docs/src/git.md`
- `docs/src/installation.md`
- `docs/src/languages.md`
- `docs/src/languages/emmet.md`
- `docs/src/languages/env.md`
- `docs/src/languages/gleam.md`
- `docs/src/languages/python.md`
- `docs/src/languages/rego.md`
- `docs/src/languages/roc.md`
- `docs/src/linux.md`
- `docs/src/reference/all-settings.md`
- `docs/src/repl.md`
- `docs/src/vim.md`
- `extensions/README.md`
- `legal/privacy-policy.md`
- `legal/terms.md`
- `legal/zed-product-terms.md`
- `script/bundle-linux`
- `script/bundle-mac`
- `script/github-pr-cleanup.py`
- `script/github-welcome-first-contribution.py`
- `script/linux`
- `script/snowflake_helpers.py`
- `script/terms/terms.rtf`
- `script/update_top_ranking_issues/uv.lock`
- `tooling/crates_io_client/Cargo.toml`
- `tooling/crates_io_client/src/crates_io_client.rs`
- `tooling/crates_io_client/src/real_crates_io_client.rs`
- `tooling/crates_io_client/src/simulated_crates_io_client.rs`
- `tooling/crates_io_client/src/tests.rs`
- `tooling/xtask/Cargo.toml`
- `tooling/xtask/src/tasks/gpui.rs`

## Resolution summary

- Restored the complete `.github/` tree from the fork baseline and omitted upstream additions that would activate upstream CI, release, collaboration or support automation.
- Rebased the fork's inert telemetry, forced `disable_ai` setting, empty language-model registration, settings/keymap filtering, startup omissions, AI-link rejection, menus and title bar onto the current upstream APIs. Shared types remain only where upstream editor and Git code require compatibility.
- Removed AI providers, Copilot/chat UI, agent startup/panels, edit-prediction startup, agent skill synchronization, crash startup, telemetry logging, channel/call initialization and AI account/layout actions from the production graph. Remote `agent_skills` remains test-only; the remote server no longer watches or emits global-skill changes.
- Preserved ordinary upstream improvements to editing, Git, debugger, notebooks, remote workspaces, file finding, tabular data, startup restore behavior, settings, path handling and window chrome.
- Reconstructed `assets/settings/default.json` as a three-way merge: required upstream defaults are present, while `edit_predictions.provider` remains `none`, its status-bar button is disabled, predictions and agent review are off, telemetry is off, automatic updates stay off for unsigned builds, and `disable_ai` remains non-overridable.
- Reconstructed the lockfile as a three-way merge and pinned `rust-embed` to `8.11.0`; the upstream `8.14` refresh was rejected because it is incompatible with the fork's embedded-assets implementation.
- Reviewed and refreshed only the protected-source fingerprints that changed, after confirming their no-AI or compatibility behavior.

## Completed local validation

- `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_DEV_OPT_LEVEL=0 cargo check --locked -p zed -p remote_server` — passed from a clean Cargo target.
- `script/audit-no-ai-source` — passed after reviewed fingerprint updates.
- Production `cargo tree` audits for `zed`, `remote_server`, `code_search` and `code_search_provider` — passed; no forbidden AI, Copilot, edit-prediction, telemetry, crash, collaboration, audio, LiveKit or WebRTC package appeared in the resolved production/build graph.
- `cargo fmt --all -- --check`, `git diff --check` and `git diff --cached --check` — passed.
- `GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=commit.gpgSign GIT_CONFIG_VALUE_0=false python3 -m unittest discover -s script/tests` — 39 tests passed.
- `cargo test --locked -p title_bar --lib` — 9 tests passed after adapting stale upstream toggle tests to the permanent no-AI setting.
- `cargo test --locked -p settings --lib no_ai_fork` — 4 tests passed, including bundled upstream keymap coverage.
- `cargo test --locked -p client --lib no_telemetry_fork` — 3 tests passed; no telemetry call reached the HTTP transport.

Apple Silicon/Intel packaging, actual-binary startup proxy capture, benchmark collection and full notebook runtime validation were not run in this Linux environment. They remain required in the repository's Mac validation workflow before merge.

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
