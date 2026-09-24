use gir::cmd::{doctor, fixup, hook, init, lint};
use gir::config::Config;
use gir::explain;
use lexopt::prelude::*;

const USAGE: &str = "\
gir: keeps git usage honest (Conventional Commits, fixups, repo hygiene)

usage:
  gir init [--force]              install hooks, .girconfig, cliff.toml; set core.hooksPath
  gir lint [<file>|-] [--fix] [--json]
  gir lint --range <A..B> [--json]
  gir fixup [<commit>] [--dry-run] create fixup! for the commit the staged lines belong to
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
            println!("gir {}", env!("CARGO_PKG_VERSION"));
            return Ok(0);
        }
        Some(Short('h') | Long("help")) | None => {
            println!("{USAGE}");
            return Ok(0);
        }
        Some(a) => return Err(err(a.unexpected())),
    };

    let mut positional = Vec::new();
    let mut flags = Vec::new();
    let mut range = None;
    while let Some(arg) = parser.next().map_err(err)? {
        match arg {
            Long("range") => range = Some(parser.value().map_err(err)?.string().map_err(err)?),
            Long(f) => flags.push(f.to_string()),
            Short('h') => flags.push("help".into()),
            Short(c) => return Err(format!("unknown option -{c}")),
            Value(v) => positional.push(v.string().map_err(err)?),
        }
    }
    let flag = |name: &str| flags.iter().any(|f| f == name);
    let allowed: &[&str] = match sub.as_str() {
        "init" => &["force"],
        "lint" => &["fix", "json"],
        "fixup" => &["dry-run"],
        "doctor" => &["fix"],
        _ => &[],
    };
    if flag("help") {
        println!("{USAGE}");
        return Ok(0);
    }
    if let Some(f) = flags.iter().find(|f| !allowed.contains(&f.as_str())) {
        return Err(format!("unknown option --{f} for `gir {sub}` (see: gir --help)"));
    }

    match (sub.as_str(), positional.as_slice()) {
        ("init", []) => init::init(flag("force")),
        ("doctor", []) => doctor::doctor(flag("fix")),
        ("fixup", []) => fixup::fixup(None, flag("dry-run")),
        ("fixup", [target]) => fixup::fixup(Some(target.clone()), flag("dry-run")),
        ("explain", []) => {
            println!("topics: {}", explain::topics().join(" "));
            Ok(0)
        }
        ("explain", [topic]) => {
            let cfg = Config::load().unwrap_or_default();
            let page = explain::page(topic, &cfg)
                .ok_or_else(|| format!("no topic `{topic}`; topics: {}", explain::topics().join(" ")))?;
            println!("{page}");
            Ok(0)
        }
        ("lint", rest) => {
            let cfg = Config::load()?;
            let opts = lint::Options { fix: flag("fix"), json: flag("json") };
            let source = match (range, rest) {
                (Some(r), []) => lint::Source::Range(r),
                (None, [] ) => lint::Source::Stdin,
                (None, [p]) if p == "-" => lint::Source::Stdin,
                (None, [p]) => lint::Source::File(p.clone()),
                _ => return Err("gir lint takes one file, `-`, or --range".into()),
            };
            lint::lint(source, opts, &cfg)
        }
        ("hook", [name, rest @ ..]) => {
            let cfg = Config::load()?;
            match (name.as_str(), rest) {
                ("commit-msg", [file]) => hook::commit_msg(file, &cfg),
                ("pre-push", [remote, ..]) => hook::pre_push(remote, &cfg),
                _ => Err(format!("unknown hook `{name}`")),
            }
        }
        _ => Err(format!("bad arguments for `{sub}`\n{USAGE}")),
    }
}

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
