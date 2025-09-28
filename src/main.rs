use newt::sim::simulation::Simulation;

fn main() -> anyhow::Result<()> {
    let mut sim = Simulation::from_config_file("src/examples/test.json")?;
    sim.run();
    dbg!(sim);

    Ok(())
}
