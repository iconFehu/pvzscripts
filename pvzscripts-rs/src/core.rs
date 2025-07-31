use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_VM_READ, PROCESS_VM_WRITE, PROCESS_VM_OPERATION, PROCESS_QUERY_INFORMATION};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};

pub struct PvzCore;

pub struct ProcessHandle(HANDLE);

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0); }
    }
}

impl PvzCore {
    pub fn find_pvz_process() -> Option<u32> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
            let mut entry = PROCESSENTRY32W::default();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
            if Process32FirstW(snapshot, &mut entry).is_ok() {
                loop {
                    let exe_name = String::from_utf16_lossy(&entry.szExeFile);
                    if exe_name.trim_end_matches('\u{0}').eq_ignore_ascii_case("PlantsVsZombies.exe") {
                        CloseHandle(snapshot);
                        return Some(entry.th32ProcessID);
                    }
                    if Process32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                }
            }
            CloseHandle(snapshot);
        }
        None
    }

    pub fn open_process(pid: u32) -> Option<ProcessHandle> {
        unsafe {
            let handle = OpenProcess(
                PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION | PROCESS_QUERY_INFORMATION,
                false,
                pid,
            );
            if let Ok(h) = handle {
                Some(ProcessHandle(h))
            } else {
                None
            }
        }
    }

    pub fn read_memory(handle: &ProcessHandle, address: usize, buf: &mut [u8]) -> bool {
        unsafe {
            let mut read = 0;
            ReadProcessMemory(
                handle.0,
                address as _,
                buf.as_mut_ptr() as _,
                buf.len(),
                Some(&mut read),
            ).is_ok() && read == buf.len()
        }
    }

    pub fn write_memory(handle: &ProcessHandle, address: usize, buf: &[u8]) -> bool {
        unsafe {
            let mut written = 0;
            WriteProcessMemory(
                handle.0,
                address as _,
                buf.as_ptr() as _,
                buf.len(),
                Some(&mut written),
            ).is_ok() && written == buf.len()
        }
    }

    /// 通过多级指针链获取最终地址
    /// base_addr: 基址
    /// offsets: 偏移数组（如 &[0x10, 0x20, 0x30]）
    /// 返回：最终目标地址（Some(addr)）或 None
    pub fn resolve_pointer_chain(handle: &ProcessHandle, base_addr: usize, offsets: &[usize]) -> Option<usize> {
        let mut addr = base_addr;
        let mut buf = [0u8; 4]; // 32位指针
        for (i, &offset) in offsets.iter().enumerate() {
            if i == 0 {
                addr = addr + offset;
            } else {
                // 读取指针
                if !Self::read_memory(handle, addr, &mut buf) {
                    return None;
                }
                addr = u32::from_le_bytes(buf) as usize + offset;
            }
        }
        Some(addr)
    }
} 