pub fn day4a_has_double(mem: &[u8]) -> bool {
    mem.windows(2).any(|w| w[0] == w[1])
}
pub fn day4a_has_double_b(mem: &[u8]) -> bool {
    let l = mem.len();
    mem.windows(2).enumerate().any(|(i, w)| {
        w[0] == w[1]
            && (i == 0 || w[0] != mem[i - 1])
            && (i + 2 == l || w[1] != mem[i + 2])
    })
}
pub fn day4a_non_decreasing(mem: &[u8]) -> bool {
    mem.windows(2).all(|w| w[0] <= w[1])
}

pub fn day4a(from: usize, to: usize, a: bool) -> usize {
    let mut cnt = 0;
    for d0 in 0..=9 {
        for d1 in d0..=9 {
            for d2 in d1..=9 {
                for d3 in d2..=9 {
                    for d4 in d3..=9 {
                        for d5 in d4..=9 {
                            let digits = &[d0, d1, d2, d3, d4, d5];
                            let double_check = if a {
                                day4a_has_double(digits)
                            } else {
                                day4a_has_double_b(digits)
                            };
                            if double_check {
                                let n = digits.iter().fold(0, |a, &c| a * 10 + c as usize);
                                if n >= from && n <= to {
                                    cnt += 1
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    const DAY4_INPUT: &[usize] = &[136760, 595730];

    #[test]
    fn day4() {
        assert_eq!(super::day4a_has_double(&[1, 1, 1, 1, 1, 1, 1]), true);
        assert_eq!(super::day4a_has_double(&[1, 1, 1, 1, 1, 2, 2]), true);
        assert_eq!(super::day4a_has_double(&[1, 2, 3, 7, 8, 9]), false);

        assert_eq!(super::day4a_has_double_b(&[1, 1, 1, 1, 1, 1, 1]), false);
        assert_eq!(super::day4a_has_double_b(&[1, 1, 1, 1, 1, 2, 2]), true);
        assert_eq!(super::day4a_has_double_b(&[2, 2, 1, 1, 1, 1, 1]), true);
        assert_eq!(super::day4a_has_double_b(&[1, 1, 2, 2, 1, 1, 1]), true);
        assert_eq!(super::day4a_has_double_b(&[1, 2, 3, 4, 4, 4]), false);

        assert_eq!(super::day4a_non_decreasing(&[1, 2, 3, 7, 8, 9]), true);
        assert_eq!(super::day4a_non_decreasing(&[1, 2, 3, 7, 9, 8]), false);
    }

    #[bench]
    fn day4a(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(super::day4a(DAY4_INPUT[0], DAY4_INPUT[1], true), 1873);
        })
    }

    #[bench]
    fn day4b(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(super::day4a(DAY4_INPUT[0], DAY4_INPUT[1], false), 1264);
        })
    }
}
