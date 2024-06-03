use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::FpCategory::Nan;
use std::path::Path;
use crate::mod_problem::Problem;

#[derive(Clone, Debug)]
pub struct Solution{
    pub order:Vec<u32>,
}

impl Solution{
    pub fn new(order:Vec<u32>)->Solution{
        Solution{
            order:order,
        }
    }

    pub fn new_simple_order(problem: &Problem)->Solution{
        let mut order =Vec::new();
        for i in 0..problem.n_free_vertices{
            order.push(i);
        }
        Solution{
            order:order,
        }
    }

    pub fn new_mean_neigbors(problem: &Problem)->Solution{
        let mut order =Vec::new();
        for i in 0..problem.n_free_vertices{
            order.push(i);
        }
        let mut means = Vec::new();
        for i in 0..problem.n_free_vertices{
            let mut mean:f32 = 0.0;
            for j in 0..problem.neighbors[i as usize].len(){
                mean+=problem.neighbors[i as usize][j] as f32;
            }
            if problem.neighbors[i as usize].len()>0{
                mean = mean/problem.neighbors[i as usize].len() as f32;
            }else{
                mean = 0.0;
            }

            means.push(mean);
        }
        order.sort_by(|a, b| means[*a as usize].partial_cmp(&means[*b as usize]).unwrap());
        Solution{
            order:order,
        }
    }

    pub fn new_insertion_based(problem:&Problem)->Solution{
        let mut order =Vec::new();
        for i in 0..problem.n_free_vertices{

            if i==0{
                order.push(i);
            }else {
                //find best insertion position
                let mut best_position = 0;
                let mut best_crossings = u32::MAX;
                for pos in 0..=order.len(){
                    let mut crossings = 0;
                    for j in 0..order.len(){
                        if pos<=j{
                            crossings+=problem.calculate_crossing_ij(i as u32, order[j] as u32);//problem.crossing_matrix[i as usize][order[j] as usize];
                        }else {
                            crossings+=problem.calculate_crossing_ij(order[j] as u32, i as u32);//problem.crossing_matrix[order[j] as usize][i as usize];
                        }
                    }
                    if crossings<best_crossings{
                        best_crossings = crossings;
                        best_position = pos;
                    }
                }

                order.insert(best_position,i);
                println!("Best position for {} is {} with cost {}, length: {}",i,best_position,best_crossings,order.len());
            }

            //println!("Order: {:?}",order);

        }



        Solution{
            order:order,
        }
    }

    pub fn print_order(&self){
        print!("Solution order: ");
        for i in 0..self.order.len(){
            print!("{} ", self.order[i]);
        }
        println!();
    }

    pub fn calculate_total_crossings(&self, problem:&Problem)->f64{
        let mut total_crossings:f64 = 0.0;
        for i in 0..self.order.len(){
            for j in i+1..self.order.len(){
                //let crossingsij = problem.crossing_matrix[self.order[i] as usize][self.order[j] as usize];
                let crossingsij = problem.calculate_crossing_ij(self.order[i],self.order[j]);
                total_crossings+=crossingsij as f64;
            }
        }
        total_crossings
    }

    /*pub fn calculate_swap_delta(&self,problem:&Problem,index_1:u32,index_2:u32)->i32{
        let mut delta = 0;


        //return 0 if the indices are the same
        if index_1==index_2{
            return delta;
        }

        //assume that index_1 < index_2, else reverse
        if index_1>index_2{
            return self.calculate_swap_delta(problem,index_2,index_1);
        }

        //calculate the delta of item at index_1 and item at index_2
        delta = delta -  problem.crossing_matrix[self.order[index_1 as usize] as usize][self.order[index_2 as usize] as usize] as i32;
        delta = delta + problem.crossing_matrix[self.order[index_2 as usize] as usize][self.order[index_1 as usize] as usize] as i32;

        //calculate the delta for all items in between
        if (index_2-index_1)>1{
            for i in index_1+1..index_2{
                delta -= problem.crossing_matrix[self.order[i as usize] as usize][self.order[index_2 as usize] as usize] as i32;
                delta -= problem.crossing_matrix[self.order[index_1 as usize] as usize][self.order[i as usize] as usize] as i32;
                delta += problem.crossing_matrix[self.order[i as usize] as usize][self.order[index_1 as usize] as usize] as i32;
                delta += problem.crossing_matrix[self.order[index_2 as usize] as usize][self.order[i as usize] as usize] as i32;
            }
        }

        delta
    }

    pub fn do_swap(&mut self,index_1:u32,index_2:u32){
        self.order.swap(index_1 as usize,index_2 as usize);
    }*/

    pub fn calculate_move_delta(&self,problem:&Problem,index_from:u32,index_to:u32)->f64 {
        let mut delta:f64 = 0.0;

        //return 0 if the indices are the same
        if index_from==index_to{
            return delta;
        }

        if index_from<index_to{
            for i in index_from+1..=index_to{
                //delta -= problem.crossing_matrix[self.order[index_from as usize] as usize][self.order[i as usize] as usize] as i32;
                delta -= problem.calculate_crossing_ij(self.order[index_from as usize],self.order[i as usize]) as f64;
                //delta += problem.crossing_matrix[self.order[i as usize] as usize][self.order[index_from as usize] as usize] as i32;
                delta+=problem.calculate_crossing_ij(self.order[i as usize],self.order[index_from as usize]) as f64;
            }
        }
        if index_from>index_to{
            for i in index_to..index_from{
                //delta -= problem.crossing_matrix[self.order[i as usize] as usize][self.order[index_from as usize] as usize] as i32;
                //delta += problem.crossing_matrix[self.order[index_from as usize] as usize][self.order[i as usize] as usize] as i32;
                delta-=problem.calculate_crossing_ij(self.order[i as usize],self.order[index_from as usize]) as f64;
                delta+=problem.calculate_crossing_ij(self.order[index_from as usize],self.order[i as usize]) as f64;
            }
        }


        delta
    }

    pub fn do_move(&mut self,index_from:u32,index_to:u32){
        let item = self.order.remove(index_from as usize);
        self.order.insert(index_to as usize,item);
    }

    /*pub fn calculate_inverse_delta(&self,problem:&Problem,index_1:u32,index_2:u32)->i32{
        let mut delta = 0;


        //return 0 if the indices are the same
        if index_1==index_2{
            return delta;
        }

        //assume that index_1 < index_2, else reverse
        if index_1>index_2{
            return self.calculate_inverse_delta(problem,index_2,index_1);
        }

        for i in index_1..index_2{
            for j in i+1..index_2+1{
                delta -= problem.crossing_matrix[self.order[i as usize] as usize][self.order[j as usize] as usize] as i32;
                delta += problem.crossing_matrix[self.order[j as usize] as usize][self.order[i as usize] as usize] as i32;
            }
        }

        delta
    }

    pub fn do_inverse(&mut self,index_1:usize,index_2:usize){
        self.order[index_1..=index_2].reverse();
    }*/

    pub fn load_solution(file_path:&str,problem: &Problem)->Solution{
        let mut order = Vec::new();

        // read file and create Problem
        if let Ok(lines) = Solution::read_lines(file_path) {
            // Consumes the iterator, returns an (Optional) String
            let mut line_counter: u32 = 0;
            for line in lines {
                if let Ok(ip) = line {
                    //println!("line {}: {}",line_counter, ip);
                    let mut v:u32 = ip.parse().unwrap();
                    v = v-1- problem.n_fixed_vertices;
                    order.push(v);
                }
            }
        }


        Solution{
            order:order,
        }
    }

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }


    pub fn print_solution_to_stdout(&self,problem:&Problem){
        for i in 0..self.order.len(){
            println!("{}",self.order[i]+1+problem.n_fixed_vertices);
        }
    }

}