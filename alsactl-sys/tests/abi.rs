// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

extern crate alsactl_sys;
extern crate shell_words;
extern crate tempfile;
use alsactl_sys::*;
use std::env;
use std::error::Error;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["alsactl"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed, self.failed, self.failed_to_compile
        )
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = Builder::new()
        .prefix("abi")
        .tempdir()
        .expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(
        "1",
        get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
        "failed to obtain correct constant value for 1"
    );

    let mut results: Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            }
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!(
                        "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                        name, rust_value, c_value
                    );
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = Builder::new()
        .prefix("abi")
        .tempdir()
        .expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(
        Layout {
            size: 1,
            alignment: 1
        },
        get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
        "failed to obtain correct layout for char type"
    );

    let mut results: Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            }
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!(
                        "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                        name, rust_layout, &c_layout
                    );
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<dyn Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout { size, alignment })
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<dyn Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    let output = str::from_utf8(&output.stdout)?.trim();
    if !output.starts_with("###gir test###") || !output.ends_with("###gir test###") {
        return Err(format!(
            "command {:?} return invalid output, {:?}",
            &abi_cmd, &output
        )
        .into());
    }

    Ok(String::from(&output[14..(output.len() - 14)]))
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "ALSACtlCard",
        Layout {
            size: size_of::<ALSACtlCard>(),
            alignment: align_of::<ALSACtlCard>(),
        },
    ),
    (
        "ALSACtlCardClass",
        Layout {
            size: size_of::<ALSACtlCardClass>(),
            alignment: align_of::<ALSACtlCardClass>(),
        },
    ),
    (
        "ALSACtlCardError",
        Layout {
            size: size_of::<ALSACtlCardError>(),
            alignment: align_of::<ALSACtlCardError>(),
        },
    ),
    (
        "ALSACtlCardInfo",
        Layout {
            size: size_of::<ALSACtlCardInfo>(),
            alignment: align_of::<ALSACtlCardInfo>(),
        },
    ),
    (
        "ALSACtlCardInfoClass",
        Layout {
            size: size_of::<ALSACtlCardInfoClass>(),
            alignment: align_of::<ALSACtlCardInfoClass>(),
        },
    ),
    (
        "ALSACtlElemAccessFlag",
        Layout {
            size: size_of::<ALSACtlElemAccessFlag>(),
            alignment: align_of::<ALSACtlElemAccessFlag>(),
        },
    ),
    (
        "ALSACtlElemEventMask",
        Layout {
            size: size_of::<ALSACtlElemEventMask>(),
            alignment: align_of::<ALSACtlElemEventMask>(),
        },
    ),
    (
        "ALSACtlElemIfaceType",
        Layout {
            size: size_of::<ALSACtlElemIfaceType>(),
            alignment: align_of::<ALSACtlElemIfaceType>(),
        },
    ),
    (
        "ALSACtlElemInfo",
        Layout {
            size: size_of::<ALSACtlElemInfo>(),
            alignment: align_of::<ALSACtlElemInfo>(),
        },
    ),
    (
        "ALSACtlElemInfoClass",
        Layout {
            size: size_of::<ALSACtlElemInfoClass>(),
            alignment: align_of::<ALSACtlElemInfoClass>(),
        },
    ),
    (
        "ALSACtlElemType",
        Layout {
            size: size_of::<ALSACtlElemType>(),
            alignment: align_of::<ALSACtlElemType>(),
        },
    ),
    (
        "ALSACtlElemValue",
        Layout {
            size: size_of::<ALSACtlElemValue>(),
            alignment: align_of::<ALSACtlElemValue>(),
        },
    ),
    (
        "ALSACtlElemValueClass",
        Layout {
            size: size_of::<ALSACtlElemValueClass>(),
            alignment: align_of::<ALSACtlElemValueClass>(),
        },
    ),
    (
        "ALSACtlEventType",
        Layout {
            size: size_of::<ALSACtlEventType>(),
            alignment: align_of::<ALSACtlEventType>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) ALSACTL_CARD_ERROR_DISCONNECTED", "1"),
    ("(gint) ALSACTL_CARD_ERROR_ELEM_EXIST", "5"),
    ("(gint) ALSACTL_CARD_ERROR_ELEM_NOT_FOUND", "2"),
    ("(gint) ALSACTL_CARD_ERROR_ELEM_NOT_SUPPORTED", "3"),
    ("(gint) ALSACTL_CARD_ERROR_ELEM_OWNED", "4"),
    ("(gint) ALSACTL_CARD_ERROR_FAILED", "0"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_INACTIVE", "256"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_LOCK", "512"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_OWNER", "1024"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_READ", "1"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_CALLBACK", "268435456"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_COMMAND", "64"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_READ", "16"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_WRITE", "32"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_USER", "536870912"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_VOLATILE", "4"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_WRITE", "2"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_ADD", "4"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_INFO", "2"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_REMOVE", "16"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_TLV", "8"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_VALUE", "1"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_CARD", "0"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_HWDEP", "1"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_MIXER", "2"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_PCM", "3"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_RAWMIDI", "4"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_SEQUENCER", "6"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_TIMER", "5"),
    ("(gint) ALSACTL_ELEM_TYPE_BOOLEAN", "1"),
    ("(gint) ALSACTL_ELEM_TYPE_BYTES", "4"),
    ("(gint) ALSACTL_ELEM_TYPE_ENUMERATED", "3"),
    ("(gint) ALSACTL_ELEM_TYPE_IEC60958", "5"),
    ("(gint) ALSACTL_ELEM_TYPE_INTEGER", "2"),
    ("(gint) ALSACTL_ELEM_TYPE_INTEGER64", "6"),
    ("(gint) ALSACTL_ELEM_TYPE_NONE", "0"),
    ("(gint) ALSACTL_EVENT_TYPE_ELEM", "0"),
];
