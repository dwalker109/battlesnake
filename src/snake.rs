use serde::Serialize;

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
