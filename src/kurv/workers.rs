// unshamelessly stolen from the pueue project (original name Children)
// it doesn't provide any advantage over using a simple HashMap or BTreeMap, for now
// but it might come in handy later on, if we want to implement running multiple workers/
// instances of an egg at the same time (like a cluster)

use command_group::GroupChild;
use std::collections::BTreeMap;

/// This structure is needed to manage worker pools for groups.
/// It's a newtype pattern around a nested BTreeMap, which implements some convenience functions.
///
/// The datastructure represents the following data:
/// BTreeMap<group_name, BTreeMap<group_worker_id, (egg_name, subprocess_handle)>
pub struct Workers(pub BTreeMap<String, BTreeMap<String, (usize, GroupChild)>>);

const DEFAULT_GROUP: &str = "default_kurv";

impl Workers {
    /// Creates a new worker pool with a single default group.
    pub fn new() -> Self {
        let mut pools = BTreeMap::new();
        pools.insert(String::from(DEFAULT_GROUP), BTreeMap::new());

        Workers(pools)
    }

    /// Returns whether there are any active tasks across all groups.
    pub fn has_active_tasks(&self) -> bool {
        self.0.iter().any(|(_, pool)| !pool.is_empty())
    }

    /// A convenience function to check whether there's child with a given task_id.
    /// We have to do a nested linear search, as these datastructure aren't indexed via task_ids.
    pub fn has_child(&self, task_id: usize) -> bool {
        for pool in self.0.values() {
            for (child_task_id, _) in pool.values() {
                if child_task_id == &task_id {
                    return true;
                }
            }
        }

        false
    }

    /// A convenience function to get a mutable child by its respective task_id.
    /// We have to do a nested linear search over all children of all pools,
    /// beceause these datastructure aren't indexed via task_ids.
    pub fn get_child_mut(&mut self, task_id: usize) -> Option<&mut GroupChild> {
        for pool in self.0.values_mut() {
            for (child_task_id, child) in pool.values_mut() {
                if child_task_id == &task_id {
                    return Some(child);
                }
            }
        }

        None
    }

    /// A convenience function to get a list with all task_ids of all children.
    pub fn all_task_ids(&self) -> Vec<usize> {
        let mut task_ids = Vec::new();
        for pool in self.0.values() {
            for (task_id, _) in pool.values() {
                task_ids.push(*task_id)
            }
        }

        task_ids
    }

    /// Inserts a new children into the worker pool of the given group.
    ///
    /// This function should only be called when spawning a new process.
    /// At this point, we're sure that the worker pool for the given group already exists, hence
    /// the expect call.
    pub fn add_child(
        &mut self,
        group: Option<&str>,
        worker_id: String,
        task_id: usize,
        child: GroupChild,
    ) {
        let group = group.unwrap_or(DEFAULT_GROUP);

        let pool = self
            .0
            .get_mut(group)
            .expect("The worker pool should be initialized when inserting a new child.");

        pool.insert(worker_id, (task_id, child));
    }
}
