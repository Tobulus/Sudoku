fn main() {
   let mut fields = [[511 as u16;9];9];
   fields[0][0] = 1 << 7;
   fields[1][2] = 1 << 2;
   fields[1][3] = 1 << 5;
   fields[2][1] = 1 << 6;
   fields[2][4] = 1 << 8;
   fields[2][6] = 1 << 1;
   fields[3][1] = 1 << 4;
   fields[3][5] = 1 << 6;
   fields[4][4] = 1 << 3;
   fields[4][5] = 1 << 4;
   fields[4][6] = 1 << 6;
   fields[5][3] = 1;
   fields[5][7] = 1 << 2;
   fields[6][2] = 1;
   fields[6][7] = 1 << 5;
   fields[6][8] = 1 << 7;
   fields[7][2] = 1 << 7;
   fields[7][3] = 1 << 4;
   fields[7][7] = 1;
   fields[8][1] = 1 << 8;
   fields[8][6] = 1 << 3;
   let mut tmp: u16;

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

fn remove(fields: &mut [[u16;9];9], row: usize, col: usize, val: u16) -> bool{
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

    println!("found candidate row={} col={}", candidate.row, candidate.col);

    for i in 1..10 {
       if (fields[candidate.row][candidate.col] & (1 << (i-1))) == 0 {
           println!("skipping value {} for {}", (1<<(i-1)), fields[candidate.row][candidate.col]);
           continue;
       }

       let mut cpy: [[u16;9];9] = unsafe { std::mem::uninitialized() };
       copy(fields, &mut cpy);
       println!("setting field {},{} to {}", candidate.row, candidate.col, i);
       if set(&mut cpy, candidate.row, candidate.col, i) {
           println!("now propagate");
           if propagate_set(&mut cpy, candidate.row, candidate.col, i) {
               println!("new depth search");
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

