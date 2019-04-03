extern crate mpi;

use mpi::topology::Rank;
use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank() as i32;
    let size = world.size() as i32;
    let root_rank = 0i32;
    let repeat = 2 as i32;
    let root_process = world.process_at_rank(root_rank);

    let mut x = vec![0;repeat];
    if rank == root_rank {
        let v = (0..size*repeat).collect::<Vec<_>>();
        root_process.scatter_into_root(&v[rank*repeat..(rank+1)*repeat], &mut x[..]);
    } else {
        root_process.scatter_into(&mut x[..]);
    }
    println!("{:?}", x);
}