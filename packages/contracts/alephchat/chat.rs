use ink::primitives::AccountId;
use crate::message::{InitMessage, Message, MessageCreateCommand};
use ink::{
    env::Environment,
    prelude::vec::Vec,
};


#[derive(scale::Decode, scale::Encode, Clone)]
#[cfg_attr(
feature = "std",
derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Chat {
    pub id: u64,
    pub participants: Vec<AccountId>,
    pub encrypted_cypher: Vec<Vec<u8>>,
    pub messages: Vec<Message>,
    pub next_message_id: u64,
}



#[derive(scale::Decode, scale::Encode, Clone)]
#[cfg_attr(
feature = "std",
derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct ChatCreateCommand {
    pub participants: Vec<AccountId>,
    pub init_message: InitMessage,
}

impl Chat {
    pub fn new(init_chat: ChatCreateCommand, id: u64) -> Self {
        let messages = Vec::new();
        let chat = Self {
            id,
            participants: init_chat.participants,
            encrypted_cypher: init_chat.init_message.encrypted_cypher,
            messages,
            next_message_id: 0,
        };

        return chat.add_message(init_chat.init_message.init_message);
    }

    pub fn add_message(mut self, message: MessageCreateCommand) -> Chat {
        self.messages.push(Message::new(message, self.next_message_id, self.id));
        self.next_message_id += 1;
        return self;
    }

    pub fn get_cypher(self, account: AccountId) -> Vec<u8> {
        for (it, i) in self.participants.iter().enumerate() {
            if i.eq(&account) {
                return self.encrypted_cypher.get(it).unwrap().clone();
            }
        }
        panic!("something went wrong");
    }

}

#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;

    use crate::chat::{Chat, ChatCreateCommand};
    use crate::message::{InitMessage, MessageCreateCommand};

    #[ink::test]
    fn new_chat_works() {
        let mut participants = Vec::new();
        let participant_1 = AccountId::from([0; 32]);
        let participant_2 = AccountId::from([1; 32]);
        let participant_3 = AccountId::from([2; 32]);

        let mut participant_1_cypher = Vec::<u8>::new();
        participant_1_cypher.push(0);
        let mut participant_2_cypher =Vec::<u8>::new();
        participant_2_cypher.push(1);
        let mut participant_3_cypher = Vec::<u8>::new();
        participant_3_cypher.push(2);

        participants.push(participant_1);
        participants.push(participant_2);
        participants.push(participant_3);

        let mut encrypted_cypher = Vec::default();
        encrypted_cypher.insert(0, participant_1_cypher.clone());
        encrypted_cypher.insert(1, participant_2_cypher.clone());
        encrypted_cypher.insert(2, participant_3_cypher.clone());


        let author = AccountId::from([0; 32]);
        let message_content = "Hello world!".to_string();
        let id = 0;

        let create_chat_command = ChatCreateCommand {
            participants: participants.clone(),
            init_message:
            InitMessage { init_message:
            MessageCreateCommand { author, message: message_content.clone() }, encrypted_cypher }
        };


        let new_chat =
            Chat::new(create_chat_command, id);

    assert_eq!(new_chat.id, id);
    assert_eq!(new_chat.participants.len(), participants.len());
    assert_eq!(new_chat.participants.contains(&participant_1), true);
    assert_eq!(new_chat.participants.contains(&participant_2), true);
    assert_eq!(new_chat.participants.contains(&participant_3), true);
    assert_eq!(new_chat.encrypted_cypher.get(0).unwrap(), &participant_1_cypher);
    assert_eq!(new_chat.encrypted_cypher.get(1).unwrap(), &participant_2_cypher);
    assert_eq!(new_chat.encrypted_cypher.get(2).unwrap(), &participant_3_cypher);
    assert_eq!(new_chat.messages.len(), 1);
    assert_eq!(new_chat.messages.get(0).unwrap().message, message_content);
    assert_eq!(new_chat.next_message_id, id + 1);
}

}
