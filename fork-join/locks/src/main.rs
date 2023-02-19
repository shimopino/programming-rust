mod mutex_lock;
mod thread_lock;

use crate::mutex_lock::locks_in_mutex;
use crate::thread_lock::locks_in_thread_mutex;

fn main() {
    locks_in_mutex();

    locks_in_thread_mutex();
}
