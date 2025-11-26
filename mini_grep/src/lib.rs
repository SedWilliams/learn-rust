

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let mut results = Vec::new();
    
    for lines in contents.lines() {
        if lines.contains(query) {
            results.push(lines);
            print!("\n{}", lines);
        }        
    }
    println!();

    //unused return
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }
}













