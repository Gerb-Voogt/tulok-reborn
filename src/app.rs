#[derive(Debug)]
pub enum RouteId {
}

#[derive(Debug)]
pub enum ActiveBlock {
}

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}


pub struct App {
}


impl Default for App {
    fn default() -> App {
        App {
        }
    }
}
