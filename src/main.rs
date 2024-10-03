use std::io::stdin;
use anyhow::{Error, Result};
use std::ffi::{c_uint, c_void};

extern {
    fn malloc(size: c_uint) -> *mut c_void;
    // fn free(ptr: *mut c_void);
}

fn main() -> Result<()> {
    // Confirmation
    println!("This program will take up all your memory rapidly.");
    println!("Input 1 to launch: ");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    if input.trim() != "1" {
        println!("Exiting...");
        return Ok(());
    }

    // Allocate memory until running out
    let mut size = 1 << 30;
    println!("Allocating {}MB memory block...", size >> 20);
    while size > 0 {
        unsafe {
            let ptr = malloc(size);
            if ptr.is_null() {
                size = size >> 1;
                println!("Bad alloc. Allocating {}MB memory block...", size >> 20);
            }
        }
    }
    println!("Memory run out!");
    loop {}
    Ok(())
}
