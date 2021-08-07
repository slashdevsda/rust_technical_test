use std::collections::BTreeMap;
use std::io;



macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn process_line(s: &String, lines: &mut Vec<char>) {
    // let lines: Vec<char> = h.keys().map(|_| 0).collect();
    let mut current_index = 0;
    // let mut lines: Vec<&char> = h.iter().map(|(_, c): (&usize, &char)| c).collect();
    //map(|(_, c) :(usize, char)| c).collect();
    println!("lines : {:?}", lines);

    let mut it = s.chars().peekable();
    let mut last_seen: char = it.next().expect("encountered empty line :(");
    // for cc in it.iter() {
    while let Some(cc) = it.next() {
        if let Some('-') = it.peek() {
            if last_seen == '|' {
                // change lane
                println!("swap, index {}", current_index);
                lines.swap(current_index, current_index + 1);
            }
        }
        // if last_seen == '-'
        // } else if stck == '-' {

        // }
        if cc == '|' {
            current_index += 1;
        }
        last_seen = cc;
    }

    // match h.get(&'A').unwrap() {
    //     Position::Starting => (),
    //     Position::Moving(x) => x
    //     _ => ()
    // }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);

    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();

    let line: String = input_line.trim_end().to_string();
    //let keys: Vec<char> = line.chars().filter(|c: char| c.is_ascii_whitespace()).collect();
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
    println!(
        "{:?}",
        labels,
    );

    for i in 1..h as usize {
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let line: String = input_line.trim_end().to_string();
        println!("current line {}", line);
        process_line(&line, &mut labels);
    }

    println!("answer: {:?}", labels);
}
