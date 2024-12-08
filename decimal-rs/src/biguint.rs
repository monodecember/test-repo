// Decimal 의 연산을 위해서 구현한 간단한 큰 수 연산
// 어차피 Decimal 의 특성상 부호 있는 연산은 이루어지지 않음.
pub struct BigUint {
    // u128 의 연산은 하드웨어 지원으로 대부분 경우 다중 정수 배열 보다 빠름.
    // 이 연산을 사용하는 사용자는 대부분 큰 수 계산을 할 것을 예측한 것임.
    // Decimal 은 다양한 사용자의 경우의 수가 있기 때문에 작은 단위로 시작할 것.
    v: Vec<u32>,
    sign: bool
}

pub trait ToBigUnt {
    fn to_biguint(&self) -> Option<BigUint>;
}