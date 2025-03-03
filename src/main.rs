use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct BaGua{
    id : u8,
    name : String,
    suyu : String,
    img : [u8;3],
}

impl BaGua{
    fn new(id : u8, name : &str,suyu : &str,img : [u8;3]) -> BaGua{
        BaGua {
            id,
            name : name.to_string(),
            suyu : suyu.to_string(),
            img,
        }
    }

    fn show(&self){
        println!("编号为 {}, 名字为 {}, 俗语为 {}, 挂画为：", self.id, self.name, self.suyu);
        self.draw();
    }

    fn draw(&self){
        for img in self.img.iter(){
            match img { 0 => {
                println!("—— ——")
            },
            1 => {
                println!("—————")
            },
            _=>{
                  println!("错误!")
            },
            };
        }
    }
}


fn main() {
    println!("Hello, 欢迎学习先天八卦! 给出编号猜名字！！");
    println!("例如： 编号1   乾卦");

    let bg_array:[BaGua;8] = [
        BaGua::new(1,"乾卦","乾三连",[1,1,1]),
        BaGua::new(2,"兑卦","兑上缺",[0,1,1]),
        BaGua::new(3,"离卦","离中虚",[1,0,1]),
        BaGua::new(4,"震卦","震仰盂",[0,0,1]),
        BaGua::new(5,"巽卦","巽下断",[1,1,0]),
        BaGua::new(6,"坎卦","坎中满",[0,1,0]),
        BaGua::new(7,"艮卦","艮覆碗",[1,0,0]),
        BaGua::new(8,"坤卦","坤六断",[0,0,0]),
        ];
    loop {
        let number = rand::thread_rng().gen_range(1..9);
        println!("请猜编号是 {} 的卦象是什么？", number);

        let item = bg_array.iter().find(|i| i.id == number).unwrap();


        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line");

            let guess = guess.trim().to_string();

            match item.name.cmp(&guess) {
                Ordering::Equal => {
                    println!("猜对了");
                    item.show();
                    break;
                },
                _ => {
                    println!("猜错了；继续猜");
                    continue;
                },
            }
        }
    }


    // for item in bg_array.iter(){
    //     item.show();
    // }

}
