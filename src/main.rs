extern crate mpi;

use mpi::topology::Rank;
use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank() as usize;
    let size = world.size() as usize;
    let root_rank = 0i32;
    let repeat = 2 as usize;
    let root_process = world.process_at_rank(root_rank);

    let mut x = vec![0;repeat as usize];
    if rank == root_rank as usize {
        let v = (0..size*repeat).collect::<Vec<_>>();
        root_process.scatter_into_root(&v[0..2], &mut x[..]);
    } else {
        root_process.scatter_into(&mut x[..]);
    }
    println!("{:?}", x);
}