// Duyệt vector nhưng trả về Result thay vì 1 vector,
// bởi vì Result có implement Vector nên có thể cast được trong trường hợp này.
// Hàm collect() sẽ chủ động dừng lại (hay chính xác hơn là interator dừng lại)
// khi có một bản ghi khiến logic bị lỗi
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| {
            println!("{}", s);
            s.parse::<i32>()
        })
        .collect();
    println!("Results: {:?}", numbers);
}