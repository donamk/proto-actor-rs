use std::any::Any;

pub trait Mailbox {
    fn user_message_count(&self) -> u32;

    fn post_user_message(&self, message: dyn Any);

    fn post_system_message(&self, message: dyn Any);

    // fn regist_handler(IMessageInvoker invoker, IDispatcher dispatcher);

    fn start(&self);
}

struct DefaultMailbox {}