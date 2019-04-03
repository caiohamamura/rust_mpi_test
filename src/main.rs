#![deny(warnings)]
extern crate mpi;

use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let rank = world.rank() as usize;
    let count = world.size() as usize;
    let repeat : usize = 2;
    let vec_res = vec![false;repeat];

    if rank == 0 {
        let mut a = vec![false; count*repeat];
        world.process_at_rank(0).gather_into_root(&(vec_res)[..], &mut a[..]);
        println!("{:?}", a);
    } else {
        world.process_at_rank(0).gather_into(&(vec_res)[..]);
    }
    // let a : Vec<i32> = (0..100).collect();
    // println!("{:?}", a);
}