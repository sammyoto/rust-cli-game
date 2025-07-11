use crate::entity::{Entity, EntityBehavior};
use crate::player::Player;
use crate::enemy::Enemy;
use crate::scripts;
use crate::util::read_csv;
use crate::util::prompt_user;
use rand::prelude::*;

pub struct Game {
    enemy_names: Vec<String>,
    player: Player
}

impl Game {
    pub fn new() -> Self {
        Self {
            enemy_names: read_csv().unwrap(),
            player: Player{entity: Entity{name: "".to_string(), hp: 100, current_hp: 100, attack_power: 20, magic_power: 10, gold: 50}}
        }
    }
    fn generate_enemy(&self) -> Enemy {
        let mut rng = rand::rng();
        let name_index = rng.random_range(0..self.enemy_names.len());
        let hp: u16 = rng.random_range(20..(self.player.get_hp() - 20));
        let name: String = self.enemy_names[name_index].clone();
        Enemy{ entity: Entity{name: name, hp: 100, current_hp: 100, attack_power: 10, magic_power: 10, gold: 100}}
    }
    pub fn print_battle(&self, player: &Player, enemy: &Enemy) {
        let p = player.entity();
        let e = enemy.entity();

        println!("\n==================== BATTLE ====================");
        println!("{:<20} VS {:>20}", p.name, e.name);
        println!("{:<20}    {:>20}", "👤 Name", "👤 Name");
        println!("{:<20}    {:>20}", format!("❤️ {}/{}", p.current_hp, p.hp), format!("❤️ {}/{}", e.current_hp, e.hp));
        println!("{:<20}    {:>20}", format!("⚔️  {}", p.attack_power), format!("⚔️  {}", e.attack_power));
        println!("{:<20}    {:>20}", format!("🪄 {}", p.magic_power), format!("🪄 {}", e.magic_power));
        println!("{:<20}    {:>20}", format!("💰 {}", p.gold), format!("💰 {}", e.gold));
        println!("===============================================\n");
    }
    fn battle(&mut self, enemy: &mut Enemy, action: scripts::BATTLE_CHOICES, player_action: bool) {
        if player_action {
            match action {
                scripts::BATTLE_CHOICES::Attack=> {
                    enemy.set_current_hp(enemy.get_current_hp().saturating_sub(self.player.get_attack_power()));
                    println!("You attack {} and deal {} damage!", enemy.get_name(), self.player.get_attack_power());
                }
                scripts::BATTLE_CHOICES::MagicAttack=>{
                    enemy.set_current_hp(enemy.get_current_hp().saturating_sub(self.player.get_magic_power()));
                    println!("You attack {} with magic and deal {} damage!", enemy.get_name(), self.player.get_magic_power());
                }
            }
        } else { // Enemy action
            match action {
                scripts::BATTLE_CHOICES::Attack=> {
                    self.player.set_current_hp(self.player.get_current_hp().saturating_sub(enemy.get_attack_power()));
                    println!("{} attacks you and deals {} damage!", enemy.get_name(), enemy.get_attack_power());
                }
                scripts::BATTLE_CHOICES::MagicAttack=>{
                    self.player.set_current_hp(self.player.get_current_hp().saturating_sub(enemy.get_magic_power()));
                    println!("{} attacks you with magic and deals {} damage!", enemy.get_name(), enemy.get_magic_power());
                }
            }
        }
    }
    fn battle_loop(&mut self) {
        let mut enemy: Enemy = self.generate_enemy();
        self.print_battle(&self.player, &enemy);
        // Action choice
        loop {
            let response: String = prompt_user(scripts::BATTLE_CHOICE);
            if (response.to_ascii_lowercase() == "attack") {
                self.battle(&mut enemy, scripts::BATTLE_CHOICES::Attack, true);
                if enemy.get_current_hp() == 0 {
                    println!("You have defeated {}", enemy.get_name());
                    break;
                }
                self.battle(&mut enemy, scripts::BATTLE_CHOICES::Attack, false);
            } else if (response.to_ascii_lowercase() == "magicattack" || response.to_ascii_lowercase() == "magic attack") {
                self.battle(&mut enemy, scripts::BATTLE_CHOICES::MagicAttack, true);
                if enemy.get_current_hp() == 0 {
                    println!("You have defeated {}", enemy.get_name());
                    break;
                }
                self.battle(&mut enemy, scripts::BATTLE_CHOICES::MagicAttack, false);
            } else {
                println!("Invalid option!\n")
            }
            if self.player.get_current_hp() == 0 {
                println!("{} has defeated you in battle... you are dead!", enemy.get_name());
                break;
            }
            self.print_battle(&self.player, &enemy);
        }
    }
    pub fn run_game(&mut self) {
        let prompt: &str = "Name your character!";
        self.player.set_name(prompt_user(prompt));
        println!("{} sets out on their first adventure!\n", self.player.get_name());

        //Battle Loop
        self.battle_loop();
    }
}