use newt::{
    sim::{geom::vec3::Vec3, simulation::Sim, system::scalar::System},
    sim_old::simulation::Simulation,
};

fn old_to_new(s: &Simulation) -> Sim {
    let (pos, vel, mass) = s.system();
    let pos: Vec<Vec3> = pos.iter().map(|v| Vec3::from(v.to_array())).collect();
    let vel: Vec<Vec3> = vel.iter().map(|v| Vec3::from(v.to_array())).collect();
    let mass: Vec<f64> = mass.iter().map(|m| *m).collect();
    Sim::from(s.dt, s.g, s.softening, System::from(pos, vel, mass))
}

fn main() -> anyhow::Result<()> {
    //INIT
    // let n = 1000;
    // let mut s = Simulation::random_system(1., dt, steps, n, e, (1., 1.), [(-100., 100.); 3], Some([(-10., 10.); 3]));
    // s.save_to_file("src/data/configs/1k.json");
    let mut s = Simulation::from_config_file("src/data/configs/1k.json")?;
    let mut sim = old_to_new(&s);

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
    sim.run(s.steps as i32);
    let t = initial_t.elapsed().as_secs_f64();
    let e0 = sim.energy();
    println!("Total E (new s) = {}\ttime = {}", e0, t);

    Ok(())
}
