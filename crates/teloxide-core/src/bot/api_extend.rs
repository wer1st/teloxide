use crate::{
    payloads,
    requests::JsonRequest,
    types::{Message, Recipient, ReplyParameters, ThreadId},
    Bot,
};

pub trait ApiExtend {
    fn send_message_to_thread<C, T>(
        &self,
        chat_id: C,
        thread_id: ThreadId,
        text: T,
    ) -> JsonRequest<payloads::SendMessage>
    where
        C: Into<Recipient>,
        T: Into<String>;

    fn reply_to_message<T>(
        &self,
        reply_to: &Message,
        text: T,
    ) -> JsonRequest<payloads::SendMessage>
    where
        T: Into<String>;
}

impl ApiExtend for Bot {
    fn send_message_to_thread<C, T>(
        &self,
        chat_id: C,
        thread_id: ThreadId,
        text: T,
    ) -> JsonRequest<payloads::SendMessage>
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        let mut msg = payloads::SendMessage::new(chat_id, text);
        msg.message_thread_id = Some(thread_id);
        JsonRequest::<payloads::SendMessage>::new(self.clone(), msg)
    }

    fn reply_to_message<T>(&self, reply_to: &Message, text: T) -> JsonRequest<payloads::SendMessage>
    where
        T: Into<String>,
    {
        let mut msg = payloads::SendMessage::new(reply_to.chat.id, text);
        msg.message_thread_id = reply_to.thread_id;
        msg.reply_parameters = Some(ReplyParameters::new(reply_to.id));
        JsonRequest::<payloads::SendMessage>::new(self.clone(), msg)
    }
}
