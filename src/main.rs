use gir::cmd::fixup::{self, Mode};
use gir::cmd::{doctor, hook, init, lint};
use gir::config::Config;
use gir::explain;
use lexopt::prelude::*;

const USAGE: &str = "\
gir: keeps git usage honest (Conventional Commits, fixups, repo hygiene)

usage:
  gir init [--force] [--optional] install hooks, .girconfig, cliff.toml, GIT-IT-RIGHT.md; set core.hooksPath
  gir lint [<file>|-] [--fix] [--json]
  gir lint --range <A..B> [--json]
  gir fixup [<commit>] [--dry-run] [--split]  fixup! for the commit the staged lines belong to
  gir amend [<commit>] [--dry-run] [--split]  amend!: like fixup, and replace its message
  gir squash [<commit>] [--dry-run] [--split] squash!: like fixup, and combine the messages
  gir reword [<commit>] [--dry-run]           amend! that replaces only the message
  gir doctor [--fix]              check .gitattributes, .gitignore, git config, index
  gir explain [<topic>]           rule details; `gir explain` lists topics
  gir hook commit-msg <file>      (called by .githooks)
  gir hook pre-push <remote> <url>";

fn main() {
    let code = match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("gir: {e}");
            2
        }
    };
    std::process::exit(code);
}

fn run() -> Result<i32, String> {
    let mut parser = lexopt::Parser::from_env();
    let sub = match parser.next().map_err(err)? {
        Some(Value(v)) => v.string().map_err(err)?,
        Some(Short('V') | Long("version")) => {
            gir::outln!("gir {}", env!("CARGO_PKG_VERSION"));
            return Ok(0);
        }
        Some(Short('h') | Long("help")) | None => {
            gir::outln!("{USAGE}");
            return Ok(0);
        }
        Some(a) => return Err(err(a.unexpected())),
    };

    let mut positional = Vec::new();
    let mut flags = Vec::new();
    let mut range = None;
    while let Some(arg) = parser.next().map_err(err)? {
        match arg {
            Long("range") if sub == "lint" => range =Some(parser.value().map_err(err)?.string().map_err(err)?),
            Long(f) => flags.push(f.to_string()),
            Short('h') => flags.push("help".into()),
            Short(c) => return Err(format!("unknown option -{c}")),
            Value(v) => positional.push(v.string().map_err(err)?),
        }
    }
    let flag = |name: &str| flags.iter().any(|f| f == name);
    let allowed: &[&str] = match sub.as_str() {
        "init" => &["force", "optional"],
        "lint" => &["fix", "json"],
        "fixup" | "amend" | "squash" => &["dry-run", "split"],
        "reword" => &["dry-run"],
        "doctor" => &["fix"],
        _ => &[],
    };
    if flag("help") {
        gir::outln!("{USAGE}");
        return Ok(0);
    }
    if let Some(f) = flags.iter().find(|f| !allowed.contains(&f.as_str())) {
        return Err(format!("unknown option --{f} for `gir {sub}` (see: gir --help)"));
    }

    match (sub.as_str(), positional.as_slice()) {
        ("init", []) => init::init(flag("force"), flag("optional")),
        ("doctor", []) => doctor::doctor(flag("fix")),
        ("fixup" | "amend" | "reword" | "squash", [] | [_]) => {
            let mode = match sub.as_str() {
                "amend" => Mode::Amend,
                "reword" => Mode::Reword,
                "squash" => Mode::Squash,
                _ => Mode::Fixup,
            };
            let opts = fixup::Options { dry_run: flag("dry-run"), split: flag("split") };
            fixup::run(mode, positional.first().cloned(), opts)
        }
        ("explain", []) => {
            gir::outln!("topics: {}", explain::topics().join(" "));
            Ok(0)
        }
        ("explain", [topic]) => {
            let cfg = load_config().unwrap_or_default();
            let page = explain::page(topic, &cfg)
                .ok_or_else(|| format!("no topic `{topic}`; topics: {}", explain::topics().join(" ")))?;
            gir::outln!("{page}");
            Ok(0)
        }
        ("lint", rest) => {
            let cfg = load_config()?;
            let opts = lint::Options { fix: flag("fix"), json: flag("json") };
            let source = match (range, rest) {
                (Some(_), []) if opts.fix => return Err("gir lint --range cannot --fix recorded commits".into()),
                (Some(r), []) => lint::Source::Range(r),
                (None, [] ) => lint::Source::Stdin,
                (None, [p]) if p == "-" => lint::Source::Stdin,
                (None, [p]) => lint::Source::File(p.clone()),
                _ => return Err("gir lint takes one file, `-`, or --range".into()),
            };
            lint::lint(source, opts, &cfg)
        }
        ("hook", [name, rest @ ..]) => {
            let cfg = load_config()?;
            match (name.as_str(), rest) {
                ("commit-msg", [file]) => hook::commit_msg(file, &cfg),
                ("pre-push", [remote, ..]) => hook::pre_push(remote, &cfg),
                _ => Err(format!("unknown hook `{name}`")),
            }
        }
        _ => Err(format!("bad arguments for `{sub}`\n{USAGE}")),
    }
}

fn load_config() -> Result<Config, String> {
    let cfg = Config::load()?;
    for w in &cfg.warnings {
        eprintln!("gir: warning: {w}");
    }
    Ok(cfg)
}

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
