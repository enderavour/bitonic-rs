# bitonic-rs - Example implementation of Bitonic Sort in Rust

## Description
- Example implementation of bitonic sort of different integer type arrays. Recursion-based approach. 
- Checks if array length is power of 2
- Maybe will work for floating types as well (not tested)
- In future, sorting may be extended using threads or coroutines

# Issues
For some reason, descending ordering of second part of random array is not performed well, 
although the algorithm should be implemented correctly. Maybe something wrong is with passing array to sorting functions. Reviews and solutions are welcome