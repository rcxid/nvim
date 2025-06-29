use rand::distr::Alphanumeric;
use rand::Rng;

/// 生成随机长度的字符串
pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::rng();
    std::iter::repeat(())
        .map(|()| char::from(rng.sample(Alphanumeric).to_ascii_lowercase()))
        .take(length)
        .collect()
}
