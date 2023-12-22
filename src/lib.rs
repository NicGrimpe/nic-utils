#![no_std]

use core::fmt::Error;

pub fn average(new_sample: u16, last_sample: u16, step: u8) -> Result<(f32, f32, f32, f32), Error> {    
    let mut first : f32 = 0.0;
    let mut second: f32 = 0.0;
    let mut third: f32 = 0.0;
    let mut fourth: f32 = 0.0;

    match step {
        0 => {
            first = f32::from(((new_sample >> 15) & 1) + ((new_sample >> 14) & 1) + ((new_sample >> 13) & 1) + ((new_sample >> 12) & 1) + ((new_sample >> 11) & 1)) / 5.0;
            second = f32::from(((new_sample >> 10) & 1) + ((new_sample >> 9) & 1) + ((new_sample >> 8) & 1) + ((new_sample >> 7) & 1) + ((new_sample >> 6) & 1)) / 5.0;
            third = f32::from(((new_sample >> 5) & 1) + ((new_sample >> 4) & 1) + ((new_sample >> 3) & 1) + ((new_sample >> 2) & 1) + ((new_sample >> 1) & 1)) / 5.0;
        }
        1 => {
            first = f32::from((last_sample & 1) + ((new_sample >> 15) & 1) + ((new_sample >> 14) & 1) + ((new_sample >> 13) & 1) + ((new_sample >> 12) & 1)) / 5.0;
            second = f32::from(((new_sample >> 11) & 1) + ((new_sample >> 10) & 1) + ((new_sample >> 9) & 1) + ((new_sample >> 8) & 1) + ((new_sample >> 7) & 1)) / 5.0;
            third = f32::from(((new_sample >> 6) & 1) + ((new_sample >> 5) & 1) + ((new_sample >> 4) & 1) + ((new_sample >> 3) & 1) + ((new_sample >> 2) & 1)) / 5.0;
        }
        2 => {
            first = f32::from(((last_sample >> 1) & 1) + (last_sample & 1) + ((new_sample >> 15) & 1) + ((new_sample >> 14) & 1) + ((new_sample >> 13) & 1)) / 5.0;
            second = f32::from(((new_sample >> 12) & 1) + ((new_sample >> 11) & 1) + ((new_sample >> 10) & 1) + ((new_sample >> 9) & 1) + ((new_sample >> 8) & 1)) / 5.0;
            third = f32::from(((new_sample >> 7) & 1) + ((new_sample >> 6) & 1) + ((new_sample >> 5) & 1) + ((new_sample >> 4) & 1) + ((new_sample >> 3) & 1)) / 5.0;
        }
        3 => {
            first = f32::from(((last_sample >> 2) & 1) + ((last_sample >> 1) & 1) + (last_sample & 1) + ((new_sample >> 15) & 1) + ((new_sample >> 14) & 1)) / 5.0;
            second = f32::from(((new_sample >> 13) & 1) + ((new_sample >> 12) & 1) + ((new_sample >> 11) & 1) + ((new_sample >> 10) & 1) + ((new_sample >> 9) & 1)) / 5.0;
            third = f32::from(((new_sample >> 8) & 1) + ((new_sample >> 7) & 1) + ((new_sample >> 6) & 1) + ((new_sample >> 5) & 1) + ((new_sample >> 4) & 1)) / 5.0;
        }
        4 => {
            first = f32::from(((last_sample >> 3) & 1) + ((last_sample >> 2) & 1) + ((last_sample >> 1) & 1) + (last_sample & 1) + ((new_sample >> 15) & 1)) / 5.0;
            second = f32::from(((new_sample >> 14) & 1) + ((new_sample >> 13) & 1) + ((new_sample >> 12) & 1) + ((new_sample >> 11) & 1) + ((new_sample >> 10) & 1)) / 5.0;
            third = f32::from(((new_sample >> 9) & 1) + ((new_sample >> 8) & 1) + ((new_sample >> 7) & 1) + ((new_sample >> 6) & 1) + ((new_sample >> 5) & 1)) / 5.0;
            fourth = f32::from(((new_sample >> 4) & 1) + ((new_sample >> 3) & 1) + ((new_sample >> 2) & 1) + ((new_sample >> 1) & 1) + ((new_sample & 1))) / 5.0;
        }
        _ => {}
    }
    
    return Result::Ok((first, second, third, fourth))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::average;

        let mut new_sample = 0b1111101110000001;
        let mut old_sample = 0b0000000000000000;
        let mut step = 0;
        let mut first : f32 = 0.0;
        let mut second: f32 = 0.0;
        let mut third: f32 = 0.0;
        let mut fourth: f32 = 0.0;

        match average(new_sample, old_sample, step) {
            Ok((s1, s2, s3, s4)) => {
                first = s1;
                second = s2;
                third = s3;
                fourth = s4;
            }
            _ => {

            }
        }
        assert_eq!(first, 1.0);
        assert_eq!(second, 0.6);
        assert_eq!(third, 0.0);
        assert_eq!(fourth, 0.0);

        old_sample = new_sample;
        new_sample = 0b0000101110001001;
        step = 1;

        match average(new_sample, old_sample, step) {
            Ok((s1, s2, s3, s4)) => {
                first = s1;
                second = s2;
                third = s3;
                fourth = s4;
            }
            _ => {

            }
        }
        assert_eq!(first, 0.2);
        assert_eq!(second, 0.8);
        assert_eq!(third, 0.2);
        assert_eq!(fourth, 0.0);

    }
}