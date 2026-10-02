"""Probe: release-binary size and dependency count that `serde_json` adds.

Builds two throwaway crates with gir's release profile in a temporary directory:
one reads a file and prints its length, the other also parses it with
`serde_json::from_str::<serde_json::Value>`. Usage: probe_json_size.py <work dir>
"""
import subprocess
import sys
from pathlib import Path

PROFILE = """
[profile.release]
lto = true
codegen-units = 1
strip = true
"""

MAIN_PLAIN = """
fn main() {
    let s = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    println!("{}", s.len());
}
"""

MAIN_JSON = """
fn main() {
    let s = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    match serde_json::from_str::<serde_json::Value>(&s) {
        Ok(v) => println!("{} {}", s.len(), v.as_array().map_or(0, Vec::len)),
        Err(e) => println!("{e} line {} column {}", e.line(), e.column()),
    }
}
"""


def build(root: Path, name: str, main: str, deps: str) -> int:
    crate = root / name
    (crate / "src").mkdir(parents=True, exist_ok=True)
    (crate / "Cargo.toml").write_text(
        f'[package]\nname = "{name}"\nversion = "0.0.0"\nedition = "2024"\n\n[dependencies]\n{deps}\n{PROFILE}'
    )
    (crate / "src" / "main.rs").write_text(main)
    r = subprocess.run(["cargo", "build", "--release", "-q"], cwd=crate, capture_output=True, text=True)
    print(f"## {name}: cargo build exit={r.returncode}")
    if r.stderr:
        print(r.stderr)
    tree = subprocess.run(["cargo", "tree", "-e", "normal", "--prefix", "none"], cwd=crate, capture_output=True, text=True)
    print(tree.stdout)
    exe = crate / "target" / "release" / (name + (".exe" if sys.platform == "win32" else ""))
    size = exe.stat().st_size
    print(f"{name} size: {size} bytes")
    return size


def main() -> int:
    root = Path(sys.argv[1])
    root.mkdir(parents=True, exist_ok=True)
    plain = build(root, "plain", MAIN_PLAIN, "")
    json = build(root, "withjson", MAIN_JSON, 'serde_json = "1"')
    print(f"delta: {json - plain} bytes")
    bad = root / "bad.json"
    bad.write_text('[\n  {"Type": "feat",}\n]\n')
    r = subprocess.run([str(root / "withjson" / "target" / "release" / "withjson"), str(bad)], capture_output=True, text=True)
    print(f"trailing comma error: {r.stdout.strip()}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
