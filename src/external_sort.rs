// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::fs::File;
use std::io::{Read, Write};

const B: usize = 36;
const M: usize = 8;

/// Sorts a file of size B blocks on external storage, using 4*B IOs.
pub fn sort_merge_join(path: &str) {
    let mut f = File::open(path).unwrap();
    let mut buffers = [[0u8; 4096]; M];
    let runs = (B + M - 1) / M;

    if runs > M - 1 {
        panic!("file too large for Multi-Way Merge-Sort");
    }

    sort_runs(&mut f, &mut buffers, runs);

    // TODO Merge and Join
}

/// Sorts a file of size B blocks on external storage, using 4*B IOs.
pub fn multiway_merge_sort(path: &str) {
    let mut f = File::open(path).unwrap();
    let mut buffers = [[0u8; 4096]; M];
    let runs = (B + M - 1) / M;

    if runs > M - 1 {
        panic!("file too large for Multi-Way Merge-Sort");
    }

    sort_runs(&mut f, &mut buffers, runs);

    // Merge sorted runs
    let mut out = File::create("out.txt").unwrap();
    let mut f_run = Vec::new();
    for run in 0..runs {
        f_run.push(File::open(format!("temp{}.txt", run)).unwrap());
        f_run[run].read(&mut buffers[run]).unwrap();
    }
    let mut pos = [0usize; M - 1];
    let mut done = [false; M - 1];
    let mut done_with = 0;
    let mut out_filled = 0;
    loop {
        // find min
        let mut min_i = 0;
        let mut min = 255;
        for i in 0..runs {
            if !done[i] && buffers[i][pos[i]] <= min {
                min_i = i;
                min = buffers[i][pos[i]];
            }
        }

        // read new block if necessary
        pos[min_i] += 1;
        if pos[min_i] == 4096 {
            if f_run[min_i].read(&mut buffers[min_i]).unwrap() == 0 {
                done[min_i] = true;
                done_with += 1;
                if done_with == runs {
                    break;
                }
            }
            pos[min_i] = 0;
        }

        // write to out buffer
        buffers[M - 1][out_filled] = min;
        out_filled += 1;
        if out_filled == 4096 {
            out.write(&buffers[M - 1]).unwrap();
            out_filled = 0;
        }
    }

    if out_filled > 0 {
        out.write(&buffers[M - 1][..out_filled + 1]).unwrap();
    }
    out.flush().unwrap();
}

fn sort_runs(f: &mut File, buffers: &mut [[u8; 4096]; M], runs: usize) {
    for run in 0..runs {
        for i in 0..M {
            if f.read(&mut buffers[i]).unwrap() == 0 {
                break;
            }
        }

        let mut sorted = buffers.concat();
        sorted.sort();

        let mut out = File::create(format!("temp{}.txt", run)).unwrap();
        out.write(&sorted).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mwms() {
        multiway_merge_sort("1.txt");
    }
}
