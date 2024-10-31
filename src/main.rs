use std::io::{BufReader, Read};

fn radians_from_degrees(degrees: f64) -> f64 {
    0.01745329251994329577 * degrees
}

// NOTE: EarthRadius is generally expected to be 6372.8
fn reference_haversine(x0: f64, y0: f64, x1: f64, y1: f64, earth_radius: f64) -> f64 {
    let lat1 = y0;
    let lat2 = y1;
    let lon1 = x0;
    let lon2 = x1;

    let d_lat = radians_from_degrees(lat2 - lat1);
    let d_lon = radians_from_degrees(lon2 - lon1);
    let lat1 = radians_from_degrees(lat1);
    let lat2 = radians_from_degrees(lat2);

    let a = ((d_lat / 2.0).sin()).powi(2) + lat1.cos() * lat2.cos() * ((d_lon / 2.0).sin()).powi(2);
    let c = 2.0 * (a.sqrt()).asin();

    let result = earth_radius * c;

    result
}

#[derive(Debug)]
struct Row {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

fn main() {
    let mut file =
        BufReader::new(std::fs::File::open("./resultado.json").expect("The file does not exist"));

    let mut pairs = Vec::<Row>::new();
    let mut current_token = String::new();
    let mut x0 = 0.0;
    let mut y0 = 0.0;
    let mut x1 = 0.0;
    let mut y1 = 0.0;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");

    let mut position = 0;
    while buffer[position] != b'[' {
        position += 1;
    }

    position += 1;

    let mut is_in_key_name = false;
    let mut current_key = String::new();

    while buffer[position] != b']' {
        match buffer[position] {
            b'"' if is_in_key_name => {
                std::mem::swap(&mut current_token, &mut current_key);
                current_token.clear();
                is_in_key_name = false;
            }
            b'"' => {
                is_in_key_name = true;
                current_token.clear();
            }
            b':' => (),
            b'{' => (),
            b'}' => {
                let number: f64 = current_token.parse().expect("Cannot read value as number");
                current_token.clear();
                match current_key.as_str() {
                    "x0" => x0 = number,
                    "y0" => y0 = number,
                    "x1" => x1 = number,
                    "y1" => y1 = number,
                    uk => eprintln!("Unknown key {uk}"),
                }
                let row = Row { x0, y0, x1, y1 };
                pairs.push(row);
            }
            b',' => {
                let prev_token = buffer[position - 1];
                if prev_token != b'}' {
                    let number: f64 = current_token.parse().expect("Cannot read value as number");
                    current_token.clear();
                    match current_key.as_str() {
                        "x0" => x0 = number,
                        "y0" => y0 = number,
                        "x1" => x1 = number,
                        "y1" => y1 = number,
                        uk => eprintln!("Unknown key {uk}"),
                    }
                }
            }
            b']' => break,
            any if is_in_key_name => {
                current_token.push(any as char);
            }
            digit => {
                current_token.push(digit as char);
            }
        }
        position += 1;
    }

    println!("Read {} keys", pairs.len());
    let haversine_sum: f64 = pairs
        .into_iter()
        .map(|Row { x0, y0, x1, y1 }| reference_haversine(x0, y0, x1, y1, 6372.8))
        .sum();

    println!("Sum: {}", haversine_sum);
}
