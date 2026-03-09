use huoziyinshua_rs::Huoziyinshua;

fn main() {
    let huoziyinshua = Huoziyinshua::new("./sources");
    match huoziyinshua {
        Ok(huoziyinshua) => {
            println!("{:?}", huoziyinshua);
            let _ = huoziyinshua.generate("我去啊诶爱你好，世界！说的道理啊啊诶你怎么死了尊");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

}
