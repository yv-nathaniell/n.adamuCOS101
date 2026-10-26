//declaring the laptop struct
struct Laptop {
    brand: String,
    price: u32,
}

//impementation to calculate the cost by multiplying the price by the quantity
impl Laptop {
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850000,
    };
    let quantity = 3;

    //printing the records of stuff that was bought
    println!("\nReceipt from Alaba Electronics\n");
    println!("{} x {} at N{} each = N{}", quantity, hp.brand, hp.price, hp.cost(quantity));
    println!("{} x {} at N{} each = N{}", quantity, ibm.brand, ibm.price, ibm.cost(quantity));
    println!("{} x {} at N{} each = N{}", quantity, toshiba.brand, toshiba.price, toshiba.cost(quantity));
    println!("{} x {} at N{} each = N{}", quantity, dell.brand, dell.price, dell.cost(quantity));

    let total = hp.cost(quantity) + ibm.cost(quantity) + toshiba.cost(quantity) + dell.cost(quantity);

    println!("\nTotal cost: N{}", total);
}