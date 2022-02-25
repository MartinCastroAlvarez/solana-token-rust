mod data;
use data::Art;
use data::Person;
use data::Category;

fn main() {
    let image: Art = Art {
        name: String::from("Dolor"),
        price: 128319823.try_into().unwrap(),
        creator: Person {
            name: String::from("Lorem"),
            phone: 123125.try_into().unwrap()
        },
        owner: Person {
            name: String::from("Ipsum"),
            phone: 58332394.try_into().unwrap()
        },
        category: Category::Image
    };
    let video: Art = Art {
        name: String::from("Dolor"),
        price: 128319823.try_into().unwrap(),
        creator: Person {
            name: String::from("Lorem"),
            phone: 123125.try_into().unwrap()
        },
        owner: Person {
            name: String::from("Ipsum"),
            phone: 58332394.try_into().unwrap()
        },
        category: Category::Video
    };
    println!("Image: {:?}", image);
    println!("Video: {:?}", video);
}
