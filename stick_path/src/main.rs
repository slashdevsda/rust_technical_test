use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/// this function is called upon each line and 
/// mutates its argument 'lines' accordingly to what it sees
/// while iterating over the line's content.
fn process_line(s: &String, lines: &mut Vec<char>) {
    let mut current_index = 0;
    let mut it = s.chars().peekable();
    let mut last_seen: char = it.next().expect("unexpected empty line :(");
    while let Some(cc) = it.next() {
        if let Some('-') = it.peek() {
            if last_seen == '|' {
                // change lane
                lines.swap(current_index, current_index + 1);
            }
        }
        if cc == '|' {
            current_index += 1;
        }
        last_seen = cc;
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    // check constrains
    if ! w > 3 {
        panic!("width should be greater than 3");
    }
    if ! 100 >= h {
        panic!("height should be less than 100");
    }

    // start reading the map
    // the first line contains labels
    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let line: String = input_line.trim_end().to_string();
    let mut labels: Vec<char> = line
        .chars()
        .filter_map(|c: char| {
            if !c.is_ascii_whitespace() {
                Some(c)
            } else {
                None
            }
        })
        .collect();

    // we'll copy this one to keep track of
    // the initial label order.
    let labels_display = labels.clone();

    for _ in 1..(h - 1) as usize {
        input_line.clear();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Unable to read line");
        let line: String = input_line.trim_end().to_string();
        process_line(&line, &mut labels);
    }

    input_line.clear();
    io::stdin()
        .read_line(&mut input_line)
        .expect("unable to read the last line");
    let end_labels: Vec<char> = input_line
        .trim_end()
        .to_string()
        .chars()
        .filter_map(|c: char| {
            if !c.is_ascii_whitespace() {
                Some(c)
            } else {
                None
            }
        })
        .collect();

    for item in labels_display {
        // this is not optimal, a BTreeMap instead would be nicer
        let index = labels.iter().position(|&c| c == item).unwrap();
        println!("{}{}", labels[index], end_labels[index]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_process_line_do_nothing() {
        let line = String::from("|  |  |  |");
        let mut labels = vec!['a', 'b', 'c', 'd'];
        process_line(&line, &mut labels);
        assert_eq!(labels[0], 'a');
        assert_eq!(labels[1], 'b');
        assert_eq!(labels[2], 'c');
        assert_eq!(labels[3], 'd');

    }

    #[test] 
    fn test_process_line_permute() {
        let line = String::from("|  |--|  |");
        let mut labels = vec!['a', 'b', 'c', 'd'];
        process_line(&line, &mut labels);
        // ensure permutation is effective
        assert_eq!(labels[1], 'c');
        assert_eq!(labels[2], 'b');
        // ensure the rest is left untouched
        assert_eq!(labels[0], 'a');
        assert_eq!(labels[3], 'd');
    }

    #[test] 
    fn test_process_line_first_turn() {
        let line = String::from("|--|  |  |");
        let mut labels = vec!['a', 'b', 'c', 'd'];
        process_line(&line, &mut labels);
        assert_eq!(labels[0], 'b');
        assert_eq!(labels[1], 'a');
        assert_eq!(labels[2], 'c');
        assert_eq!(labels[3], 'd');
    }    
    #[test] 
    #[should_panic]    
    fn test_process_line_panic_empty_line() {
        let line = String::from("");
        let mut labels = vec!['a', 'b', 'c', 'd'];
        process_line(&line, &mut labels);
    }
}
