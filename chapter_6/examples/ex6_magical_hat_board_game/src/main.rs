#[derive(Debug)]
enum MagicalEquipment {
    Hat(String),
    Wand,
    Cloak,
    // Add more equipment types here
}

struct Player {
    name: String,
    equipment: Vec<MagicalEquipment>,
}

impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            equipment: Vec::new(),
        }
    }

    fn add_equipment(&mut self, equipment: MagicalEquipment) {
        println!("Adding a {:?} to {}'s collection!", equipment, self.name);
        self.equipment.push(equipment);
    }

    fn remove_hat(&mut self) {
        let original_len = self.equipment.len();
        self.equipment.retain(|e| match e {
            MagicalEquipment::Hat(_) => false,
            _ => true,
        });
        
        if original_len > self.equipment.len() {
            println!("A magical creature removes a hat from {}'s collection.", self.name);
        } else {
            println!("{} has no hat to remove.", self.name);
        }
    }
}

fn main() {
    let mut player = Player::new("Alice");

    // Simulate adding a fancy hat
    player.add_equipment(MagicalEquipment::Hat("Fancy Hat".to_string()));

    // Demonstrate removing a hat
    player.remove_hat();

    // Show the player's current equipment
    println!("{:?}'s equipment: {:?}", player.name, player.equipment);
}

