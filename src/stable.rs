pub struct Vitals {
    apiversion: &'static str,
    author: &'static str,
    color: &'static str,
    head: &'static str,
    tail: &'static str,
    version: &'static str,
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

    fn start();
    fn r#move();
    fn end();
}

mod starter {
    use super::Webhooks;
    use crate::api::Game;

    struct Starter(Game);

    impl Webhooks for Starter {
        fn start() {
            todo!()
        }

        fn r#move() {
            todo!()
        }

        fn end() {
            todo!()
        }
    }
}

