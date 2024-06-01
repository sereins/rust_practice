// 宏的分类：1. 申明式宏 macros_rules 2.过程宏。
// 采用了 模式匹配的方式。模式匹配 去匹配参数里面的内容。

// 参数的类型：
// item ——一个项（item），像一个函数，结构体，模块等。
// block ——一个块 （block）（即一个语句块或一个表达式，由花括号所包围）
// stmt —— 一个语句（statement）
// pat ——一个模式（pattern）
// expr —— 一个表达式（expression）
// ty ——一个类型（type）数据类型 i32,iu
// ident—— 一个标识符（indentfier）
// path —— 一个路径（path）（例如，foo，::std::mem::replace，transmute::<_, int>，...）
// meta —— 一个元数据项；位于#[...]和#![...]属性
// tt——一个词法树
// vis——一个可能为空的Visibility限定词 pub
//
// 变量的使用 必须要加 $ 符号
// 重复 *：零次或者多次 将一个模式 用$(),+ 包裹。
// 卫生性：宏内定义的变量，只能在宏内部使用
// 作用域: 文本作用域，在后后面声明的都可以用。或者 宏需要到处 macro_export:等于将宏声明到根下 ,macros_use向外暴露一层
// TT-递归行
// 内用规则:在模式的时候使用@符号，代表只能在宏自己使用。

#[allow(unused_macros)]
macro_rules! add {
    ($a:expr ,$b:expr) => {
        $a + $b
    };
}

#[allow(unused_macros)]
macro_rules! add_as {
    ($a :expr,$b:expr,$c:ty) => {
        ($a + $b) as $c
    };
}

#[allow(unused_macros)]
macro_rules! sum {
    ($($a:expr),*) => {
        0 $(+ $a)*
    };
}

#[allow(unused_macros)]
macro_rules! min {
    ($a:expr) => {$a};
    ($a:expr,$($tail:tt)*) => {
        std::cmp::min(
            $a,
            min!($($tail)*)
        )
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_add() {
        let b = add!(1, 2);
        println!("b = {}", b);
    }

    #[test]
    fn test_add_as() {
        let b = add_as!(1, 2, u16);
        println!("b = {}", b);
    }

    #[test]
    fn test_sum() {
        let b = sum!(1, 2, 3);
        println!("b = {}", b);
    }

    #[test]
    fn test_min() {
        let b = min!(1, 2, 3, 4, 6, 7, -1);
        println!("b = {}", b);
    }
}
