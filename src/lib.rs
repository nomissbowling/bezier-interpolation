#![doc(html_root_url = "https://docs.rs/bezier-interpolation/0.1.0")]
//! bezier interpolation for Rust
//!
//! link freetype.lib
//!

pub mod bezier_interpolation;

/*
#[cfg(test)]
mod tests {
  use super::bezier_interpolation::*;
  use freetype as ft;

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn check_ncr_i32() {
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr(2, 0), 1);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr(2, 1), 2);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr(2, 2), 1);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr(3, 0), 1);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr(3, 1), 3);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr(3, 2), 3);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr(3, 3), 1);
  }

  #[test]
  fn check_ncr_i64_skip() {
//    assert_eq!(<Itpl<i64> as Path2d<_>>::ncr(0, 0), 1);
  }

  #[test]
  fn check_ncr_table_i32() {
    assert_eq!(<Itpl<_> as Path2d<i32>>::ncr_table(-1), [1]);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr_table(-1), [1]);

    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr_table(0), [1]);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr_table(1), [1, 1]);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr_table(2), [1, 2, 1]);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr_table(3), [1, 3, 3, 1]);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr_table(4), [1, 4, 6, 4, 1]);
    assert_eq!(<Itpl<i32> as Path2d<_>>::ncr_table(5), [1, 5, 10, 10, 5, 1]);
  }

  #[test]
  fn check_ncr_table_i64_skip() {
//    assert_eq!(<Itpl<_> as Path2d<i64>>::ncr_table(-1), [1i32]);
//    assert_eq!(<Itpl<i64> as Path2d<_>>::ncr_table(-1), [1i32]);

//    assert_eq!(<Itpl<i64> as Path2d<_>>::ncr_table(5), [1, 5, 10, 10, 5, 1]);
  }

  #[test]
  fn check_add2d_inv_i32() {
    assert_eq!((0, 0).add2d((0, 0)), (0, 0));
    assert_eq!((0, 0).inv(&ft::Vector{x: 0, y: 0}), (0, 0));
    assert_eq!((1, 2).add2d((3, 4)), (4, 6));
    assert_eq!((1, 2).inv(&ft::Vector{x: 3, y: 4}), (4, -6));

    assert_eq!((0i32, 0i32).add2d((0, 0)), (0i32, 0i32));
    assert_eq!((0i32, 0i32).inv(&ft::Vector{x: 0, y: 0}), (0i32, 0i32));
    assert_eq!((1i32, 2i32).add2d((3, 4)), (4, 6));
    assert_eq!((1i32, 2i32).inv(&ft::Vector{x: 3, y: 4}), (4, -6));
  }

  #[test]
  fn check_add2d_inv_i64_skip() {
//    assert_eq!((0i64, 0i64).add2d((0, 0)), (0i64, 0i64));
//    assert_eq!((0i64, 0i64).inv(&ft::Vector{x: 0, y: 0}), (0i64, 0i64));
//    assert_eq!((1i64, 2i64).add2d((3, 4)), (4, 6));
//    assert_eq!((1i64, 2i64).inv(&ft::Vector{x: 3, y: 4}), (4, -6));
  }

  #[test]
  fn check_diff2d_mul2df64_cross2d_i32() {
    assert_eq!((0, 0).diff2d((0, 0)), (0, 0));
    assert_eq!((1, 2).diff2d((3, 4)), (-2, -2));
    assert_eq!((1, 2).diff2d((3i32, 4i32)), (-2, -2));
    assert_eq!((1, 2).diff2d((3, 4)), (-2i32, -2i32));
    assert_eq!((1i32, 2i32).diff2d((3i32, 4i32)), (-2i32, -2i32));
    assert_eq!((4i32, 3i32).diff2d((2, 5)), (2i32, -2i32));

    assert_eq!((0, 0).mul2df64(0.0), (0, 0));
    assert_eq!((1, 2).mul2df64(3.6), (3, 7));
    assert_eq!((1, 2).mul2df64(3.6), (3i32, 7i32));
    assert_eq!((1i32, 2i32).mul2df64(3.6), (3i32, 7i32));
    assert_eq!((4i32, 3i32).mul2df64(2.4), (9i32, 7i32));

    assert_eq!((0, 0).cross2d((0, 0)), 0);
    assert_eq!((1, 2).cross2d((3, 4)), -2);
    assert_eq!((1, 2).cross2d((3i32, 4i32)), -2);
    assert_eq!((1, 2).cross2d((3, 4)), -2i32);
    assert_eq!((1i32, 2i32).cross2d((3i32, 4i32)), -2i32);
    assert_eq!((4i32, 3i32).cross2d((2, 5)), 14);
  }

  #[test]
  fn check_diff2d_mul2df64_cross2d_i64_skip() {
  }
}
*/
