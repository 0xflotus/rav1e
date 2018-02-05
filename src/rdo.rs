// Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved
// Copyright (c) 2017, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(non_camel_case_types)]

// Sum of Squared Error for a 64x64 block
pub fn sse_64x64(a: &Vec<u16>, b: &Vec<u16>, x: usize, y: usize, stride: usize) -> u64 {
    let mut sse: u64 = 0;
    for j in 0..64 {
        for i in 0..64 {
            let pos = (y+j)*stride + x+i;
            let dist = (a[pos] as i16 - b[pos] as i16) as i64;
            sse += (dist * dist) as u64;
        }
    }
    sse
}
