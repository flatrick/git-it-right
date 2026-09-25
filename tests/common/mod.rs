#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub struct Repo {
    _tmp: tempfile::TempDir,
    pub dir: PathBuf,
    pub global: PathBuf,
}

impl Repo {
    pub fn new() -> Repo {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("repo");
        std::fs::create_dir(&dir).unwrap();
        let global = tmp.path().join("gitconfig");
        std::fs::write(&global, "[user]\n\tname = T\n\temail = t@example.com\n[init]\n\tdefaultBranch = main\n[commit]\n\tgpgsign = false\n").unwrap();
        let repo = Repo { _tmp: tmp, dir, global };
        repo.git(&["init", "-q"]);
        repo
    }

    pub fn cmd(&self, program: &str) -> Command {
        let bin_dir = Path::new(env!("CARGO_BIN_EXE_gir")).parent().unwrap().to_path_buf();
        let mut paths = vec![bin_dir];
        paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()));
        let mut c = Command::new(program);
        c.current_dir(&self.dir)
            .env("PATH", std::env::join_paths(paths).unwrap())
            .env("GIT_CONFIG_GLOBAL", &self.global)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE");
        c
    }

    pub fn git_out(&self, args: &[&str]) -> Output {
        self.cmd("git").args(args).output().unwrap()
    }

    pub fn git(&self, args: &[&str]) -> String {
        let out = self.git_out(args);
        assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stdout).trim_end().to_string()
    }

    pub fn git_with(&self, args: &[&str], envs: &[(&str, &str)]) -> String {
        let out = self.cmd("git").args(args).envs(envs.iter().copied()).output().unwrap();
        assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stdout).trim_end().to_string()
    }

    pub fn gir(&self, args: &[&str]) -> Output {
        self.cmd(env!("CARGO_BIN_EXE_gir")).args(args).output().unwrap()
    }

    pub fn write(&self, rel: &str, content: &str) {
        let p = self.dir.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, content).unwrap();
    }

    pub fn commit_file(&self, rel: &str, content: &str, msg: &str) {
        self.write(rel, content);
        self.git(&["add", rel]);
        self.git(&["commit", "-q", "--no-verify", "-m", msg]);
    }

    pub fn head_message(&self) -> String {
        self.git(&["log", "-1", "--format=%B"])
    }
}

pub fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).to_string()
}
