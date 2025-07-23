use pvzscripts_rs::{PvzCore, PvzExtra};

fn main() {
    match PvzCore::find_pvz_process() {
        Some(pid) => println!("找到 PVZ 进程，PID: {}", pid),
        None => println!("未找到 PVZ 进程"),
    }
    PvzExtra::log_info("核心库测试完成");
}
