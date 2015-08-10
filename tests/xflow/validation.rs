#[cfg(test)]
mod test {

    extern crate xfdocs;
    use self::xfdocs::xflow::xfstruct::*;
    use self::xfdocs::xflow::validation::*;

    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    fn read_json_file(filename:&str) -> String {
        // Create a path to the desired file
        let path = Path::new(filename);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!(
                "couldn't open {}: {}",
                display,
                Error::description(&why)
                ),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       Error::description(&why)),
            Ok(_)    => {}, // print!("{} contains:\n{}", display, s),
        };

        s
    }

#[test]
    fn test_init_validation() {
        let json_string = read_json_file("data/flows/10_steps.json");
        let xfs = XFlowStruct::from_json(&json_string);
        Validation::all_edges_have_nodes(&xfs);
    }

}

