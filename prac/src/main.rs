use sysinfo::{CpuExt, System, SystemExt, RefreshKind, CpuRefreshKind};

fn main() {
let s = System::new_with_specifics(
    RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
);
for cpu in s.cpus() {
    println!("{}", cpu.brand());
}
}