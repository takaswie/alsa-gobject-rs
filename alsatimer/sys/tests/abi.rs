// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![cfg(unix)]

use alsatimer_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["alsatimer"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {cmd:?} failed, {status}").into());
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
        Err(err) => Err(format!("{name} {err}").into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    cmd.stderr(Stdio::inherit());
    let out = cmd.output()?;
    if !out.status.success() {
        let (status, stdout) = (out.status, String::from_utf8_lossy(&out.stdout));
        return Err(format!("command {cmd:?} failed, {status:?}\nstdout: {stdout}").into());
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
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
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
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {rust_name}\nRust: {rust_value:?}\nC:    {c_value:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!("Layout mismatch for {rust_name}\nRust: {rust_layout:?}\nC:    {c_layout:?}",);
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut cmd = Command::new(exe);
    cmd.stderr(Stdio::inherit());
    let out = cmd.output()?;
    if !out.status.success() {
        let (status, stdout) = (out.status, String::from_utf8_lossy(&out.stdout));
        return Err(format!("command {cmd:?} failed, {status:?}\nstdout: {stdout}").into());
    }

    Ok(String::from_utf8(out.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "ALSATimerClass",
        Layout {
            size: size_of::<ALSATimerClass>(),
            alignment: align_of::<ALSATimerClass>(),
        },
    ),
    (
        "ALSATimerDeviceInfo",
        Layout {
            size: size_of::<ALSATimerDeviceInfo>(),
            alignment: align_of::<ALSATimerDeviceInfo>(),
        },
    ),
    (
        "ALSATimerDeviceInfoClass",
        Layout {
            size: size_of::<ALSATimerDeviceInfoClass>(),
            alignment: align_of::<ALSATimerDeviceInfoClass>(),
        },
    ),
    (
        "ALSATimerDeviceInfoFlag",
        Layout {
            size: size_of::<ALSATimerDeviceInfoFlag>(),
            alignment: align_of::<ALSATimerDeviceInfoFlag>(),
        },
    ),
    (
        "ALSATimerDeviceParams",
        Layout {
            size: size_of::<ALSATimerDeviceParams>(),
            alignment: align_of::<ALSATimerDeviceParams>(),
        },
    ),
    (
        "ALSATimerDeviceParamsClass",
        Layout {
            size: size_of::<ALSATimerDeviceParamsClass>(),
            alignment: align_of::<ALSATimerDeviceParamsClass>(),
        },
    ),
    (
        "ALSATimerDeviceStatus",
        Layout {
            size: size_of::<ALSATimerDeviceStatus>(),
            alignment: align_of::<ALSATimerDeviceStatus>(),
        },
    ),
    (
        "ALSATimerDeviceStatusClass",
        Layout {
            size: size_of::<ALSATimerDeviceStatusClass>(),
            alignment: align_of::<ALSATimerDeviceStatusClass>(),
        },
    ),
    (
        "ALSATimerEventType",
        Layout {
            size: size_of::<ALSATimerEventType>(),
            alignment: align_of::<ALSATimerEventType>(),
        },
    ),
    (
        "ALSATimerInstanceInfo",
        Layout {
            size: size_of::<ALSATimerInstanceInfo>(),
            alignment: align_of::<ALSATimerInstanceInfo>(),
        },
    ),
    (
        "ALSATimerInstanceInfoClass",
        Layout {
            size: size_of::<ALSATimerInstanceInfoClass>(),
            alignment: align_of::<ALSATimerInstanceInfoClass>(),
        },
    ),
    (
        "ALSATimerInstanceParamFlag",
        Layout {
            size: size_of::<ALSATimerInstanceParamFlag>(),
            alignment: align_of::<ALSATimerInstanceParamFlag>(),
        },
    ),
    (
        "ALSATimerInstanceParams",
        Layout {
            size: size_of::<ALSATimerInstanceParams>(),
            alignment: align_of::<ALSATimerInstanceParams>(),
        },
    ),
    (
        "ALSATimerInstanceParamsClass",
        Layout {
            size: size_of::<ALSATimerInstanceParamsClass>(),
            alignment: align_of::<ALSATimerInstanceParamsClass>(),
        },
    ),
    (
        "ALSATimerInstanceStatus",
        Layout {
            size: size_of::<ALSATimerInstanceStatus>(),
            alignment: align_of::<ALSATimerInstanceStatus>(),
        },
    ),
    (
        "ALSATimerInstanceStatusClass",
        Layout {
            size: size_of::<ALSATimerInstanceStatusClass>(),
            alignment: align_of::<ALSATimerInstanceStatusClass>(),
        },
    ),
    (
        "ALSATimerRealTimeEventType",
        Layout {
            size: size_of::<ALSATimerRealTimeEventType>(),
            alignment: align_of::<ALSATimerRealTimeEventType>(),
        },
    ),
    (
        "ALSATimerSlaveClass",
        Layout {
            size: size_of::<ALSATimerSlaveClass>(),
            alignment: align_of::<ALSATimerSlaveClass>(),
        },
    ),
    (
        "ALSATimerSpecificGlobalDevice",
        Layout {
            size: size_of::<ALSATimerSpecificGlobalDevice>(),
            alignment: align_of::<ALSATimerSpecificGlobalDevice>(),
        },
    ),
    (
        "ALSATimerUserInstance",
        Layout {
            size: size_of::<ALSATimerUserInstance>(),
            alignment: align_of::<ALSATimerUserInstance>(),
        },
    ),
    (
        "ALSATimerUserInstanceClass",
        Layout {
            size: size_of::<ALSATimerUserInstanceClass>(),
            alignment: align_of::<ALSATimerUserInstanceClass>(),
        },
    ),
    (
        "ALSATimerUserInstanceError",
        Layout {
            size: size_of::<ALSATimerUserInstanceError>(),
            alignment: align_of::<ALSATimerUserInstanceError>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) ALSATIMER_CLASS_CARD", "2"),
    ("(gint) ALSATIMER_CLASS_GLOBAL", "1"),
    ("(gint) ALSATIMER_CLASS_NONE", "-1"),
    ("(gint) ALSATIMER_CLASS_PCM", "3"),
    ("(guint) ALSATIMER_DEVICE_INFO_FLAG_SLAVE", "1"),
    ("(gint) ALSATIMER_EVENT_TYPE_REAL_TIME", "1"),
    ("(gint) ALSATIMER_EVENT_TYPE_TICK_TIME", "0"),
    ("(guint) ALSATIMER_INSTANCE_PARAM_FLAG_AUTO", "1"),
    ("(guint) ALSATIMER_INSTANCE_PARAM_FLAG_EARLY_EVENT", "4"),
    ("(guint) ALSATIMER_INSTANCE_PARAM_FLAG_EXCLUSIVE", "2"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_CONTINUE", "4"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_EARLY", "6"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_MCONTINUE", "14"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_MPAUSE", "15"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_MRESUME", "18"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_MSTART", "12"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_MSTOP", "13"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_MSUSPEND", "17"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_PAUSE", "5"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_RESOLUTION", "0"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_RESUME", "8"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_START", "2"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_STOP", "3"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_SUSPEND", "7"),
    ("(gint) ALSATIMER_REAL_TIME_EVENT_TYPE_TICK", "1"),
    ("(gint) ALSATIMER_SLAVE_CLASS_APPLICATION", "1"),
    ("(gint) ALSATIMER_SLAVE_CLASS_NONE", "0"),
    ("(gint) ALSATIMER_SLAVE_CLASS_SEQUENCER", "2"),
    ("(gint) ALSATIMER_SPECIFIC_GLOBAL_DEVICE_HRTIMER", "3"),
    ("(gint) ALSATIMER_SPECIFIC_GLOBAL_DEVICE_SYSTEM", "0"),
    ("(gint) ALSATIMER_USER_INSTANCE_ERROR_ATTACHED", "3"),
    ("(gint) ALSATIMER_USER_INSTANCE_ERROR_FAILED", "0"),
    ("(gint) ALSATIMER_USER_INSTANCE_ERROR_NOT_ATTACHED", "2"),
    ("(gint) ALSATIMER_USER_INSTANCE_ERROR_TIMER_NOT_FOUND", "1"),
];
