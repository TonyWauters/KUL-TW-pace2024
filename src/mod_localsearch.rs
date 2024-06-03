use std::cmp::{max, min};
use std::time::{Duration, Instant};
use crate::mod_problem::Problem;
use crate::mod_solution::Solution;


const SEED: u64 = 123456789;

pub fn run_simulated_annealing(problem:&Problem, initial_solution:Solution,verbose:bool,time_limit_secs:u64,now:Instant)->Solution{
    fastrand::seed(SEED); //fix seed

    if verbose {
        println!("Running local search");
    }


    let window:i32 = 15000;

    let mut outer_iterations = 0;
    let mut temperature = 100.0;
    let min_temperature = 0.01;
    let mut cooling_rate = 0.992;
    let mut iterations_at_temperature = 5000;
    let mut current_solution = initial_solution.clone();
    let mut best_solution = current_solution.clone();
    let mut current_obj:f64 = 0.0;
    let mut best_obj:f64 = current_obj;


    let mut stagnation_threshold=100;
    let mut stagnation_counter =0;
    let mut stagnation_detected = false;

    if verbose {
        println!("Initial solution: {}",current_obj);
    }


    while /*temperature>min_temperature &&*/ !stagnation_detected && now.elapsed()<Duration::from_secs(time_limit_secs) /*&& best_obj>0.0*/{
        if verbose {
            println!("{} - Current solution: {} , Best Solution: {} , Temperature: {}",outer_iterations,current_obj,best_obj,temperature);
        }
        let current_obj_before = current_obj;
        for itt in 0..iterations_at_temperature {
            if itt%100==0 && now.elapsed()>=Duration::from_secs(time_limit_secs){
                stagnation_detected = true;
                break;
            }

            let mut index_1 = fastrand::u32(0.. problem.n_free_vertices);//rng.gen_range(0.. problem.n_free_vertices);
            let mut index_2 = fastrand::u32(max(index_1 as i32-window,0) as u32..min(index_1 as i32+window, problem.n_free_vertices as i32) as u32);//rng.gen_range(0.. problem.n_free_vertices);
            while index_1==index_2{
                index_1 = fastrand::u32(0.. problem.n_free_vertices);//rng.gen_range(0.. problem.n_free_vertices);
                index_2 = fastrand::u32(max(index_1 as i32-window,0) as u32..min(index_1 as i32+window, problem.n_free_vertices as i32) as u32);//rng.gen_range(0.. problem.n_free_vertices);
            }



            let mut delta = current_solution.calculate_move_delta(problem,index_1,index_2);

            if delta<=0.0 {
                //if better accept!
                current_solution.do_move(index_1, index_2);
                current_obj = (current_obj + delta);
                if current_obj<best_obj{
                    best_obj = current_obj;
                    best_solution = current_solution.clone(); //todo: check if clone is necessary
                    if verbose {
                        println!("* {}-{} *Found a better solution: {}",outer_iterations,itt, best_obj);
                    }
                }
            }else {
                //accept with probability
                let p = std::f64::consts::E.powf(-(delta as f64)/temperature);
                let r:f64 = fastrand::f64();//rng.gen();
                if r<p {
                    //accept
                    current_solution.do_move(index_1, index_2);
                    current_obj=(current_obj + delta );
                }
            }
        }
        temperature = temperature*cooling_rate; //cooling


        if current_obj==current_obj_before {
            stagnation_counter += 1;
            if stagnation_counter>stagnation_threshold{
                if (verbose){
                    println!("Stagnation detected!");
                }
                stagnation_detected = true;
            }
        }else {
            stagnation_counter = 0;
        }

        outer_iterations+=1;
    }
    if verbose {
        println!("Finished local search");
    }

    best_solution
}



/*pub fn run_full_SD(problem:&Problem, initial_solution:Solution,verbose:bool)->Solution{
    let mut current_solution = initial_solution.clone();
    let mut best_solution = current_solution.clone();
    let mut current_obj = current_solution.calculate_total_crossings(problem);
    let mut best_obj = current_obj;

    let mut improved:bool = true;

    let mut iterations = 0;


    while improved {
        iterations+=1;
        improved=false;


            for i in 0..problem.n_free_vertices{
                for j in 0..problem.n_free_vertices{
                    if (i==j){
                        continue;
                    }
                    let delta = current_solution.calculate_move_delta(problem,i,j);
                    if delta<0{
                        let new_objective = (current_obj as i32 + delta) as u32;

                        //check calculation with full calculation
                        /*let mut new_solution = current_solution.clone();
                        new_solution.do_move(i, j);
                        let mut full_obj = new_solution.calculate_total_crossings(problem);
                        if new_objective!=full_obj{
                            panic!("Error in calculation of delta crossings! {} {}",new_objective,full_obj);
                        }*/


                        if new_objective<best_obj{
                            best_obj = new_objective;
                            best_solution = current_solution.clone();
                            best_solution.do_move(i, j);
                            if verbose {
                                println!("{} MOVE Found a better solution: {}",iterations, best_obj);
                            }
                        }
                        improved=true;
                    }
                }
            }


        if !improved{
            println!("Starting swap phase");
            for i in 0..problem.n_free_vertices-1{
                for j in i+1..problem.n_free_vertices{
                    let delta = current_solution.calculate_swap_delta(problem,i,j);
                    if delta<0{
                        let new_objective = (current_obj as i32 + delta) as u32;
                        if new_objective<best_obj{
                            best_obj = new_objective;
                            best_solution = current_solution.clone();
                            best_solution.do_swap(i, j);
                            if verbose {
                                println!("{} SWAP Found a better solution: {}",iterations, best_obj);
                            }
                        }
                        improved=true;
                    }
                }
            }
        }


        if !improved{
            println!("Starting inverse phase");
            for i in 0..problem.n_free_vertices-1{
                for j in i+1..problem.n_free_vertices{
                    let delta = current_solution.calculate_inverse_delta(problem,i,j);
                    if delta<0{
                        let new_objective = (current_obj as i32 + delta) as u32;
                        if new_objective<best_obj{
                            best_obj = new_objective;
                            best_solution = current_solution.clone();
                            best_solution.do_inverse(i as usize, j as usize);
                            if verbose {
                                println!("{} INVERSE Found a better solution: {}",iterations, best_obj);
                            }
                        }
                        improved=true;
                    }
                }
            }
        }




        if improved{
            current_solution = best_solution.clone();
            current_obj = best_obj;
        }
    }
    best_solution
}*/

