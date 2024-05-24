fn main() {
    let a = 1;
    let b = 2;

    let _ = a < b;
    let _ = a < -b;
    let _ = a < (-b);

    let _ = a > -b;
    let _ = a <= -b;
    let _ = a < || { 123 }();
}
