#![no_std]
#![no_main]


use core::ptr;

unsafe fn reserve_memory(size: usize) -> *mut u8 {

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


}

