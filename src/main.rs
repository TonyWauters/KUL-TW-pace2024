use std::fmt::format;
use std::io;
use std::time::Instant;
use crate::mod_localsearch::{run_simulated_annealing};
use crate::mod_problem::Problem;
use crate::mod_solution::Solution;

mod mod_problem;
mod mod_solution;
mod mod_localsearch;

/***
    * This is the main function that runs the code for the pace 2024 challenge and ready for submission to optil.io
 */

fn main() {

    //this runs the code ready for the optil.io platform
    run_submission_optilio();

    //run this if you want to run the benchmark
    //run_benchmark();
    
}

fn run_submission_optilio() {
    let now = Instant::now();
    let verbose = false;
    let time_limit_in_seconds = 265;

    //read the problem from stdin
    let problem = Problem::read_problem_from_stdin();

    //create an initial solution using the mean heuristic
    let init_solution = Solution::new_mean_neigbors(&problem);


    //run simulated annealing
    let ls_solution = run_simulated_annealing(&problem, init_solution,verbose,time_limit_in_seconds,now);

    //print solution to stdout
    ls_solution.print_solution_to_stdout(&problem);

}


fn run_benchmark(){

    let verbose = false;

    //let list_of_instances = vec![1,2,3,4,5,6,7,8,9,10,44];

    for inid in 1..101{
        let now = Instant::now();
        let mut instance_id =format!("{}",inid);
        /*if inid<10{
            instance_id =format!("0{}",inid);
        }*/

        let instance_path = format!("data/heuristic-public/instances/{}.gr",instance_id);
        let problem = Problem::read_problem(instance_path.as_str());

        if verbose {
            println!("Finished reading problem. Problem name: {}", problem.name);
            problem.print_problem();
            println!("------------------------------------");
        }


        let init_solution = Solution::new_mean_neigbors(&problem);
        //init_solution.print_order();
        let initial_obj = 0;//init_solution.calculate_total_crossings(&problem);
        if verbose {
            println!("Finished constructing Initial solution with objective: {}", initial_obj);
            let elapsed = now.elapsed();
            println!("Elapsed time: {:.2?}", elapsed);
            println!("------------------------------------");
        }


        //run simulated annealing

        let ls_solution = run_simulated_annealing(&problem, init_solution,verbose,265,now);
        let elapsed = now.elapsed();
        let ls_obj = ls_solution.calculate_total_crossings(&problem);
        if verbose {
            println!("Finished running local search with objective: {}", ls_obj);
            println!("Elapsed time: {:.2?}", elapsed);
            println!("------------------------------------");
        }




        /*let opt_solution_path = format!("data/medium-test-set/solutions/{}_OPT.sol",instance_id);
        let opt_solution = Solution::load_solution(opt_solution_path.as_str(),&problem);
        //opt_solution.print_order();
        let opt_total_crossings = opt_solution.calculate_total_crossings(&problem);
        //println!("Total optimal crossings: {}", opt_total_crossings);*/

        println!("{} {} {:.2?}",instance_id,ls_obj,elapsed);
    }
}
