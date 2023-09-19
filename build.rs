use std::{
  borrow::Borrow,
  env,
  ffi::OsStr,
  path::{Path, PathBuf},
  process::Command,
};

fn main() {
  if !Path::new("r8brain/LICENSE").exists() {
    eprintln!("Setting up submodules");
    run_command_or_fail("./", "git", &["submodule", "update", "--init"]);
  }

  let mut dst = cmake::Config::new(".").build_target("r8brain").build();
  println!("cargo:rustc-link-search=native={}", dst.display());
  dst.push("build");
  println!("cargo:rustc-link-search=native={}", dst.display());
  dst.push(env::var("PROFILE").expect("Profile must be specified"));
  println!("cargo:rustc-link-search=native={}", dst.display());
  println!("cargo:rustc-link-lib=static=r8brain");

  let target = env::var("TARGET").unwrap();
  if target.contains("apple") {
    println!("cargo:rustc-link-lib=dylib=c++");
  } else if target.contains("linux") {
    println!("cargo:rustc-link-lib=dylib=stdc++");
  }
}

// thank you rdkafka-sys for this code
fn run_command_or_fail<P, S>(dir: &str, cmd: P, args: &[S])
  where P: AsRef<Path>,
        S: Borrow<str> + AsRef<OsStr>
{
  let cmd = cmd.as_ref();
  let cmd = if cmd.components().count() > 1 && cmd.is_relative() {
    // If `cmd` is a relative path (and not a bare command that should be
    // looked up in PATH), absolutize it relative to `dir`, as otherwise the
    // behavior of std::process::Command is undefined.
    // https://github.com/rust-lang/rust/issues/37868
    PathBuf::from(dir).join(cmd).canonicalize().expect("canonicalization failed")
  } else {
    PathBuf::from(cmd)
  };
  eprintln!("Running command: \"{} {}\" in dir: {}", cmd.display(), args.join(" "), dir);
  let ret = Command::new(cmd).current_dir(dir).args(args).status();
  match ret.map(|status| (status.success(), status.code())) {
    | Ok((true, _)) => (),
    | Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
    | Ok((false, None)) => panic!("Command got killed"),
    | Err(e) => panic!("Command failed with error: {}", e),
  }
}
