[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
![Rust](https://img.shields.io/badge/rust-v1.54.0-brown.svg)
![Python](https://img.shields.io/badge/python-v3.9-blue.svg)
[![Donate](https://img.shields.io/badge/donate-PayPal-yellow.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=XH8R7VFJQE3YQ&currency_code=USD)

# ROTX-Solver
This program is a ROTX solver written in Python.  There's also a *Rust* port as my first Rust project.  It's very fast.

## Description
This ROTX solver takes a unique approach to solving for ROT by calculating the numerical 'distance' between each letter of both the dictionary set and the ciphered sentences. (Note: this ROTX solver only performs alphabetic rotations.)

### Traditional Approach
Traditionally, ROTX solvers will take either the ciphered sentence or the dictionary set and rotate all characters up to 26 times.  Following this, the dictionary words are compared against each sentence.  Since one of these two key components have been duplicated many times, the compute time for comparisons can be very taxing.

For an example using the traditional approach, lets take a single ROT ciphered sentence with 8 words.  Rotating the entire sentence 26 times (for each possible ROT) involves the program going through Char->Int->Int+1->Char for each letter in the sentence.  The 26 rotations will result in 208 comparisons for each word included in the dictionary list.  Using a Scrabble Dictionary list, with almost 275 thousand words, will result in over 50 million comparisons.  This lengthy compute time can especially be seen when working with thousands of unique ROT ciphered sentences.

### Unique Approach
The unique approach of this program starts by taking the dictionary list and converting every word into an array of numerical 'distances'.  Each letter in the word requires the program to go from Char->Int and calculates the 'distance' between the neighboring letter's integer.  For example, the word "the", will have an array of [14,23] while the word "and" will have an array of [13,16].  Next, the program takes the ROT sentence (again, we'll use an example of 8 words), and finds the distances for those words as well.  Lastly, the program compares all 275 thousand dictionary arrays and compares them to the 8 ROT-sentence array for a total of just over 2 million comparisons.

The benefits of the much lowered number of comparisons greatly out-way the drawback of upfront compute cost when converting the dictionary list at the start of the program.  Although these numbers above are estimates (and the word "comparison" is used loosely), I firmly believe that this approach is much faster when attempting to solve many ROT-ciphered sentences.

## Usage
Change the following lines to reflect the file locations of each:

#### Python
```
load_dictionary("Dictionary.txt")
cipher_file_in = open("ROTSentences.txt", "r")
plaintext_file_out = open("Plaintext.txt", "w")
```
#### Rust
```
load_dictionary(("../Dictionary.txt").to_string(), &mut dictionary_list);
let cipher_file_in = File::open("../ROTSentences.txt").unwrap();
let mut plaintext_file_out = File::create("Plaintext.txt").unwrap();
```

## Benchmark
As a (rough) comparison, the Python script was profiled using cProfiler and completed in 4.893 seconds.  The Rust release was profiled using Windows Performance Recorder & Analyzer and it completed in 0.707 seconds.

Notes:
- These benchmarks were conducted on a Ryzen 5 2600.  Both scripts solved the same 1024 ROT-ciphered sentences (of varying length) correctly.
- Refining the idea and optimizing the Python script reduced computation time, for the same ciphered sentences tested above, from 2 minutes down to under 5 seconds.  While I won't be able to make nearly the same *logistical* gains with Rust, it's still not optimized and I'm confident I can make it more efficient.

## Roadmap
- [ ] Add ability to change ROT alphabet (numbers & special characters).  Currently limited to A-Z.
- [ ] Check for Rust performance enhancements using HashMap alternatives.

## Author
The contents of this repository were created by [Cole Chapman](https://github.com/Endrem/).

## License
"ROTX-Solver" is made available under the [MIT License](https://choosealicense.com/licenses/mit/).
