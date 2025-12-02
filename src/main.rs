use std::io::{self, BufRead};
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut password= 0;
    let mut dial = 50;
    
    for line_result in reader.lines() {
        let line = line_result?;
        let (dir, num_string) = line.split_at(1);
        let num: i32 = num_string.parse().expect("Failed to parse ");
        let mask: i32;

        // TODO: figure out how to not double count landings vs passes...
        if dir == "R" {
            let new_dial = dial + num;
            password += (new_dial / 100) - (dial / 100);
            dial = new_dial;   
        } else {
            let new_dial = dial - num;
            password += (-(new_dial) / 100) - (-(dial) / 100);
            if new_dial % 100 == 0 {
                password += 1;
            }
            dial = new_dial;
        }
        mask = dial >> 31;
        if dial < 0 {
            dial = 100 - (((dial ^ mask) - mask) % 100);
        }
        if dial > 99 {
            dial = dial % 100;
        }
    }
    println!("Password: {}", password);
    Ok(())
}
