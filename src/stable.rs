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

const SNAKE_VITALS: Vitals = Vitals {
    apiversion: "1",
    author: "Dan Walker",
    color: "#cccccc",
    head: "default",
    tail: "default",
    version: "0.0.1",
};

pub trait Webhooks {
    fn get(&self) -> &Vitals {
        &SNAKE_VITALS
    }

    fn start(&self);
    fn r#move(&self);
    fn end(&self);
}

mod starter {
    use super::Webhooks;
    use crate::api::Game;

    struct Starter(Game);

    impl Webhooks for Starter {
        fn start(&self) {
            todo!()
        }

        fn r#move(&self) {
            todo!()
        }

        fn end(&self) {
            todo!()
        }
    }
}
