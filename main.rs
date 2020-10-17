use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
   let args: Vec<String> = env::args().collect();
   let mut fields = [[511 as u16;9];9];
   let mut tmp: u16;

   read_file(&args[1], &mut fields);

   // remove trivial values
   for i in 0 .. 9 {
      for j in 0 .. 9 {
         if is_single_val(fields[i][j]) {
             tmp = fields[i][j];
             propagate_set(&mut fields, i, j, one_hot_to_decimal(tmp));
         }
      }
   }

   depth_search(&mut fields);

   print(fields);
}

fn read_file(file_name: & String, fields: &mut [[u16;9];9]) {
    let mut row: usize = 0;
    let mut col: usize = 0;

    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(ip) = line {
                for field in ip.split("|") {
                    match field {
                        "." | "-" | "x" => fields[row][col] = 511,
                        _ => fields[row][col] = 1u16 << (field.parse::<u16>().unwrap() - 1u16),
                    }
                    col += 1;
                }
                col = 0;
                row += 1;
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Candidate {
   row: usize,
   col: usize,
   valid: bool
}

fn one_hot_to_decimal(one_hot: u16) -> u16 {
    for i in 1 .. 10 {
       if ((one_hot >> (i-1)) & 1) == 1 {
           return i;
       }
    }
    return 0;
}

fn print(fields: [[u16;9];9]) {
    for i in 0..9 {
        for j in 0..9 {
               print!(" {} ", one_hot_to_decimal(fields[i][j]));
        }
        print!("\n");
    }
}

fn is_single_val(val: u16) -> bool{
    let mut count: u8 = 0; 

    for i in 0 .. 9 {
       if (val >> i) & 1 == 1 {
           count += 1;
       }
    }

    return count == 1;
}

fn set(fields: &mut [[u16;9];9], row: usize, col: usize, val: u16) -> bool {
    let bitmask: u16 = 1 << (val - 1);

    // something is wrong - the value is no possible value for the field anymore
    if (fields[row][col] & bitmask) == 0 {
        return false;
    }

    fields[row][col] = bitmask;

    return true;
}

fn remove(fields: &mut [[u16;9];9], row: usize, col: usize, val: u16) -> bool {
    let is_set: bool = is_single_val(fields[row][col]);
    let bitmask: u16 = 1 << (val - 1);
    fields[row][col] &= !bitmask;

    if fields[row][col] == 0 {
        return false;
    }
    // propagate if the field was set in this call
    if !is_set && is_single_val(fields[row][col]) {
        return propagate_set(fields, row, col, one_hot_to_decimal(fields[row][col]));
    }

    return true;
}

fn propagate_set(fields: &mut[[u16;9];9], row: usize, col: usize, val: u16) -> bool {
    // remove the value for all fields in the same column
    for i in 0..9 {
        if i == row {
            continue;
        }

        if !remove(fields, i, col, val) {
            return false;
        }
    }

    // remove the value for all fields in the same row
    for i in 0..9 {
        if i == col {
            continue;
        }

        if !remove(fields, row, i, val) {
            return false;
        }
    }

    // remove the value in the same 3x3 square
    for i in 0 .. 3 {
        for j in 0 .. 3 {
            let row_idx = (row / 3) * 3 + i;
            let col_idx = (col / 3) * 3 + j;
            
            if row == row_idx || col == col_idx {
                continue;
            }

            if !remove(fields, row_idx, col_idx, val) {
                return false;
            }
        }
    }

    return true;
}

fn depth_search(fields: &mut[[u16;9];9]) -> bool {
    let mut candidate = Candidate {
       valid : false,
       row : 0,
       col : 0
    };
    
    find_candidate(fields, &mut candidate);
    
    if !candidate.valid {
        return true;
    }

    for i in 1..10 {
       if (fields[candidate.row][candidate.col] & (1 << (i-1))) == 0 {
           continue;
       }

       let mut cpy: [[u16;9];9] = unsafe { std::mem::uninitialized() };
       copy(fields, &mut cpy);
       if set(&mut cpy, candidate.row, candidate.col, i) {
           if propagate_set(&mut cpy, candidate.row, candidate.col, i) {
               if depth_search(&mut cpy) {
                   copy(&cpy, fields); // TODO: slow
                   return true;
               }
           }
       }
    }

    return false
}

fn copy(array: & [[u16;9];9], copy: &mut [[u16;9];9]) {
    for i in 0 .. 9 {
        for j in 0 .. 9 {
           copy[i][j] = array[i][j];
        }
    }
}

fn find_candidate(fields: &mut [[u16;9];9], candidate: &mut Candidate) {
   let mut min: u8 = 10;

   for i in 0 .. 9 {
      for j in 0 .. 9 {
         let num: u8 = num_remaining(fields[i][j]);
         if min > num && num > 1{
            min = num;
            candidate.row = i;
            candidate.col = j;
            candidate.valid = true;
         }
      }
   }
}

fn num_remaining(val: u16) -> u8 {
    let mut count: u8 = 0;

    for i in 0..9 {
       if ((val >> i) & 1) == 1 {
          count += 1;
       }
    }

    return count;
}