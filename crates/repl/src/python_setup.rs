use anyhow::{Context as _, Result, ensure};
use gpui::{App, AppContext as _, PromptLevel, Window, actions};
use std::path::PathBuf;
use workspace::{Workspace, notifications::NotificationId};

use crate::{PythonEnvKernelSpecification, ReplStore};

actions!(repl, [SetUpPython]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &SetUpPython, window, cx| {
            let project = workspace.project().clone();
            if !project.read(cx).is_local() {
                show_message(window, cx, PromptLevel::Info, "Set up Python on the remote host",
                    Some("Create a Python environment and install ipykernel on the remote host, then select it from the kernel menu."));
                return;
            }
            let location = project.read(cx).visible_worktrees(cx).next().map(|worktree| {
                let tree = worktree.read(cx);
                let root = tree.abs_path().to_path_buf();
                let directory = if tree.root_entry().is_some_and(|entry| entry.is_dir()) {
                    root
                } else {
                    root.parent().unwrap_or(&root).to_path_buf()
                };
                (tree.id(), directory)
            });
            let Some((worktree_id, directory)) = location else {
                show_message(window, cx, PromptLevel::Info, "Open a Python project folder first", None);
                return;
            };
            let venv = directory.join(".venv");
            let details = format!("Create or use {} and install ipykernel in it. Existing environments are preserved.", venv.display());
            let approval = window.prompt(PromptLevel::Info, "Set up Python for this folder", Some(&details), &["Set Up Python", "Cancel"], cx);
            let fs = project.read(cx).fs().clone();
            cx.spawn_in(window, async move |workspace, cx| {
                if approval.await.ok() != Some(0) { return; }
                workspace.update(cx, |workspace, cx| {
                    workspace.show_toast(workspace::Toast::new(NotificationId::Named("python-setup".into()), "Setting up the Python environment…"), cx);
                }).ok();
                let setup = cx.background_spawn(async move {
                    if fs.metadata(&venv).await?.is_none() {
                        let output = util::command::new_command("python3").args(["-m", "venv"]).arg(&venv).output().await.context("Python 3 is required. Install Python 3 and try again.")?;
                        ensure!(output.status.success(), "Could not create the Python environment: {}", String::from_utf8_lossy(&output.stderr));
                    }
                    let python = python_in_venv(&venv);
                    install_ipykernel(&python, false).await?;
                    Ok::<_, anyhow::Error>((venv, python))
                }).await;
                workspace.update_in(cx, |workspace, window, cx| {
                    match setup {
                        Ok((venv, python)) => {
                            let spec = PythonEnvKernelSpecification {
                                name: "Python (.venv)".into(),
                                path: python.clone(),
                                has_ipykernel: true,
                                environment_kind: Some("venv".into()),
                                kernelspec: jupyter_protocol::JupyterKernelspec {
                                    argv: vec![python.to_string_lossy().into_owned(), "-m".into(), "ipykernel_launcher".into(), "-f".into(), "{connection_file}".into()],
                                    display_name: "Python (.venv)".into(), language: "python".into(),
                                    interrupt_mode: None, metadata: None,
                                    env: Some(std::collections::HashMap::from([("VIRTUAL_ENV".into(), venv.to_string_lossy().into_owned())])),
                                },
                            };
                            ReplStore::global(cx).update(cx, |store, cx| store.register_python_kernel(worktree_id, spec, cx));
                            workspace.show_toast(workspace::Toast::new(NotificationId::Named("python-setup".into()), "Python is ready. Choose Python (.venv) from the kernel menu."), cx);
                        }
                        Err(error) => { show_message(window, cx, PromptLevel::Warning, "Python setup failed", Some(&error.to_string())); }
                    }
                }).ok();
            }).detach();
        });
    }).detach();
}

fn python_in_venv(venv: &std::path::Path) -> PathBuf {
    if cfg!(target_os = "windows") {
        venv.join("Scripts/python.exe")
    } else {
        venv.join("bin/python")
    }
}

pub async fn install_ipykernel(python: &std::path::Path, use_uv: bool) -> Result<()> {
    let output = if use_uv {
        util::command::new_command("uv")
            .args(["pip", "install", "--python"])
            .arg(python)
            .arg("ipykernel")
            .output()
            .await
    } else {
        util::command::new_command(python)
            .args(["-m", "pip", "install", "ipykernel"])
            .output()
            .await
    }
    .context("Could not start the package installer for this Python environment")?;
    ensure!(
        output.status.success(),
        "ipykernel installation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let check = util::command::new_command(python)
        .args(["-c", "import ipykernel"])
        .output()
        .await?;
    ensure!(
        check.status.success(),
        "ipykernel could not be imported after installation"
    );
    Ok(())
}

pub(crate) fn show_message(
    window: &mut Window,
    cx: &mut App,
    level: PromptLevel,
    message: &str,
    details: Option<&str>,
) {
    let response = window.prompt(level, message, details, &["OK"], cx);
    cx.spawn(async move |_| {
        let _ = response.await;
    })
    .detach();
}
