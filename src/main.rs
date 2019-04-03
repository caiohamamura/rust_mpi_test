extern crate mpi;

use mpi::datatype::Partition;
use mpi::traits::*;
use mpi::Count;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let rank = world.rank();
    let size = world.size();
    let repeat = 2;

    let root_rank = 0;
    let root_process = world.process_at_rank(root_rank);

    let mut buf = vec![0; repeat as usize];

    if rank == root_rank {
        let msg: Vec<_> = (0..size*repeat).collect();
        let counts: Vec<Count> = vec![2;size as usize];
        let displs: Vec<Count> = counts
            .iter()
            .scan(0, |acc, &x| {
                let tmp = *acc;
                *acc += x;
                Some(tmp)
            })
            .collect();
        println!("msg: {:?}", msg);
        println!("counts: {:?}", counts);
        println!("displs: {:?}", displs);
        let partition = Partition::new(&msg[..], counts, &displs[..]);
        root_process.scatter_varcount_into_root(&partition, &mut buf[..]);
    } else {
        root_process.scatter_varcount_into(&mut buf[..]);
    }

    println!("Process {} got message: {:?}", rank, buf);
}