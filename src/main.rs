trait Foo {
    
} 

/*
Параметр T — это универсальный параметр типа (generic type parameter).
Он означает «абсолютно любой тип».
Такая конструкция называется реализацией-покрытием (blanket implementation).
Она говорит компилятору: «Реализуй трейт Foo для любого типа T, который 
  только существует в Rust».
*/
impl<T> Foo for T {
    
}

/* conflicting implementations of trait `Foo` for type `i32`
impl Foo for i32 {
    
}
*/

fn main() {
    println!("Hello, world!");
}
