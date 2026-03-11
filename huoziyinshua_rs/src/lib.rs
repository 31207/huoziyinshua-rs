use anyhow::{Ok, Result};
use hound::Sample;
use pinyin::ToPinyin;
use std::collections::HashMap;

macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

#[derive(Debug, Default)]
pub struct Huoziyinshua {
    word_map: HashMap<String, String>,
    audio_data: Option<Vec<i16>>,
}

impl Huoziyinshua {
    pub fn new(path: &str) -> Result<Self> {
        let mut word_map: HashMap<String, String> = HashMap::new();
        let file_names = std::fs::read_dir(path)?;
        // 处理路径分隔符，确保在不同操作系统上都能正确拼接路径
        let sep = if let Some(last_char) = path.chars().last() {
            if last_char == '/' || last_char == '\\' {
                "".to_string()
            } else {
                std::path::MAIN_SEPARATOR.to_string()
            }
        } else {
            "".to_string()
        };
        // 遍历目录中的文件，将文件名（去除扩展名）作为键，文件路径作为值存储在word_map中
        for file in file_names {
            let file = file?;
            let file_name = file.file_name().into_string().unwrap_or_default();
            let file_name_without_extension = file_name.split('.').next().unwrap_or("").to_string();
            if file_name_without_extension.is_empty() {
                continue;
            }
            word_map.insert(
                file_name_without_extension,
                path.to_string() + &sep + &file_name,
            );
        }
        Ok(Self {
            word_map: word_map,
            audio_data: None,
        })
    }
    pub fn generate(&mut self, sentence: &str, original_sound: bool) -> Result<()> {
        // 原声大碟替换表
        let yuan_sheng_da_die: HashMap<&str, &str> = HashMap::from([
            ("ai ni zen me si le", "anzmsl"),
            ("ei ni zen me si le", "anzmsl"),
            ("shuo de dao li", "sddl"),
            ("da jia hao a", "djha"),
            ("ji bai", "jibai"),
            (
                "jin tian lai dian da jia xiang kan de dong xi a",
                "jtlaidian",
            ),
            ("xiong di ni dong a", "xdnda"),
            ("kui you", "Q"),
            ("wo shi dian gun", "wsdg"),
            ("a mi yu shuo de dao li", "miyu"),
            ("ei wu zi", "euz"),
            ("a ma bo bi shi wo die", "bobi"),
            ("jiu cai he zi", "jchz"),
            ("hao han", "hh"),
            ("ou nei de shou", "onds"),
            ("ou xi gei", "oxg"),
            ("zou wei", "zw"),
            ("wa ao", "waao")
        ]);

        let sentence = ascii_to_pinyin(sentence);

        // 将输入的句子转换为拼音
        let mut pinyin_str = sentence
            .as_str()
            .to_pinyin()
            .map(|pinyin| match pinyin {
                Some(p) => p.plain().to_string(),
                None => "silence".to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        debug_log!("{:?} ", pinyin_str);

        // 处理拼音，使用原声大碟替换
        if original_sound {
            for (key, value) in yuan_sheng_da_die.iter() {
                pinyin_str = pinyin_str.replace(key, value);
            }
        }
        // 将处理后的拼音字符串分割成单个词语，并将每个词语转换为对应的音频文件路径
        debug_log!("{:?} ", pinyin_str);
        let pinyin_vec: Vec<String> = pinyin_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        debug_log!("{:?} ", pinyin_vec);
        let mut paths: Vec<&str> = Vec::new();
        for word in pinyin_vec.iter() {
            if let Some(path) = self.word_map.get(word) {
                paths.push(path.as_str());
            } else {
                debug_log!("Word '{}' not found in word_map, skipping.", word);
            }
        }
        // 将所有音频文件的路径传递给audio_processor模块进行合成，并将合成后的音频数据存储在audio_data中
        let samples = audio_processor::concat_audio(&paths)?;
        self.audio_data.replace(samples);
        Ok(())
    }
    // 获取当前的音频数据，如果没有生成过音频则返回None
    pub fn get_audio_data(&self) -> Option<&Vec<i16>> {
        self.audio_data.as_ref()
    }
    // 将当前的音频数据保存为一个wav文件，如果没有生成过音频则返回错误
    pub fn save_wav(&self, path: &str) -> Result<()> {
        if let Some(audio_data) = &self.audio_data {
            audio_processor::write_wav(audio_data, path)?;
            return Ok(());
        }
        Err(anyhow::anyhow!("No data to save"))
    }
    // 以下是一些音频处理函数，可以对生成的音频数据进行各种变换，例如调整音量、反转、失真、回声、平滑和变速等
    pub fn volume(&mut self, factor: f32) -> Result<()> {
        if let Some(data) = &mut self.audio_data {
            for s in data.iter_mut() {
                let v = (*s as f32 * factor).round() as i32;
                *s = v.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            }
            return Ok(());
        }
        Err(anyhow::anyhow!("No data to transform"))
    }
    pub fn reverse(&mut self) -> Result<()> {
        if let Some(data) = &mut self.audio_data {
            data.reverse();
            return Ok(());
        }
        Err(anyhow::anyhow!("No data to transform"))
    }
    pub fn distortion(&mut self, threshold: i16) -> Result<()> {
        if let Some(data) = &mut self.audio_data {
            for s in data.iter_mut() {
                *s = s.as_i16().clamp(-threshold, threshold);
            }
            return Ok(());
        }
        Err(anyhow::anyhow!("No data to transform"))
    }
    pub fn echo(&mut self, delay_sec: f32, decay: f32) -> Result<()> {
        if let Some(data) = &mut self.audio_data {
            let delay = (delay_sec * 44100 as f32) as usize;

            for i in delay..data.len() {
                let echo = (data[i - delay] as f32 * decay) as i16;
                data[i] = data[i].saturating_add(echo);
            }
            return Ok(());
        }
        Err(anyhow::anyhow!("No data to transform"))
    }
    pub fn smooth(&mut self) -> Result<()> {
        if let Some(data) = &mut self.audio_data {
            if data.len() < 3 {
                return Err(anyhow::anyhow!("Not enough data to smooth"));
            }

            let mut new = data.clone();

            for i in 1..data.len() - 1 {
                let v = data[i - 1] as i32 + data[i] as i32 + data[i + 1] as i32;

                new[i] = (v / 3) as i16;
            }

            *data = new;
            return Ok(());
        }
        Err(anyhow::anyhow!("No data to transform"))
    }
    pub fn change_speed(&mut self, factor: f32) -> Result<()> {
        if factor <= 0.0 {
            return Err(anyhow::anyhow!("Speed factor must be greater than 0"));
        }

        if let Some(data) = &mut self.audio_data {
            let src = data.clone();
            let src_len = src.len();

            let new_len = (src_len as f32 / factor) as usize;
            let mut new_data = Vec::with_capacity(new_len);

            for i in 0..new_len {
                let pos = i as f32 * factor;
                let idx = pos.floor() as usize;
                let frac = pos - idx as f32;

                if idx + 1 < src_len {
                    let s0 = src[idx] as f32;
                    let s1 = src[idx + 1] as f32;

                    let sample = s0 + (s1 - s0) * frac;
                    new_data.push(sample as i16);
                } else {
                    new_data.push(src[idx]);
                }
            }

            *data = new_data;
            return Ok(());
        }
        Err(anyhow::anyhow!("No data to transform"))
    }
}

fn ascii_to_pinyin(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                match c {
                    'a' | 'A' => "诶".to_string(),
                    'b' | 'B' => "比".to_string(),
                    'c' | 'C' => "西".to_string(),
                    'd' | 'D' => "迪".to_string(),
                    'e' | 'E' => "伊".to_string(),
                    'f' | 'F' => "艾弗".to_string(),
                    'g' | 'G' => "吉".to_string(),
                    'h' | 'H' => "诶尺".to_string(),
                    'i' | 'I' => "艾".to_string(),
                    'j' | 'J' => "杰".to_string(),
                    'k' | 'K' => "开".to_string(),
                    'l' | 'L' => "艾勒".to_string(),
                    'm' | 'M' => "艾姆".to_string(),
                    'n' | 'N' => "摁".to_string(),
                    'o' | 'O' => "哦".to_string(),
                    'p' | 'P' => "屁".to_string(),
                    'q' | 'Q' => "亏有".to_string(),
                    'r' | 'R' => "阿".to_string(),
                    's' | 'S' => "艾丝".to_string(),
                    't' | 'T' => "提".to_string(),
                    'u' | 'U' => "伊吾".to_string(),
                    'v' | 'V' => "维".to_string(),
                    'w' | 'W' => "大不留".to_string(),
                    'x' | 'X' => "艾克斯".to_string(),
                    'y' | 'Y' => "吾艾".to_string(),
                    'z' | 'Z' => "贼".to_string(),
                    '0' => "零".to_string(),
                    '1' => "一".to_string(),
                    '2' => "二".to_string(),
                    '3' => "三".to_string(),
                    '4' => "四".to_string(),
                    '5' => "五".to_string(),
                    '6' => "六".to_string(),
                    '7' => "七".to_string(),
                    '8' => "八".to_string(),
                    '9' => "九".to_string(),
                    _ => " ".to_string(),
                }
            } else if c.is_whitespace() {
                " ".to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}

pub mod audio_processor;
