# 숫자를 문자열로 표기하기
## 유한수의 경우
* 계수는 먼저 선행 0이 없는 0부터 9까지의 문자를 사용하여 10진수의 문자열로 변환됩니다(값이 0인 경우 0 문자 하나만 사용됨).
* 다음으로 조정된 지수가 계산되는데, 이는 지수에 변환된 계수의 문자 수를 더한 값에서 1을 뺀 값입니다. 즉, 지수+(길이-1)이며, 여기서 길이는 소수점 이하로 표시된 계수의 길이입니다.
* 지수가 0보다 작거나 같고 조정된 지수가 -6보다 크면 지수 표기법을 사용하지 않고 숫자가 문자 형식으로 변환됩니다. 이 경우 지수가 0이면 소수점이 추가되지 않습니다. 그렇지 않은 경우(지수가 음수인 경우) 소수점 오른쪽에 문자 수를 지정하는 지수의 절대값과 함께 소수점이 삽입됩니다. 필요에 따라 변환된 계수의 왼쪽에 '0' 문자가 추가됩니다. 이 삽입 후 소수점 앞에 오는 문자가 없으면 기존의 '0' 문자가 앞에 붙습니다.
* 그렇지 않은 경우(즉, 지수가 양수이거나 조정된 지수가 -6보다 작은 경우)에는 지수 표기법을 사용하여 숫자가 문자 형식으로 변환됩니다. 이 경우 변환된 계수의 자릿수가 두 자리 이상이면 첫 번째 자리 뒤에 소수점이 삽입됩니다. 
* 그런 다음 변환된 계수(소수점이 삽입된 경우)에 문자 형식의 지수가 접미사로 붙는데, 문자 'E' 뒤에 바로 뒤에 문자 형식으로 변환된 조정된 지수가 붙습니다. 후자는 10진법으로, 선행 0이 없는 0부터 9까지의 문자를 사용하며 항상 부호 문자(계산된 지수가 음수인 경우 '-', 그렇지 않은 경우 '+')가 앞에 붙습니다.

## 그렇지 않으면 (숫자는 특수 값)
* 특수 값이 조용한 NaN인 경우 결과 문자열은 'NaN'이며, 선택적으로 진단 정보를 나타내는 하나 이상의 자리가 뒤에옵니다. 숫자에는 선행 0이 없습니다.
* 특수 값이 신호 NaN인 경우 결과 문자열은 'sNaN'이며,[4] 선택적으로 조용한 NaN과 같이 진단 정보를 나타내는 하나 이상의 숫자가 뒤따릅니다.
* 특수 값이 무한대인 경우 결과 문자열은 '무한대'입니다.

``` Rust
// Decimal 을 생각해보자.
// 문자를 받지만, 소유권을 가져가는 것이 아무 문제도 되지 않는다.
// 사용자가 소유권을 지킬 생각이 없을 가능성이 크기 때문이다.
// 또한 문자열을 수정할 필요가 있을 수 있기에 `String` 이 적절한 타입일 수 있다는 것을 생각하자.

enum NumericStringSign {
    Positive,
    Negative
}

enum NumericStringDigits {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

enum NumericString {
    Sign(NumericStringSign),
    Digit(NumericStringDigits)
}

struct Decimal<'a> {
    s: &'a str,
    prec: usize // Precision
}

impl <'a>Decimal<'a> {
    fn new(prec: usize) -> Self {
        Self {s: "", prec}
    }

    fn display(&self) {
        println!("{}", &self.s);
    }
}

fn main() {
    let d1: Decimal = Decimal::new(100);
    d1.display();

    let ns1: Vec<NumericString> = vec![NumericString::Sign(NumericStringSign::Negative), NumericString::Digit(NumericStringDigits::Five)];
}
```