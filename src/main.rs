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

    let arr2: &mut [u16] = &mut [4, 3, 6, 8, 7, 5, 2, 1];
    sort_array(arr2, true)?;

    // First assertion
    assert_eq!(*arr, [1, 2, 3, 4]);
    // Second Assertion
    assert_ne!(*arr2, [4, 3, 6, 8, 7, 5, 2, 1]);

    let seed = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    unsafe { srand(seed) };

    // Filling array with random numbers
    let mut rand_arr = [0u8; 32];
    rand_arr.iter_mut().for_each(|n| *n = (unsafe { rand() } % 256) as u8);
    println!("Initial array:");
    rand_arr.iter().for_each(|n| print!("{} ", *n));

    // Sort in descending order
    sort_array(&mut rand_arr, false)?;

    println!("Sorted array:");
    rand_arr.iter().for_each(|n| print!("{} ", *n));
    println!();

    Ok(())
}