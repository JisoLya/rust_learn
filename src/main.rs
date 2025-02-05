use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    /*
    闭包：可以捕获其所在环境的匿名函数
     */
    simulate_expensive_caculation(3);
    generate_workout(20, 3);
    /*
       闭包的类型推断，由于闭包只在小部分上下文工作，编译器一般可以推断出相关的类型
       但是一般只会推断出一个唯一的类型,不能同时是两个类型
       let ex = |x| x;
       let s = ex(String::from("1"));
       let a = ex(1);
       这样的代码就会出错
    */

    let closure = |x: i32| -> i32 { x + 1 };
    println!("{}", closure(3));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}
impl<T> Cacher<T>
where T : Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T>{
        Cacher{
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) { 
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}
/*
   eg 生成自定义运动计划的程序
*/
fn simulate_expensive_caculation(intensity: u32) -> u32 {
    println!("Simulate slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut cache = Cacher::new(expensive_closure);
    // let expensive_result = simulate_expensive_caculation(intensity);
    /*
       这时由于上边的语句会阻塞一段时间，else 中的 if random_number 语句不需要调用这个语句,我们可以使用闭包

    */
    if intensity < 25 {
        //这里调用了两次, 不太合适,一种可行的解决方案是定义一个结构体，存储闭包并保存他的运行结果
        //在调用value方法的时候，如果value字段使用了Option<u32>类型的情况下，每次调用.value()只会获得第一次调用闭包所储存的值
        //我们进一步改进利用HashMap代替Value字段，让Cache可以保存不同的字段值
        /*
           所有的闭包都至少实现了以下的几个trait之一
           Fn
           FnMut
           FnOnce
        */
        println!("Today, do {} pushups!", cache.value(intensity));
        println!("Today, do {} situps!", cache.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cache.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        println!("v1 = {}, v2 = {}",v1,v2);
    }
}