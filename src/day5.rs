use std::io::prelude::*;

pub fn day5a_exec(
    mut mem: Vec<isize>,
    ip: usize,
    input: isize,
    mut out: Vec<u8>,
) -> (Vec<isize>, Option<usize>, Vec<u8>) {
    let instr = mem[ip];
    let opcode = instr % 100;
    let omodes = instr / 100;
    match opcode {
        1 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            let op2 = mem[ip + 3] as usize;
            mem[op2] = op0 + op1;
            (mem, Some(ip + 4), out)
        }
        2 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            let op2 = mem[ip + 3] as usize;
            mem[op2] = op0 * op1;
            (mem, Some(ip + 4), out)
        }
        3 => {
            let op0 = mem[ip + 1] as usize;
            mem[op0] = input;
            (mem, Some(ip + 2), out)
        }
        4 => {
            let (_, m0) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            write!(&mut out, "{} ", op0).unwrap();
            (mem, Some(ip + 2), out)
        }
        5 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_omodes, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            if op0 != 0 {
                (mem, Some(op1 as usize), out)
            } else {
                (mem, Some(ip + 3), out)
            }
        }
        6 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_omodes, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            if op0 == 0 {
                (mem, Some(op1 as usize), out)
            } else {
                (mem, Some(ip + 3), out)
            }
        }
        7 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_omodes, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            let op2 = mem[ip + 3] as usize;
            mem[op2] = if op0 < op1 { 1 } else { 0 };
            (mem, Some(ip + 4), out)
        }
        8 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_omodes, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            let op2 = mem[ip + 3] as usize;
            mem[op2] = if op0 == op1 { 1 } else { 0 };
            (mem, Some(ip + 4), out)
        }
        99 => (mem, None, out),
        _ => unreachable!(format!("bad opcode {}", opcode)),
    }
}

fn mode_pls(omodes: isize) -> (isize, isize) {
    let mode = omodes % 10;
    let quot = omodes / 10;
    (quot, mode)
}

fn val_pls(mode: isize, mem: &Vec<isize>, pos: usize) -> isize {
    let v = mem[pos];
    match mode {
        0 => mem[v as usize],
        1 => v,
        _ => unreachable!(format!("bad mode {}", mode)),
    }
}

pub fn day5a(mut mem: Vec<isize>, input: isize) -> (Vec<isize>, String) {
    let mut ip = 0;
    let mut out = vec![];
    loop {
        match day5a_exec(mem, ip, input, out) {
            (next_mem, Some(next_ip), next_out) => {
                mem = next_mem;
                ip = next_ip;
                out = next_out;
            }
            (final_mem, None, final_out) => {
                return (final_mem, String::from_utf8_lossy(&final_out).to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    #[test]
    fn day5() {
        assert_eq!(
            super::day5a_exec(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 0, 1, vec![]),
            (
                vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                Some(4),
                vec![]
            )
        );
        assert_eq!(
            super::day5a_exec(
                vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                4,
                1,
                vec![]
            ),
            (
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                Some(8),
                vec![]
            )
        );
        assert_eq!(
            super::day5a_exec(
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                8,
                1,
                vec![]
            ),
            (
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                None,
                vec![]
            )
        );

        assert_eq!(
            super::day5a(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 1),
            (
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                "".to_string()
            )
        );

        assert_eq!(
            super::day5a_exec(vec![104, 5, 99], 0, 1, vec![]),
            (vec![104, 5, 99], Some(2), vec![b'5', b' '])
        );
        assert_eq!(
            super::day5a_exec(vec![3, 3, 104, 0, 99], 0, 1, vec![]),
            (vec![3, 3, 104, 1, 99], Some(2), vec![])
        );
        assert_eq!(
            super::day5a_exec(vec![3, 3, 104, 1, 99], 2, 1, vec![]),
            (vec![3, 3, 104, 1, 99], Some(4), vec![b'1', b' '])
        );

        assert_eq!(
            super::day5a(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 1).1,
            "0 "
        );
        assert_eq!(
            super::day5a(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8).1,
            "1 "
        );
        assert_eq!(
            super::day5a(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 1).1,
            "1 "
        );
        assert_eq!(
            super::day5a(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8).1,
            "0 "
        );
        assert_eq!(
            super::day5a(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 1).1,
            "0 "
        );
        assert_eq!(
            super::day5a(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 1).1,
            "1 "
        );
    }

    #[bench]
    fn day5_2a(b: &mut Bencher) {
        b.iter(|| {
            let mut day2_input = DAY2_INPUT.to_vec();
            day2_input[1] = 12;
            day2_input[2] = 2;

            assert_eq!(super::day5a(day2_input, 1).0[0], 5866663);
        })
    }

    #[bench]
    fn day5_2b(b: &mut Bencher) {
        b.iter(|| {
            for x in 0..99 {
                for y in 0..99 {
                    let mut day2_input = DAY2_INPUT.to_vec();
                    day2_input[1] = x;
                    day2_input[2] = y;
                    if super::day5a(day2_input, 1).0[0] == 19690720 {
                        assert_eq!((x, y), (42, 59));
                        assert_eq!(100 * x + y, 4259);
                        return;
                    }
                }
            }
            unreachable!("no answer");
        })
    }

    #[bench]
    fn day5a(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                super::day5a(DAY5_INPUT.to_vec(), 1).1,
                "0 0 0 0 0 0 0 0 0 13787043 ".to_string()
            );
        })
    }

    #[bench]
    fn day5b(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                super::day5a(DAY5_INPUT.to_vec(), 5).1,
                "3892695 ".to_string()
            );
        })
    }

    const DAY2_INPUT: &[isize] = &[
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 2, 23, 6, 27,
        1, 27, 5, 31, 2, 6, 31, 35, 1, 5, 35, 39, 2, 39, 9, 43, 1, 43, 5, 47, 1, 10, 47, 51, 1, 51,
        6, 55, 1, 55, 10, 59, 1, 59, 6, 63, 2, 13, 63, 67, 1, 9, 67, 71, 2, 6, 71, 75, 1, 5, 75,
        79, 1, 9, 79, 83, 2, 6, 83, 87, 1, 5, 87, 91, 2, 6, 91, 95, 2, 95, 9, 99, 1, 99, 6, 103, 1,
        103, 13, 107, 2, 13, 107, 111, 2, 111, 10, 115, 1, 115, 6, 119, 1, 6, 119, 123, 2, 6, 123,
        127, 1, 127, 5, 131, 2, 131, 6, 135, 1, 135, 2, 139, 1, 139, 9, 0, 99, 2, 14, 0, 0,
    ];

    const DAY5_INPUT: &[isize] = &[
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 9, 19, 225, 1, 136, 139, 224, 101,
        -17, 224, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223, 2, 218, 213,
        224, 1001, 224, -4560, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224, 1, 223, 224, 223,
        1102, 25, 63, 224, 101, -1575, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224, 1,
        223, 224, 223, 1102, 55, 31, 225, 1101, 38, 15, 225, 1001, 13, 88, 224, 1001, 224, -97,
        224, 4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 224, 223, 223, 1002, 87, 88, 224, 101,
        -3344, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7, 224, 1, 224, 223, 223, 1102, 39,
        10, 225, 1102, 7, 70, 225, 1101, 19, 47, 224, 101, -66, 224, 224, 4, 224, 1002, 223, 8,
        223, 1001, 224, 6, 224, 1, 224, 223, 223, 1102, 49, 72, 225, 102, 77, 166, 224, 101, -5544,
        224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224, 1, 223, 224, 223, 101, 32, 83, 224,
        101, -87, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 1101,
        80, 5, 225, 1101, 47, 57, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1,
        99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 1008, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 329, 1001, 223, 1, 223,
        107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 1007, 677, 677,
        224, 1002, 223, 2, 223, 1006, 224, 359, 1001, 223, 1, 223, 8, 677, 226, 224, 102, 2, 223,
        223, 1005, 224, 374, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1006, 224,
        389, 1001, 223, 1, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 404, 1001, 223,
        1, 223, 1107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 419, 1001, 223, 1, 223, 1008,
        226, 226, 224, 102, 2, 223, 223, 1005, 224, 434, 101, 1, 223, 223, 8, 226, 677, 224, 1002,
        223, 2, 223, 1006, 224, 449, 101, 1, 223, 223, 1007, 677, 226, 224, 102, 2, 223, 223, 1005,
        224, 464, 1001, 223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 479, 1001,
        223, 1, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 494, 1001, 223, 1, 223, 7,
        677, 677, 224, 102, 2, 223, 223, 1006, 224, 509, 101, 1, 223, 223, 1007, 226, 226, 224,
        1002, 223, 2, 223, 1005, 224, 524, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223,
        1005, 224, 539, 101, 1, 223, 223, 8, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 554, 101,
        1, 223, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 569, 101, 1, 223, 223, 1108,
        677, 226, 224, 1002, 223, 2, 223, 1005, 224, 584, 101, 1, 223, 223, 108, 677, 677, 224,
        1002, 223, 2, 223, 1006, 224, 599, 101, 1, 223, 223, 107, 226, 226, 224, 1002, 223, 2, 223,
        1006, 224, 614, 101, 1, 223, 223, 1108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 629,
        1001, 223, 1, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 644, 101, 1, 223,
        223, 108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 659, 101, 1, 223, 223, 1108, 226,
        677, 224, 1002, 223, 2, 223, 1005, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
    ];
}
