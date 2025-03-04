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

fn menu(){
    println!("================= 输入序号开始游戏 ======================");
    println!("================= 1.根据编号说名字 ======================");
    println!("================= 2.根据卦象说名字 ======================");
    println!("======================================================");
}

fn main() {


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

    menu();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input:u8 = input.trim().parse().expect("Please type a number!");

    loop {
        match input {
            1 => {
                play_a(&bg_array);
            },
            2 => {
                play_b(&bg_array);
            },
            _ => {
                println!("请输入正确的编号");
            },
        }
    }

    // for item in bg_array.iter(){
    //     item.show();
    // }

}

fn play_a(bg_array:&[BaGua;8]) {
    println!("Hello, 欢迎学习先天八卦! 给出编号答名字！！");
    println!("例如： 编号1   乾卦");
    loop {
        let number = rand::thread_rng().gen_range(1..9);
        println!("请猜编号是 {} 的卦名是什么？", number);

        let item = bg_array.iter().find(|i| i.id == number).unwrap();


        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line");

            let guess = guess.trim().to_string();

            match item.name.cmp(&guess) {
                Ordering::Equal => {
                    println!("答对了");
                    item.show();
                    break;
                },
                _ => {
                    println!("答错了；继续答");
                    continue;
                },
            }
        }
    }
}

fn play_b(bg_array:&[BaGua;8]) {
    println!("Hello, 欢迎学习先天八卦! 给出卦象答名字！！");
    loop {
        let number = rand::thread_rng().gen_range(1..9);
        let item = bg_array.iter().find(|i| i.id == number).unwrap();
        item.draw();
        println!("请问这是什么卦？");

        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line");

            let guess = guess.trim().to_string();

            match item.name.cmp(&guess) {
                Ordering::Equal => {
                    println!("答对了");
                    //item.show();
                    break;
                },
                _ => {
                    println!("答错了；继续答");
                    continue;
                },
            }
        }
    }
}
