//
//
// Iterating Through Options
//
//
//

fn exfunc() -> Option<String> {
    Some("Some String".to_string())
}

fn main() {
    // let some_product = Some("laptop");
    // let mut products = vec!["cellphone", "battery", "charger"];

    // match some_product {
    //     Some(product) => products.push(product),
    //     _ => {}
    // };

    // if let Some(product) = some_product {
    //     products.push(product);
    // }

    // products.extend(some_product);
    // // println!("{:?}", products);
    // let product_iterator = products.iter().chain(some_product.iter());

    // for product in product_iterator {
    //     println!("{}", product);
    // }

    let products = vec![Some("Laptop"), Some("Cable"), Some("Keyboard"), None];
    // let mut filtered_products = Vec::new();

    // for product in products {
    //     if product.is_some() {
    //         filtered_products.push(product.unwrap());
    //     }
    // }

    // let filtered_products = products
    //     .into_iter()
    //     .filter(|x| x.is_some())
    //     .map(|x| x.unwrap())
    //     .collect::<Vec<&str>>();

    let filtered_products: Vec<&str> = products.into_iter().flatten().collect();
    println!("{:?}", filtered_products);
}
