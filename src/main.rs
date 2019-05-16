extern crate clap;
use clap::{App, Arg};
use git2::Repository;
use std::fs;
use std::path::Path;
fn main() {
    let matches = App::new("getip")
        .version("v0.0.1-beta")
        .author("Andrey Y. <painhardcore@gmail.com>")
        .arg(
            Arg::with_name("cluster")
                .help("cluster")
                .required(true)
                .index(1),
        )
        .get_matches();
    let url = "https://github.com/painhardcore/testclusterinv";
    let path = String::from("./cluster");
    if Path::new(&path).exists() {
        let repo = git2::Repository::discover(&path).unwrap();
        let oid = repo.refname_to_id("refs/remotes/origin/master").unwrap();
        let object = repo.find_object(oid, None).unwrap();
        repo.reset(&object, git2::ResetType::Hard, None).unwrap();
    } else {
        Repository::clone(url, &path).unwrap();
    }

    let input = matches.value_of("cluster").unwrap().to_string();
    match input.as_ref() {
        "list" => get_clusters_list(),
        _ => get_cluster_info(&input),
    }
}

fn get_cluster_info(name: &String) {
    let cluster_path = String::from("./cluster/clusters/") + &name;
    if Path::new(&cluster_path).exists() {
        println!("Cluster name : {}", name);
        let contents = fs::read_to_string(cluster_path + "/cluster.ini")
            .expect("Something went wrong reading the file");
        println!("Content of cluster.ini:\n{}", contents);
    } else {
        println!("There is no cluster like {}", name);
    }
}

fn get_clusters_list() {
    let paths = fs::read_dir("./cluster/clusters").unwrap();
    for path in paths {
        let file = path.expect("Failed to get file");
        if file.path().is_dir() {
            let filename = file.file_name();
            let str_filename = filename.into_string().expect("Failed to get string name");
            println!("{}", str_filename);
        }
    }
}

// fn printdir(dir: io::Result<DirEntry>) {
//     let f = dir?;
//     let dirname = f.file_name().into_string();
//     println!("{}", dirname);
// }
