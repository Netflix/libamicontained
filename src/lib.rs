use std::ffi::c_int;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path::PathBuf;

const ENOENT: i32 = 2;
const EINVAL: i32 = 22;

type Result<T> = std::result::Result<T, Errno>;

#[derive(Debug, Clone)]
struct Errno {
    pub errno: i32,
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "errno {}", self.errno)
    }
}

impl From<io::Error> for Errno {
    fn from(ioe: io::Error) -> Self {
        Errno {
            errno: ioe.raw_os_error().unwrap_or(EINVAL),
        }
    }
}

impl From<ParseIntError> for Errno {
    fn from(_: ParseIntError) -> Self {
        Errno { errno: EINVAL }
    }
}

fn my_cgroup() -> Result<PathBuf> {
    let cgroups = fs::read_to_string("/proc/self/cgroup")?;

    // we only support cgroupv2 for now
    if !cgroups.starts_with("0::") {
        return Err(Errno { errno: EINVAL });
    }

    return Ok(PathBuf::from(cgroups[4..].trim()));
}

fn parse_effective_cpus(raw_cpus: String) -> Result<c_int> {
    let intervals = raw_cpus.trim().split(",");
    let counts = intervals
        .map(|i| -> Result<c_int> {
            let membs: Vec<_> = i.split("-").collect();
            if let [start, end] = &membs[..] {
                Ok(end.parse::<c_int>()? - start.parse::<c_int>()? + 1)
            } else if let [single] = &membs[..] {
                Ok(single.parse::<c_int>()?)
            } else {
                Err(Errno { errno: EINVAL })
            }
        })
        .collect::<Result<Vec<c_int>>>()?;
    Ok(counts.iter().sum())
}

fn effective_cpus_count(base_cg: PathBuf) -> Result<c_int> {
    let mut cur = Some(base_cg.as_path());

    while let Some(cg) = cur {
        let path: PathBuf = [
            "/sys/fs/cgroup",
            &cg.to_string_lossy(),
            "cpuset.cpus.effective",
        ]
        .iter()
        .collect();

        if let Ok(v) = fs::read_to_string(&path) {
            return parse_effective_cpus(v);
        }

        cur = cg.parent()
    }

    Err(Errno { errno: ENOENT })
}

fn r_num_cpus() -> Result<c_int> {
    let cg = my_cgroup()?;
    // TODO: we should do more than just query effective CPUs.
    effective_cpus_count(cg)
}

fn flatten_result(r: Result<c_int>) -> c_int {
    return match r {
        Ok(i) => i,
        Err(e) => -e.errno,
    };
}

#[no_mangle]
pub extern "C" fn num_cpus() -> c_int {
    flatten_result(r_num_cpus())
}

#[no_mangle]
pub extern "C" fn recommended_threads() -> c_int {
    // TODO: we should do something fancier here
    num_cpus()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpus_threads_equal_default() {
        assert_eq!(num_cpus(), recommended_threads());
    }
}
