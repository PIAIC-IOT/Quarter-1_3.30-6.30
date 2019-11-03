fn main() {
    let v1:Vec<i32> = Vec::new();
    println!("v1= {:?}",v1);

    let v2= vec![1,2,3];


    let mut shopping_list = vec!["vegetables","snacks","eggs"];

    shopping_list.push("chicken");
    println!("Shopping list: {:?}",shopping_list);


    let elem = shopping_list.pop();
    println!("Shopping list: {:?}",shopping_list);

    //GET using [] and &
    let second = &shopping_list[2];
    println!("Second element of Shopping list: {}",second);

    //Get using get method
    let second_shopping = shopping_list.get(2);    
    println!("Second element of Shopping list: {:?}",second_shopping);

    match shopping_list.get(3) {
        Some(data) => println!("The value at index 2 is: {}",data),
        None => println!("data doesn't exists"),
    }

    #[derive(Debug)]
    enum Human {
        Height(f32),
        Name(String),
        Age(i32),
    };


    let mut human_vec : Vec<Human> = Vec::new();
    human_vec.push(Human::Height(3.3));
    human_vec.push(Human::Name(String::from("Saif Ali")));
    human_vec.push(Human::Age(24));


    println!("Human vector: {:?}",human_vec);

    let mut vector_in_vector : Vec<Vec<i32>> = Vec::new();
    let inner_vector = vec![1,2,3];
    let inner_vector2 = vec![4,5,6];

    vector_in_vector.push(inner_vector);
    vector_in_vector.push(inner_vector2);


    println!("vector in vector: {:?}",vector_in_vector);


    let mut tuple_vector : Vec<(i32,i32)> = Vec::new();

    // enum Options {
    //     Some(T),
    //     None,
    // };




}
