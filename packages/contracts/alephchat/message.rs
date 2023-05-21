use ink::env;
use ink::env::DefaultEnvironment;
use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::{
    prelude::vec::Vec,
};


#[derive(scale::Decode, scale::Encode, Clone)]
#[cfg_attr(
feature = "std",
derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Message {
    pub id: u64,
    pub chat_id: u64,
    pub author: AccountId,
    pub message: ink::prelude::string::String,
    pub timestamp: u64,
}

#[derive(scale::Decode, scale::Encode, Clone)]
#[cfg_attr(
feature = "std",
derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct MessageCreateCommand {
    pub author: AccountId,
    pub message: String,
}

impl Message {
    pub fn new(init_message: MessageCreateCommand, id: u64, chat_id: u64) -> Self {
        Self {
            id,
            chat_id,
            author: init_message.author,
            message: init_message.message,
            timestamp: env::block_timestamp::<DefaultEnvironment>()
        }
    }
}

#[derive(scale::Decode, scale::Encode, Clone)]
#[cfg_attr(
feature = "std",
derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct InitMessage {
    pub init_message: MessageCreateCommand,
    pub encrypted_cypher: Vec<Vec<u8>>
}

impl InitMessage {
    pub fn new(init_message: MessageCreateCommand, encrypted_cypher: Vec<Vec<u8>>) -> Self {
        Self {
            init_message,
            encrypted_cypher
        }
    }
}

#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;
    use crate::message::{Message, MessageCreateCommand};

    #[ink::test]
    fn new_message_works() {
        let author = AccountId::from([0; 32]);
        let message_content = "Hello world!".to_string();
        let id = 0;
        let chat_id = 1;
        let new_message =
            Message::new(MessageCreateCommand{author, message: message_content.clone()}, id, chat_id);

        assert_eq!(new_message.id, id);
        assert_eq!(new_message.chat_id, chat_id);
        assert_eq!(new_message.author, author);
        assert_eq!(new_message.message, message_content);
    }

}
