enum Discount {
    Offer(i32),
    Flat(i32)
}

struct Product {
    price: Discount,
    name: String,
    remarks: Option<String>
}

impl Product {
    fn print_product(&self) {
        println!("{:?}", self.name);

        match &self.remarks {
            Some(remark) => println!("Remark : {:?}", remark),
            None => println!("No remark for this item")
        }

        match self.price {
            Discount::Flat(flat) => println!("{:?}", flat),
            Discount::Offer(offer) => println!("{:?}", offer)
        }
    }
}

fn main() {
    let products = vec![
            Product{
                price: Discount::Offer(10),
                name: String::from("Mac"),
                remarks: Some("2nd hand product".to_owned())
            },
            Product{
                price: Discount::Offer(20),
                name: String::from("Windows"),
                remarks: None
            }
        ];

    for product in &products {
        product.print_product();
    }

    for product in &products {
        match product {
            Product { name, ..} => {
                println!("{:?}", name);
            },
            _ => ()
        }
    }
}