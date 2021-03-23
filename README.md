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

Results on *my* machine (myrrlyn):

```text
scalar_eq_bitmap        time:   [1.4907 us 1.5238 us 1.5689 us]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

scalar_eq_bitmap1       time:   [1.2963 us 1.3569 us 1.4259 us]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

scalar_eq_bool          time:   [928.68 ns 936.22 ns 944.95 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

bv_scalar_eq_loop       time:   [143.56 ns 144.95 ns 146.48 ns]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

bv_scalar_eq_collect    time:   [36.424 us 37.196 us 38.124 us]
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) high mild
  4 (4.00%) high severe
```

Results:

- `<BitVec as FromIterator>::from_iterator` is *atrocious* and I really need to
  trace it to figure out why. I suspect there's an imperfect allocation, because
  *thirty-seven microseconds* is laughably unacceptable.
- `<&mut BitSlice as IntoIterator>` is *obscenely fast*. I am honestly blown
  away. I thought I would at best *tie* `<&mut [bool] as IntoIterator>` and I
  have *no* explanation for this at all.

For reference, here's `bv_scalar_eq_loop`:

```rust
pub fn bv_scalar_eq_loop(lhs: &[i32], rhs: i32) -> BitVec<Lsb0, u8> {
    let mut buf = BitVec::with_capacity(lhs.len());
    for (mut bit, &val) in buf.iter_mut().zip(lhs.iter()) {
        *bit = val == rhs;
    }
    buf
}
```
