use crate::{
    payloads,
    requests::JsonRequest,
    types::{MessageId, Recipient},
    Bot,
};

pub trait ApiExtend {
    fn send_message_to_thread<C, T>(
        &self,
        chat_id: C,
        message_thread_id: i32,
        text: T,
    ) -> JsonRequest<payloads::SendMessage>
    where
        C: Into<Recipient>,
        T: Into<String>;

    fn reply_to_message<C, T>(
        &self,
        chat_id: C,
        message_id: MessageId,
        text: T,
    ) -> JsonRequest<payloads::SendMessage>
    where
        C: Into<Recipient>,
        T: Into<String>;
}

impl ApiExtend for Bot {
    fn send_message_to_thread<C, T>(
        &self,
        chat_id: C,
        message_thread_id: i32,
        text: T,
    ) -> JsonRequest<payloads::SendMessage>
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        let mut msg = payloads::SendMessage::new(chat_id, text);
        msg.message_thread_id = Some(message_thread_id);
        JsonRequest::<payloads::SendMessage>::new(self.clone(), msg)
    }

    fn reply_to_message<C, T>(
        &self,
        chat_id: C,
        message_id: MessageId,
        text: T,
    ) -> JsonRequest<payloads::SendMessage>
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        let mut msg = payloads::SendMessage::new(chat_id, text);
        msg.reply_to_message_id = Some(message_id);
        JsonRequest::<payloads::SendMessage>::new(self.clone(), msg)
    }
}
