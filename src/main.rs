use newt::sim::simulation::Simulation;

fn main() -> anyhow::Result<()> {
    let s = Simulation::random_system(1., 0.01, 1, 10000, 0.01, (1., 1.), [(-100., 100.); 3], Some([(-10., 10.); 3]));
    // let mut sim = s.clone();
    let mut par_sim = s.clone();
    let mut simd_sim = s.clone();

    // let e0 = sim.total_e();
    // println!("Total E = {}", e0);

    //par run
    let initial_t = std::time::Instant::now();
    par_sim.par_run();
    let par_t = initial_t.elapsed().as_secs_f64();
    let par_e = par_sim.total_e();
    println!("Total E (parall) = {}\ttime = {}", par_e, par_t);

    //run
    // let initial_t = std::time::Instant::now();
    // sim.run();
    // let t = initial_t.elapsed().as_secs_f64();
    // let e = sim.total_e();
    // println!("Total E (normal) = {}\ttime = {}", e, t);

    //simd
    simd_sim.simd_run();

    Ok(())
}
