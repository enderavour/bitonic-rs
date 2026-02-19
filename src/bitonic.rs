use std::cmp::Ord;
use std::error::Error;
use std::mem::swap;

// For checking if provided array size is power of 2
fn _pow2(n: usize) -> bool
{
    n > 0 && n & (n - 1) == 0
}

pub fn _bitonic_sort<T: Into<u64> + Ord + Copy>(arr: &mut [T], low: usize, size: usize, direction: u8) -> Result<(), Box<dyn Error>>
{
    if size <=  1
    {
        return Ok(())
    }

    let middle = size / 2;

    _bitonic_sort(arr, low, middle, 0)?;
    _bitonic_sort(arr, low + middle, middle, 1)?;

    _bitonic_merge(arr, size, direction);

    Ok(())
}

fn _compare_and_swap<T: Into<u64> + Ord + Copy>(i: &mut T, j: &mut T, direction: u8)
{
    if (direction == 1 && i > j) || (direction == 0 && i < j)
    {
        swap(i, j);
    }
}

fn _bitonic_merge<T: Into<u64> + Ord + Copy>(arr: &mut [T], size: usize, direction: u8)
{
    if size > 1
    {
        let mid = size / 2;

        let (first_half, second_half) = arr.split_at_mut(mid);

        for i in 0..mid
        {
            _compare_and_swap(&mut first_half[i], &mut second_half[i], direction);
        }
    }
}

// is_ascending: true - sort ascending, false - sort descending
pub fn sort_array<T: Into<u64> + Ord + Copy>(arr: &mut [T], is_ascending: bool) -> Result<(), Box<dyn Error>>
{
    if !_pow2(arr.len())
    {
        return Err("Length of array is not power of 2".into())
    }

    let direction = if is_ascending { 1 } else { 0 };
    _bitonic_sort(arr, 0, arr.len(), direction)
}