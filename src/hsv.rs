// author: Kolja 
// added this library module to experiment with Rust language features
use sdl2::pixels::Color;
use std::cmp;
use std::fmt;

const MAX_DIFF: f32 = 0.00005;

#[derive(Copy, Clone, Debug)]
pub struct RGB {
    //expected ranges of r,g,b: 0..255
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

//impl<T> fmt::Display for HSV<T> {
impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.5}, {:.5}, {:.5})", self.r, self.g, self.b)
    }
}

impl PartialEq for RGB {
    //impl<T> PartialEq for HSV<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.r - other.r > MAX_DIFF || self.g - other.g > MAX_DIFF || self.b - other.b > MAX_DIFF
        {
            false
        } else {
            true
        }
    }
}

#[derive(Copy, Clone, Debug)] // https://doc.rust-lang.org/std/fmt/trait.Debug.html
pub struct HSV {
    //pub struct HSV<T> {
    //expected ranges of h,s,v: 0..1
    pub h: f32,
    pub s: f32,
    pub v: f32, // 0..1, not 0..360 degrees
}

// https://stackoverflow.com/questions/41447678/comparison-of-two-floats-in-rust-to-arbitrary-level-of-precision
impl PartialEq for HSV {
    //impl<T> PartialEq for HSV<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.h - other.h > MAX_DIFF || self.s - other.s > MAX_DIFF || self.v - other.v > MAX_DIFF
        {
            false
        } else {
            true
        }
    }
}

//impl<T> fmt::Display for HSV<T> {
impl fmt::Display for HSV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.5}, {:.5}, {:.5})", self.h, self.s, self.v)
    }
}

pub trait Convert<T> {
    fn to_rgb(&self) -> T;
    fn from_rgb(c: T) -> HSV;
}

impl Convert<Color> for HSV {
    #[allow(unused)]
    fn to_rgb(&self) -> Color {
        let h6 = self.h * 6.0;
        let i6 = h6 as i32;
        let f = h6 - i6 as f32;
        let p = self.v * (1.0 - self.s);
        let q = self.v * (1.0 - f * self.s);
        let t = self.v * (1.0 - (1.0 - f) * self.s);
        let rgb: (f32, f32, f32) = match i6 % 6 {
            0 => (self.v, t, p),
            1 => (q, self.v, p),
            2 => (p, self.v, t),
            3 => (p, q, self.v),
            4 => (t, p, self.v),
            5 => (self.v, p, q),
            _ => (0.0, 0.0, 0.0), // mission impossible
        };
        Color::RGB(
            (rgb.0 * 255.0).round() as u8,
            (rgb.1 * 255.0).round() as u8,
            (rgb.2 * 255.0).round() as u8,
        )
    }

    #[allow(unused)]
    fn from_rgb(c: Color) -> HSV {
        // Note: Since rgb values are being rounded to whole numbers
        // a conversion of (h,s,v) with to_rgb and back again with
        // from_rgb will differ slightly from original (h,s,v)
        let mut min: u8 = 0;
        let mut max: u8 = 0;
        min = cmp::min(c.r, c.g);
        min = cmp::min(min, c.b);
        max = cmp::max(c.r, c.g);
        max = cmp::max(max, c.b);

        let mut hsv = HSV {
            h: 0.0,
            s: 0.0,
            v: max as f32 / 255.0, // set v
        };
        let delta = max - min;
        if delta > 0 {
            if max > 0 {
                hsv.s = delta as f32 / max as f32; // set s
                let mut hsv_h: f32 = 0.0;
                if max > c.r {
                    if max > c.g {
                        hsv_h = 4.0 + ((c.r as i16 - c.g as i16) as f32 / delta as f32);
                    } else {
                        hsv_h = 2.0 + ((c.b as i16 - c.r as i16) as f32 / delta as f32);
                    }
                } else {
                    hsv_h = (c.g as i16 - c.b as i16) as f32 / delta as f32;
                }
                hsv_h = hsv_h * 60.0;
                if hsv_h < 0.0 {
                    hsv_h = hsv_h + 360.0;
                }
                hsv.h = hsv_h / 360.0; // set h, degrees normalized to 0..1
            } else {
                hsv.v = f32::NAN;
            }
        }

        hsv
    }
}

impl Convert<RGB> for HSV {
    #[allow(unused)]
    fn to_rgb(&self) -> RGB {
        let h6 = self.h * 6.0;
        let i6 = h6 as i32;
        let f = h6 - i6 as f32;
        let p = self.v * (1.0 - self.s);
        let q = self.v * (1.0 - f * self.s);
        let t = self.v * (1.0 - (1.0 - f) * self.s);
        let rgb: (f32, f32, f32) = match i6 % 6 {
            0 => (self.v, t, p),
            1 => (q, self.v, p),
            2 => (p, self.v, t),
            3 => (p, q, self.v),
            4 => (t, p, self.v),
            5 => (self.v, p, q),
            _ => (0.0, 0.0, 0.0), // mission impossible
        };
        RGB {
            r: (rgb.0 * 255.0),
            g: (rgb.1 * 255.0),
            b: (rgb.2 * 255.0),
        }
    }

    #[allow(unused)]
    fn from_rgb(c: RGB) -> HSV {
        let mut min: f32 = 0.0;
        let mut max: f32 = 0.0;
        min = c.r.min(c.g);
        min = c.b.min(min);
        max = c.r.max(c.g);
        max = c.b.max(max);

        let mut hsv = HSV {
            h: 0.0,
            s: 0.0,
            v: max as f32 / 255.0, // set v
        };
        let delta = max - min;
        if delta > 0.00001 {
            if max > 0.0 {
                hsv.s = delta as f32 / max as f32; // set s
                let mut hsv_h: f32 = 0.0;
                if max > c.r {
                    if max > c.g {
                        hsv_h = 4.0 + ((c.r as i16 - c.g as i16) as f32 / delta);
                    } else {
                        hsv_h = 2.0 + ((c.b as i16 - c.r as i16) as f32 / delta);
                    }
                } else {
                    hsv_h = (c.g as i16 - c.b as i16) as f32 / delta;
                }
                hsv_h = hsv_h * 60.0;
                if hsv_h < 0.0 {
                    hsv_h = hsv_h + 360.0;
                }
                hsv.h = hsv_h / 360.0; // set h, degrees normalized to 0..1
            } else {
                hsv.v = f32::NAN;
            }
        }

        hsv
    }
}

impl HSV {
    //impl<T> HSV<T> where T: Color {
    #[inline]
    #[allow(non_snake_case)]
    #[allow(unused)]
    pub const fn zero() -> HSV {
        HSV {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(unused)]
    pub const fn new(h: f32, s: f32, v: f32) -> HSV {
        HSV { h, s, v }
    }
}

//https://users.rust-lang.org/t/how-check-type-of-variable/33845/2
// use std::any::type_name;
// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }
//struct HSVTest {
//}
// TODO: explore instead of implementing each return type of to_rgb do s.th. like this ???
// impl HSVTest<T: Color> {
//     fn to_rgb<T>() -> T {
//         do calculations
//         return type according to T
//         if std::any::type_name::<T>() = "sdl2::pixels::Color"
//         {
//             return Color(0,0,0);
//         }
//         if std::any::type_name::<T>() = "RGB"
//         {
//             return RGB {r:0.0, g:0.0, b:0.0};
//         }
//     }
// }

#[cfg(test)]
mod test_hsv {
    use super::*;

    #[test]
    fn test_hsv_to_color_rgb() {
        // Test data generated in GNU Octave
        // hsv = [5.0/360 0 .05;120.0/360 .25 .25; 1.0 .75 0.75; 330.0/360 1 1; 1 1 1];
        // rgb = hsv2rgb(hsv);
        // round(rgb .* 255)
        // ans =
        //   13    13    13
        //   48    64    48
        //  191    48    48
        //  255     0   128
        //  255     0     0
        // let t:HSV = HSV {
        //     h: 5.0 / 360.0,
        //     s: 0.0,
        //     v: 0.05
        // };
        //let s = <Person as Into<String>>::into(me);

        //let t2:RGB = t::Convert.to_rgb();
        // // HSV: 5deg, 0%, 5% -> RGB: 13, 13, 13
        assert_eq!(
            Color::RGB(13, 13, 13),
            HSV {
                h: 5.0 / 360.0,
                s: 0.0,
                v: 0.05
            }
            .to_rgb()
        );
        // HSV: 60deg, 50%, 50% -> RGB: 128, 128, 64
        assert_eq!(
            Color::RGB(128, 128, 64),
            HSV {
                h: 60.0 / 360.0,
                s: 0.5,
                v: 0.5
            }
            .to_rgb()
        );
        // HSV: 120deg, 25%, 25% -> RGB: 48, 64, 48
        assert_eq!(
            Color::RGB(48, 64, 48),
            HSV {
                h: 120.0 / 360.0,
                s: 0.25,
                v: 0.25
            }
            .to_rgb()
        );
        // HSV: 360deg, 75%, 75% -> RGB: 191, 48, 48
        assert_eq!(
            Color::RGB(191, 48, 48),
            HSV {
                h: 1.0,
                s: 0.75,
                v: 0.75
            }
            .to_rgb()
        );
        // HSV: 330deg, 100%, 100% -> RGB: 255, 0, 128
        assert_eq!(
            Color::RGB(255, 0, 128),
            HSV {
                h: 330.0 / 360.0,
                s: 1.0,
                v: 1.0
            }
            .to_rgb()
        );
        // HSV: 360deg, 75%, 75% -> RGB: 255, 0, 0
        assert_eq!(
            Color::RGB(255, 0, 0),
            HSV {
                h: 1.0,
                s: 1.0,
                v: 1.0
            }
            .to_rgb()
        );

        // let mut col1 = HSV {
        //     h: 1.0,
        //     s: 0.75,
        //     v: 0.75,
        // }
        // .to_rgb();
        // println!("RGB: {}, {}, {}", col1.r, col1.g, col1.b);
    }

    #[test]
    fn test_color_rgb_to_hsv() {
        // Test data generated in GNU Octave
        // rgb = [13 13 13;48 64 48;191 48 48;255 0 128;255 0 0]
        //rgb
        //   13    13    13
        //   48    64    48
        //  191    48    48
        //  255     0   128
        //  255     0     0
        // hsv = rgb2hsv(rgb./255);
        //hsv =
        //    0.00000   0.00000   0.05098
        //    0.33333   0.25000   0.25098
        //    0.00000   0.74869   0.74902
        //    0.91634   1.00000   1.00000
        //    0.00000   1.00000   1.00000
        // "ideal" hsv
        //    0.00000   0.00000   0.05000
        //    0.33333   0.25000   0.25000
        //    0.00000   0.75000   0.75000
        //    0.91667   1.00000   1.00000
        //    0.00000   1.00000   1.00000

        // HSV: 5, 0%, 5% -> RGB: 13, 13, 13
        let mut hsv: HSV = HSV::from_rgb(Color::RGB(13, 13, 13));
        let mut hsv_ref = HSV {
            h: 0.0,
            s: 0.0,
            v: 0.05098,
        };
        println!("HSV tuple (h,s,v): {}", hsv);
        println!("HSV ref tuple (h,s,v): {}", hsv_ref);
        assert_eq!(hsv, hsv_ref);
        assert_eq!(
            format!("HSV tuple: {}", hsv),
            "HSV tuple: (0.00000, 0.00000, 0.05098)"
        );

        // HSV: 120, 25%, 25% -> RGB: 48, 64, 48
        hsv = HSV::from_rgb(Color::RGB(48, 64, 48));
        hsv_ref = HSV {
            h: 0.33333,
            s: 0.25,
            v: 0.25098,
        };
        println!("HSV tuple (h,s,v): {}", hsv);
        println!("HSV ref tuple (h,s,v): {}", hsv_ref);
        assert_eq!(hsv, hsv_ref);

        // HSV: 360, 75%, 75% -> RGB: 191, 48, 48
        hsv = HSV::from_rgb(Color::RGB(191, 48, 48));
        hsv_ref = HSV {
            h: 0.0,
            s: 0.74869,
            v: 0.74902,
        };
        println!("HSV tuple (h,s,v): {}", hsv);
        println!("HSV ref tuple (h,s,v): {}", hsv_ref);
        assert_eq!(hsv, hsv_ref);

        // HSV: 330, 100%, 100% -> RGB: 255, 0, 128
        hsv = HSV::from_rgb(Color::RGB(255, 0, 128));
        hsv_ref = HSV {
            h: 0.91634,
            s: 1.0,
            v: 1.0,
        };
        println!("HSV tuple (h,s,v): {}", hsv);
        println!("HSV ref tuple (h,s,v): {}", hsv_ref);
        assert_eq!(hsv, hsv_ref);

        // HSV: 360, 75%, 75% -> RGB: 255, 0, 0
        hsv = HSV::from_rgb(Color::RGB(255, 0, 0));
        hsv_ref = HSV {
            h: 0.0,
            s: 1.0,
            v: 1.0,
        };
        println!("HSV tuple (h,s,v): {}", hsv);
        println!("HSV ref tuple (h,s,v): {}", hsv_ref);
        assert_eq!(hsv, hsv_ref);

        // let mut hsv = HSV {
        //     h: 0.0,
        //     s: 0.0,
        //     v: 0.0,
        // };
        //let mut hsv: HSV = HSV::zero();
        //hsv.from_rgb_method(Color::RGB(255, 0, 128));
    }

    #[test]
    fn test_hsv_to_rgb_and_back_to_hsv() {
        // different ways to call implemented trait Convert to_rgb
        let mut s = <HSV as Convert<RGB>>::to_rgb(&HSV {
            h: 5.0 / 360.0,
            s: 0.0,
            v: 0.05,
        });
        println!("{}", s);
        s = Convert::<RGB>::to_rgb(&HSV {
            h: 120.0 / 360.0,
            s: 0.0,
            v: 0.5,
        });
        println!("{}", s);
        s = Convert::to_rgb(&HSV {
            h: 120.0 / 360.0,
            s: 0.4,
            v: 0.5,
        });
        println!("{}", s);
        s = HSV::to_rgb(&HSV {
            h: 120.0 / 360.0,
            s: 0.5,
            v: 0.5,
        });
        println!("{}", s);

        // Convert to rgb and back to hsv
        let hsv1: HSV = HSV {
            h: 120.0 / 360.0,
            s: 0.5,
            v: 0.5,
        };
        let rgb: RGB = hsv1.to_rgb();
        println!("{}", hsv1);
        let hsv2 = HSV::from_rgb(rgb);
        println!("{}", hsv2);
        assert_eq!(hsv1, hsv2);

        let hsv1: HSV = HSV {
            h: 330.0 / 360.0,
            s: 1.0,
            v: 1.0,
        };
        let rgb: RGB = hsv1.to_rgb();
        println!("{}", hsv1);
        let hsv2 = HSV::from_rgb(rgb);
        println!("{}", hsv2);
        assert_eq!(hsv1, hsv2);
    }
}
