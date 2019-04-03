extern crate mpi;

use mpi::topology::Rank;
use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size() as usize;
    let root_rank = 0;
    let repeat = 2 as usize;
    let root_process = world.process_at_rank(root_rank);

    let mut x = vec![0;repeat+1];
    if rank == root_rank {
        let v = (0..size*repeat).collect::<Vec<_>>();
        root_process.scatter_into_root(&v[..], &mut x[..]);
    } else {
        root_process.scatter_into(&mut x[..]);
    }
    println!("{:?}", x);
}