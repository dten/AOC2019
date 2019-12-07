use std::collections::VecDeque;
use std::io::prelude::*;

pub fn day7a_exec(
    mut mem: Vec<isize>,
    ip: usize,
    inputs: &mut VecDeque<isize>,
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
            let input = inputs.pop_front().expect("no input");
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

pub fn day7a(mut mem: Vec<isize>, mut input: VecDeque<isize>) -> (Vec<isize>, String) {
    let mut ip = 0;
    let mut out = vec![];
    loop {
        match day7a_exec(mem, ip, &mut input, out) {
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
    fn day7() {
        {
            let prog = vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ];
            let settings = &[0, 1, 2, 3, 4];
            let mut v = 0;
            for i in 0..5 {
                v = super::day7a(prog.clone(), vec![settings[i], v].into())
                    .1
                    .trim()
                    .parse()
                    .unwrap();
            }
            assert_eq!(v, 54321);
        }
        {
            let prog = vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ];
            let settings = &[4, 3, 2, 1, 0];
            let mut v = 0;
            for i in 0..5 {
                v = super::day7a(prog.clone(), vec![settings[i], v].into())
                    .1
                    .trim()
                    .parse()
                    .unwrap();
            }
            assert_eq!(v, 43210);
        }
        {
            let prog = vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ];
            let settings = &[1, 0, 4, 3, 2];
            let mut v = 0;
            for i in 0..5 {
                v = super::day7a(prog.clone(), vec![settings[i], v].into())
                    .1
                    .trim()
                    .parse()
                    .unwrap();
            }
            assert_eq!(v, 65210);
        }
    }

    #[bench]
    fn day7a(b: &mut Bencher) {
        b.iter(|| {
            let mut max = isize::min_value();
            for x0 in 0..5 {
                for x1 in 0..5 {
                    for x2 in 0..5 {
                        for x3 in 0..5 {
                            'inner: for x4 in 0..5 {
                                let settings = vec![x0, x1, x2, x3, x4];
                                for i in 0..5 {
                                    if settings.contains(&i) == false {
                                        continue 'inner;
                                    }
                                }

                                let mut v = 0;
                                for i in 0..5 {
                                    v = super::day7a(
                                        DAY7_INPUT.to_vec(),
                                        vec![settings[i], v].into(),
                                    )
                                    .1
                                    .trim()
                                    .parse()
                                    .unwrap();
                                }
                                if v > max {
                                    max = v;
                                }
                            }
                        }
                    }
                }
            }
            assert_eq!(max, 92663);
        })
    }

    #[bench]
    fn day7b(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                super::day7a(DAY7_INPUT.to_vec(), vec![5].into()).1,
                "3892695 ".to_string()
            );
        })
    }

    const DAY7_INPUT: &[isize] = &[
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 34, 47, 72, 81, 102, 183, 264, 345, 426, 99999, 3,
        9, 102, 5, 9, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9, 1002, 9, 3, 9, 4, 9, 99, 3,
        9, 102, 3, 9, 9, 101, 2, 9, 9, 102, 5, 9, 9, 1001, 9, 3, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9,
        101, 5, 9, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 1002, 9, 5, 9, 101, 4, 9, 9, 102, 2, 9, 9, 4,
        9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9,
        1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9,
        99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9,
        99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9,
        99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1,
        9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
        3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101,
        2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99,
    ];
}
