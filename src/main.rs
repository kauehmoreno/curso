#![allow(dead_code)]
use std::mem;

struct Point{
    x: f64, 
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y:0.0}
}


pub fn stack_and_heap(){
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
}

fn if_statement(temp:u8){
    if temp > 30 {
        println!("realy hot outised");
    }else if temp < 10{
        println!("really cold!")
    }else{
        println!("temparature is ok!")
    }
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);
}

fn while_and_loop(){
    let mut x = 1;
    while x < 1000{
        x *= 2;
        if x  == 64 || x == 128 { continue; }
        println!("x={}", x);
    }

    let mut y = 1;
    // while true
    loop{
        y *= 2;
        println!("y = {}", y);
        if y ==  1<<10 { break; }
    }
}

fn for_loop(){
    for mut x in 1..11{
        x *= 2;
        if x == 8 { break; }
        println!("x = {}", x);
    }

    for (pos,y) in (30 ..41).enumerate(){
        println!("{}:{}",pos, y);
    }

}


fn match_statement(country_code:u16){

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        55 => "Brasil",
        1...999 => "Not mapped",
        _ => "Not Valid"
    };
    println!("the country with code {} is {}", country_code, country)
}


enum Color{
    Red,Green,Blue,
    RgbColor(u8,u8,u8)
}

fn enums(){
    let c:Color = Color::RgbColor(10,0,0);

    match c {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::RgbColor(0,0,0) => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})",r,g,b)
    }
}


union IntOrFloat{
    i:i32,
    f:f32
}

fn union_int_or_float(){
    let mut iof = IntOrFloat{i: 123};
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);
}

fn process_value(iof: IntOrFloat){
    unsafe{
        match iof{
            IntOrFloat{i: 42} => {
                println!("meaning of life value");
            }
            IntOrFloat{ f }  => {
                println!("value = {}", f);
            }
        }
    }
}

fn option(x:f64, y:f64){
    let result:Option<f64> = if y != 0.0{Some(x/y)} else{ None };
    println!("{:?}",result);

    // another approach
    match result{
        Some(z) => println!("{}/{} = {}",x,y,z),
        None => println!("cannot divede {} by {}",x,y)
    }
    // another approach
    if let Some(z) = result{println!("z = {}",z);}
}

// array always has fix sizes non fixes use vector
fn array(){
    let mut a:[i32;5] = [1,2,3,4,5];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 432;

    println!("a[0] = {}",a[0]);
    println!("{:?}",a);

    let b = [1u8;10];

    for elem in 0..b.len(){
        println!("element: {}", b[elem]);
    }
    println!("b took up {} bytes", mem::size_of_val(&b));

    matrix();
}

fn matrix(){
    let mtx:[[f32;3];2] =[[1.0,0.0,0.0], [1.0,2.0,0.0]];
    println!("{:?}",mtx);

    for row in 0..mtx.len(){
        for cl in 0..mtx[row].len(){
            if row == cl {
                println!("mtx[{}][{}] = {}", row, cl, mtx[row][cl]);
            }
        }
    }
}


fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);

    // to access index it must be usize to match memory machine size and must be unsigned to avoid negative numbers
    let idx:usize = 1;
    println!("a[1] = {}", a[idx]);

    // get return a option type - options allow matching
    match a.get(6){
        Some(x) => println!("a[6] = {}",x),
        None => println!("no such element on a[6]")
    }

    for el in &a{println!("{}",el );}

    a.push(44);
    a.push(77);
    // pop return a Options which can be None or Some(value)
    let last_el = a.pop();
    println!("{:?}", last_el);
}

// borrowing a part of an array i32
fn use_slice(slice: &mut[i32]){
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 32423;
}

fn slices(){
    let mut data = [1,2,3,4,5];
    println!("before {:?}",data);
    use_slice(&mut data[1..4]);
    // use_slice(&mut data);
    println!("after {:?}",data);
}


fn strings(){
    // &str = string slice 
    // is not allow to reassing a string slice
    // is valid sequence of utf-8
    let s:&'static str = "Hello there"; 

    for c in s.chars().rev(){
        println!("char {}", c);
    }

    if let Some(first_char) = s.chars().nth(0){
        println!("first letter is {}", first_char);
    }

    // heap
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8){
        letters.push(a as char);
        letters.push_str(",");
        a +=1;
    }
    println!("{}", letters);


    // &str <> String
    let u:&str = &letters;
    println!("u now is {}", u);

    // concatentation
    //  String + &str
    let z = letters + "abc";
    // let zz = letters + &letters;
    println!("z = {}", z);

    let mut abc = "hello word".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("abc = {}", abc.replace("ello", "goodbye"));
}


fn sum_and_product(x:i32,y:i32) -> (i32,i32) {
    (x+y, x*y)
}

fn tuples(){
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    println!("tuple result {:?}", sp);
    println!("{0} + {1} = {2} <-> {0} * {1} = {3} ", x,y, sp.0, sp.1);
}

fn main() {
    tuples();
}
