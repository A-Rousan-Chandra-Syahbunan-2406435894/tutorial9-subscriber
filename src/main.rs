use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        std::thread::sleep(std::time::Duration::from_secs(1)); 
        println!("In Rousan’s Computer [2406435894]. Message received: {:?}", message);
        Ok(())
    }
    fn get_handler_action(&self) -> String { "".to_string() }
}

fn main() {

    let listener = CrosstownBus::new_queue_listener("amqps://ojzbiupq:BQhrSm_-y4CCijnHLuB9gyOzzyuZPgXp@armadillo.rmq.cloudamqp.com/ojzbiupq".to_owned()).unwrap();
    _ = listener.listen("user_created".to_owned(), UserCreatedHandler{},
        crosstown_bus::QueueProperties { auto_delete: false, durable: false, use_dead_letter: true }
    );
    loop { std::thread::sleep(std::time::Duration::from_millis(100)); }
}