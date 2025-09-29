use newt::sim::simulation::Simulation;

fn main() -> anyhow::Result<()> {
    //let mut sim = Simulation::from_config_file("src/data/configs/run1.json")?;
    let mut sim = Simulation::random_system(
        1.,
        0.1,
        100,
        1000,
        0.01,
        (1., 1.),
        [(-100., 100.); 3],
        Some([(-10., 10.); 3]),
    );
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
