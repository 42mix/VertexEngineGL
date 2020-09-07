pub trait VertexEngineApplication {
    fn run(&mut self);
}

pub fn run(application: &mut impl VertexEngineApplication) {
    application.run();
}
