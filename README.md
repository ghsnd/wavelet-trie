# Wavelet trie
[![Build Status](https://travis-ci.org/ghsnd/wavelet-trie.svg?branch=master)](https://travis-ci.org/ghsnd/wavelet-trie)

A wavelet trie implementation in Rust, based on the paper by Grossi et al. [1] ([link](https://arxiv.org/abs/1204.3581)).

In short, it is a succinct data structure that allows fast prefix-search on _sequences_ of binary strings.
Note that the strings have to be prefix-free, i.e., no string can be a prefix of another. Append a terminator symbol
to each string to avoid this problem. 

Documentation and examples are coming up in the near future. Until then, take a look at
the [tests](https://github.com/ghsnd/wavelet-trie/blob/master/src/wavelet_trie/tests.rs) to see how to use it.

[1] Grossi, Roberto & Ottaviano, Giuseppe. (2012). _The Wavelet Trie: Maintaining an Indexed Sequence of Strings in
Compressed Space_. Proceedings of the ACM SIGACT-SIGMOD-SIGART Symposium on Principles of Database Systems. . 10.1145/2213556.2213586.

## Examples


A simple example with strings:

```rust
extern crate wavelet_trie;

use wavelet_trie::wavelet_trie::WaveletTrie;

// create a wavelet trie with two strings:
let mut wt = WaveletTrie::new();
wt.append_str("Hello world!");
wt.append_str("Hello everybody!");

// See on which positions the strings start with "Hell"
assert_eq!(vec![0, 1], wt.select_all_str("Hell"));

// get the string at position 1
assert_eq!("Hello everybody!", wt.access_str(1).unwrap());
```

Creating a wavelet trie from 'static' binary content, and performing some queries:

```rust
extern crate dyn_bit_vec;
extern crate wavelet_trie;

use dyn_bit_vec::DBVec;
use wavelet_trie::wavelet_trie::WaveletTrie;

// We start with some binary sequences to put into the wavelet trie
let sequence1 = DBVec::from_bytes(&[0b00001000]);
let sequence2 = DBVec::from_bytes(&[0b10000000]);
let sequence3 = DBVec::from_bytes(&[0b10000100]);
let sequence4 = DBVec::from_bytes(&[0b11000100, 0b10000000]);

// Here we create a wavelet trie from these sequences
let wt = WaveletTrie::from_sequences(&[sequence1, sequence2, sequence3, sequence4]);

// There should be 4 sequences in the trie now:
assert_eq!(4, wt.len());

// Let's see at which positions prefix '001' occurs (should be 2 and 3).
let mut prefix_001 = DBVec::new();
prefix_001.push(false);
prefix_001.push(false);
prefix_001.push(true);
assert_eq!(vec![2, 3], wt.select_all(&prefix_001));
```

## Features at this moment
* Dynamic: insert or delete a string at any position
* Fast (prefix) count
* Fast (prefix) search

## Features planned
* Exact count & search
* Helper methods to work with texts.
* Range methods
* Many optimisations!

## A note on performance
Since this library heavily depends on the CPU feature [POPCNT](https://en.wikipedia.org/wiki/SSE4#POPCNT_and_LZCNT), it might be a good idea to include a `.cargo/config`file in your project and enable certain compiler flags and boost performance. The commented-out example in this repository optimises the code for the CPU it is compiled on. This makes the resulting binary not very portable though.

See also <https://doc.rust-lang.org/cargo/reference/config.html>.
