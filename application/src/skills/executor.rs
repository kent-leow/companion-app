use anyhow::{Context, Result};
use std::process::Command;

use super::Skill;

pub struct SkillExecutor;

impl SkillExecutor {
    pub fn execute(skill: &Skill, args: &str) -> Result<String> {
        let mut output_parts = Vec::new();

        for cmd_template in &skill.commands {
            let cmd = cmd_template.replace("{query}", args).replace("{input}", args);

            let result = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
                .with_context(|| format!("failed to execute: {}", cmd))?;

            let stdout = String::from_utf8_lossy(&result.stdout).to_string();
            let stderr = String::from_utf8_lossy(&result.stderr).to_string();

            if !stdout.is_empty() {
                output_parts.push(stdout);
            }
            if !result.status.success() && !stderr.is_empty() {
                output_parts.push(format!("[stderr]: {}", stderr));
            }
        }

        Ok(output_parts.join("\n"))
    }
}
