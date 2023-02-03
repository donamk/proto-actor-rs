use std::{any::Any, sync::atomic::AtomicBool};

use crate::{pid::PID, mailbox::mailbox::Mailbox};

/// Process is an interface that defines the base contract for interaction of actors.
pub trait Process {
    fn send_user_message(&self, message: &dyn Any);
    fn send_system_message(&self, message: &dyn Any);
    fn stop(&self, pid: &PID);
    fn as_any(&self) -> &dyn Any;
}

// use std::{fmt::{Display, Debug, Formatter, Result as FmtResult}};
// impl Display for dyn Mailbox {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         write!(f, "{:?}", self.type_id())
//     }
// }

pub struct ActorProcess {
    mailbox: Box<dyn Mailbox>,
    pub dead_flag: AtomicBool
}

impl Process for ActorProcess {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn send_user_message(&self, message: &dyn Any) {
        todo!()
    }

    fn send_system_message(&self, message: &dyn Any) {
        todo!()
    }

    fn stop(&self, pid: &PID) {
        todo!()
    }
}

#[derive(Debug)]
pub struct DeadLetterProcess {}

impl Process for DeadLetterProcess {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn send_user_message(&self, message: &dyn Any) {
        todo!()
    }

    fn send_system_message(&self, message: &dyn Any) {
        todo!()
    }

    fn stop(&self, pid: &PID) {
        todo!()
    }
}
