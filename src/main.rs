use newt::sim::simulation::Simulation;

fn main() -> anyhow::Result<()> {
    let p = (-10000., 10000.);
    let mut sim = Simulation::random_system(1., 0.001, 1000, 1000, 0.01, (1., 1.), [p; 3], None);
    let e0 = sim.total_e();
    println!("Total E = {}", e0);
    sim.run();
    let e = sim.total_e();
    println!("Total E = {}", e);

    Ok(())
}
