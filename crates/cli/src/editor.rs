use std::{collections::HashMap, process::Command, sync::LazyLock};

use tempfile::{NamedTempFile};

static EDITOR_ARGS: LazyLock<HashMap<&str, Vec<&str>>> = LazyLock::new(|| {
    return HashMap::from([
        ("code", vec!["--wait"]),
        ("subl", vec!["--wait"])
    ]);
});

fn is_command_available(command: &str) -> bool {
    return Command::new("which").arg(command).output().is_ok();
}

fn get_external_editor() -> String {
    let visual = std::env::var("VISUAL").unwrap_or("".to_string());

    if visual != "" {
        return visual;
    }

    let editor = std::env::var("EDITOR").unwrap_or("".to_string());

    if editor != "" {
        return editor;
    }

    if cfg!(target_os = "windows") {
        return "start /wait notepad".to_string();
    }

    let editors: [&str; 5] = ["code", "subl", "cursor", "vi", "nano"];

    for editor in editors {
        if is_command_available(editor) {
            return editor.to_string();
        }
    }

    return "".to_string();
}

pub fn open_editor_and_get_string() -> String {
    let editor = get_external_editor();

    if editor == "" {
        return String::new();
    }

    let default_args: Vec<&str> = vec![];
    let editor_args = EDITOR_ARGS
        .get(editor.as_str())
        .clone()
        .unwrap_or(&default_args);

    let temp_file = NamedTempFile::new().unwrap();

    Command::new(editor)
        .args(editor_args)
        .arg(temp_file.path())
        .status()
        .unwrap();

    return std::fs::read_to_string(temp_file.path()).unwrap_or_default().to_string();
}
