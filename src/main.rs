use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let filename = String::from("test.txt");
    let graph = read_file_to_vector(&filename);

    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            print!("{},", graph[i][j]);
        }
        println!();
    }
}

fn read_file_to_vector(filename: &String) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let f = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Read file error"),
    };

    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let mut line_vector = Vec::new();
        let v: Vec<&str> = l.split(',').collect();
        for i in 0..v.len() {
            line_vector.push(v[i].parse::<i32>().unwrap());
        }
        result.push(line_vector);
    }
    return result;
}
