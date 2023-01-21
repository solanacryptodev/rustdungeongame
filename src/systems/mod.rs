use crate::prelude::*;

// An execution plan for our systems of which there could be hundreds. Systems allow you to organize
// your code into logical chunks that can be executed in parallel.
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .build()
}