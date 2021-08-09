use std::io;


fn can_permute(idx: usize, seq: &Vec<u8>) -> bool {
    if idx == seq.len() - 1 {
        true
    } else if seq[idx + 1] == 1 && idx + 2 == seq.len() {
        // last star constrain
        true
    } else if seq[idx + 1] == 1 && 
        seq
        .split_at(idx + 2)
        .1
        .into_iter()
        .filter(|u| **u == 1 as u8)
        .count() == 0 {
            println!("can permute? (index {}) yes! tail is {:?}", idx, seq.split_at(idx + 2).1);
            true
    } else {
        false
    }
}

fn permute_to(symbol: u8, idx: usize, seq: &mut Vec<u8>) -> usize {
    let mut steps: usize = 0;

    if seq[idx] == symbol {
        return 0
    } else if !can_permute(idx, seq) {
        if seq[idx + 1] != 1 {
            steps += permute_to(1, idx + 1, seq);
        };
        if  seq
            .split_at(idx + 2)
            .1
            .into_iter()
            .any(|u| *u == 1 as u8) {
                // println!("yoho ({}) {} {:?} {:?}", symbol, idx, seq, seq.split_at(idx + 2).1);
                // for (offset, _i) in seq.clone().split_at(idx + 2).1.into_iter().enumerate() {
                    for offset in idx+2..seq.len() {
                    // a check
                    // println!("idx: {}, offset: {}, idx+offset, {}", idx, offset, idx+offset);
                    steps += permute_to(0, offset, seq);
                }
            }

    };
    if can_permute(idx, seq){
    // permutation allowed
        seq[idx] = symbol;
        // println!("permut {:?}", seq);
        steps + 1
    } else {
        0
    }
}

fn starlight(start: &mut Vec<u8>, target: &Vec<u8>) -> usize {
    let mut acc: usize = 0;

    for i in 0..start.len() {
        if start[i] == target[i] {
            continue
        } else {
            acc += permute_to(target[i], i, start);
        }
    };
    acc
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let start: String = input_line.trim_end().to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let target = input_line.trim_end().to_string();
    println!("{} to {}", start, target);

    let mut start: Vec<u8> = start
        .into_bytes()
        .into_iter()
        .map(|x| match x {
                48 => 0,
                49 => 1,
                _ => panic!("oh god no")
            })
         .collect();

    let target: Vec<u8> = target.into_bytes()
        .into_iter()
        .map(|x| 
            match x {
                48 => 0,
                49 => 1,
                _ => panic!("oh god no")
                })
        .collect();
    
        println!("{}", starlight(&mut start, &target));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_permute_1() {
        let seq = vec![0, 1, 0, 1];
        // assert_eq!(10, value);
        assert!(can_permute(0, &seq) == false);
        assert!(can_permute(1, &seq) == false);
        assert!(can_permute(3, &seq) == true);
        assert!(can_permute(2, &seq) == true);
    }

    #[test]
    fn test_can_permute_2() {
        let seq = vec![0, 1, 0, 0];
        // assert_eq!(10, value);
        assert!(can_permute(0, &seq) == true);
        assert!(can_permute(1, &seq) == false);
        assert!(can_permute(2, &seq) == false);
        assert!(can_permute(3, &seq) == true);
    }

    #[test]
    fn test_subject() {

        let mut seq = vec![1,1,0,0,1,0,0,1,0,0,0];
        let target = vec![1,0,0,0,0,1,1,0,0,1,1];
        // assert_eq!(10, value);
        assert_eq!(starlight(&mut seq, &target), 877);
    }
}
