use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub commands: Vec<String>,
    pub prompt: String,
    pub path: PathBuf,
}

pub struct SkillRegistry {
    skills: HashMap<String, Skill>,
}

impl SkillRegistry {
    pub fn empty() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }

    pub fn load_from_dir(skills_dir: &Path) -> Result<Self> {
        let mut skills = HashMap::new();

        if !skills_dir.exists() {
            return Ok(Self { skills });
        }

        let entries = std::fs::read_dir(skills_dir)
            .with_context(|| format!("failed to read skills dir: {}", skills_dir.display()))?;

        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let skill_md = entry.path().join("skill.md");
                if skill_md.exists() {
                    match Self::parse_skill(&skill_md) {
                        Ok(skill) => {
                            skills.insert(skill.name.clone(), skill);
                        }
                        Err(e) => {
                            eprintln!("[skills] failed to load {}: {}", skill_md.display(), e);
                        }
                    }
                }
            }
        }

        Ok(Self { skills })
    }

    pub fn get(&self, name: &str) -> Option<&Skill> {
        self.skills.get(name)
    }

    pub fn list(&self) -> Vec<&str> {
        self.skills.keys().map(|s| s.as_str()).collect()
    }

    fn parse_skill(path: &Path) -> Result<Skill> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let mut name = String::new();
        let mut description = String::new();
        let mut commands = Vec::new();
        let mut prompt = String::new();

        let mut in_commands = false;
        let mut in_prompt = false;

        for line in content.lines() {
            if line.starts_with("# ") {
                name = line.trim_start_matches("# ").trim().to_string();
                continue;
            }

            if line.starts_with("## Description") {
                in_commands = false;
                in_prompt = false;
                continue;
            }

            if line.starts_with("## Commands") {
                in_commands = true;
                in_prompt = false;
                continue;
            }

            if line.starts_with("## Prompt") {
                in_commands = false;
                in_prompt = true;
                continue;
            }

            if line.starts_with("## ") {
                in_commands = false;
                in_prompt = false;
                continue;
            }

            if in_commands {
                let trimmed = line.trim();
                if trimmed.starts_with("```") {
                    continue;
                }
                if !trimmed.is_empty() {
                    commands.push(trimmed.to_string());
                }
            } else if in_prompt {
                prompt.push_str(line);
                prompt.push('\n');
            } else if description.is_empty() && !line.trim().is_empty() && !name.is_empty() {
                description = line.trim().to_string();
            }
        }

        if name.is_empty() {
            let dir_name = path.parent().and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            name = dir_name;
        }

        Ok(Skill {
            name,
            description,
            commands,
            prompt: prompt.trim().to_string(),
            path: path.to_path_buf(),
        })
    }
}
