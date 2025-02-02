// Copyright 2024 The Bazel examples and tutorials Authors & Contributors. // All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use tokio;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting the tokio example program");

    // Create three async tasks
    let task1 = tokio::spawn(async_timer(1, "Task 1"));
    let task2 = tokio::spawn(async_timer(2, "Task 2"));
    let task3 = tokio::spawn(async_timer(3, "Task 3"));

    // Wait for all tasks to complete
    let _ = tokio::join!(task1, task2, task3);

    println!("All tasks completed");
}

async fn async_timer(seconds: u64, task_name: &str) {
    println!("{} started", task_name);
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    println!("{} finished after {} second(s)", task_name, seconds);
}