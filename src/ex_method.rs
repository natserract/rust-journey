
pub fn use_method() {
    fn func_map(){
        let v = vec![1, 2, 3, 4, 5];
        let mapped: Vec<i32> = v.into_iter().map(|x| x + 1).collect();
        println!("fn func_map() called, {:?}", mapped)
    }

    fn func_reverse(){
        let v = vec![1, 2, 3, 4];
        let reverse_this: Vec<i32> = v.into_iter().rev().collect();
        println!("fn func_reverse() called, {:?}", reverse_this);
    }

    fn func_push(){
        let mut v = Vec::new();
        for i in 0..10 {
            v.push(i);
        }

        println!("fn func_push() called, {:?}", v);
    }

    fn func_checklength(){
        let v = [1, 2, 3, 4, 5];
        println!("fn func_check_length() called, {:?}", v.len());
    }

    fn func_firstlast(){
        let v = [10, 20, 30, 40];
        println!("fn func_first() called, {:?}", v.first());
        println!("fn func_first() called, {:?}", v.last());
    }

    fn func_check_index(){
        let v = [10, 20, 30, 40];
        println!("Check where the index: {:?}", v.get(1));
    }

    func_map();  
    func_reverse();
    func_push();
    func_checklength();
    func_firstlast();
    func_check_index();
}
