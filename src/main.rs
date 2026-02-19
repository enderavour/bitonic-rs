mod bitonic;
use bitonic::sort_array;
use std::error::Error;
use std::ffi::{c_int, c_ulong, c_void};
use std::time::{SystemTime, UNIX_EPOCH};

unsafe extern "C"
{
    unsafe fn rand() -> c_int;
    unsafe fn srand(seed: c_ulong) -> c_void;
}

fn main() -> Result<(), Box<dyn Error>>
{
    let arr: &mut [u32] = &mut [4, 1, 3, 2];
    sort_array(arr, true)?;

    // First assertion
    assert_eq!(*arr, [1, 2, 3, 4]);

    let seed = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    unsafe { srand(seed) };

    // Filling array with random numbers
    let mut arr = [0u8; 32];
    arr.iter_mut().for_each(|n| *n = (unsafe { rand() } % 256) as u8);
    println!("Initial array:");
    arr.iter().for_each(|n| print!("{} ", *n));

    // Sort in descending order
    sort_array(&mut arr, false)?;

    println!("Sorted array:");
    arr.iter().for_each(|n| print!("{} ", *n));
    println!();

    Ok(())
}