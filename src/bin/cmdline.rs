/*
import sys
for arg in sys.argv:
    print(arg)
 */

fn main() {
    for arg in std::env::args() {
        println!("{arg}");
    }
}