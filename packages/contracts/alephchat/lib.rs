#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate core;

mod chat;
mod message;

#[ink::contract]
mod alephchat {
    use ink::storage::Mapping;
    use crate::chat::{Chat, ChatCreateCommand};
    use crate::message::{Message, MessageCreateCommand};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct AlephChat {
        pub chats: Mapping<u64, Chat>,
        pub user_chats: Mapping<AccountId, Vec<u64>>,
        pub next_chat_id: u64,
    }


    impl AlephChat {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                chats: Mapping::default(),
                user_chats: Mapping::default(),
                next_chat_id: 0,
            }
        }

        #[ink(message)]
        pub fn init_chat(&mut self, init_chat: ChatCreateCommand) -> u64 {
            let chat = Chat::new(init_chat, self.next_chat_id);
            self.next_chat_id += 1;

            self.chats.insert(chat.id, &chat.clone());

            for i in chat.participants.iter() {
                //self.user_chats.get(i).unwrap().push(chat.id);
                match self.user_chats.get(i) {
                    None => {
                        let mut chats = Vec::<u64>::new();
                        chats.push(chat.id);
                        self.user_chats.insert(i, &chats);
                    }
                    Some(chats) => {
                        let mut new_chats = chats.clone();
                        new_chats.push(chat.id);
                        self.user_chats.insert(i, &new_chats);
                    }
                }
            }

            return chat.id;
        }

        #[ink(message)]
        pub fn add_message(&mut self, chat_id: u64, init_message: MessageCreateCommand) -> Message {
            let chat = self.chats.get(chat_id).unwrap();
            let next_message_id = chat.next_message_id as usize;
            let updated_chat = chat.add_message(init_message);
            self.chats.insert(chat_id, &updated_chat);
            updated_chat.messages.get(next_message_id).unwrap().clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;

    use crate::alephchat::AlephChat;
    use crate::chat::ChatCreateCommand;
    use crate::message::{InitMessage, MessageCreateCommand};

    #[ink::test]
    fn init_chat_works() {
        let mut aleph_chat = AlephChat::new();

        let mut participants = Vec::new();
        let participant_1 = AccountId::from([0; 32]);
        let participant_2 = AccountId::from([1; 32]);
        let participant_3 = AccountId::from([2; 32]);

        let mut participant_1_cypher = Vec::<u8>::new();
        participant_1_cypher.push(0);
        let mut participant_2_cypher = Vec::<u8>::new();
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
            InitMessage {
                init_message:
                MessageCreateCommand { author, message: message_content.clone() },
                encrypted_cypher,
            },
        };


        let chat_id = aleph_chat.init_chat(create_chat_command);
        let new_chat = aleph_chat.chats.get(chat_id).unwrap();

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

        assert_eq!(aleph_chat.user_chats.get(participant_1).unwrap().len(), 1);
        assert_eq!(aleph_chat.user_chats.get(participant_2).unwrap().len(), 1);
        assert_eq!(aleph_chat.user_chats.get(participant_3).unwrap().len(), 1);
        assert_eq!(aleph_chat.user_chats.get(participant_1).unwrap().get(0).unwrap(), &chat_id);
        assert_eq!(aleph_chat.user_chats.get(participant_2).unwrap().get(0).unwrap(), &chat_id);
        assert_eq!(aleph_chat.user_chats.get(participant_3).unwrap().get(0).unwrap(), &chat_id);
    }

    #[ink::test]
    fn add_message_works() {
        let mut aleph_chat = AlephChat::new();

        let mut participants = Vec::new();
        let participant_1 = AccountId::from([0; 32]);
        let participant_2 = AccountId::from([1; 32]);
        let participant_3 = AccountId::from([2; 32]);

        let mut participant_1_cypher = Vec::<u8>::new();
        participant_1_cypher.push(0);
        let mut participant_2_cypher = Vec::<u8>::new();
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
            InitMessage {
                init_message:
                MessageCreateCommand { author, message: message_content.clone() },
                encrypted_cypher,
            },
        };

        let chat_id = aleph_chat.init_chat(create_chat_command);
        let new_message_content = "Goodbye world!".to_string();

        let new_message = MessageCreateCommand {
            author: participant_2,
            message: new_message_content.clone(),
        };

        let returned_message = aleph_chat.add_message(chat_id.clone(), new_message);

        assert_eq!(returned_message.message, new_message_content);
        assert_eq!(returned_message.author, participant_2);
        assert_eq!(returned_message.chat_id, chat_id);

        let updated_chat = aleph_chat.chats.get(chat_id).unwrap();

        assert_eq!(updated_chat.id, id);
        assert_eq!(updated_chat.participants.len(), participants.len());
        assert_eq!(updated_chat.participants.contains(&participant_1), true);
        assert_eq!(updated_chat.participants.contains(&participant_2), true);
        assert_eq!(updated_chat.participants.contains(&participant_3), true);
        assert_eq!(updated_chat.encrypted_cypher.get(0).unwrap(), &participant_1_cypher);
        assert_eq!(updated_chat.encrypted_cypher.get(1).unwrap(), &participant_2_cypher);
        assert_eq!(updated_chat.encrypted_cypher.get(2).unwrap(), &participant_3_cypher);

        eprint!("{}", updated_chat.messages.get(0).unwrap().message);
        eprint!("{}", updated_chat.messages.get(1).unwrap().message);


        assert_eq!(updated_chat.messages.len(), 2);
        assert_eq!(updated_chat.messages.get(0).unwrap().message, message_content);
        assert_eq!(updated_chat.next_message_id, 2);

        assert_eq!(aleph_chat.user_chats.get(participant_1).unwrap().len(), 1);
        assert_eq!(aleph_chat.user_chats.get(participant_2).unwrap().len(), 1);
        assert_eq!(aleph_chat.user_chats.get(participant_3).unwrap().len(), 1);
        assert_eq!(aleph_chat.user_chats.get(participant_1).unwrap().get(0).unwrap(), &chat_id);
        assert_eq!(aleph_chat.user_chats.get(participant_2).unwrap().get(0).unwrap(), &chat_id);
        assert_eq!(aleph_chat.user_chats.get(participant_3).unwrap().get(0).unwrap(), &chat_id);
    }
}
