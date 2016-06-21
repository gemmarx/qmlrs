#[macro_use]
extern crate qmlrs;
mod math;

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("factorial", math::Factorial);

    let mut qml_string = include_str!("factorial_ui.qml").to_owned();
    qml_string = qml_string.replace("Factorial", "Factorial (from string)");
    engine.load_data(&qml_string);

    engine.exec();
}
