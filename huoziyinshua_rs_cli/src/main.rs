use anyhow::Ok;
use huoziyinshua_rs::Huoziyinshua;

fn main() -> anyhow::Result<()> {
    let mut huoziyinshua = Huoziyinshua::new("./sources")?;
    println!("{:?}", huoziyinshua);
    huoziyinshua.generate("诶你怎么死了？说得道理。大家好啊！击败！今天来点大家想看的东西啊！兄弟你懂啊！")?;
    huoziyinshua.save_wav("./output.wav")?;
    Ok(())
}
