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
        // i + 1 should be `1`
        // i + 2 ... i + N should be `0`
            true
    } else {
        false
    }
}

/// permutes `seq[idx]` to `symbol` by going recursively to each required steps.
/// always returns the number of steps achieved.
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
                    for offset in idx+2..seq.len() {
                    steps += permute_to(0, offset, seq);
                }
            }

    };
    if can_permute(idx, seq){
        // permutation allowed, so we proceed.
        seq[idx] = symbol;
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
        assert!(can_permute(0, &seq) == false);
        assert!(can_permute(1, &seq) == false);
        assert!(can_permute(3, &seq) == true);
        assert!(can_permute(2, &seq) == true);
    }

    #[test]
    fn test_can_permute_2() {
        let seq = vec![0, 1, 0, 0];
        assert!(can_permute(0, &seq) == true);
        assert!(can_permute(1, &seq) == false);
        assert!(can_permute(2, &seq) == false);
        assert!(can_permute(3, &seq) == true);
    }

    #[test]
    fn test_subject_simple() {
        let mut seq = vec![1,0,1,0,1,0];
        let target = vec![0,1,0,1,0,1];
        assert_eq!(starlight(&mut seq, &target), 26);
    }

    #[test]
    fn test_subject() {
        let mut seq = vec![1,1,0,0,1,0,0,1,0,0,0];
        let target = vec![1,0,0,0,0,1,1,0,0,1,1];
        assert_eq!(starlight(&mut seq, &target), 877);
    }
}
