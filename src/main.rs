extern crate git2;
extern crate mockito;

fn main() {
    git2::Repository::init(".").unwrap();
    mockito::mock("GET", "/test").create();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        mockito::mock("GET", "/test").create();
    }

}
