#![feature(test, std_misc, thread_sleep)]
extern crate task_parallelism;

extern crate rand;
extern crate test;

use self::rand::thread_rng;
use self::rand::distributions::{IndependentSample, Range};
use self::test::Bencher;

use task_parallelism::TaskMaster;
use std::iter::{IteratorExt};
use std::sync::Arc;
use std::ops::Deref;

fn mul(i: Arc<f32>) -> Box<Fn(&f32) -> f32 + Send> {
  Box::new(move |x: &f32| {
    std::thread::sleep(std::time::duration::Duration::nanoseconds(500000));
    i.deref() * x
  } )
}

#[bench]
fn in_parallel_2_sleep(bench: &mut Bencher) {
  in_parallel(bench, 2)
}

#[bench]
fn in_parallel_4_sleep(bench: &mut Bencher) {
  in_parallel(bench, 4)
}

#[bench]
fn in_parallel_8_sleep(bench: &mut Bencher) {
  in_parallel(bench, 8)
}

#[bench]
fn in_parallel_16_sleep(bench: &mut Bencher) {
  in_parallel(bench, 16)
}

#[bench]
fn in_parallel_32_sleep(bench: &mut Bencher) {
  in_parallel(bench, 32)
}

#[bench]
fn in_serial_sleep(bench: &mut Bencher) {
  bench.iter(|| {
    let mut rng = thread_rng();
    let test_range: Range<f32> = Range::new(2.0, 100.0);

    let multipliers: Vec<Box<Fn(&f32) -> f32 + Send>> = (1..32).map(|_| mul(Arc::new(test_range.ind_sample(&mut rng))))
                                                                 .collect();
    let test_val: f32 = test_range.ind_sample(&mut rng);
    let expected_result: Vec<f32> = multipliers.iter().map(|task| task(&test_val)).collect();

    test::black_box(&expected_result);
  });
}

fn in_parallel(bench: &mut Bencher, thread_count: usize) {
  let mut pool: TaskMaster = TaskMaster::new(thread_count);

  bench.iter(|| {
    let mut rng = thread_rng();
    let test_range: Range<f32> = Range::new(2.0, 100.0);

    let multipliers: Vec<Box<Fn(&f32) -> f32 + Send>> = (1..32).map(|_| mul(Arc::new(test_range.ind_sample(&mut rng))))
                                                                 .collect();
    let test_val: f32 = test_range.ind_sample(&mut rng);

    let pool_result: Vec<f32> = pool.in_parallel(&test_val, multipliers);

    test::black_box(&pool_result);
  });
}
