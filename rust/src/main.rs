use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn char_code_at(letter: char) -> u32 {
    u32::from(letter)
}

#[cfg(test)]
mod tests {
    use crate::char_code_at;

    #[test]
    fn char_code_at_test() {
        assert_eq!(char_code_at('A'), 65);
    }

    #[test]
    fn pad() {
        assert_eq!(format!("{:<1$}","ab", 3).len(), 3);
    }
}

struct Alphabet {
    h: i32,
    l: i32,
    rows: Vec<String>
}

fn concat_str(a: &str, b: &str) -> String {
    a.to_string() + b
}

impl Alphabet {
    fn new(l: i32, h: i32) -> Alphabet {
        Alphabet {
            l,
            h,
            rows: vec![]
        }
    }

    fn set_row(&mut self, row: String) -> () {
        let line = format!("{:<1$}", row, (25 + 2) * self.l as usize);
        self.rows.push(line);
    }

    fn get(&self, word: String) -> Vec<String> {
        let a_position = char_code_at('A');
        let mut rows:Vec<String> = std::iter::repeat(String::from("")).take(self.h as usize).collect();
        for letter in word.chars() {
            let mut pos:i32 = char_code_at(letter.to_ascii_uppercase()) as i32 - a_position as i32;
            if pos < 0 || pos > 25 {
                pos = (self.rows[0].len() as i32 / self.l - 1) as i32
            }

            for i in 0..self.h {
                rows[i as usize] = concat_str(
                    &rows[i as usize],
                    &self.rows[i as usize][((pos as usize) *(self.l as usize)) as usize..(((pos+1) as usize)*(self.l as usize)) as usize]
                )
            }
        }

        rows
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let l = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t = input_line.trim_matches('\n').to_string();

    let mut a = Alphabet::new(l, h);

    for _i in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string();
        a.set_row(row)
    }

    let res = a.get(t);
    for r in res {
        println!("{}", r.trim_end_matches(" "));
    }
}
