use newt::sim::simulation::Simulation;

fn main() -> anyhow::Result<()> {
    let mut sim = Simulation::from_config_file("src/examples/run1.json")?;
    let e0 = sim.total_e();
    println!("Total E = {}", e0);
    let ti = std::time::Instant::now();
    sim.run();
    let t = ti.elapsed().as_secs_f64();
    let e = sim.total_e();
    println!("Total E = {}", e);
    println!("Time: {}", t);

    Ok(())
}
