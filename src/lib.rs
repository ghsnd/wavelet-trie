// Copyright 2017 Gerald Haesendonck
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// **wavelet-trie**, a library that implements the wavelet trie data structure,
/// based on the paper _The Wavelet Trie: Maintaining an Indexed Sequence of Strings in
/// Compressed Space_ by Grossi et al ([link](https://arxiv.org/abs/1204.3581)).
///
/// A wavelet trie is a succinct data structure that holds a _sequence_ of binary strings
/// (and thus also text strings, numbers, ... in short, anything that can be
/// represented as bits or bytes). The strings have to be prefix-free, meaning
/// that one string cannot be prefix of another, but this can be avoided by
/// appending a terminator symbol to each string. Strings can occur multiple times.
/// It provides efficient prefix search.
///
/// A wavelet trie is a generalisation of a wavelet tree: the alphabet does
/// not need to be known in advance; it is created dynamically as strings are
/// inserted. This implementation is dynamic in the sense that strings can be
/// inserted and deleted at any position in the trie.
///
/// Situations where a wavelet trie is a good choice:
///
/// * many strings share prefixes;
/// * many occurrences of the same string appear;
/// * fast prefix or exact search is required;
/// * the original sequence of strings needs to be reconstructed without storing the original "text"
///
/// **Note on this implementation:** While aiming to evolve to a good and
/// efficient implementation, the code as it is now is a beginning. There
/// is a lot of room for improvements.
/// For instance, it depends heavily on bitvector operations. The underlying
/// library is not designed for the requirements of a wavelet trie, so some
/// operations are very slow.
///
/// # Examples
///
/// Creating a wavelet trie from 'static' binary content, and performing some queries.
///
/// ```rust
/// extern crate dyn_bit_vec;
/// extern crate wavelet_trie;
///
/// use dyn_bit_vec::DBVec;
///	use wavelet_trie::wavelet_trie::WaveletTrie;
///
/// // We start with some binary sequences to put into the wavelet trie
/// let sequence1 = DBVec::from_bytes(&[0b00001000]);
/// let sequence2 = DBVec::from_bytes(&[0b10000000]);
/// let sequence3 = DBVec::from_bytes(&[0b10000100]);
/// let sequence4 = DBVec::from_bytes(&[0b11000100, 0b10000000]);
///
/// // Here we create a wavelet trie from these sequences
/// let wt = WaveletTrie::from_sequences(&[sequence1, sequence2, sequence3, sequence4]);
///
/// // There should be 4 sequences in the trie now:
/// assert_eq!(4, wt.len());
///
/// // Let's see at which positions prefix '001' occurs (should be 2 and 3).
/// let mut prefix_001 = DBVec::new();
/// prefix_001.push(false);
/// prefix_001.push(false);
/// prefix_001.push(true);
/// assert_eq!(vec![2, 3], wt.select_all(&prefix_001));
///
/// // 
/// ```

#[macro_use]
extern crate serde_derive;

pub mod wavelet_trie;
