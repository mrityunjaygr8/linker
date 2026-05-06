use linker::{Link, Store};

fn main() {
    let links = [
        "https://dev.parham.in",
        "https://demo.parham.in",
        "https://google.com",
        "http://localhost:8000",
        "test",
    ];
    let mut store = Store::new();
    let mut valid_urls: Vec<Link> = Vec::new();
    let mut invalid_urls: Vec<_> = Vec::new();

    links.iter().for_each(|l| match Link::new(l) {
        Ok(l) => valid_urls.push(l),
        Err(e) => invalid_urls.push(e),
    });

    for link in valid_urls {
        store.add(&link);
    }

    for e in invalid_urls {
        eprintln!("{}", e);
    }

    store.list().into_iter().for_each(|l| println!("{}", l));
    store.delete("woo-123".to_string());
    match store.get("woo-123".to_string()) {
        Some(v) => println!("{}", v),
        None => eprintln!("woo-123 not found"),
    };
}
