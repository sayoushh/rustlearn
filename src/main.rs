#[derive(Debug)]
struct Rectangle {
    width : usize,
    height : usize,
}

fn draw_rect(width : usize, height : usize) -> Rectangle {
    Rectangle {width, height}
}

fn area (rect : &Rectangle) -> usize {
    rect.width * rect.height
}

fn main() {
    let rect = draw_rect(100,200);
    println!("Aire du rectangle : {}", area(&rect));
    println!("Caractéristique du rectangle : {:#?}", rect);
}
