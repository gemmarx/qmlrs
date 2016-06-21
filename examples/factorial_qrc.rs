#[macro_use]
extern crate qmlrs;
mod math;

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("factorial", math::Factorial);
    engine.load_url("qrc:/factorial_ui.qml");

    engine.exec();
}
