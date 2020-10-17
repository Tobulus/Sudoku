# Sudoku
Provides a super fast solver for 9x9 Sudokus written in Rust!

Uses a depth-search approach to find a valid solution

# Usage
Sudokus can be provided in a file. Example format:

x|x|x|4|x|x|x|x|x  
x|2|x|x|x|x|x|x|x  
x|x|x|1|x|x|x|x|x  
x|1|x|x|x|x|x|2|x  
x|x|x|x|x|x|x|x|x  
x|x|x|x|x|x|x|x|x  
x|x|7|x|x|x|x|9|x  
x|x|x|x|x|x|x|x|x  
x|x|x|x|6|x|x|x|x  

Unset fields can be expresses by "x", "." or "-".

Call the compiled program and provide a file as first command line argument: **./main sudokus.txt**
