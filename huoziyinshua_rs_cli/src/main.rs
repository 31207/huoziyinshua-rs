use anyhow::Ok;
use huoziyinshua_rs::Huoziyinshua;

fn main() -> anyhow::Result<()> {
    // 创建一个Huoziyinshua实例，指定音频样本所在的目录
    let mut huoziyinshua = Huoziyinshua::new("./sources")?;
    println!("{:?}", huoziyinshua);
    // 调用generate方法生成音频数据，参数是要生成的文本和一个布尔值，表示是否使用原生大碟替换文本中的特定词语
    huoziyinshua.generate(
        "诶你怎么死了？说得道理。大家好啊！击败！今天来点大家想看的东西啊！兄弟你懂啊！abcdefghijklmnopqrstuvwxyz1234567890!QQQQQ!我是电棍！挖澳!啊谜语说的道理！诶物资阿妈波比是我跌韭菜盒子浩瀚欧内的手偶戏给走位",
        true,
    )?;
    // 更改生成的音频数据，例如调整音量、反转、失真、回声、平滑和变速等，这些方法都是可选的，可以根据需要调用
    huoziyinshua.change_speed(1.8)?;
    // 将处理后的音频数据保存为一个wav文件，参数是要保存的文件路径
    huoziyinshua.save_wav("./output.wav")?;
    Ok(())
}
