extern crate task_parallelism;

extern crate rand;

use self::rand::thread_rng;
use self::rand::distributions::{IndependentSample, Range};

use task_parallelism::{TaskMaster, TaskMultiplexer};
use std::iter::{IteratorExt};
use std::sync::Arc;
use std::ops::Deref;

const TEST_TASKS: usize = 4;

fn mul(i: Arc<i32>) -> Box<Fn(&i32) -> i32 + Send> {
  Box::new(move |x: &i32| {
    i.deref()*x
  } )
}

#[test]
fn test_works() {
  let mut rng = thread_rng();
  let test_range: Range<i32> = Range::new(2, 100);

  let multipliers: Vec<Box<Fn(&i32) -> i32 + Send>> = (1..1000).map(|_| mul(Arc::new(test_range.ind_sample(&mut rng))))
                                                               .take(1000)
                                                               .collect();

  let test_val: i32 = test_range.ind_sample(&mut rng);

  let mut pool: TaskMaster = TaskMaster::new(TEST_TASKS);
  let mut expected_result: Vec<i32> = multipliers.iter().map(|task| task(&test_val)).collect();
  expected_result.sort();

  println!("Generated: {:?}", expected_result);

  let mut pool_result: Vec<i32> = pool.in_parallel(&test_val, multipliers);
  pool_result.sort();

  assert_eq!(expected_result, pool_result);
}
