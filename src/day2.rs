pub fn day2a_exec(mut mem: Vec<usize>, ip: usize) -> (Vec<usize>, Option<usize>) {
    let opcode = mem[ip];
    match opcode {
        1 => {
            let out_addr = mem[ip + 3];
            mem[out_addr] = mem[mem[ip + 1]] + mem[mem[ip + 2]];
            (mem, Some(ip + 4))
        }
        2 => {
            let out_addr = mem[ip + 3];
            mem[out_addr] = mem[mem[ip + 1]] * mem[mem[ip + 2]];
            (mem, Some(ip + 4))
        }
        99 => (mem, None),
        _ => unreachable!(),
    }
}

pub fn day2a(mut mem: Vec<usize>) -> Vec<usize> {
    let mut ip = 0;
    loop {
        match day2a_exec(mem, ip) {
            (next_mem, Some(next_ip)) => {
                mem = next_mem;
                ip = next_ip;
            }
            (next_mem, None) => return next_mem,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    const DAY2_INPUT: &[usize] = &[
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 2, 23, 6, 27,
        1, 27, 5, 31, 2, 6, 31, 35, 1, 5, 35, 39, 2, 39, 9, 43, 1, 43, 5, 47, 1, 10, 47, 51, 1, 51,
        6, 55, 1, 55, 10, 59, 1, 59, 6, 63, 2, 13, 63, 67, 1, 9, 67, 71, 2, 6, 71, 75, 1, 5, 75,
        79, 1, 9, 79, 83, 2, 6, 83, 87, 1, 5, 87, 91, 2, 6, 91, 95, 2, 95, 9, 99, 1, 99, 6, 103, 1,
        103, 13, 107, 2, 13, 107, 111, 2, 111, 10, 115, 1, 115, 6, 119, 1, 6, 119, 123, 2, 6, 123,
        127, 1, 127, 5, 131, 2, 131, 6, 135, 1, 135, 2, 139, 1, 139, 9, 0, 99, 2, 14, 0, 0,
    ];

    #[test]
    fn day2() {
        assert_eq!(
            super::day2a_exec(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 0),
            (vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], Some(4))
        );
        assert_eq!(
            super::day2a_exec(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], 4),
            (vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], Some(8))
        );
        assert_eq!(
            super::day2a_exec(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], 8),
            (vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], None)
        );

        assert_eq!(
            super::day2a(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[bench]
    fn day2a(b: &mut Bencher) {
        b.iter(|| {
            let mut day2_input = DAY2_INPUT.to_vec();
            day2_input[1] = 12;
            day2_input[2] = 2;

            assert_eq!(super::day2a(day2_input)[0], 5866663);
        })
    }

    #[bench]
    fn day2b(b: &mut Bencher) {
        b.iter(|| {
            for x in 0..99 {
                for y in 0..99 {
                    let mut day2_input = DAY2_INPUT.to_vec();
                    day2_input[1] = x;
                    day2_input[2] = y;
                    if super::day2a(day2_input)[0] == 19690720 {
                        assert_eq!((x, y), (42, 59));
                        assert_eq!(100 * x + y, 4259);
                        return;
                    }
                }
            }
            unreachable!();
        })
    }
}
