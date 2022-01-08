use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env;
use near_sdk::near_bindgen;

pub use crate::events::*;

mod events;

pub const TODO_METADATA_SPEC: &str = "unknown";
pub const TODO_STANDARD_NAME: &str = "unknown";

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Task {
    id: u8,
    content: String,
    completed: bool,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ToDoListV1 {
    tasks: LookupMap<u8, Task>,
    task_counter: u8,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ToDoList {
    tasks: LookupMap<u8, Task>,
    task_counter: u8,
    participants_interactions: LookupMap<String, u8>,
}

impl Default for ToDoList {
    fn default() -> Self {
        let task: Task = Task {
            id: 0,
            content: "Default task".to_string(),
            completed: false,
        };
        let mut tasks: LookupMap<u8, Task> = LookupMap::new(0);
        tasks.insert(&0, &task);
        let mut participants_init: LookupMap<String, u8> = LookupMap::new(0);
        participants_init.insert(&"Admin".to_string(), &1);
        Self {
            tasks,
            task_counter: 1,
            participants_interactions: participants_init,
        }
    }
}

#[near_bindgen]
impl ToDoList {
    /*
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        let task: Task = Task {
            id: 0,
            content: "Default task".to_string(),
            completed: false,
        };
        let mut tasks: LookupMap<u8, Task> = LookupMap::new(0);
        tasks.insert(&0, &task);
        Self {
            tasks,
            task_counter: 1,
        }
    }
    */

    #[payable]
    pub fn create_task(&mut self, _content: String) {
        let _completed = false;
        let task: Task = Task {
            id: self.task_counter,
            content: _content,
            completed: _completed,
        };

        self.tasks.insert(&self.task_counter, &task);
        //define task creation event
        let _create_task_log: EventLog = EventLog {
            standard: TODO_STANDARD_NAME.to_string(),
            version: TODO_METADATA_SPEC.to_string(),
            event: EventLogVariant::CreateTask(vec![CreateTaskLog {
                id: self.task_counter,
                content: task.content,
                completed: _completed,
            }]),
        };
        self.task_counter = self.task_counter + 1;

        //Add participant...A V2 functionality
        let message_sender = env::signer_account_id();
        if self.participants_interactions.contains_key(&message_sender) {
            let n_interaction = self.participants_interactions.get(&message_sender).unwrap() + 1;
            self.participants_interactions
                .insert(&message_sender, &n_interaction);
        } else {
            // First interaction
            self.participants_interactions.insert(&message_sender, &1);
        }
        // End of V2 functionality
        //emit task creation event
        env::log(&_create_task_log.to_string().as_bytes());
    }

    #[payable]
    pub fn toggle_completed(&mut self, id: u8) {
        let _task: Option<Task> = self.tasks.get(&id);
        assert_eq!(_task.is_some(), true, "Given ID does not exist!");
        let mut _unwrapped_task: Task = _task.unwrap();

        _unwrapped_task.completed = !_unwrapped_task.completed;

        self.tasks.insert(&id, &_unwrapped_task);

        //define toggle completed event
        let _toggle_completed_log: EventLog = EventLog {
            standard: TODO_STANDARD_NAME.to_string(),
            version: TODO_METADATA_SPEC.to_string(),
            event: EventLogVariant::ToggleCompleted(vec![ToggleCompletedLog {
                id: id,
                completed: _unwrapped_task.completed,
            }]),
        };
        //emit toggle completed event
        env::log(&_toggle_completed_log.to_string().as_bytes());
    }

    pub fn get_participations(&self) -> u8 {
        assert_eq!(
            self.participants_interactions
                .contains_key(&env::signer_account_id()),
            true,
            "You did not interact with the contract yet"
        );
        self.participants_interactions
            .get(&env::signer_account_id())
            .unwrap()
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: ToDoListV1 = env::state_read().expect("failed");
        Self {
            tasks: old_state.tasks,
            task_counter: old_state.task_counter,
            participants_interactions: LookupMap::new(b"b".to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    // simple helper function to take a string literal and return a ValidAccountId
    fn to_valid_account(account: &str) -> ValidAccountId {
        ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn create_new_task() {
        // set up the mock context into the testing environment
        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());

        let mut contract = ToDoList::default();

        assert_eq!(1, contract.task_counter);

        contract.create_task("Play with the dog".to_string());
        println!(
            "Number of tasks in the ToDo List: {}",
            contract.task_counter
        );
        assert_eq!(2, contract.task_counter);

        contract.create_task("".to_string());
        println!(
            "Number of tasks in the ToDo List: {}",
            contract.task_counter
        );
        assert_eq!(3, contract.task_counter);
    }

    #[test]
    fn toggle_task_status() {
        // set up the mock context into the testing environment
        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());
        let task_id: u8 = 1;
        let mut contract = ToDoList::default();

        contract.create_task("Play with the dog".to_string());
        println!(
            "Task id {} has status {}",
            task_id,
            contract.tasks.get(&task_id).unwrap().completed
        );
        assert_eq!(false, contract.tasks.get(&1).unwrap().completed);

        println!("Toggle task id {}", task_id);
        contract.toggle_completed(task_id);
        println!(
            "Task id {} has status {}",
            1,
            contract.tasks.get(&1).unwrap().completed
        );
        assert_eq!(true, contract.tasks.get(&1).unwrap().completed);
    }

    #[test]
    #[should_panic(expected = r#"Given ID does not exist!"#)]
    fn toggle_wrong_id() {
        // set up the mock context into the testing environment
        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());
        let task_id: u8 = 5;
        let mut contract = ToDoList::default();

        contract.toggle_completed(task_id);
    }

    #[test]
    fn test_interactions() {
        // set up the mock context into the testing environment
        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());
        let mut contract = ToDoList::default();

        contract.create_task("First task of the day".to_string());
        assert_eq!(1, contract.get_participations());
        contract.create_task("Second task of the day".to_string());
        assert_eq!(2, contract.get_participations());

        println!("{}", env::signer_account_id());
    }

    #[test]
    #[should_panic(expected = r#"You did not interact with the contract yet"#)]
    fn test_no_interactions() {
        // set up the mock context into the testing environment
        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());
        let contract = ToDoList::default();

        contract.get_participations();
    }
}
