use std::collections::VecDeque;
use std::io::prelude::*;

pub fn day7a_exec(
    mut mem: Vec<isize>,
    ip: usize,
    inputs: &mut VecDeque<isize>,
    mut out: Vec<u8>,
) -> (Vec<isize>, Doneness, Vec<u8>) {
    use Doneness::*;

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
            (mem, Running(ip + 4), out)
        }
        2 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            let op2 = mem[ip + 3] as usize;
            mem[op2] = op0 * op1;
            (mem, Running(ip + 4), out)
        }
        3 => {
            let op0 = mem[ip + 1] as usize;
            let input = match inputs.pop_front() {
                Some(i) => i,
                None => return (mem, Interrupt(ip), out),
            };
            mem[op0] = input;
            (mem, Running(ip + 2), out)
        }
        4 => {
            let (_, m0) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            write!(&mut out, "{} ", op0).unwrap();
            (mem, Running(ip + 2), out)
        }
        5 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_omodes, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            if op0 != 0 {
                (mem, Running(op1 as usize), out)
            } else {
                (mem, Running(ip + 3), out)
            }
        }
        6 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_omodes, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            if op0 == 0 {
                (mem, Running(op1 as usize), out)
            } else {
                (mem, Running(ip + 3), out)
            }
        }
        7 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_omodes, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            let op2 = mem[ip + 3] as usize;
            mem[op2] = if op0 < op1 { 1 } else { 0 };
            (mem, Running(ip + 4), out)
        }
        8 => {
            let (omodes, m0) = mode_pls(omodes);
            let (_omodes, m1) = mode_pls(omodes);
            let op0 = val_pls(m0, &mem, ip + 1);
            let op1 = val_pls(m1, &mem, ip + 2);
            let op2 = mem[ip + 3] as usize;
            mem[op2] = if op0 == op1 { 1 } else { 0 };
            (mem, Running(ip + 4), out)
        }
        99 => (mem, Halt, out),
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

#[derive(Debug)]
pub enum Doneness {
    Halt,
    Interrupt(usize),
    Running(usize),
}

pub fn day7a(
    mut mem: Vec<isize>,
    start: usize,
    mut input: VecDeque<isize>,
) -> (Vec<isize>, Doneness, String) {
    use Doneness::*;

    let mut ip = start;
    let mut out = vec![];
    loop {
        match day7a_exec(mem, ip, &mut input, out) {
            (next_mem, Running(next_ip), next_out) => {
                mem = next_mem;
                ip = next_ip;
                out = next_out;
            }
            (final_mem, ip, final_out) => {
                return (
                    final_mem,
                    ip,
                    String::from_utf8_lossy(&final_out).to_string(),
                )
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
                v = super::day7a(prog.clone(), 0, vec![settings[i], v].into())
                    .2
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
                v = super::day7a(prog.clone(), 0, vec![settings[i], v].into())
                    .2
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
                v = super::day7a(prog.clone(), 0, vec![settings[i], v].into())
                    .2
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
                                let settings = &[x0, x1, x2, x3, x4];
                                for i in 0..5 {
                                    if settings.contains(&i) == false {
                                        continue 'inner;
                                    }
                                }

                                let mut v = 0;
                                for i in 0..5 {
                                    v = super::day7a(
                                        DAY7_INPUT.to_vec(),
                                        0,
                                        vec![settings[i], v].into(),
                                    )
                                    .2
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
            let mut max = isize::min_value();
            for x0 in 5..10 {
                for x1 in 5..10 {
                    for x2 in 5..10 {
                        for x3 in 5..10 {
                            'inner: for x4 in 5..10 {
                                use super::Doneness::*;
                                let settings = &[x0, x1, x2, x3, x4];
                                for i in 5..10 {
                                    if settings.contains(&i) == false {
                                        continue 'inner;
                                    }
                                }
                                let mut a0 = DAY7_INPUT.to_vec();
                                let mut a1 = DAY7_INPUT.to_vec();
                                let mut a2 = DAY7_INPUT.to_vec();
                                let mut a3 = DAY7_INPUT.to_vec();
                                let mut a4 = DAY7_INPUT.to_vec();
                                let accs = &mut [&mut a0, &mut a1, &mut a2, &mut a3, &mut a4];
                                let ips = &mut [
                                    Running(0),
                                    Running(0),
                                    Running(0),
                                    Running(0),
                                    Running(0),
                                ];
                                let mut v = 0;
                                for i in 0..5 {
                                    let ip = match ips[i] {
                                        Halt => break,
                                        Interrupt(ip) => ip,
                                        Running(ip) => ip,
                                    };
                                    let (mem, ip, o) = super::day7a(
                                        accs[i].clone(),
                                        ip,
                                        vec![settings[i], v].into(),
                                    );
                                    *accs[i] = mem;
                                    ips[i] = ip;
                                    v = o.trim().parse().unwrap();
                                }
                                'outer: loop {
                                    for i in 0..5 {
                                        let ip = match ips[i] {
                                            Halt => break 'outer,
                                            Interrupt(ip) => ip,
                                            Running(ip) => ip,
                                        };
                                        let (mem, ip, o) =
                                            super::day7a(accs[i].clone(), ip, vec![v].into());
                                        *accs[i] = mem;
                                        ips[i] = ip;
                                        v = o.trim().parse().unwrap();
                                    }
                                }
                                if v > max {
                                    max = v;
                                }
                            }
                        }
                    }
                }
            }
            assert_eq!(max, 139629729);
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
