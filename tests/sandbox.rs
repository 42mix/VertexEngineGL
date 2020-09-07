struct Application {}

impl Application {
    fn new() -> Self {
        Application {}
    }
}

impl vertex_engine::VertexEngineApplication for Application {
    fn run(&mut self) {
        assert!(true);
    }
}

#[test]
fn main_test() {
    vertex_engine::run(&mut Application::new());
}
