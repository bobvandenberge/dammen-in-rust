mod core;
mod ui;

fn main() {
    let spel = core::Spel::met_standaard_regels();

    ui::print_spel(&spel);
}
