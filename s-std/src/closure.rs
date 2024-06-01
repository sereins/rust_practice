/// 关于rust闭包
/// 闭包设计的作用:
/// 1.我在框架设计的时候,我需要得到一个值,但是如何去得到这个值是我不关心的问题,那么我可以来使用闭包,
/// 声明一个函数来返回我需要的值。
/// 2.我可以捕获外部的一个变量,并将其保存在内存中。
/// 3.在rust中对于闭包的理解,将其当作实现了closure相关trait的struct来理解
///
/// Fn : 环境变量是复制语义 ；引用类型为什么也是fn ?
/// FnOnce: 环境变量是移动语义
/// FnMuT : 环境变量是mut 类型且内部发生了修改
#[cfg(test)]
mod test {
    #[test]
    fn simple_closure() {
        // 简单的实现闭包,闭包的参数是由调用者传递的,说起来像一句废话,但是看框架
        // 源码的时候总是在想这个参数是那个传递的。
        let c = |a: i32| a + 1;
        assert_eq!(3, c(2));

        // 可以省略参数的类型,在调用时编译器自行推导
        let c = |a, b| a + b;
        assert_eq!(2, c(1, 1));
    }

    #[test]
    // 只能执行一次的闭包
    fn once_closure() {
        let s = String::from("hello closure");
        let s_str = &s;
        println!("{}", s_str)
    }
}

