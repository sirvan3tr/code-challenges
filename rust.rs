// // Convert integer to string.
// 
// // Convert string to integer.
// let num_str: String = "42".to_string();
// 
// // Create a mutable empty string
// let mut s: String = String::new();
// 
// // Loop through string
// for c in num_str.chars() {
//     let num_int: u64 = num_str.parse().unwrap();
//     let x: u64 = (num_int ** 2).to_digit(10).unwrap() as u64;
//     s.push(x.to_string());
// }
// 
// 
// let num_int: u64 = s.parse().unwrap();
// 
// // Push a character to the string
// s.push('a');
// 
// // convert string to integer
// let num_int: u64 = num_str.parse().unwrap();

use std::collections::HashMap;

fn square_digits(num: u64) -> u64 {
    // convert the input u64 to a string
    let num_str = num.to_string();

    // iterate over the characters in the string to square each digit and concatenate them
    let mut squared_str = String::new();
    for ch in num_str.chars() {
        // square ch
        let ch_int = ch.to_digit(10).unwrap() as u64;
        let squared_ch = ch_int * ch_int;
        squared_str.push_str(&squared_ch.to_string());
    }


    // convert the resulting string to an u64 and return it
    squared_str.parse::<u64>().unwrap()
 }

 fn square_digits4(num: u64) -> u64 {
    let mut s = num.to_string();
    let mut vec = vec![];
    for c in s.chars() {
        let mut v = c.to_digit(10).unwrap();
        v *= v;
        vec.push(v.to_string());
    }
    s = vec.join("");
    return s.parse().unwrap()
}

fn square_digits2(num: u64) -> u64 {
    num
        .to_string()
        .chars()
        .map(|i| i.to_digit(10).expect("char isnt digit").pow(2).to_string())
        .collect::<String>()
        .parse()
        .expect("result isnt u64 parsable")
}


fn square_digits3(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|c| format!("{}", c.to_digit(10).unwrap().pow(2)))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn create_phone_number(numbers: &[u8]) -> String {
    // create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"
    let mut phone_number = String::new();
    phone_number.push('(');
    for i in 0..numbers.len() {
        phone_number.push_str(&numbers[i].to_string());
        if i == 2 {
            phone_number.push_str(") ");
        }
        if i == 5 {
            phone_number.push_str("-");
        }
    }
    phone_number
}



fn likes3(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        _ => format!("{}, {} and {} others like this", names[0], names[1], names.len() - 2),
    }

}


fn disemvowel(s: &str) -> String {
    const vowels: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let mut new_string = String::new();

    for c in s.chars() {
        if !vowels.contains(&c) {
            new_string.push(c);
        }
    }

    return new_string;

}


// fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
//     let mut seen: i32 = 0;
//     let mut prev_height: f64 = h;
// 
//     if h > 0 && bounce > 0 && bounce < 1 && windows < h {
//         while prev_height > window {
//             seen += 1;
//             prev_height *= bounce;
//             if prev_height > window {
//                 seen += 1;
//             }
//         }
// 
//         return seen;
//     } else {
//         return -1;
//     }
// }

fn time_correct(time_str: &str) -> Option<String> {

    if time_str.len() != 8 {
        return None;
    }

    if time_str.len() == 0 {
        return None;
    }

    // Count the number of colons ':' in the time_str
    let mut colon_count = 0;
    for c in time_str.chars() {
        if c == ':' {
            colon_count += 1;
        }
    }

    if colon_count != 2 {
        return None;
    }

    let mut split_time = time_str
                    .split(':')
                    .map(|x| x.parse::<i32>().unwrap_or_else(|_| -1))
                    .collect::<Vec<i32>>();

    for i in 0..split_time.len() {
        if split_time[i] == -1 {
            return None;
        }
    }


    if split_time[2] > 59 {
        let extramins = split_time[2] / 60;
        let newsecs = split_time[2] % 60;
        split_time[1] += extramins;
        split_time[2] = newsecs;
    }

    if split_time[1] > 59 {
        let extrahours = split_time[1] / 60;
        let newmins = split_time[1] % 60;
        split_time[0] += extrahours;
        split_time[1] = newmins;
    }

    if split_time[0] > 23 {
        let extradays = split_time[0] % 24;
        split_time[0] = extradays;
    }

    return Some(format!("{:02}:{:02}:{:02}", split_time[0], split_time[1], split_time[2]));

}

fn remove_parentheses(s: &str) -> String {
    todo!();
}

fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    vec![]
}

// https://www.codewars.com/kata/587731fda577b3d1b0001196/train/rust
fn camel_case(str: &str) -> String {
    if str == "" {
        return "".to_string();
    }

    let new_string: String = str.to_string()
                    .trim()
                    .split(" ")
                    .map(|x| x[0..1].to_uppercase() + &x[1..])
                    .collect::<Vec<String>>()
                    .join("");
    return new_string;
}

// https://www.codewars.com/kata/52597aa56021e91c93000cb0/solutions/rust
fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut new_arr: Vec<u8> = Vec::new();
    let mut zeros: Vec<u8> = Vec::new();

    for i in 0..arr.len() {
        if arr[i] == 0 {
            zeros.push(0);
        } else {
            new_arr.push(arr[i]);
        }
    }

    new_arr.append(&mut zeros);
    return new_arr;
}


// fn sum_pairs(array: &[i32], sum: i32) -> Option<(i32, i32)> {
//     let mut seen = HashMap::new();
//     for (i, &value) in array.iter().enumerate() {
//         if let Some(_j) = seen.get(&(sum - value)) {
//             return Some((sum - value, value));
//         }
//         seen.insert(value, i);
//     }
//     None
// }
// 
// fn sum_pairs2(ints: &[i8], s: i8) -> Option<(i8, i8)> {
//     let mut seen = HashSet::new();
//     for &i in ints {
//         if seen.contains(&(s - i)) {
//             return Some((s - i, i));
//         }
//         seen.insert(i);
//     }
//     None
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn direction_to_vector(direction: Direction) -> Vec<i8> {
    match direction {
        Direction::North => vec![0, 1],
        Direction::East => vec![1, 0],
        Direction::West => vec![-1, 0],
        Direction::South => vec![0, -1],
    }
}

fn dir_reduc2(arr: &[Direction]) -> Vec<Direction> {
    let mut vec_sum: Vec<i8> = vec![0, 0];
    let mut new_dir: Vec<Direction> = vec![];

    for &d in arr {
        let v = direction_to_vector(d);
        vec_sum = vec_sum
                    .iter()
                    .zip(v.iter())
                    .map(|(x, y)| x + y)
                    .collect();
        println!("{:?} {:?}", v, vec_sum);
    }

    let mut closest_vec = vec![0, 0];
    let mut closest_dist = closest_vec
                            .iter()
                            .zip(vec_sum.iter())
                            .map(|(x, y)| (x - y).pow(2))
                            .sum::<i8>();

    for &d in arr {
        let v = direction_to_vector(d);
        let dist = v
                    .iter()
                    .zip(vec_sum.iter())
                    .map(|(x, y)| (x - y).pow(2))
                    .sum::<i8>();
        if dist < closest_dist {
            closest_dist = dist;
            closest_vec = v;
            println!("{} {:?} {:?}", dist, closest_dist, d);
            new_dir.push(d);
        }
    }
    new_dir
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut newarr = arr.to_vec();
    let mut index = 0;
    let mut itemremoved = false;
    while true {
        let v1 = direction_to_vector(newarr[index]);
        let mut v2 = vec![0, 0];

        if index + 1 < newarr.len() {
            v2 = direction_to_vector(newarr[index + 1]);
        }

        let thesum: Vec<i8> = v1
                    .iter()
                    .zip(v2.iter())
                    .map(|(x, y)| x + y)
                    .collect();

        if thesum == vec![0, 0] {
            newarr.remove(index);
            newarr.remove(index);
            index = 0;
        } else {
            index += 1;
        }

        if index >= newarr.len() && !itemremoved {
            break;
        }
    }
    newarr
}

// use std::collections::HashSet;
fn find_the_number_plate(n: u32) -> String {
    // list of alphabets efficient memory
    let alphabet: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    // Indeces of each position
    let mut i: [usize; 3] = [0, 0, 0];
    let mut sum: u32 = 0;

    while true {
        let mut s = String::new();
        if n <= sum + 999 {
            // get the third element of alphabet
            s.push(*alphabet.iter().nth(i[0]).unwrap());
            s.push(*alphabet.iter().nth(i[1]).unwrap());
            s.push(*alphabet.iter().nth(i[2]).unwrap());

            if (n - sum).to_string().chars().count() < 3 {
                for k in 0..(3 - (n - sum + 1).to_string().chars().count()) {
                    s.push('0');
                }
            }

            s.push_str(&(n - sum + 1).to_string());
            return s;
        }

        let mut j = 0;
        while true {
            if *alphabet.iter().nth(i[j]).unwrap() == 'z' {
                i[j] = 0;
                j += 1;
            } else {
                i[j] += 1;
                break;
            }
        }
        sum += 999;
    }
    return "aaa".to_string();
}

fn find_the_number_plate2(n: u32) -> String {
    let (n, d) = (n / 999, 1 + n % 999);
    let (n, r) = (n / 26, (b'a' + (n % 26) as u8) as char);
    let (n, m) = (n / 26, (b'a' + (n % 26) as u8) as char);
    let l = (b'a' + (n % 26) as u8) as char;
    format!("{r}{m}{l}{d:03}")
}

fn find_the_number_plate3(n: u32) -> String {
    let num_letters: u32 = n / 999;
    let remaining_num = (n % 999) + 1;

    const A: u32 = 'a' as u32;

    let letters: (char, char, char) = (
        char::from_u32(A + num_letters % 26).unwrap(),
        char::from_u32(A + num_letters / 26 % 26 ).unwrap(),
        char::from_u32(A + num_letters / 26 / 26 % 26).unwrap(),
    );

    let formatted_letters = format!("{}{}{}", letters.0, letters.1, letters.2);
    let formatted_numbers = format!(
        "{}{}",
        "0".repeat(3 - remaining_num.to_string().len()),
        remaining_num
    );

    format!("{}{}", formatted_letters, formatted_numbers)
}

fn heron(a: f64, b: f64, c: f64) -> f64 {
    let s = (a + b + c) / 2.0;
    return (s * (s - a) * (s - b) * (s - c)).sqrt();
}

fn high_and_low(numbers: &str) -> String {
    let (min, max) = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((std::i32::MAX, std::i32::MIN), |(min, max), x| (min.min(x), max.max(x)));
    format!("{} {}", max, min)
}

fn dna_strand(dna: &str) -> String {
    let mut newdna = String::new();
    for d in dna.chars() {
        newdna.push_str(match d {
            'A' => "T",
            'T' => "A",
            'C' => "G",
            'G' => "C",
            _ => "",
        });
    }
    newdna
}

fn dna_strand2(dna: &str) -> String {
    dna
        .chars()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => panic!("Unexpected DNA instruction {}", c),
        })
        .collect()
}

fn zeros(n: u64) -> u64 {
    // https://mathworld.wolfram.com/Factorial.html
    let kmax = (n as f64).log(5.0).floor() as u64;
    (1..=kmax).map(|k| n / 5u64.pow(k as u32)).sum()
}

fn zeros2(n: u64) -> u64 {

    (1..)
        .map(|exp| n / 5_u64.pow(exp))
        .take_while(|&x| x != 0)
        .sum()

}
fn rot13(message: &str) -> String {
    // Create a ROT13 cipher mapping
    let mut rot13 = HashMap::new();
    for i in 0..26 {
        rot13.insert((b'a' + i) as char, (b'a' + (i + 13) % 26) as char);
        rot13.insert((b'A' + i) as char, (b'A' + (i + 13) % 26) as char);
    }
    let mut res: String = String::new();
    message.chars().for_each(|c| {
        res.push(*rot13.get(&c).unwrap_or(&c));
    });
    res
}

fn rot132(message: &str) -> String {
    message.chars().map(|c| {
        match c {
            'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
            'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
            _ => c,
        }
    }).collect()
}

fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return "now".to_string();
    }

    // years, days, hours, minutes, seconds
    let mut date = [0, 0, 0, 0, 0];
    let mut res = 0.0;
    // seconds in a year
    const Y: f64 = 31536000.0;
    const D: f64 = 86400.0;
    const H: f64 = 3600.0;
    const M: f64 = 60.0;

    if seconds as f64 / Y >= 1.0 {
        date[0] = (seconds as f64 / Y).floor() as u64;
        res = seconds as f64 % Y;
    } else {
        res = seconds as f64;
    }

    if res / D >= 1.0 {
        date[1] = (res / D).floor() as u64;
        res = res % D;
    }

    if res / H >= 1.0 {
        date[2] = (res / H).floor() as u64;
        res = res % H;
    }

    if res / M >= 1.0 {
        date[3] = (res / M).floor() as u64;
        res = res % M;
    }

    date[4] = res as u64;

    const DATE: [&str; 5] = ["year", "day", "hour", "minute", "second"];
    let p = date
            .iter()
            .enumerate()
            .filter(|&(_, &x)| x != 0)
            .map(|(i, &x)| {
                if x == 1 {
                    format!("{} {}", x, DATE[i])
                } else {
                    format!("{} {}s", x, DATE[i])
                }
            })
            .collect::<Vec<String>>();
    let mut s = p.join(", ");
    if p.len() > 1 {
        let i = s.rfind(",").unwrap();
        s.replace_range(i..i+1, " and");
    }
    s
}

fn format_duration2(seconds: u64) -> String {
    let result = [
        ("year", 31536000, 100000),
        ("day", 86400, 365),
        ("hour", 3600, 24),
        ("minute", 60, 60),
        ("second", 1, 60),
    ].iter()
    .map(|(unit, duration, modulo)| (seconds / duration % modulo, unit))
    .filter_map(|(duration, unit)| match duration {
        _ if duration == 1 => Some(format!("{} {}", duration, unit)),
        _ if duration != 0 => Some(format!("{} {}s", duration, unit)),
        _ => None,
    }).collect::<Vec<String>>();

    match result.len() {
        0 => String::from("now"),
        1 => result.join(""),
        _ => result
            .split_last()
            .map(|(r, l)| l.join(", ") + " and " + r)
            .unwrap(),
    }
}

fn hamming(n: usize) -> u64 {
    // https://en.wikipedia.org/wiki/Hamming_number
    let mut h = vec![1];
    let mut i2 = 0;
    let mut i3 = 0;
    let mut i5 = 0;
    for _ in 1..n {
        let m2 = h[i2] * 2;
        let m3 = h[i3] * 3;
        let m5 = h[i5] * 5;
        let m = m2.min(m3.min(m5));
        if m == m2 {
            i2 += 1;
        }
        if m == m3 {
            i3 += 1;
        }
        if m == m5 {
            i5 += 1;
        }
        h.push(m);
    }
    return h[n-1];
}

fn balanced_parens(n: u16) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let mut stack = vec![];
    stack.push(vec![(0, 0)]);
    let mut moves = vec![];
    // while stack is not empty
    while !stack.is_empty() {
        let mut movelist = stack.pop().unwrap();
        // movelist = [(0, 0)]
        if movelist.len() as u16 != n + 1 {
            for (x, y) in movelist.iter() {
                println!("{:?} {} {}", movelist, x, y);
                if x <= &n {
                    let mut newlist = movelist.clone();
                    newlist.push((x+1, *y));
                    stack.push(newlist);
                }
                if y <= &n && y <= x {
                    let mut newlist = movelist.clone();
                    newlist.push((*x, y+1));
                    stack.push(newlist);
                } 
            }
        } else {
            // println!("{:?}", movelist);
            moves.push(movelist);
        }
    }

    let mut endstr = HashSet::new(); 

    for movex in moves.iter() {
        let mut x = movex.clone();
        let mm = x.iter().fold(0, |acc, t| acc.max(t.0));

        let mut lowesty = x.iter().fold(0, |acc, t| acc.min(t.1));
        let mut currentrow: i32 = mm.into();
        let lastrow: i32 = mm.into();
        let mut movestr = String::new();
        println!("{:?}", x);
        while currentrow >= 0 {
            println!("Current row: {:?} x: {:?}", currentrow, x);
            // count the number of items in x that (x, y) x is equal to mm max
            let mut count = 0;

            for (c, d) in x.iter() {
                if *c == currentrow as u16 {
                    count += 1;
                }
            }

            if count > 0 {
                if currentrow != 0 { movestr.push_str("("); }
                if currentrow != lastrow {
                    count -= 1;
                }
                for _ in 0..count {
                    movestr.push_str("()");
                }
                if currentrow != 0 { movestr.push_str(")"); }
                println!("Move string: {:?}, Count: {}", movestr, count);
                count = 0;
            }
            currentrow -= 1;
        }
        endstr.insert(movestr);
    }
    let mut v = vec![];
    for i in endstr.iter() {
        v.push(i.to_string());
    }
    println!("{} moves", v.len());
    return v;
}



use std::collections::HashSet;

fn balanced_parens2(n: u16) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    //let prevstr = String::new();
    let one: String = "()".to_string();
    // let mut prevstr = vec!["()".to_string()];
    // create a hashset for a string
    let mut prevstr = HashSet::new();
    prevstr.insert("()".to_string());

    for i in 0..n -1  {
        // let mut newstr = vec![];
        let mut newstr = HashSet::new();
        for j in prevstr.iter() {
            let mut s3 = String::new();
            s3.push_str("(");
            s3.push_str(j);
            s3.push_str(")");
            newstr.insert(s3);

            let mut s = String::new();
            s.push_str(&one);
            s.push_str(j);
            newstr.insert(s);

            let mut s2 = String::new();
            s2.push_str(j);
            s2.push_str(&one);
            newstr.insert(s2);
        }
        prevstr = newstr.clone();
    }
    let mut v = vec![];
    for i in prevstr.iter() {
        v.push(i.to_string());
    }
    println!("{} moves", v.len());
    return v;
}

fn balanced_parens3(n: u16) -> Vec<String> {
    let mut result = Vec::new();
    if n == 0 {
        result.push("".to_string());
    } else {
        for i in 0..n {
            let left = balanced_parens3(i);
            let right = balanced_parens3(n - 1 - i);
            println!("left: {:?}, right: {:?}", left, right);
            for l in left {
                for r in &right {
                    result.push(format!("({}){}", l, r));
                }
            }
        }
    }
    result
}

// use itertools::Itertools;
fn permutations_without_duplicates(nums: &[u64]) -> Vec<Vec<u64>> {
    if nums.is_empty() {
        return vec![];
    }

    if nums.len() == 1 {
        return vec![nums.to_vec()];
    }

    let mut permutations = HashSet::new();

    for (i, &num) in nums.iter().enumerate() {
        // Recursively generate all the permutations of the subvector without num
        let sub_perms = permutations_without_duplicates(&[&nums[..i], &nums[i + 1..]].concat());
        // For each sub_perm, insert a new permutation with num inserted at the beginning
        println!("    {} num: {:?}, sub_perms: {:?} -->{:?}", i, num, sub_perms, &[&nums[..i], &nums[i + 1..]].concat());
        for sub_perm in sub_perms {
            if num != sub_perm[0] {
                let mut new_perm = vec![num];
                new_perm.extend(sub_perm);
                println!("-- new_perm: {:?}", new_perm);
                permutations.insert(new_perm);
            }
        }
    }
    // Return the permutations as a Vec<Vec<u64>>
    permutations.into_iter().collect()
}


fn u64_to_vec(num: &u64) -> Vec<u64> {
    let mut digits = vec![];
    let mut num = *num;
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.reverse();
    digits
}

fn vec_to_u64(vec: Vec<u64>) -> u64 {
    let mut num = 0;
    for d in vec {
        num = num * 10 + d;
    }
    num
}
fn issmallersolution(perm: Vec<u64>, n: u64) -> bool {
    let mut n_string = u64_to_vec(&n);
    let permdig: u64 = vec_to_u64(perm.clone());
    n_string.sort();
    let mut perm_string = perm.clone();
    perm_string.sort();

    if perm_string == n_string && permdig < n && perm[0] != 0 {
        println!("true: perm: {:?}, n: {}, permdig: {}, n_string: {:?}, perm_string: {:?}", perm, n, permdig, n_string, perm_string);
        return true;
    }
    println!("false: perm: {:?}, n: {}, permdig: {}, n_string: {:?}, perm_string: {:?}", perm, n, permdig, n_string, perm_string);
    false
}

fn next_smaller_number(n: u64) -> Option<u64> {
    let n_string = u64_to_vec(&n);
    let mut sorted_n_string = n_string.clone();
    sorted_n_string.sort();

    let mut digs = 2;
    while digs <= n_string.len() {
        let lastdigs: Vec<u64> = n_string[n_string.len() - digs..].to_vec();
        let mut perms = perma(&lastdigs, n, n_string[..n_string.len() - digs].to_vec());
        // println!("lastdigs: {:?}, perms: {:?}", lastdigs, perms);
        perms.sort_by(|a, b| vec_to_u64(b.clone()).cmp(&vec_to_u64(a.clone())));


        for j in perms.iter() {
            let i = [n_string[..n_string.len() - digs].to_vec(), j.to_vec()].concat();
            if i[0] == 0 {
                continue;
            }
            

            if vec_to_u64(i.clone()) >= n {
                continue;
            }

            let mut sorted_i = i.clone();
            sorted_i.sort();

            // println!("i: {:?}, j: {:?}, n_string: {:?}, perms: {:?}", i, j, n_string, perms);

            // let n_string2 = u64_to_vec(&i);
            // println!("n_string2: {:?}, i {}", n_string2, i);
            if i.len() == n_string.len() {
                let dig = vec_to_u64(i.clone());
                if sorted_i == sorted_n_string && dig < n {
                    return Some(dig);
                }
            } else if i.len() < n_string.len() {
                return None;
            }
        }
        digs += 1;
    }

    return None
}

fn perma(nums: &[u64], n: u64, digs: Vec<u64>) -> Vec<Vec<u64>> {
    if nums.is_empty() { return vec![]; }
    if nums.len() == 1 { return vec![nums.to_vec()]; }

    let mut permutations = HashSet::new();
    let mut orderednums = nums.to_vec();
    orderednums.sort_by(|a, b| b.cmp(a));

    println!("nums: {:?}, {:?}, digs: {:?}", nums, orderednums, digs);
    for (i, &num) in orderednums.iter().enumerate() {
        // println!("num: {:?}, i: {:?}", num, i);
        for j in i..nums.len() {
            let mut new_perm = orderednums.to_vec();

            if new_perm[j] < num && i != j {
                let mut new_config = new_perm.clone();
                new_config.swap(i, j);
                new_config[i + 1..].sort_by(|a, b| b.cmp(a));
                permutations.insert(new_config.clone());
                let issol: bool = issmallersolution([digs.clone(), new_config.clone()].concat(), n);

                println!(" .  new_perm: {:?} i: {:?}, j: {:?}, {:?}, {:?}", new_perm, i, j, new_config, issol);
                if issol {
                    return vec![new_config.clone()];
                } else {
                    continue;
                }
                let sub_perms = perma(&new_config, n, digs.clone());
                for sub_perm in sub_perms {
                    permutations.insert(sub_perm);
                }
            } else {
                // let issol: bool = issmallersolution([digs.clone(), new_perm.clone()].concat(), n);
                // // println!("new_perm: {:?} i: {:?}, j: {:?}, {:?}, {:?}", new_perm, i, j, new_perm, issol);

                // if issol {
                //     return vec![new_perm];
                // }
            }
        }
    }
    permutations.into_iter().collect()
}

fn next_smaller_number_clean(mut n: u64) -> Option<u64> {
    println!("n: {}", n);
    let mut digits = vec![];
    while n > 0 {
        let d = n % 10;
        n /= 10;
        digits.push(d);
    }

    for i in 1..digits.len() {
        if let Some(pos) = digits[0..i].iter().position(|&x| x < digits[i]) {

            println!("digits: {:?}, i: {}, pos: {:?}", digits, i, pos);

            digits.swap(pos, i);

            println!("  Swap: digits: {:?}, i: {}, pos: {:?}", digits, i, pos);
            digits[0..i].sort_unstable();

            println!("    Sort: digits: {:?}, i: {}, pos: {:?}", digits, i, pos);
            digits.reverse();
            println!("      Reverse: digits: {:?}, i: {}, pos: {:?}", digits, i, pos);


            if digits[0] == 0 {
                return None;
            }

            let mut res = 0;
            for d in digits {
                res *= 10;
                res += d;
            }

            return Some(res);
        }
    }
    // let mut sorted_n_string = n_string.clone();
    // sorted_n_string.sort();

    // let mut digs = 2;
    // while digs <= n_string.len() {
    //     let lastdigs: Vec<u64> = n_string[n_string.len() - digs..].to_vec();
    //     let mut perms = perma(&lastdigs, n, n_string[..n_string.len() - digs].to_vec());
    //     // println!("lastdigs: {:?}, perms: {:?}", lastdigs, perms);
    //     perms.sort_by(|a, b| vec_to_u64(b.clone()).cmp(&vec_to_u64(a.clone())));


    //     for j in perms.iter() {
    //         let i = [n_string[..n_string.len() - digs].to_vec(), j.to_vec()].concat();
    //         if i[0] == 0 {
    //             continue;
    //         }
    //         

    //         if vec_to_u64(i.clone()) >= n {
    //             continue;
    //         }

    //         let mut sorted_i = i.clone();
    //         sorted_i.sort();

    //         // println!("i: {:?}, j: {:?}, n_string: {:?}, perms: {:?}", i, j, n_string, perms);

    //         // let n_string2 = u64_to_vec(&i);
    //         // println!("n_string2: {:?}, i {}", n_string2, i);
    //         if i.len() == n_string.len() {
    //             let dig = vec_to_u64(i.clone());
    //             if sorted_i == sorted_n_string && dig < n {
    //                 return Some(dig);
    //             }
    //         } else if i.len() < n_string.len() {
    //             return None;
    //         }
    //     }
    //     digs += 1;
    // }

    // return None
    return Some(107);
}

fn next_smaller_number3(mut n: u64) -> Option<u64> {
    let mut digits = vec![];
    while n > 0 {
        let d = n % 10;
        n /= 10;
        digits.push(d);
    }

    for i in 1..digits.len() {
        if let Some(pos) = digits[0..i].iter().position(|&j| j < digits[i]) {
            digits.swap(pos, i);
            digits[0..i].sort_unstable();
            digits.reverse();

            if digits[0] == 0 {
                return None;
            }

            let mut res = 0;
            for d in digits {
                res *= 10;
                res += d;
            }

            return Some(res);
        }
    }

    None
}


fn u642vec(num: &u64) -> Vec<u64> {
    let mut digits = vec![];
    let mut num = *num;
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits
}

fn vec2u64(vec: Vec<u64>) -> u64 {
    let mut num = 0;
    for d in vec {
        num = num * 10 + d;
    }
    num
}

fn find_reverse_number(n: u64) -> u64 {
    println!("n: {}", n);
    let mut comm_n = vec![0, 10, 19, 109, 199, 1099, 1999, 10999, 19999, 109999, 199999, 1099999, 1999999, 10999999, 19999999, 109999999, 199999999, 1099999999, 1999999999, 10999999999, 19999999999, 109999999999, 199999999999, 1099999999999, 1999999999999, 10999999999999, 19999999999999, 109999999999999, 199999999999999, 1099999999999999, 1999999999999999, 10999999999999999, 19999999999999999, 109999999999999999, 199999999999999999, 1099999999999999999, 1999999999999999999, 10999999999999999999];
    let mut comm_change = vec![0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000, 10_000_000_000, 100_000_000_000, 1_000_000_000_000, 10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000, 10_000_000_000_000_000];
    let mut alpha: u64 = 1;
    let mut change_digits: u64 = 0;
    let mut figures: u64 = 0;

    for cn in 0..comm_n.len() {
        if n <= comm_n[cn] {
            for _ in 1..cn { alpha *= 10; }
            figures = cn as u64;
            if cn % 2 == 0 {
                change_digits = (cn as u64 + 1) / 2;
            } else {
                change_digits = ((cn as u64) + 1) / 2;
            }
            break;
        }
    }

    let mut alpha_vec: Vec<u64> = vec![];
    if figures > 1 {
        alpha += 1;
        alpha_vec = u642vec(&alpha);
    } else {
        change_digits = 1;
        alpha = 0;
        alpha_vec = vec![0];
    }


    let mut new_n = comm_n[(figures - 1) as usize] + 1;
    for digit in 0..change_digits {
        let mut change_amount = 0;
        if change_digits % 2 == 0 || change_digits == 1 {
            change_amount = comm_change[(change_digits - digit) as usize];
        } else {
            change_amount = comm_change[(change_digits - digit) as usize];
        }

        for i in alpha_vec[digit as usize]..9 {
            if new_n == n {
                let mut res = 0;
                for d in alpha_vec {
                    res *= 10;
                    res += d;
                }
                return res
            }

            if new_n + change_amount > n {
                break;
            } else {
                new_n += change_amount;
                alpha_vec[digit as usize] += 1;
                if (figures - digit -1) != digit {
                    alpha_vec[(figures - digit - 1) as usize] += 1;
                }
            }

        }
    }

    let mut res = 0;
    for d in alpha_vec {
        res *= 10;
        res += d;
    }
    res
    
}

fn find_reverse_number_1(n: u64) -> u64 {
    let n = n-1;
    if n < 10 { 
        return n;
    }
    let x = n / 11;
    let width = x.to_string().len();
    let nines: u64 = "9".repeat(width).parse::<u64>().unwrap();
    let lh = (n-nines).to_string();
    let rh: String = lh[0..width].chars().rev().collect();

    (lh + &rh).parse::<u64>().expect("Output too large")

}

fn find_reverse_number_2(mut n: u64) -> u64 {
    if n == 1 { return 0 }
    n -= 2;
    let mut digits = 1_usize;
    loop {
        let half_d = (digits as u32 + 1) / 2;
        let p = 10_u64.pow(half_d - 1);
        let count = 9 * p;
        if n < count {
            let ns = (n + p).to_string();
            return (ns.chars().chain(ns.chars().rev().skip(digits % 2)).collect::<String>()).parse::<_>().unwrap();
        }
        n -= count;
        digits += 1;
    }
}

// def pentagonal_number(k):
//     print(type(k), k)
//     return int(k*(3*k-1) / 2)





// def compute_partitions(goal):
//     partitions = [1]
//     for n in range(1,goal+1):
//         partitions.append(0)
//         for k in range(1,n+1):
//             coeff = (-1)**(k+1)
//             for t in [pentagonal_number(k), pentagonal_number(-k)]:
//                 if (n-t) >= 0:
//                     partitions[n] = partitions[n] + coeff*partitions[n-t]
//     return partitions[-1]

fn pentagonal_number(k: i32) -> i32 {
    k * (3 * k - 1) / 2
}

fn partitions(n: u32) -> u32 {
    // Convert goal to i32
    let goal = n as i32;

    let i = -1f32;
    let mut partitions = vec![1];
    for n in 1..goal+1 {
        partitions.push(0);
        for k in 1..n+1 {
            let coeff = i.powi(k+1);
            // println!("Coeff: {:?}, k: {:}", coeff, k);
            for t in [pentagonal_number(k), pentagonal_number(-k)] {
                if (n-t) >= 0 {
                    partitions[n as usize] = partitions[n as usize] + (coeff as i32)*partitions[(n-t) as usize];
                }
            }
        }
    }
    // println!("{:?}", partitions);
    partitions[goal as usize] as u32

}

/// See https://stackoverflow.com/a/27051927/5637701
fn partitions2(n: u32) -> u32 {
    fn p(k: u32, n: u32) -> u32 {
        if k > n { 0 }
        else if k == n { 1 }
        else { p(k + 1, n) + p(k, n - k) }
    }
    p(1, n)
}


fn next_bigger_number(mut n: i64) -> i64 {
    /*
    1. Find the largest index k such that a[k] < a[k + 1]. If no such index exists, the permutation is the last permutation.
    2. Find the largest index l greater than k such that a[k] < a[l].
    3. Swap the value of a[k] with that of a[l].
    4. Reverse the sequence from a[k + 1] up to and including the final element a[n].
    */
    let mut digits = vec![];
    while n > 0 {
        let d = n % 10;
        n /= 10;
        digits.push(d);
    }
    digits.reverse();

    for i in (0..(digits.len() - 1)).rev() {
        if digits[i] < digits[i + 1] {
            for j in ((i+1)..digits.len()).rev() {
                if digits[j] > digits[i] {
                    digits.swap(i, j);
                    digits[i+1..].reverse();
                    if digits[0] == 0 {
                        return -1;
                    }
                    let mut res = 0;
                    for d in digits {
                        res *= 10;
                        res += d;
                    }
                    return res 
                }
            }
        }
    }
    -1
}

fn next_bigger_number2(n: i64) -> i64 {
    // n = 918651
    // represent number as vector of chars in Big Endian notation
    // 1 5 6 8 1 9
    let mut digits = n.to_string().chars().rev().collect::<Vec<char>>();

    // start from the left and find digit which less then previous one
    // 1 5 6 8 [1] 9
    let i = match (0..digits.len()-1).position(|i| digits[i] > digits[i+1]) {
        Some(i) => i+1,
        None => return -1,
    };

    // find index 'j' of first digit from the left
    // which grater than current digit
    // 1 |5| 6 8 [1] 9
    let j = (0..i).position(|j| digits[j] > digits[i] ).unwrap();

    // swap current digit with previous smallest greater
    // 1 |1| 6 8 [5] 9
    digits.swap(j,i);

    // reverse sort left part
    // 8 6 1 1 [5] 9
    digits[0..i].sort_by_key(|&i| std::cmp::Reverse(i));

    // make string from vector then parse string as i64
    digits.into_iter().rev().collect::<String>().parse().unwrap()

}

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!(
        "{:02X}{:02X}{:02X}",
        r.clamp(0, 255),
        g.clamp(0, 255),
        b.clamp(0, 255)
    ) 
}

fn get_sum(a: i64, b: i64) -> i64 {
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    (a..=b).sum()
}

fn dbl_linear(n: u32) -> u32{
    // y = 2 * x + 1 and z = 3 * x + 1
    let mut u: Vec<u32> = vec![1];
    let mut x = 0;
    let mut y = 0;

    while u.len() <= n as usize {
       let a = 2 * u[x] + 1; 
       let b = 3 * u[y] + 1;

       if a > b {
            u.push(b);
            y += 1;
         } else if a < b {
            u.push(a);
            x += 1;
         } else {
            u.push(a);
            x += 1;
            y += 1;
       }
    }

    u[n as usize]
}

fn reverse_word(words: &str) -> String {
    // Reverse string

    words.split_whitespace().rev().collect::<Vec<&str>>().join(" ")

}

fn pal(x: i32) -> bool {
    let mut y = x.to_string();
    let mut z = y.clone();
    y == y.chars().rev().collect::<String>()
    if y == z {
        return true;
    } else {
        return false;
    }

}

fn main() {
    // let x = create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"
    // let x = likes3(&["Alex", "Jacob", "Mark", "Max"] ); // returns "Alex, Jacob and 2 others like this"
    // let x = time_correct("24:01:01");
    // let x = camel_case("camel case method");
    // let x = sum_pairs(&[10, 5, 2, 3, 7, 5], 10);
    // let a = [Direction::North, Direction::South, Direction::South, Direction::East, Direction::West,
    // Direction::North, Direction::West];
    // let x = dir_reduc(&a);
    // let x = find_the_number_plate(3);
    // let x = heron(3.0, 4.0, 5.0);
    // let x = format_duration(3600);
    // let x = hamming(10);
    // let x = balanced_parens3(3);
    // let x = next_smaller_number_clean(3437911111);
    // let x2 = next_smaller_number3(3437911111);

    // let x = find_reverse_number(1_000);
    // let x = partitions(100);
    // let x = find_reverse_number(1166926);
    // let x = permutations_without_duplicates(&[9, 0, 7]);
    // let x = perma(&[9, 0, 7]);
    // 38720923 38720392
    // let x = next_bigger_number(513);
    // let x = rgb(255, 255, 255);
    // let x = get_sum(5, -1);
    let x = dbl_linear(10);
    println!("{:?}", x);
}