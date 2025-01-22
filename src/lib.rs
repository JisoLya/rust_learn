pub mod example_mod{
    pub mod inner{
        pub fn test01(){
            println!("call test01");
        }

        fn test02(){}
    }
}

mod example_mod2{
    fn call_func(){
        crate::example_mod::inner::test01();
    }
}