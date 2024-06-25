// Copyright 2024 The Bazel examples and tutorials Authors & Contributors. // All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use bzip2::read::BzEncoder;
use bzip2::Compression;
use std::io::Read;

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut raw_counter = CountingStream::new(stdin);

    let compressed_count = {
        let compressor = BzEncoder::new(&mut raw_counter, Compression::Best);
        let mut compressed_counter = CountingStream::new(compressor);
        std::io::copy(&mut compressed_counter, &mut std::io::sink()).unwrap();
        compressed_counter.count
    };

    println!(
        "Compressed {} to {} bytes",
        raw_counter.count, compressed_count
    );
}

struct CountingStream<R: Read> {
    stream: R,
    count: usize,
}

impl<R: Read> CountingStream<R> {
    fn new(stream: R) -> Self {
        CountingStream { stream, count: 0 }
    }
}

impl<R: Read> Read for CountingStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let result = self.stream.read(buf);
        if let Ok(read_bytes) = result {
            self.count += read_bytes;
        }
        result
    }
}