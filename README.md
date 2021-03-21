# Perf comparison LSB vs Vec<bool>

Goal: improve performance of `scalar_eq_bitmap`.

Results on my machine:

```
scalar_eq_bitmap        time:   [1.4113 us 1.4302 us 1.4512 us]                    
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

scalar_eq_bool          time:   [971.39 ns 986.40 ns 1.0033 us]                  
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe
```
