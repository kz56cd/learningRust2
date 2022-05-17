fn main() {
    let dog = Dog{};
    let cat = Cat{};
    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    fn lifespan(&self) -> u32; // 寿命
    fn scientific_name(&self) -> String; // 学術名
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }

    fn scientific_name(&self) -> String {
        "フレンチブルドッグ".to_string()
    }
}

impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        16
    }

    fn scientific_name(&self) -> String {
        // "マンチカン".ToString();
        "マンチカン".to_string()
    }
}

fn show_animal_data<T: Animal>(animal: T) {
    println!("lifespan: {} years.", animal.lifespan());
    println!("scientific_name: {}.", animal.scientific_name());
}