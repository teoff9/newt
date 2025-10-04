use newt::{
    sim::{geom::Vec3, simulation::Sim, system::System},
    sim_old::simulation::Simulation,
};

fn main() -> anyhow::Result<()> {
    let dt = 0.01;
    let e = 0.01;
    let steps = 1;
    // let mut s = Simulation::random_system(1., dt, steps, n, e, (1., 1.), [(-100., 100.); 3], Some([(-10., 10.); 3]));
    // s.save_to_file("src/data/configs.10k.json");
    let mut s = Simulation::from_config_file("src/data/configs/10k.json")?;
    let (pos, vel, mass) = s.system();
    let pos: Vec<Vec3> = pos.iter().map(|v| Vec3::from(v.to_array())).collect();
    let vel: Vec<Vec3> = vel.iter().map(|v| Vec3::from(v.to_array())).collect();
    let mass: Vec<f64> = mass.iter().map(|m| *m as f64).collect();
    let mut sim: Sim = Sim::from(dt as f64, 1.0, e as f64, System::from(pos, vel, mass));

    let e0 = s.total_e();
    println!("Total E = {}", e0);

    //par run
    let initial_t = std::time::Instant::now();
    s.par_run();
    let par_t = initial_t.elapsed().as_secs_f64();
    let par_e = s.total_e();
    println!("Total E (parall) = {}\ttime = {}", par_e, par_t);

    //new sim
    let initial_t = std::time::Instant::now();
    sim.run(steps as i32);
    let t = initial_t.elapsed().as_secs_f64();
    let e = sim.sys().measure_e((e * e) as f64, 1.0);
    println!("Total E (new s) = {}\ttime = {}", e, t);

    Ok(())
}
