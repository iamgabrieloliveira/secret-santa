use rand::seq::SliceRandom;
use std::collections::HashMap;
use uuid::Uuid;

pub fn shuffle_and_generate_links(participants: &[String]) -> HashMap<String, String> {
    let mut shuffled = participants.to_vec();
    shuffled.shuffle(&mut rand::thread_rng());

    let mut links = HashMap::new();
    for (_, receiver) in participants.iter().zip(shuffled.iter()) {
        let id = Uuid::new_v4().to_string();
        links.insert(id, receiver.clone());
    }
    links
}
