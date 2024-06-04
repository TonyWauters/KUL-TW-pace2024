use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub struct Problem {
    pub name: String,
    pub n_fixed_vertices: u32, //number of vertices on the top line
    pub n_free_vertices: u32, //number of vertices on the bottom line
    pub n_edges: u32,
    //pub edges: Vec<(u32,u32)>, //edges
    pub neighbors: Vec<Vec<u32>>, //neigbors (at the top line) of each vertex on the bottom line
    //pub crossing_matrix: Vec<Vec<u32>>, //crossing matrix, crossing_matrix[i][j] =number of edges that cross if i-th vertex is before j-th vertex on the bottom line
}

impl Problem{

    pub fn read_problem_from_stdin() -> Problem{
        let mut name = "stdin".to_string();
        let mut n_fixed_vertices=0;
        let mut n_free_vertices=0;
        let mut n_edges=0;
        let mut edges:Vec<(u32,u32)>=Vec::new();
        let mut neighbors:Vec<Vec<u32>>=Vec::new();

        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        let mut line_counter:u32 = 0;
        while let Some(line) = lines.next() {


            if let Ok(ip) = line {
                //println!("line through stdin: {}", ip);
                //read info from the first line
                if line_counter==0{
                    let mut parts = ip.split(' '); //split line by space
                    let first = parts.next().unwrap(); //get the first part
                    let second = parts.next().unwrap(); //get the second part
                    let third = parts.next().unwrap(); //get the third part
                    let fourth = parts.next().unwrap(); //get the fourth part
                    let fifth = parts.next().unwrap(); //get the fifth part
                    n_fixed_vertices = third.parse().unwrap();
                    n_free_vertices = fourth.parse().unwrap();
                    n_edges = fifth.parse().unwrap();
                    //println!("n_fixed_vertices: {}", n_fixed_vertices);
                    //println!("n_free_vertices: {}", n_free_vertices);
                    //println!("n_edges: {}", n_edges);
                }else {
                    //read edges
                    if line_counter>=1 && line_counter<=n_edges {
                        let mut parts = ip.split(' '); //split line by space
                        let first = parts.next().unwrap(); //get the first part
                        let second = parts.next().unwrap(); //get the second part
                        let mut from:u32 = first.parse().unwrap();
                        let mut to:u32 = second.parse().unwrap();
                        edges.push((from-1,to-1));
                        //println!("edge: ({},{})", first, second);
                    }
                }
            }



            line_counter+=1;
        }
        //check if the number of edges is correct
        if edges.len()!=n_edges as usize{
            panic!("Number of edges does not correspond to info line! {} {}", edges.len(), n_edges);
        }


        //calculate neighbors
        for i in 0..n_free_vertices{
            neighbors.push(Vec::new());
        }
        for edge in edges.iter(){
            neighbors[(edge.1-n_fixed_vertices) as usize].push(edge.0);
        }




        Problem{name, n_fixed_vertices, n_free_vertices, n_edges, neighbors}
    }

    pub fn read_problem(file_path:&str) -> Problem{
        //println!("Reading problem from file: {} ", file_path);

        let mut name = file_path.to_string().clone();
        let mut n_fixed_vertices=0;
        let mut n_free_vertices=0;
        let mut n_edges=0;
        let mut edges:Vec<(u32,u32)>=Vec::new();
        let mut neighbors:Vec<Vec<u32>>=Vec::new();
        //let mut crossing_matrix:Vec<Vec<u32>>=Vec::new();

        // read file and create Problem
        if let Ok(lines) = Problem::read_lines(file_path) {
            // Consumes the iterator, returns an (Optional) String
            let mut line_counter:u32 = 0;
            for line in lines {
                if let Ok(ip) = line {
                    //println!("line {}: {}",line_counter, ip);

                    //read info from the first line
                    if line_counter==0{
                        let mut parts = ip.split(' '); //split line by space
                        let first = parts.next().unwrap(); //get the first part
                        let second = parts.next().unwrap(); //get the second part
                        let third = parts.next().unwrap(); //get the third part
                        let fourth = parts.next().unwrap(); //get the fourth part
                        let fifth = parts.next().unwrap(); //get the fifth part
                        n_fixed_vertices = third.parse().unwrap();
                        n_free_vertices = fourth.parse().unwrap();
                        n_edges = fifth.parse().unwrap();
                        //println!("n_fixed_vertices: {}", n_fixed_vertices);
                        //println!("n_free_vertices: {}", n_free_vertices);
                        //println!("n_edges: {}", n_edges);
                    }else {
                        //read edges
                        if line_counter>=1 && line_counter<=n_edges {
                            let mut parts = ip.split(' '); //split line by space
                            let first = parts.next().unwrap(); //get the first part
                            let second = parts.next().unwrap(); //get the second part
                            let mut from:u32 = first.parse().unwrap();
                            let mut to:u32 = second.parse().unwrap();
                            edges.push((from-1,to-1));
                            //println!("edge: ({},{})", first, second);
                        }
                    }

                }
                line_counter+=1;
            }
        }

        //check if the number of edges is correct
        if edges.len()!=n_edges as usize{
            panic!("Number of edges does not correspond to info line! {} {}", edges.len(), n_edges);
        }


        //calculate neighbors
        for i in 0..n_free_vertices{
            neighbors.push(Vec::new());
        }
        for edge in edges.iter(){
            neighbors[(edge.1-n_fixed_vertices) as usize].push(edge.0);
        }
        //assume sorted neighbors
        /*for i in 0..n_free_vertices{
            neighbors[i as usize].sort_by(|a, b| a.cmp(b));
        }*/

        //calculate crossing matrix
        /*for i in 0..n_free_vertices{
            crossing_matrix.push(vec![0;n_free_vertices as usize]);
        }
        //let mut max_crossings = 0;
        for i in 0..n_free_vertices{
            for j in i+1..n_free_vertices{
                let mut crossings_ij =0;
                let mut crossings_ji =0;
                for nbi in &neighbors[i as usize]{
                    for nbj in &neighbors[j as usize]{
                        //if (nbi==nbj) continue;
                        if (nbi<nbj){
                            crossings_ji+=1;
                        }
                        if (nbj<nbi) {
                            crossings_ij+=1;
                        }
                    }
                }
                crossing_matrix[i as usize][j as usize]=crossings_ij;
                crossing_matrix[j as usize][i as usize]=crossings_ji;
                /*if crossings_ij>max_crossings{
                    max_crossings=crossings_ij;
                }
                if crossings_ji>max_crossings{
                    max_crossings=crossings_ji;
                }*/
            }
        }*/

        //println!("Maximal number of crossings: {}",max_crossings);

        Problem{name, n_fixed_vertices, n_free_vertices, n_edges, neighbors}
    }

    pub fn calculate_crossing_ij_old(&self, i:u32, j:u32) ->u32{
        let mut crossings_ij =0;
        for nbi in &self.neighbors[i as usize]{
            for nbj in &self.neighbors[j as usize]{
                if (nbj<nbi) {
                    crossings_ij+=1;
                }else {
                    break;
                }
            }
        }
        crossings_ij
    }

    /**chatgpt version :-)
    **/
    pub fn calculate_crossing_ij(&self, i: u32, j: u32) -> u32 {
        let mut crossings_ij = 0;

        let neighbors_i = &self.neighbors[i as usize];
        let neighbors_j = &self.neighbors[j as usize];



        for &nbi in neighbors_i.iter().rev() {
            let mut iter_j = neighbors_j.iter().peekable();
            let mut counti=0;
            while let Some(&&nbj) = iter_j.peek() {
                if nbj < nbi {
                    crossings_ij += 1;
                    counti+=1;
                    iter_j.next();
                } else {
                    break;
                }
            }
            if counti==0{
                break;
            }
        }

        crossings_ij
    }




    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn print_problem(&self){
        println!("Problem name: {}", self.name);
        println!("n_fixed_vertices: {}", self.n_fixed_vertices);
        println!("n_free_vertices: {}", self.n_free_vertices);
        println!("n_edges: {}", self.n_edges);
    }

    /*pub fn print_crossingmatrix(&self){
        println!("Crossing matrix:");
        for i in 0..self.crossing_matrix.len(){
            println!("{:?}",self.crossing_matrix[i]);
        }
    }*/
}