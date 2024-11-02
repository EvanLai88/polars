use super::*;

#[inline]
fn new_is_min<T: IsFloat + PartialOrd>(old: &T, new: &T) -> bool {
    compare_fn_nan_min(old, new).is_ge()
}

#[inline]
fn new_is_min_bool(old: bool, new: bool) -> bool {
    compare_fn_nan_min(&old, &new).is_ge()
    // old | !new
}

#[inline]
fn new_is_max<T: IsFloat + PartialOrd>(old: &T, new: &T) -> bool {
    compare_fn_nan_max(old, new).is_le()
}

#[inline]
fn new_is_max_bool(old: bool, new: bool) -> bool {
    compare_fn_nan_max(&old, &new).is_le()
    // !old | new
}

#[inline]
unsafe fn get_min_and_idx<T>(
    slice: &[T],
    start: usize,
    end: usize,
    sorted_to: usize,
) -> Option<(usize, &T)>
where
    T: NativeType + IsFloat + PartialOrd,
{
    if sorted_to >= end {
        // If we're sorted past the end we can just take the first element because this function
        // won't be called on intervals that contain the previous min
        Some((start, slice.get_unchecked(start)))
    } else if sorted_to <= start {
        // We have to inspect the whole range
        // Reversed because min_by returns the first min if there's a tie but we want the last
        slice
            .get_unchecked(start..end)
            .iter()
            .enumerate()
            .rev()
            .min_by(|&a, &b| compare_fn_nan_min(a.1, b.1))
            .map(|v| (v.0 + start, v.1))
    } else {
        // It's sorted in range start..sorted_to. Compare slice[start] to min over sorted_to..end
        let s = (start, slice.get_unchecked(start));
        slice
            .get_unchecked(sorted_to..end)
            .iter()
            .enumerate()
            .rev()
            .min_by(|&a, &b| compare_fn_nan_min(a.1, b.1))
            .map(|v| {
                if new_is_min(s.1, v.1) {
                    (v.0 + sorted_to, v.1)
                } else {
                    s
                }
            })
    }
}

#[inline]
unsafe fn get_min_and_idx_bool(
    bitmap: &Bitmap,
    start: usize,
    end: usize,
    sorted_to: usize,
) -> Option<(usize, bool)> {
    if sorted_to >= end {
        Some((start, bitmap.get_bit_unchecked(start)))
    } else if sorted_to <= start {
        // let slice = bitmap.clone().sliced_unchecked(start, end - start);
        // match slice.not().true_idx_iter().last() {
        //     Some(idx) => Some((start + idx, false)),
        //     None => Some((end, true)),
        // }
        bitmap
            .clone()
            .sliced_unchecked(start, end - start)
            .iter()
            .enumerate()
            .rev()
            .min_by(|&a, &b| compare_fn_nan_min(&(a.1), &(b.1)))
            .map(|v| (v.0 + start, v.1))
    } else {
        let s = (start, bitmap.get_bit_unchecked(start));
        // let slice = bitmap.clone().sliced_unchecked(start, end - start);
        // match slice.not().true_idx_iter().last() {
        //     Some(idx) => Some((sorted_to + idx, false)),
        //     None => {
        //         if new_is_min_bool(s.1, true) {
        //             Some((end, true))
        //         } else {
        //             Some(s)
        //         }
        //     },
        // }
        bitmap
            .clone()
            .sliced_unchecked(start, end - start)
            .iter()
            .enumerate()
            .rev()
            .min_by(|&a, &b| compare_fn_nan_min(&(a.1), &(b.1)))
            .map(|v| {
                if new_is_min(&(s.1), &(v.1)) {
                    (v.0 + sorted_to, v.1)
                } else {
                    s
                }
            })
    }
}

#[inline]
unsafe fn get_max_and_idx<T>(
    slice: &[T],
    start: usize,
    end: usize,
    sorted_to: usize,
) -> Option<(usize, &T)>
where
    T: NativeType + IsFloat + PartialOrd,
{
    if sorted_to >= end {
        Some((start, slice.get_unchecked(start)))
    } else if sorted_to <= start {
        slice
            .get_unchecked(start..end)
            .iter()
            .enumerate()
            .max_by(|&a, &b| compare_fn_nan_max(a.1, b.1))
            .map(|v| (v.0 + start, v.1))
    } else {
        let s = (start, slice.get_unchecked(start));
        slice
            .get_unchecked(sorted_to..end)
            .iter()
            .enumerate()
            .max_by(|&a, &b| compare_fn_nan_max(a.1, b.1))
            .map(|v| {
                if new_is_max(s.1, v.1) {
                    (v.0 + sorted_to, v.1)
                } else {
                    s
                }
            })
    }
}

#[inline]
unsafe fn get_max_and_idx_bool(
    bitmap: &Bitmap,
    start: usize,
    end: usize,
    sorted_to: usize,
) -> Option<(usize, bool)> {
    if sorted_to >= end {
        Some((start, bitmap.get_bit_unchecked(start)))
    } else if sorted_to <= start {
        // let slice = bitmap.clone().sliced_unchecked(start, end - start);
        // match slice.true_idx_iter().last() {
        //     Some(idx) => Some((start + idx, true)),
        //     None => Some((end, false)),
        // }
        bitmap
            .clone()
            .sliced_unchecked(start, end - start)
            .iter()
            .enumerate()
            .max_by(|&a, &b| compare_fn_nan_max(&(a.1), &(b.1)))
            .map(|v| (v.0 + start, v.1))
    } else {
        let s = (start, bitmap.get_bit_unchecked(start));
        // let slice = bitmap.clone().sliced_unchecked(start, end - start);
        // match slice.true_idx_iter().last() {
        //     Some(idx) => Some((sorted_to + idx, true)),
        //     None => {
        //         if new_is_max_bool(s.1, false) {
        //             Some((end, false))
        //         } else {
        //             Some(s)
        //         }
        //     },
        // }
        bitmap
            .clone()
            .sliced_unchecked(start, end - start)
            .iter()
            .enumerate()
            .max_by(|&a, &b| compare_fn_nan_max(&(a.1), &(b.1)))
            .map(|v| {
                if new_is_max(&(s.1), &(v.1)) {
                    (v.0 + sorted_to, v.1)
                } else {
                    s
                }
            })
    }
}

#[inline]
fn n_sorted_past_min<T: NativeType + IsFloat + PartialOrd>(slice: &[T]) -> usize {
    slice
        .windows(2)
        .position(|x| compare_fn_nan_min(&x[0], &x[1]).is_gt())
        .unwrap_or(slice.len() - 1)
}

#[inline]
fn n_sorted_past_min_bool(bitmap: &Bitmap) -> usize {
    let mut iter = bitmap.iter();

    match unsafe { bitmap.get_bit_unchecked(0) } {
        true => iter.take_leading_ones(),
        false => {
            let leading_zeros = iter.take_leading_zeros();
            let trailing_ones = iter.take_leading_ones();
            leading_zeros + trailing_ones
        },
    }
}

#[inline]
fn n_sorted_past_max<T: NativeType + IsFloat + PartialOrd>(slice: &[T]) -> usize {
    slice
        .windows(2)
        .position(|x| compare_fn_nan_max(&x[0], &x[1]).is_lt())
        .unwrap_or(slice.len() - 1)
}

#[inline]
fn n_sorted_past_max_bool(bitmap: &Bitmap) -> usize {
    let mut iter = bitmap.iter();

    match unsafe { bitmap.get_bit_unchecked(0) } {
        true => {
            let leading_ones = iter.take_leading_ones();
            let trailing_zeros = iter.take_leading_zeros();
            leading_ones + trailing_zeros
        },
        false => iter.take_leading_zeros(),
    }
}

// Min and max really are the same thing up to a difference in comparison direction, as represented
// here by helpers we pass in. Making both with a macro helps keep behavior synchronized
macro_rules! minmax_window {
    ($m_window:tt, $get_m_and_idx:ident, $new_is_m:ident, $n_sorted_past:ident) => {
        pub struct $m_window<'a, T: NativeType + PartialOrd + IsFloat> {
            slice: &'a [T],
            m: T,
            m_idx: usize,
            sorted_to: usize,
            last_start: usize,
            last_end: usize,
        }

        impl<'a, T: NativeType + IsFloat + PartialOrd> $m_window<'a, T> {
            #[inline]
            unsafe fn update_m_and_m_idx(&mut self, m_and_idx: (usize, &T)) {
                self.m = *m_and_idx.1;
                self.m_idx = m_and_idx.0;
                if self.sorted_to <= self.m_idx {
                    // Track how far past the current extremum values are sorted. Direction depends on min/max
                    // Tracking sorted ranges lets us only do comparisons when we have to.
                    self.sorted_to =
                        self.m_idx + 1 + $n_sorted_past(&self.slice.get_unchecked(self.m_idx..));
                }
            }
        }

        impl<'a, T: NativeType + IsFloat + PartialOrd> RollingAggWindowNoNulls<'a, T>
            for $m_window<'a, T>
        {
            fn new(
                slice: &'a [T],
                start: usize,
                end: usize,
                _params: Option<RollingFnParams>,
            ) -> Self {
                let (idx, m) =
                    unsafe { $get_m_and_idx(slice, start, end, 0).unwrap_or((0, &slice[start])) };
                Self {
                    slice,
                    m: *m,
                    m_idx: idx,
                    sorted_to: idx + 1 + $n_sorted_past(&slice[idx..]),
                    last_start: start,
                    last_end: end,
                }
            }

            unsafe fn update(&mut self, start: usize, end: usize) -> Option<T> {
                //For details see: https://github.com/pola-rs/polars/pull/9277#issuecomment-1581401692
                self.last_start = start; // Don't care where the last one started
                let old_last_end = self.last_end; // But we need this
                self.last_end = end;
                let entering_start = std::cmp::max(old_last_end, start);
                let entering = if end - entering_start == 1 {
                    // Faster in the special, but common, case of a fixed window rolling by one
                    Some((entering_start, self.slice.get_unchecked(entering_start)))
                } else if old_last_end == end {
                    // Edge case for shrinking windows
                    None
                } else {
                    $get_m_and_idx(self.slice, entering_start, end, self.sorted_to)
                };
                let empty_overlap = old_last_end <= start;

                if entering.map(|em| $new_is_m(&self.m, em.1) || empty_overlap) == Some(true) {
                    // The entering extremum "beats" the previous extremum so we can ignore the overlap
                    self.update_m_and_m_idx(entering.unwrap());
                    return Some(self.m);
                } else if self.m_idx >= start || empty_overlap {
                    // The previous extremum didn't drop off. Keep it
                    return Some(self.m);
                }
                // Otherwise get the min of the overlapping window and the entering min
                match (
                    $get_m_and_idx(self.slice, start, old_last_end, self.sorted_to),
                    entering,
                ) {
                    (Some(pm), Some(em)) => {
                        if $new_is_m(pm.1, em.1) {
                            self.update_m_and_m_idx(em);
                        } else {
                            self.update_m_and_m_idx(pm);
                        }
                    },
                    (Some(pm), None) => self.update_m_and_m_idx(pm),
                    (None, Some(em)) => self.update_m_and_m_idx(em),
                    // This would mean both the entering and previous windows are empty
                    (None, None) => unreachable!(),
                }

                Some(self.m)
            }
        }
    };
}

minmax_window!(MinWindow, get_min_and_idx, new_is_min, n_sorted_past_min);
minmax_window!(MaxWindow, get_max_and_idx, new_is_max, n_sorted_past_max);

macro_rules! minmax_window_bool {
    ($m_window_bool:tt, $get_m_and_idx:ident, $new_is_m:ident, $n_sorted_past:ident) => {
        pub struct $m_window_bool<'a> {
            bitmap: &'a Bitmap,
            m: bool,
            m_idx: usize,
            sorted_to: usize,
            last_start: usize,
            last_end: usize,
        }

        impl<'a> $m_window_bool<'a> {
            #[inline]
            unsafe fn update_m_and_m_idx(&mut self, m_and_idx: (usize, bool)) {
                self.m = m_and_idx.1;
                self.m_idx = m_and_idx.0;
                if self.sorted_to <= self.m_idx {
                    let slice = self
                        .bitmap
                        .clone()
                        .sliced_unchecked(self.m_idx, self.bitmap.len() - self.m_idx);
                    self.sorted_to = self.m_idx + $n_sorted_past(&slice);
                }
            }
        }

        impl<'a> RollingAggWindowBoolNoNulls<'a> for $m_window_bool<'a> {
            fn new(bitmap: &'a Bitmap, start: usize, end: usize) -> Self {
                let (idx, m) = unsafe {
                    $get_m_and_idx(bitmap, start, end, 0)
                        .unwrap_or((0, bitmap.get_bit_unchecked(start)))
                };
                let slice = unsafe { bitmap.clone().sliced_unchecked(idx, bitmap.len() - idx) };
                Self {
                    bitmap,
                    m: m,
                    m_idx: idx,
                    sorted_to: idx + $n_sorted_past(&slice),
                    last_start: start,
                    last_end: end,
                }
            }

            unsafe fn update(&mut self, start: usize, end: usize) -> Option<bool> {
                self.last_start = start;
                let old_last_end = self.last_end;
                self.last_end = end;
                let entering_start = std::cmp::max(old_last_end, start);
                let entering = if end - entering_start == 1 {
                    Some((
                        entering_start,
                        self.bitmap.get_bit_unchecked(entering_start),
                    ))
                } else if old_last_end == end {
                    None
                } else {
                    $get_m_and_idx(self.bitmap, entering_start, end, self.sorted_to)
                };
                let empty_overlap = old_last_end <= start;

                if entering.map(|em| $new_is_m(self.m, em.1) || empty_overlap) == Some(true) {
                    self.update_m_and_m_idx(entering.unwrap());
                    return Some(self.m);
                } else if self.m_idx >= start || empty_overlap {
                    return Some(self.m);
                }

                match (
                    $get_m_and_idx(self.bitmap, start, old_last_end, self.sorted_to),
                    entering,
                ) {
                    (Some(pm), Some(em)) => {
                        if $new_is_m(pm.1, em.1) {
                            self.update_m_and_m_idx(em);
                        } else {
                            self.update_m_and_m_idx(pm);
                        }
                    },
                    (Some(pm), None) => self.update_m_and_m_idx(pm),
                    (None, Some(em)) => self.update_m_and_m_idx(em),
                    (None, None) => unreachable!(),
                }

                Some(self.m)
            }
        }
    };
}

minmax_window_bool!(
    MinWindowBool,
    get_min_and_idx_bool,
    new_is_min_bool,
    n_sorted_past_min_bool
);
minmax_window_bool!(
    MaxWindowBool,
    get_max_and_idx_bool,
    new_is_max_bool,
    n_sorted_past_max_bool
);

pub(crate) fn compute_min_weights<T>(values: &[T], weights: &[T]) -> T
where
    T: NativeType + PartialOrd + std::ops::Mul<Output = T>,
{
    values
        .iter()
        .zip(weights)
        .map(|(v, w)| *v * *w)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

pub(crate) fn compute_max_weights<T>(values: &[T], weights: &[T]) -> T
where
    T: NativeType + PartialOrd + IsFloat + Bounded + Mul<Output = T>,
{
    let mut max = T::min_value();
    for v in values.iter().zip(weights).map(|(v, w)| *v * *w) {
        if T::is_float() && v.is_nan() {
            return v;
        }
        if v > max {
            max = v
        }
    }

    max
}

// Same as the window definition. The dispatch is identical up to the name.
macro_rules! rolling_minmax_func {
    ($rolling_m:ident, $window:tt, $wtd_f:ident) => {
        pub fn $rolling_m<T>(
            values: &[T],
            window_size: usize,
            min_periods: usize,
            center: bool,
            weights: Option<&[f64]>,
            _params: Option<RollingFnParams>,
        ) -> PolarsResult<ArrayRef>
        where
            T: NativeType + PartialOrd + IsFloat + Bounded + NumCast + Mul<Output = T> + Num,
        {
            let offset_fn = match center {
                true => det_offsets_center,
                false => det_offsets,
            };
            match weights {
                None => rolling_apply_agg_window::<$window<_>, _, _>(
                    values,
                    window_size,
                    min_periods,
                    offset_fn,
                    None,
                ),
                Some(weights) => {
                    assert!(
                        T::is_float(),
                        "implementation error, should only be reachable by float types"
                    );
                    let weights = weights
                        .iter()
                        .map(|v| NumCast::from(*v).unwrap())
                        .collect::<Vec<_>>();
                    no_nulls::rolling_apply_weights(
                        values,
                        window_size,
                        min_periods,
                        offset_fn,
                        $wtd_f,
                        &weights,
                    )
                },
            }
        }
    };
}

rolling_minmax_func!(rolling_min, MinWindow, compute_min_weights);
rolling_minmax_func!(rolling_max, MaxWindow, compute_max_weights);

// weights are handled by rolling_minmax
macro_rules! rolling_minmax_func_bool {
    ($rolling_m_bool:ident, $window_bool:tt) => {
        pub fn $rolling_m_bool(
            arr: &Bitmap,
            window_size: usize,
            min_periods: usize,
            center: bool,
        ) -> PolarsResult<ArrayRef> {
            let offset_fn = match center {
                true => det_offsets_center,
                false => det_offsets,
            };
            rolling_apply_agg_window_bool::<$window_bool, _>(
                arr,
                window_size,
                min_periods,
                offset_fn,
            )
        }
    };
}

rolling_minmax_func_bool!(rolling_min_bool, MinWindowBool);
rolling_minmax_func_bool!(rolling_max_bool, MaxWindowBool);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rolling_min_max() {
        let values = &[1.0f64, 5.0, 3.0, 4.0];

        let out = rolling_min(values, 2, 2, false, None, None).unwrap();
        let out = out.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
        let out = out.into_iter().map(|v| v.copied()).collect::<Vec<_>>();
        assert_eq!(out, &[None, Some(1.0), Some(3.0), Some(3.0)]);
        let out = rolling_max(values, 2, 2, false, None, None).unwrap();
        let out = out.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
        let out = out.into_iter().map(|v| v.copied()).collect::<Vec<_>>();
        assert_eq!(out, &[None, Some(5.0), Some(5.0), Some(4.0)]);

        let out = rolling_min(values, 2, 1, false, None, None).unwrap();
        let out = out.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
        let out = out.into_iter().map(|v| v.copied()).collect::<Vec<_>>();
        assert_eq!(out, &[Some(1.0), Some(1.0), Some(3.0), Some(3.0)]);
        let out = rolling_max(values, 2, 1, false, None, None).unwrap();
        let out = out.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
        let out = out.into_iter().map(|v| v.copied()).collect::<Vec<_>>();
        assert_eq!(out, &[Some(1.0), Some(5.0), Some(5.0), Some(4.0)]);

        let out = rolling_max(values, 3, 1, false, None, None).unwrap();
        let out = out.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
        let out = out.into_iter().map(|v| v.copied()).collect::<Vec<_>>();
        assert_eq!(out, &[Some(1.0), Some(5.0), Some(5.0), Some(5.0)]);

        // test nan handling.
        let values = &[1.0, 2.0, 3.0, f64::nan(), 5.0, 6.0, 7.0];
        let out = rolling_min(values, 3, 3, false, None, None).unwrap();
        let out = out.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
        let out = out.into_iter().map(|v| v.copied()).collect::<Vec<_>>();
        // we cannot compare nans, so we compare the string values
        assert_eq!(
            format!("{:?}", out.as_slice()),
            format!(
                "{:?}",
                &[
                    None,
                    None,
                    Some(1.0),
                    Some(f64::nan()),
                    Some(f64::nan()),
                    Some(f64::nan()),
                    Some(5.0)
                ]
            )
        );

        let out = rolling_max(values, 3, 3, false, None, None).unwrap();
        let out = out.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
        let out = out.into_iter().map(|v| v.copied()).collect::<Vec<_>>();
        assert_eq!(
            format!("{:?}", out.as_slice()),
            format!(
                "{:?}",
                &[
                    None,
                    None,
                    Some(3.0),
                    Some(f64::nan()),
                    Some(f64::nan()),
                    Some(f64::nan()),
                    Some(7.0)
                ]
            )
        );
        // test bool
        let values = Bitmap::from([true, true, false, false, true]);

        let out = rolling_min_bool(&values, 2, 2, false).unwrap();
        let out = out.as_any().downcast_ref::<BooleanArray>().unwrap();
        let out = out.into_iter().collect::<Vec<_>>();
        assert_eq!(out, &[None, Some(true), Some(false), Some(false), Some(false)]);
        let out = rolling_max_bool(&values, 2, 2, false).unwrap();
        let out = out.as_any().downcast_ref::<BooleanArray>().unwrap();
        let out = out.into_iter().collect::<Vec<_>>();
        assert_eq!(out, &[None, Some(true), Some(true), Some(false), Some(true)]);

        let out = rolling_min_bool(&values, 2, 1, false).unwrap();
        let out = out.as_any().downcast_ref::<BooleanArray>().unwrap();
        let out = out.into_iter().collect::<Vec<_>>();
        assert_eq!(out, &[Some(true), Some(true), Some(false), Some(false), Some(false)]);
        let out = rolling_max_bool(&values, 2, 1, false).unwrap();
        let out = out.as_any().downcast_ref::<BooleanArray>().unwrap();
        let out = out.into_iter().collect::<Vec<_>>();
        assert_eq!(out, &[Some(true), Some(true), Some(true), Some(false), Some(true)]);

        let out = rolling_min_bool(&values, 3, 1, false).unwrap();
        let out = out.as_any().downcast_ref::<BooleanArray>().unwrap();
        let out = out.into_iter().collect::<Vec<_>>();
        assert_eq!(out, &[Some(true), Some(true), Some(false), Some(false), Some(false)]);
        let out = rolling_max_bool(&values, 3, 1, false).unwrap();
        let out = out.as_any().downcast_ref::<BooleanArray>().unwrap();
        let out = out.into_iter().collect::<Vec<_>>();
        assert_eq!(out, &[Some(true), Some(true), Some(true), Some(true), Some(true)]);

    }
    
}
