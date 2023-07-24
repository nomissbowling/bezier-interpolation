//! bezier interpolation for Rust
//!
//! link freetype.lib
//!

use std::ops::*;

use freetype as ft;

/// recursive call lambda with self name and parameters
pub fn recurse<P, R>(p: P, f: &dyn Fn(P, &dyn Fn(P) -> R) -> R) -> R {
  f(p, &|p: P| recurse(p, &f))
}

/// Integer tuple for coords (expect I: i32 | i64) now support i32
pub type Itpl<I> = (I, I);

/// trait for coords (expect I: i32 | i64) now support i32
pub trait Path2d<I> {
  /// to f64
  fn to_f64(&self) -> (f64, f64);
  /// from f64
  fn from_f64(p: &(f64, f64)) -> Self;

  /// as i32
  fn as_i32(&self) -> (i32, i32);

  /// from freetype::Vector
  fn from_ft(p: &ft::Vector) -> Self;
  /// mirror top bottom copy from freetype::Vector
  fn inv(&self, p: &ft::Vector) -> Self;
  /// add2d
  fn add2d(&self, p: Self) -> Self;
  /// diff2d
  fn diff2d(&self, p: Self) -> Self;
  /// mul2d f64
  fn mul2df64(&self, f: f64) -> Self;

  /// cross2d
  fn cross2d(&self, v: Self) -> I;

  /// start of contour
  fn move_to(pt: (i32, i32),
    o: &mut Vec<(i32, i32)>, c: &mut Vec<(i32, i32)>, s: &mut Vec<u8>) -> () {
/*
    println!("M {:?}", pt);
*/
    o.push(pt);
    c.push(pt);
    s.push(0);
    ()
  }

  /// end of line
  fn line_to(pt: (i32, i32),
    o: &mut Vec<(i32, i32)>, c: &mut Vec<(i32, i32)>, s: &mut Vec<u8>) -> () {
/*
    println!("L {:?}", pt);
*/
    o.push(pt);
    c.push(pt);
    s.push(2);
    ()
  }

  /// make ncr table (fast)
  fn ncr_table(n: i32) -> Vec<i32> {
    if n == 2 { vec![1, 2, 1] } // pre calc for bezier2
    else if n == 3 { vec![1, 3, 3, 1] } // pre calc for bezier3
    else if n <= 0 { vec![1] }
    else {
      let mut v: Vec<i32> = vec![1i32; n as usize + 1]; // v[0] = v[n] = 1;
      let p: Vec<i32> = Self::ncr_table(n - 1);
      for i in 1..n as usize { v[i] = p[i - 1] + p[i]; }
      v
    }
  }

  /// get ncr (slow)
  fn ncr(n: i32, r: i32) -> i32 {
    let mut p = 1i32;
    if r < n {
      for k in (r + 1)..(n + 1) { p *= k; }
      for k in 2..(n - r + 1) { p /= k; }
    }
    p
    // Self::ncr_table(n)[r as usize] // fast
  }

  /// number of divides
  fn divides() -> i32 {
    16 // 32
  }

  /// bezier interpolation common
  fn bezier_interpolation(pts: &[(i32, i32)], o: &mut Vec<(i32, i32)>) -> () {
    let p = pts.len() as i32;
/*
    print!("{}", match p {3 => 'Q', 4 => 'C', _ => 'B'});
    for pt in pts { print!(" {:?}", pt); }
    println!("");
*/
    let ncr_tbl = Self::ncr_table(p - 1);
    let n = Self::divides();
    for t in 1..n {
      let pt = (0..p).into_iter().map(|j| {
        let r = t as f64 / n as f64;
        let a = (1.0 - r).powi(p - 1 - j) * r.powi(j);
        pts[j as usize].mul2df64(ncr_tbl[j as usize] as f64 * a)
      }).reduce(|p, q| p.add2d(q)).unwrap();
      o.push(pt);
    }
  }

  /// bezier2 conic
  fn bezier_conic(pts: &[(i32, i32)], // 0,1,2
    o: &mut Vec<(i32, i32)>, c: &mut Vec<(i32, i32)>, s: &mut Vec<u8>) -> () {
    c.push(pts[1]);
    s.push(6);
    Self::bezier_interpolation(pts, o);
    o.push(pts[2]);
    c.push(pts[2]);
    s.push(4);
    ()
  }

  /// bezier3 cubic
  fn bezier_cubic(pts: &[(i32, i32)], // 0,1,2,3
    o: &mut Vec<(i32, i32)>, c: &mut Vec<(i32, i32)>, s: &mut Vec<u8>) -> () {
    c.push(pts[1]);
    s.push(12);
    c.push(pts[2]);
    s.push(10);
    Self::bezier_interpolation(pts, o);
    o.push(pts[3]);
    c.push(pts[3]);
    s.push(8);
    ()
  }

  /// get rotation left: false or right: true
  fn get_lr(c: &Vec<(i32, i32)>) -> bool { // not perfect
    let mut sz = c.len();
    if sz > 0 && c[0] == c[sz - 1] { sz -= 1; } // check closed contour
    if sz < 3 { false }
//    else { c[sz / 2].diff2d(c[0]).cross2d(c[sz - 1].diff2d(c[sz / 2])) < 0 }
//    else { c[1].diff2d(c[0]).cross2d(c[0].diff2d(c[sz - 1])) < 0 }
//    else { c[1].diff2d(c[0]).cross2d(c[sz - 1].diff2d(c[1])) < 0 }
    else {
      (1..sz-1).into_iter().map(|i| {
        c[i].diff2d(c[0]).cross2d(c[i+1].diff2d(c[0]))
      }).reduce(|p, q| p + q).unwrap() < 0
    }
  }

  /// outline to GlyphContour
  fn outline2contours(&self, outline: &ft::Outline) -> Vec::<GlyphContour>;
}

/// trait for coords (expect I: i32 | i64) now support i32
impl<I: Neg<Output = I> + Sub<Output = I> + Add<Output = I> + Mul<Output = I>
 + Copy + std::convert::From::<i32>> Path2d<I> for Itpl<I>
 where f64: From<I>, i32: From<I> {
  /// to f64
  fn to_f64(&self) -> (f64, f64) {
    (self.0.into(), self.1.into())
  }

  /// from f64
  fn from_f64(p: &(f64, f64)) -> Self {
//    (p.0.into(), p.1.into()) // + std::convert::From::<f64>
    ((p.0 as i32).into(), (p.1 as i32).into()) // now support i32
  }

  /// as i32
  fn as_i32(&self) -> (i32, i32) {
    (self.0.into(), self.1.into())
  }

  /// from freetype::Vector
  fn from_ft(p: &ft::Vector) -> Self {
    (p.x.into(), p.y.into())
  }

  /// mirror top bottom copy from freetype::Vector
  fn inv(&self, p: &ft::Vector) -> Self {
    let q = Self::from_ft(p);
    (self.0 + q.0, - (self.1 + q.1))
  }

  /// add2d
  fn add2d(&self, p: Self) -> Self {
    (self.0 + p.0, self.1 + p.1)
  }

  /// diff2d
  fn diff2d(&self, p: Self) -> Self {
    (self.0 - p.0, self.1 - p.1)
  }

  /// mul2d f64
  fn mul2df64(&self, f: f64) -> Self {
    let q: (f64, f64) = self.to_f64();
    Self::from_f64(&(q.0 * f, q.1 * f))
  }

  /// cross2d
  fn cross2d(&self, v: Self) -> I {
    self.0 * v.1 - self.1 * v.0
  }

  /// outline to GlyphContour
  fn outline2contours(&self, outline: &ft::Outline) -> Vec::<GlyphContour> {
/*
    let v = vec![ // test (static tree)
      GlyphContour::new(false, vec![
        (100, 240), (200, 300), (400, 200), (240, 240), (100, 200)], // open
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![
          GlyphContour::new(true, vec![
            (140, 220), (240, 260), (220, 280), (140, 240)], // open
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![])]
      )];
*/
/*
    let mut v = Vec::<GlyphContour>::new(); // test (dynamic tree)
    // v.push(GlyphContour::new(false, vec![], vec![], vec![], vec![]));
    let o0 = vec![(100, 240), (200, 300), (400, 200), (240, 240), (100, 200)];
    let c0 = vec![0, 0, 0, 0, 0];
    let s0 = vec![0, 0, 0, 0, 0];
    let mut vc = Vec::<GlyphContour>::new();
    let o1 = vec![(140, 220), (240, 260), (220, 280), (140, 240)];
    let c1 = vec![0, 0, 0, 0];
    let s1 = vec![0, 0, 0, 0];
    vc.push(GlyphContour::new(true, o1, c1, s1, vec![]));
    v.push(GlyphContour::new(false, o0, c0, s0, vc));
*/
//    println!("flags: {:x}", outline.raw.flags); // can not access private
    let mut v = Vec::<GlyphContour>::new(); // dynamic tree
    for contour in outline.contours_iter() {
      let mut o = vec![]; // outline
      let mut c = vec![]; // control points
      let mut s = vec![]; // special points (same length of control points)
      let start = contour.start();
      Self::move_to(self.inv(start).as_i32(),
        &mut o, &mut c, &mut s); // push to
      for curve in contour {
//        recurse(curve, &|curve: ft::outline::Curve, draw_curve| {
//        }); // if curve has children
        match curve {
        ft::outline::Curve::Line(pt) => {
          Self::line_to(self.inv(&pt).as_i32(),
            &mut o, &mut c, &mut s); // push to
        },
        ft::outline::Curve::Bezier2(pt1, pt2) => {
          Self::bezier_conic(&[c[c.len() - 1],
            self.inv(&pt1).as_i32(),
            self.inv(&pt2).as_i32()], &mut o, &mut c, &mut s); // push to
        },
        ft::outline::Curve::Bezier3(pt1, pt2, pt3) => {
          Self::bezier_cubic(&[c[c.len() - 1],
            self.inv(&pt1).as_i32(), self.inv(&pt2).as_i32(),
            self.inv(&pt3).as_i32()], &mut o, &mut c, &mut s); // push to
        }
        }
      }
//      println!("");
      v.push(GlyphContour::new(Self::get_lr(&c), o, c, s, vec![]));
    }
    v
  }
}

/// offset (x, y) scale (sc) pitch (p)
pub type OfsSclPch = ((i32, i32), f64, i32);

/// GlyphContour
pub struct GlyphContour {
  /// rotation left: false or right: true
  pub lr: bool,
  /// outline contours
  pub contour: Vec<(i32, i32)>,
  /// control points
  pub control: Vec<(i32, i32)>,
  /// special points
  pub spec: Vec<u8>,
  /// children
  pub children: Vec<GlyphContour>
}

/// GlyphContour
impl GlyphContour {
  /// constructor
  pub fn new(lr: bool, contour: Vec<(i32, i32)>,
    control: Vec<(i32, i32)>, spec: Vec<u8>,
    children: Vec<GlyphContour>) -> GlyphContour {
    GlyphContour{lr: lr, contour: contour,
      control: control, spec: spec, children: children}
  }

  /// get contour or control processed (offset scale pitch)
  pub fn osp_pts(&self, osp: OfsSclPch, ctrl: bool) -> Vec<(i32, i32)>{
    match ctrl {
    false => self.contour.iter(),
    true => self.control.iter()
    }.map(|&p| osp.0.add2d(p.mul2df64(osp.1))).collect()
  }
}
