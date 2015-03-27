extern crate simple_parallel;

use simple_parallel::Pool;
use std::ops::Fn;

pub struct TaskMaster {
  pool: Pool
}

impl TaskMaster {
  pub fn new(thread_count: usize) -> TaskMaster {
    TaskMaster { pool: Pool::new(thread_count) }
  }

  pub fn in_parallel<I: Sync, O: Send, F: ?Sized + Send + Fn(&I) -> O>
    (&mut self, data: &I, tasks: Vec<Box<F>>)
    -> Vec<O>
  {
    let apply = |task: Box<F>| task(data);
    let results = self.pool.unordered_map(tasks, &apply).map(|(_, result)| result);
    results.collect()
  }
}
