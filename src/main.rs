mod optim_main;
mod resistors;

// macros モジュール内のマクロを使えるようにする
pub mod unit;
pub use unit::electric_units::*; // electric_unitsモジュール内の全てのアイテムをスコープに入れる

fn main() {
    optim_main::optim_function::run();
}
