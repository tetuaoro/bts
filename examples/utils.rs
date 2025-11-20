use std::{cmp::Ordering, io::Write, sync::Mutex};

pub const CAPACITY: usize = 5;
type T = ((usize, (usize, usize, usize)), f64);

pub struct SharedResults {
    total_balances: Mutex<Vec<T>>,
    errors: Mutex<Vec<T>>,
    current_iter: Mutex<usize>,
    total_iterations: usize,
}

impl SharedResults {
    pub fn new(total: usize) -> Self {
        Self {
            total_balances: Mutex::new(Vec::with_capacity(CAPACITY)),
            errors: Mutex::new(Vec::with_capacity(CAPACITY)),
            current_iter: Mutex::new(0),
            total_iterations: total,
        }
    }

    pub fn total_balances(&self) -> &Mutex<Vec<T>> {
        &self.total_balances
    }

    pub fn errors(&self) -> &Mutex<Vec<T>> {
        &self.errors
    }

    pub fn push_result(&self, item: T, is_error: bool) {
        let mut guard = if is_error {
            self.errors.lock().unwrap()
        } else {
            self.total_balances.lock().unwrap()
        };

        guard.push(item);
        guard.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(Ordering::Equal));
        if guard.len() > CAPACITY {
            guard.truncate(CAPACITY);
        }
    }

    pub fn increment_iter(&self) -> usize {
        let mut iter = self.current_iter.lock().unwrap();
        *iter += 1;
        *iter
    }

    pub fn print_progress(&self) {
        let iter = *self.current_iter.lock().unwrap();
        let progress = (iter as f64 / self.total_iterations as f64) * 100.0;
        print!("\rProgress: {progress:.2}% ({iter}/{})", self.total_iterations);
        std::io::stdout().flush().unwrap();
    }
}

#[allow(dead_code)]
fn main() {}
