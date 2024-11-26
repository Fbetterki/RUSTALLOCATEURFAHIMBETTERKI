#![no_std]
#![no_main]


use core::ptr;

unsafe fn reserve_memory(size: usize) -> *mut u8 
{
    let mut base_address: usize = 0;
    let mut new_address: usize = 0;

    asm!(
        "mov rax, 12",           
        "xor rdi, rdi",          
        "syscall",
        "mov {}, rax",           
        out(reg) base_address
    );

    if base_address == 0 {
        return ptr::null_mut(); 
    }


    let target_address = base_address + size;
    asm!(
        "mov rax, 12",           
        "mov rdi, {}",           
        "syscall",
        "mov {}, rax",           
        in(reg) target_address, out(reg) new_address
    );

    if new_address < target_address {
        return ptr::null_mut(); 
    }

    base_address as *mut u8
}

