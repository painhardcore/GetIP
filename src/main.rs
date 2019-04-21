extern crate clap;
use clap::{Arg, App};

fn main() {
  let matches = App::new("getip").version("v0.0.1-beta").author("Andrey Y. <painhardcore@gmail.com>")
                               .arg(Arg::with_name("cluster")
                               .help("cluster")
                               .required(true)
                               .index(1))
                               .get_matches();

  let input = matches.value_of("cluster").unwrap().to_string();
  match input.as_ref() {
    "list" => get_clusters_list(),
    _ => get_cluster_info(&input),
  }   
}

fn get_cluster_info(name: &String){
    println!("Cluster name : {}",name)
}
fn get_clusters_list(){
    println!("Cluster list")
}