use std::io;
// use clearscreen::clear;
use rand::Rng;
use std::{thread, time};

struct GameState {
    year: u32, // Год (интерация игры)
    population: u32, // Население города
    immigrants: u32,
    peaple_with_full_tummi: u32, // Люди у которых хватает еды для жизни
    starvision_die: u32, // Умершие от голода
    acres: u32, // Количество обрабатываемых акров
    acres_to_sow: u32, // Засеянные акры
    productivity: u32, // Урожайность c акра
    cost_per_acre: u32,
    bushels: u32, // Зерно в хранилище
    loss_bushels: u32,
    bushels_for_food: u32,
    grain_yield: u32, // Общая урожайность с посевов

}

impl GameState {
    fn new() -> GameState {
        GameState {
            year: 1,
            population: 100,
            immigrants: 0,
            peaple_with_full_tummi: 0,
            starvision_die: 0,
            acres: 1000,
            acres_to_sow: 0,
            productivity: 1,
            cost_per_acre: rand::thread_rng().gen_range(10..= 30),
            bushels: 2800,
            loss_bushels: 0,
            bushels_for_food: 0,
            grain_yield: 0,
        }
    }

    fn print_status(&self) {
        //clear().unwrap();
        println!("ХАММУРАППИ: я прошу доложить мне\n");
        println!("В год правления: {}", self.year);
        if self.year > 1 { print!("Население: {}, прибыло {}, умерло от голода {} граждан. ", self.population, self.immigrants, self.starvision_die );
        }
        else {
            println!("Население: {} граждан", self.population, );
        };
        println!("Людям выдано: {} бушелей.", self.bushels_for_food);
        print!("Город владеет: {} акрами земли, ", self.acres);
        print!(" из них засеяно: {}. ", self.acres_to_sow);
        println!("Стоимость акра: {}.", self.cost_per_acre);
        print!("У вас {} бушелей зерна на складе. ", self.bushels);
        if self.year > 1 { print!("Вы собрали урожай за прошлый год: {} бушелей с акра. ", self.productivity);
        println!("Крысы съели {}", self.loss_bushels);
        };
    }

    fn handle_input(&mut self, action: i32, quantity: i32) {
        match action {
            1 => self.buy_land(quantity as u32),
            2 => self.sell_land(quantity as u32),
            3 => self.feed_people(quantity as u32),
            4 => self.plant_acres(quantity as u32),
            _ => println!("Неправильный ввод. Пожалуйста повторите."),
        }
    }

    fn buy_land(&mut self, acres_to_buy: u32) {
                let total_cost = acres_to_buy * self.cost_per_acre;

        if self.bushels >= total_cost {
            self.acres += acres_to_buy;
            self.bushels -= total_cost;
        } else {
            println!("У вас недостаточно зерна чтобы купить эту землю.");
        }
    }

    fn sell_land(&mut self, acres_to_sell: u32) {
        let sell_price_per_acre = 15; // Selling price of land in bushels per acre

        if self.acres >= acres_to_sell {
            self.acres -= acres_to_sell;
            self.bushels += acres_to_sell * sell_price_per_acre;
        } else {
            println!("У вас недостаточно земли для продажи.");
        }
    }

    fn feed_people(&mut self, bushels_for_food: u32) {
        if bushels_for_food <= self.bushels {
            self.bushels_for_food = bushels_for_food;
            self.bushels -= bushels_for_food;
        } else {
            println!("У вас недостаточно зерна на складе чтобы столько выдать людям.");
        }
    }

    fn plant_acres(&mut self, acres_to_plant: u32) {
        let bushels_per_acre = 2; // Bushels required per acre to plant
        let total_bushels_needed = acres_to_plant * bushels_per_acre;
        println!("Один человек засевает {} акров", acres_to_plant / self.population);
        if self.population * 9 >= acres_to_plant {
            if total_bushels_needed <= self.bushels {
                self.acres_to_sow += acres_to_plant;
                self.bushels -= total_bushels_needed;
            } else {
                println!("Недостаточно зерна для засева этих площадей.");
            }
        } else {
            println!("У вас только {} людей. Каждый может обработать не более 9 акров.", self.population);
        }
    }

    fn calculate_loss_from_rats(&mut self) {
        let mut rng = rand::thread_rng();
        let loss_percentage = rng.gen_range(10..30); // Random loss percentage between 10-30%
        self.loss_bushels = (self.bushels as f32 * (loss_percentage as f32 / 100.0)) as u32;
        self.bushels -= self.loss_bushels;
        }

    fn handle_year_end(&mut self) {
        self.productivity = rand::thread_rng().gen_range(5..=20);
        self.grain_yield = self.acres_to_sow * self.productivity; // каждый акр дает 1-10 единиц зерна
        self.bushels += self.grain_yield;
        self.immigrants = self.productivity * (20 * self.acres + self.bushels) / self.population / 100 + 1;
        self.peaple_with_full_tummi = self.bushels_for_food / 20;
        if self.population > self.peaple_with_full_tummi {
            self.starvision_die = self.population - self.peaple_with_full_tummi;
            self.immigrants = 0;
        } else {
            self.starvision_die = 0;
            self.immigrants = self.productivity * (20 * self.acres + self.bushels) / self.population / 100 + 1;
        };
        self.population = self.population + self.immigrants - self.starvision_die;
        self.cost_per_acre = self.productivity * 3;
        self.grain_yield = 0;
        self.acres_to_sow = 0;
        self.bushels_for_food = 0;
        self.peaple_with_full_tummi = 0;
        self.calculate_loss_from_rats();
        self.year += 1;
    }
}

fn main() {
    println!("ПОПРОБУЙТЕ СВОИ СИЛЫ В УПРАВЛЕНИИ ДРЕВНЕЙ ШУМЕРИЕЙ\n Правьте успешно в течение 10-летнего срока полномочий.");

    let mut game_state = GameState::new();

    loop {
        if game_state.year > 10 || game_state.population == 0 {
            println!("Game Over!");
            break; };
        game_state.print_status();
        println!("Выберите действие:");
        println!("1. Купить землю");
        println!("2. Продать землю");
        println!("3. Раздать людям зерно для еды");
        println!("4. Засеять землю");
        println!("5. Следующий год");
        let ten_millis = time::Duration::from_millis(5000);
        let now = time::Instant::now();
        thread::sleep(ten_millis);
        assert!(now.elapsed() >= ten_millis);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не могу прочитать строку.");
        let action: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Неправильный ввод. Пожалуйста введите правильное число.");
                continue;
            }
        };

        if action == 5 {
            game_state.handle_year_end();
        } else {
            println!("Введите количество:");
            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).expect("Не могу прочитать строку");
            let quantity: i32 = match quantity.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Неправильный ввод. Пожалуйста введите правильное число.");
                    continue;
                }
            };
            game_state.handle_input(action, quantity);
        }
    }
}