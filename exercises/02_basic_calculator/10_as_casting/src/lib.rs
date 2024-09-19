// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = 47; // u32型の数値47？
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // The compiler is smart enough to know that the value 255 cannot fit
        // inside an i8, so it'll emit a hard error. We intentionally disable
        // this guardrail to make this (bad) conversion possible.
        // The compiler is only able to pick on this because the value is a
        // literal. If we were to use a variable, the compiler wouldn't be able to
        // catch this at compile time.
        #[allow(overflowing_literals)]
        let x = { 255 as i8 }; // 255をi8型に変換する?

        // 上記とまったく同じ式を使ってこれを解決することもできる、
        // しかし、それでは練習の意味がなくなってしまう。代わりに、本物の
        // u8` から変換すると `255` と等価になる本物の `i8` 値を使用する。
        let y: i8 = -1; // i8型の-1 = 255?

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1; // ガチで意味がわからん
        assert_eq!(true as u8, v);
    }
}
