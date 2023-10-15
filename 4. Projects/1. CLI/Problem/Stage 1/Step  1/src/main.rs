mod models;

fn main() {
    println!("Welcome To My-Jira!");
    
    let status = models::Status::Open;
    println!("{status:?}");

    let story = models::Story::new("delete old data".to_string(), "".to_string());
    println!("{story:?}");

    let epic = models::Epic::new("clean db".to_string(), "".to_string());
    println!("{epic:?}");

    let db = models::DBState {
        last_item_id: 1,
        epics: vec![epic],
        stories: vec![story],
    };
    println!("{db:?}");
}
