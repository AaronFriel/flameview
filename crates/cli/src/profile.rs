use anyhow::{anyhow, Context};
use clap::Args;
use inferno::collapse::perf::{Folder, Options as PerfOptions};
use inferno::collapse::Collapse;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::Builder;

use crate::build::{find_crate_root, CommandExecutor};
use crate::opts::Opt;

#[derive(Args, Debug, Clone)]
pub struct ProfileOptions {
    #[clap(long, default_value_t = 99)]
    pub frequency: u32,
    #[clap(long)]
    pub output: Option<PathBuf>,
    #[clap(long)]
    pub keep_perf_data: bool,
    #[clap(long, value_name = "CMD")]
    pub cmd: Option<String>,
}

pub struct ProfileResult {
    pub folded_path: PathBuf,
}

fn profile_name(opt: &Opt) -> &str {
    if let Some(p) = &opt.cargo_profile {
        p
    } else if opt.release {
        "release"
    } else {
        "debug"
    }
}

pub fn profile(
    exec: &dyn CommandExecutor,
    workload: &[String],
    opt: &Opt,
) -> anyhow::Result<ProfileResult> {
    if workload.is_empty() {
        return Err(anyhow!("empty workload"));
    }

    let temp = Builder::new().prefix("perf.data").tempfile()?;
    let perf_path = temp.into_temp_path();

    let crate_root = find_crate_root(opt.manifest_path.as_deref())?;
    let profile_name = profile_name(opt);
    let target_name = Path::new(&workload[0])
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("invalid workload path"))?;
    let folded_path = if let Some(out) = &opt.profile.output {
        out.clone()
    } else {
        crate_root
            .join("target")
            .join("flameview")
            .join(profile_name)
            .join(format!("{}.folded", target_name))
    };

    if let Some(parent) = folded_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let perf_cmd = opt.profile.cmd.clone().unwrap_or_else(|| {
        format!(
            "record -F {} --call-graph dwarf,64000 -g",
            opt.profile.frequency
        )
    });
    let mut cmd = Command::new("perf");
    for token in perf_cmd.split_whitespace() {
        cmd.arg(token);
    }
    cmd.arg("-o").arg(perf_path.as_os_str());
    cmd.arg("--");
    for w in workload {
        cmd.arg(w);
    }
    cmd.stderr(Stdio::inherit());
    let output = exec.run(&mut cmd).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            anyhow!("failed to execute `perf`: not found in PATH")
        } else {
            anyhow!("failed to execute `perf`: {e}")
        }
    })?;
    if !output.status.success() {
        return Err(anyhow!("perf record failed"));
    }

    let mut script = Command::new("perf");
    script.args(["script", "--force", "-i"]);
    script.arg(perf_path.as_os_str());
    script.stderr(Stdio::inherit());
    let output = exec.run(&mut script).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            anyhow!("failed to execute `perf script`: not found in PATH")
        } else {
            anyhow!("failed to execute `perf script`: {e}")
        }
    })?;
    if !output.status.success() {
        return Err(anyhow!("perf script failed"));
    }

    let collapse_opts = PerfOptions::default();
    let mut folder = Folder::from(collapse_opts);
    let folded_file = File::create(&folded_path).context("create folded file")?;
    folder.collapse(output.stdout.as_slice(), BufWriter::new(folded_file))?;

    if opt.profile.keep_perf_data {
        perf_path.keep()?;
    }

    Ok(ProfileResult { folded_path })
}
