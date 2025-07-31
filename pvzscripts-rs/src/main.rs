use pvzscripts_rs::{PvzCore, PvzExtra};

fn main() {
    // 1. 查找 PVZ 进程
    let pid = match PvzCore::find_pvz_process() {
        Some(pid) => pid,
        None => {
            println!("未找到 PVZ 进程");
            return;
        }
    };
    println!("找到 PVZ 进程，PID: {}", pid);

    // 2. 打开进程句柄
    let handle = match PvzCore::open_process(pid) {
        Some(h) => h,
        None => {
            println!("无法打开进程");
            return;
        }
    };

    // 3. 通过多级指针链读取阳光数
    let base = 0x6A9EC0; // 常见中文版基址
    let offsets = [0x0, 0x768, 0x5560]; // 阳光数一级偏移
    if let Some(sun_addr) = PvzCore::resolve_pointer_chain(&handle, base, &offsets) {
        let mut buf = [0u8; 4];
        if PvzCore::read_memory(&handle, sun_addr, &mut buf) {
            let sun = i32::from_le_bytes(buf);
            println!("当前阳光数: {}", sun);

            // 4. 写入阳光数（如改为9999）
            let new_sun = 9999i32.to_le_bytes();
            if PvzCore::write_memory(&handle, sun_addr, &new_sun) {
                println!("阳光数已修改为 9999");
            } else {
                println!("写入阳光数失败");
            }
        } else {
            println!("读取阳光数失败");
        }
    } else {
        println!("多级指针链解析失败");
    }
}
