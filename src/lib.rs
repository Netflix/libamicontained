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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

// Starting from the base_cg, traverses the cgroup heirarchy and returns the first set/non-empty
// contents of the given cgroup controller.
// Returns an error if there is no such file.
fn read_cg_controller(base_cg: PathBuf, controller_name: &str) -> Result<String> {
    let mut cur = Some(base_cg.as_path());

    while let Some(cg) = cur {
        let path: PathBuf = ["/sys/fs/cgroup", &cg.to_string_lossy(), controller_name]
            .iter()
            .collect();

        match fs::read_to_string(&path) {
            Ok(v) if !v.trim().is_empty() => return Ok(v),
            _ => cur = cg.parent(),
        }
    }

    Err(Errno { errno: ENOENT })
}

fn parse_effective_cpus(raw_cpus: String) -> Result<c_int> {
    let intervals = raw_cpus.trim().split(',');
    let counts = intervals
        .map(|i| -> Result<c_int> {
            let membs: Vec<_> = i.split('-').collect();
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
    let raw_cpus = read_cg_controller(base_cg, "cpuset.cpus.effective")?;
    parse_effective_cpus(raw_cpus)
}

fn parse_cfs_quota_as_cpus(cpus_max: String) -> Result<c_int> {
    let parts: Vec<&str> = cpus_max.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(Errno { errno: EINVAL });
    }
    let quota: c_int = parts[0].parse().map_err(|_| Errno { errno: EINVAL })?;
    let period: c_int = parts[1].parse().map_err(|_| Errno { errno: EINVAL })?;

    if period == 0 {
        return Err(Errno { errno: EINVAL }); // Avoid division by zero
    }

    // The way we convert quota to cpu count is by modelling full time on one cfs_period as one cpu.
    // Thus 100% is 1 cpu, 200% is 2 and so on. We return 1 cpu if the quota is <100% as well.
    // This is how systemd models CPUQuota as well as lxcfs.
    let cpu_count = (quota / period) as c_int;
    Ok(c_int::max(cpu_count, 1))
}

fn cpu_count_from_quota(base_cg: PathBuf) -> Result<c_int> {
    let cpus_max = read_cg_controller(base_cg, "cpu.max")?;
    parse_cfs_quota_as_cpus(cpus_max)
}

fn r_num_cpus() -> Result<c_int> {
    let cg = my_cgroup()?;
    // TODO: we should do more than just query effective CPUs.
    effective_cpus_count(cg)
}

fn r_num_threads() -> Result<c_int> {
    let phys_cpus = r_num_cpus()?;
    let cg = my_cgroup()?;

    // Take cfs quota into account
    match cpu_count_from_quota(cg) {
        Ok(quota_cpus) if quota_cpus < phys_cpus => Ok(quota_cpus),
        _ => Ok(phys_cpus),
    }
}

fn flatten_result(r: Result<c_int>) -> c_int {
    match r {
        Ok(i) => i,
        Err(e) => -e.errno,
    }
}

#[no_mangle]
pub extern "C" fn num_cpus() -> c_int {
    flatten_result(r_num_cpus())
}

#[no_mangle]
pub extern "C" fn recommended_threads() -> c_int {
    flatten_result(r_num_threads())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpus_threads_equal_default() {
        assert_eq!(num_cpus(), recommended_threads());
    }
}
