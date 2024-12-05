use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Location {
    Home,
    BAL,
    WAS,
    NCR,
    ROS,
    ARL,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Transportation {
    Walk, 
    PersonalBike,
    Bikeshare,
    ElectricBikeshare, 
    Car,
    AmtrakAcela,
    AmtrakNE,
    Marc,
    Metro
}

#[derive(Debug, Clone)]
struct Vertex{
    location: Location,
    cumulative_time: i32,
    cumulative_wait_time: i32,
    cumulative_cost: f32,
    cumulative_hassle_units: i32,
    predecessor: Option<Location>,
    mode_of_transportation: Option<Transportation>,
    edges: Vec<Edge>
}

#[derive(Debug, Clone)]
struct Edge{
    total_time: i32,
    destination: Location,
    transportation: Transportation,
    travel_time: i32,
    wait_time: i32,
    cost: f32,
    hassle_units: i32
}


fn new_edge(destination: Location,
            transportation: Transportation,
            travel_time: i32,
            wait_time: i32,
            cost: f32,
            hassle_units: i32) -> Edge{

    Edge { destination: destination,
            transportation: transportation,
            travel_time: travel_time,
            wait_time: wait_time,
            total_time: travel_time + wait_time,
            cost: cost,
            hassle_units: hassle_units }

}

fn new_vertex (location: Location,  edges: Vec<Edge>) -> Vertex {
    Vertex {
        location: location,
        cumulative_time: 0,
        cumulative_wait_time: 0,
        cumulative_cost: 0.0,
        cumulative_hassle_units: 0,
        predecessor: None,
        mode_of_transportation: None,
        edges:edges
    }
}

fn get_possible_paths(source: Location, vertex_hash_map: HashMap<Location, Vertex>, current_path: Vec<Edge>, total_paths: &mut Vec<Vec<Edge>>){
    
    match source {
        Location::ARL => {
            total_paths.push(current_path);
        },

        Location::Home => {
            for edge in &vertex_hash_map.get(&source).expect("should have been good").edges{
                let mut new_path = current_path.clone();
                new_path.push(edge.clone());
                get_possible_paths(edge.destination, vertex_hash_map.clone(), new_path, total_paths);
            }

        }
        _ => {
            for edge in &vertex_hash_map.get(&source).expect("should have been good").edges{
                let mut new_path = current_path.clone();
                new_path.push(edge.clone());
                get_possible_paths(edge.destination, vertex_hash_map.clone(), new_path, total_paths);
            }
        }
        
    }
}

fn dijkstra_time(mut source: Vertex, mut vertex_hash_map: HashMap<Location, Vertex>) -> i32{
    for vertex in vertex_hash_map.values_mut(){
        vertex.cumulative_time = i32::MAX;

    };

    source.cumulative_time = 0;
    vertex_hash_map.get_mut(&source.location).unwrap().cumulative_time = 0;

    #[derive(Debug)]
    struct EdgeTuple{
        distance: i32,
        edge: Edge
    }

    impl Ord for EdgeTuple {
        fn cmp(&self, other: &Self) -> Ordering{
            (other.distance).cmp(&self.distance)
        }
    }

    impl PartialOrd for EdgeTuple {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl PartialEq for EdgeTuple {
        fn eq(&self, other: &Self) -> bool {
            self.distance == other.distance &&
             self.edge.destination == other.edge.destination &&
             self.edge.transportation == other.edge.transportation
        }
    }

    impl Eq for EdgeTuple {
    }


    // starting dijkstras
    // TODO need to implement restrictions on bike
    
    // create queue 
    let mut queue:BinaryHeap<EdgeTuple> = BinaryHeap::new();
    let mut visited: HashSet<Location> = HashSet::new();

    if let Some(vertex) = vertex_hash_map.get(&source.location) {
        for edge in &vertex.edges {
            queue.push(EdgeTuple{
                distance: edge.total_time,
                edge: edge.clone()
            })
        }

    }

    while !queue.is_empty(){

        let edge_tuple = queue.pop().unwrap();

        let destination = edge_tuple.edge.destination;
        if visited.contains(&destination) {
            continue;
        }

        visited.insert(destination);

        {
            let source_cumulative_time = vertex_hash_map.get(&source.location).expect("Source vertex not found").cumulative_time;

            let current_vertex = vertex_hash_map.get_mut(&edge_tuple.edge.destination).expect("Destination vertex not found");
            let current_vertex_cumulative_time = current_vertex.cumulative_time;

            let new_cumulative_time = source_cumulative_time + edge_tuple.distance;
            let new_cumulative_wait_time = source_cumulative_time + edge_tuple.edge.wait_time;

            if new_cumulative_time < current_vertex_cumulative_time {
                current_vertex.cumulative_time = new_cumulative_time;
                current_vertex.predecessor = Some(source.location);
                current_vertex.mode_of_transportation = Some(edge_tuple.edge.transportation);
                current_vertex.cumulative_wait_time = new_cumulative_wait_time;
            }
        }



        if let Some(updated_vertex) = vertex_hash_map.get(&destination) {
            for next_edge in &updated_vertex.edges {
                queue.push(EdgeTuple{
                    distance: updated_vertex.cumulative_time + next_edge.total_time,
                    edge: next_edge.clone(),
                });
            }
        }



        if destination == Location::ARL {
            break;
        }
    }


    dbg!(vertex_hash_map);
    0

}


fn main() {
    // creating all of the vetices 
    
    let mut home = new_vertex(Location::Home, vec![]);
    let mut bal = new_vertex(Location::BAL, vec![]);
    let mut was = new_vertex(Location::WAS, vec![]);
    let mut ncr = new_vertex(Location::NCR, vec![]);
    let mut ros = new_vertex(Location::ROS, vec![]);
    let arl = new_vertex(Location::ARL, vec![]);

    let mut hashmap:HashMap<Location, Vertex> = HashMap::new();



    // creating all of the edges from home
    let home_arl = new_edge(Location::ARL, Transportation::Car, 85, 0, 22.03, 7);
    let home_was = new_edge(Location::WAS, Transportation::Car, 78, 0, 33.3, 0);
    let home_bal_walk = new_edge(Location::BAL, Transportation::Walk, 18, 0, 0.0, -7);
    let home_bal_bike = new_edge(Location::BAL, Transportation::PersonalBike, 7, 0, 0.0, -2);
    let home_bal_drive = new_edge(Location::BAL, Transportation::Car, 8, 0, 28.22, 2);
    let home_ncr = new_edge(Location::NCR, Transportation::Car, 58, 0, 11.99, 0);

    let home_vec = vec![home_arl, home_was, home_bal_walk, home_bal_bike, home_bal_drive, home_ncr];

    // creating all edges from BAL

    let bal_was = new_edge(Location::WAS, Transportation::AmtrakAcela, 38, 10, 24.50, 0); // count the total
                                                                                // hassle units at
                                                                                // the end
    let bal_ncr_amtrak = new_edge(Location::NCR, Transportation::AmtrakNE, 30, 10, 12.00, 0);
    let bal_ncr_mark = new_edge(Location::NCR, Transportation::Marc, 40, 10, 9.0, 1);

    let bal_vec = vec![bal_was, bal_ncr_amtrak, bal_ncr_mark];


    // creating all edges from WAS

    let was_arl_bike = new_edge(Location::ARL, Transportation::PersonalBike, 40, 0, 0.0, -5);
    let was_arl_bikeshare = new_edge(Location::ARL, Transportation::Bikeshare, 40, 0, 3.00, -1);
    let was_arl_ebike = new_edge(Location::ARL, Transportation::ElectricBikeshare, 40, 0, 7.00, -1);
    let was_ros = new_edge(Location::ROS, Transportation::Metro, 24, 7, 2.55, 2);

    let was_vec = vec![was_arl_bike, was_arl_bikeshare, was_arl_ebike, was_ros];

    // creating all edges from ncr
    let ncr_was_metro = new_edge(Location::WAS, Transportation::Metro, 45, 7, 5.45, 2);
    let ncr_was_marc = new_edge(Location::WAS, Transportation::Marc, 17, 10, 9.0, 1); // set cost to 0 if
                                                                             // already on marc
    // set cost to 0 if already on amtrak
    let ncr_was_amtrak_ne = new_edge(Location::WAS, Transportation::AmtrakNE, 17, 10, 12.00, 0);

    let ncr_ros = new_edge(Location::ROS, Transportation::Metro, 35, 7, 6.60, 2);

    let ncr_vec = vec![ncr_was_metro, ncr_was_marc, ncr_was_amtrak_ne, ncr_ros];


    // creating all edges from ros
    let ros_arl_bike = new_edge(Location::ARL, Transportation::PersonalBike, 4, 0, 0.0, 0);
    let ros_arl_walk = new_edge(Location::ARL, Transportation::Walk, 5, 0, 0.0, 0);

    let ros_vec = vec![ros_arl_walk, ros_arl_bike];

    // assigning all of the edges to each of the vertices
    home.edges = home_vec;
    bal.edges = bal_vec;  
    was.edges = was_vec;
    ncr.edges = ncr_vec;
    ros.edges = ros_vec;


    hashmap.insert(Location::Home, home.clone());
    hashmap.insert(Location::BAL, bal.clone());
    hashmap.insert(Location::WAS, was.clone());
    hashmap.insert(Location::NCR, ncr.clone());
    hashmap.insert(Location::ROS, ros.clone());
    hashmap.insert(Location::ARL, arl.clone());

    //dijkstra_time(home, hashmap);
    
    let mut total_paths: Vec<Vec<Edge>> = Vec::new(); 
    let mut path_values: Vec<Result<(i32, f32, i32), &str>> = Vec::new();
    get_possible_paths(home.location, hashmap, Vec::new(), &mut total_paths);


    for path in &total_paths {
        path_values.push(calculate(path));
    }

    let result = total_paths.iter().zip(path_values.iter()).filter(|(_edges, values)| values.is_ok()).collect::<Vec<_>>();

    let mut cleaned_result = result.iter().map(|(edges, values)| {(edges, values.unwrap())}
            ).collect::<Vec<_>>();

    cleaned_result.sort_by_key(|(edges, (time, cost, hassle_units))| {*time});

    //cleaned_result.sort_by(|(_, (_, cost, _)), (_, (_, other_cost, _))| {
    //    cost.partial_cmp(other_cost).unwrap_or(std::cmp::Ordering::Equal)
    //});

    for result in cleaned_result {
        for edge in result.0.iter() {
            println!("{:?}",edge);
        }
        println!("Time: {:?}", result.1.0);
        println!("Cost: {:?}", result.1.1);
        println!("Hassle Units: {:?}", result.1.2);
        println!();
    }


}

fn calculate(path: &Vec<Edge>) -> Result<(i32, f32, i32), &str> {
    let mut has_bike = false;
    let mut current_transportaion: Transportation = Transportation::Walk;
    let mut current_location = Location::Home;
    let mut total_time = 0;
    let mut total_wait_time = 0;
    let mut total_cost = 0.0;
    let mut total_hassle_units = 0;


    for edge in path{
        if edge.transportation == Transportation::PersonalBike {
            if has_bike == false && current_location != Location::Home{
                return Err("Not able to ride personal bike in this path");
            }

            if current_location == Location::Home{
                has_bike = true;
            }
        }

        if edge.transportation == Transportation::AmtrakAcela {
            if has_bike {
                return Err("Not allowed to bring bike on Acela");
            }
        }

        if current_transportaion != edge.transportation{
            total_cost += edge.cost;
        }

        total_time += edge.total_time; 
        total_wait_time += edge.wait_time;
        current_transportaion = edge.transportation;
        current_location = edge.destination;


        total_hassle_units += edge.hassle_units;

    }

    if total_wait_time > 0 && total_wait_time <= 15 {
        total_hassle_units += 1
    }

    if total_wait_time >=16 && total_wait_time <= 60 {
        total_hassle_units += 3
    }

    Ok((total_time, total_cost, total_hassle_units))

}
