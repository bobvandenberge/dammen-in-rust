mod core;
mod ui;

fn main() {
    let spel = core::Spel::new();

    ui::print_spel(&spel);
}
