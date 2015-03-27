#![feature(test)]
extern crate task_parallelism;
extern crate test;

use self::test::Bencher;
use task_parallelism::{TaskMaster, TaskMultiplexer};

#[bench]
fn pool_spawning_2(bench: &mut Bencher) {
  bench.iter(|| {
    let pool: TaskMaster = TaskMaster::new(2);

    test::black_box(&pool);
  });
}


#[bench]
fn pool_spawning_4(bench: &mut Bencher) {
  bench.iter(|| {
    let pool: TaskMaster = TaskMaster::new(4);

    test::black_box(&pool);
  });
}

#[bench]
fn pool_spawning_8(bench: &mut Bencher) {
  bench.iter(|| {
    let pool: TaskMaster = TaskMaster::new(8);

    test::black_box(&pool);
  });
}

#[bench]
fn pool_spawning_16(bench: &mut Bencher) {
  bench.iter(|| {
    let pool: TaskMaster = TaskMaster::new(16);

    test::black_box(&pool);
  });
}
