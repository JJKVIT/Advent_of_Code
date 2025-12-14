use std::fs;

fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).unwrap();
    return contents;
}

#[allow(dead_code)]

fn task1(contents: String) {
    let mut pos: i16 = 50;
    let mut res: u32 = 0;
    for line in contents.lines() {
        let (dir, count) = line.split_at(1);
        let val = (count.parse::<i16>().expect("unable to parse str to int")) % 100;
        if dir == "L" {
            pos -= val;
            if pos < 0 {
                pos += 100;
            };
        } else {
            pos += val;
            pos %= 100;
        }
        if pos == 0 {
            res += 1;
        }
    }
    println!("{res}");
}

fn task2(contents: String) {
    let mut pos: i16 = 50;
    let mut res: u32 = 0;
    for line in contents.lines() {
        let (dir, count) = line.split_at(1);
        let mut val = count.parse::<i16>().expect("unable to parse str to int");
        res += (val / 100) as u32;
        val %= 100;
        if dir == "L" {
            if pos - val <= 0 && pos > 0 {
                res += 1;
            }
            pos -= val;
            if pos < 0 {
                pos += 100;
            };
        } else {
            pos += val;
            if pos >= 100 {
                res += 1;
                pos %= 100;
            }
        }
    }
    println!("{res}");
}

fn main() {
    //let contents:String = read_file("./inputs/sample.txt");
    let contents: String = read_file("./inputs/input.txt");
    //task1(contents);
    task2(contents);
}
