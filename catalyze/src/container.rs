

use crate::{
    file::{File},
    message::Message,
};

pub enum Container {
    Message(Message),
    File(File),
}
