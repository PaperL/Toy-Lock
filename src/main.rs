use std::{
    env,
    io::{stdin, stdout, Write},
};

use rand::{thread_rng, Rng};

fn pos_hash(p: i64, x: i64, k: i64) -> char {
    ((x + p + k) % 10 + 48) as u8 as char
}

fn neg_hash(p: i64, x: i64, k: i64) -> char {
    ((x + p + 10 - k) % 10 + 48) as u8 as char
}

fn pos_hash_back(p: i64, x: i64, k: i64) -> char {
    ((x + 1000 - p - k) % 10 + 48) as u8 as char
}

fn neg_hash_back(p: i64, x: i64, k: i64) -> char {
    ((x + 1000 - p + k) % 10 + 48) as u8 as char
}

fn lock(mut str: String, is_debug: bool) -> String {
    // let x = str.trim().parse::<i64>().unwrap();
    let len = str.len() as i64;

    let mut rng = thread_rng();
    let k = (rng.gen::<i64>().abs()) % 10 % len;
    // 数码和 ≡ k (mod 10)
    // 0 ≤ k ≤ 9

    let mut p = 0;
    let mut step = k % 3 + 1; // step = 1, 2, 3

    if is_debug {
        println!("l1 step = {};", step);
    }

    let mut ret = String::new();
    while p < len {
        // 后续 step 个数字作为整体处理
        if p + step >= len {
            step = len - p;
        }
        let tmp = p as usize;
        let substr = &str[tmp..tmp + (step as usize)];
        match step {
            1 => {
                let y1 = substr.parse::<i64>().unwrap();
                ret.push(pos_hash(p, y1, k));
            }
            2 => {
                let y1 = substr[0..1].parse::<i64>().unwrap();
                let y2 = substr[1..2].parse::<i64>().unwrap();
                ret.push(neg_hash(p + 2, y2, k));
                ret.push(pos_hash(p + 4, y1, k));
            }
            3 => {
                let y1 = substr[0..1].parse::<i64>().unwrap();
                let y2 = substr[1..2].parse::<i64>().unwrap();
                let y3 = substr[2..3].parse::<i64>().unwrap();
                ret.push(neg_hash(p + 3, y3, k));
                ret.push(pos_hash(p + 6, y1, k));
                ret.push(neg_hash(p + 9, y2, k));
            }
            _ => panic!("ouch2"),
        }

        p += step;
        step = if step == 1 { 3 } else { step - 1 };
    }

    let mut sum = 0;
    for i in 0..len {
        let tmp = i as usize;
        sum += ret[tmp..tmp + 1].parse::<i64>().unwrap();
    }
    let e = (10 + k - (sum % 10)) % 10;
    // 在第 (len-k) 位处插入数码 e 作为校验码
    // 0 ≤ e ≤ 9
    if is_debug {
        println!(
            "\nLock\nrst without e = {};\nk = {}; e = {}; step = {}",
            ret, k, e, step
        );
    }
    ret.insert((len - k) as usize, (e + 48) as u8 as char);
    ret
}

fn unlock(mut str: String, is_debug: bool) -> String {
    let mut len = str.len() as i64;

    let mut sum = 0;
    for i in 0..len {
        let it = i as usize;
        sum += str[it..it + 1].parse::<i64>().unwrap();
    }
    let k = sum % 10;
    len -= 1;
    let kt = (len - k) as usize;
    let e = str[kt..kt + 1].parse::<i64>().unwrap();
    str.remove(kt);

    // 数码和 ≡ k (mod 10)
    // 在第 k 位数码 e 为校验码
    // 0 ≤ k,e ≤ 9

    let mut p = 0;
    let mut step = k % 3 + 1; // step = 1, 2, 3
    let mut ret = String::new();
    if is_debug {
        println!("u1 {} {} {} {}", str, k, e, step);
    }
    while p < len {
        // 后续 step 个数字作为整体处理
        if p + step >= len {
            step = len - p;
        }
        let tmp = p as usize;
        let substr = &str[tmp..tmp + (step as usize)];
        match step {
            1 => {
                let z1 = substr.parse::<i64>().unwrap();
                ret.push(pos_hash_back(p, z1, k));
            }
            2 => {
                let z1 = substr[0..1].parse::<i64>().unwrap();
                let z2 = substr[1..2].parse::<i64>().unwrap();
                ret.push(pos_hash_back(p + 4, z2, k));
                ret.push(neg_hash_back(p + 2, z1, k));
            }
            3 => {
                let z1 = substr[0..1].parse::<i64>().unwrap();
                let z2 = substr[1..2].parse::<i64>().unwrap();
                let z3 = substr[2..3].parse::<i64>().unwrap();
                ret.push(pos_hash_back(p + 6, z2, k));
                ret.push(neg_hash_back(p + 9, z3, k));
                ret.push(neg_hash_back(p + 3, z1, k));
            }
            _ => panic!("ouch2"),
        }

        p += step;
        step = if step == 1 { 3 } else { step - 1 };
    }

    if is_debug {
        println!(
            "\nUnlock:\nret = {};\nk = {}; e = {}; step = {}",
            ret, k, e, step
        );
    }

    ret
}

fn main() {
    let mut cmd_debug = false;
    let mut cmd_pure = false;
    let mut cmd_lock = false;
    for arg in env::args() {
        if arg == "debug" {
            cmd_debug = true;
        }
        if arg == "l" || arg == "u" {
            cmd_pure = true;
            if arg == "l" {
                cmd_lock = true;
            }
        }
    }

    // print!("{}[2J", 27 as char); // clear screen
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // set cursor at 1,1
    if cmd_pure {
        let mut str = String::new();
        match stdin().read_line(&mut str) {
            Ok(_) => str = str.trim().to_string(),
            Err(_) => panic!("ouch1"),
        }
        println!(
            "{}",
            if cmd_lock {
                lock(str, cmd_debug)
            } else {
                unlock(str, cmd_debug)
            }
        );
    } else {
        print!("Lock or Unlock [L/u]:");
        stdout().flush();

        let mut buf = String::new();
        stdin().read_line(&mut buf);

        let is_lock =
            buf.trim().len() == 0 || buf.chars().nth(0).unwrap().to_ascii_lowercase() == 'l';
        if is_lock {
            print!("Input plaintext:  ");
        } else {
            print!("Input ciphertext: ");
        }
        stdout().flush();

        let mut str = String::new();
        match stdin().read_line(&mut str) {
            Ok(_) => str = str.trim().to_string(),
            Err(_) => panic!("ouch1"),
        }

        // if !is_debug {
        //     if is_lock {
        //         assert_eq!(str.clone(), unlock(lock(str.clone(), false), false));
        //     } else {
        //              有随机数，无法检查 str -> unlock -> lock -> str
        //         assert_eq!(str.clone(), lock(unlock(str.clone(), false), false));
        //     }
        // }

        if is_lock {
            println!("Ciphertext:       {}", lock(str, cmd_debug));
        } else {
            println!("Plaintext:        {}", unlock(str, cmd_debug));
        }
    }
}
