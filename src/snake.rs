use serde::Serialize;

#[derive(Serialize)]
pub struct Vitals {
    apiversion: &'static str,
    author: &'static str,
    color: &'static str,
    head: &'static str,
    tail: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
pub enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub trait Webhooks {
    fn start(&self);
    fn r#move(&self) -> Dir;
    fn end(&self);
}

pub const SNAKE_VITALS: Vitals = Vitals {
    apiversion: "1",
    author: "Dan Walker",
    color: "#E80978",
    head: "default",
    tail: "curled",
    version: "0.0.1",
};

pub mod starter {
    use super::{Dir, Webhooks};

    pub struct Starter;

    impl Webhooks for Starter {
        fn start(&self) {
            todo!()
        }

        fn r#move(&self) -> Dir {
            todo!()
        }

        fn end(&self) {
            todo!()
        }
    }
}
