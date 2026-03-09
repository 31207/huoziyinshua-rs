use anyhow::{Ok, Result};
use pinyin::ToPinyin;
use std::collections::HashMap;
#[derive(Debug, Default)]
pub struct Huoziyinshua {
    word_map: HashMap<String, String>,
    audio_data: Option<Vec<i16>>,
}

impl Huoziyinshua {
    pub fn new(path: &str) -> Result<Self> {
        let mut word_map: HashMap<String, String> = HashMap::new();
        let file_names = std::fs::read_dir(path)?;
        let sep = if let Some(last_char) = path.chars().last() {
            if last_char == '/' || last_char == '\\' {
                "".to_string()
            } else {
                std::path::MAIN_SEPARATOR.to_string()
            }
        } else {
            "".to_string()
        };
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
    pub fn generate(&mut self, sentence: &str) -> Result<()> {
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
        ]);

        // 将输入的句子转换为拼音
        let mut pinyin_str = sentence
            .to_pinyin()
            .map(|pinyin| match pinyin {
                Some(p) => p.plain().to_string(),
                None => "silence".to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{:?} ", pinyin_str);

        // 处理拼音，使用原声大碟替换
        for (key, value) in yuan_sheng_da_die.iter() {
            pinyin_str = pinyin_str.replace(key, value);
        }

        println!("{:?} ", pinyin_str);
        let pinyin_vec: Vec<String> = pinyin_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        println!("{:?} ", pinyin_vec);

        let mut paths: Vec<&str> = Vec::new();
        for word in pinyin_vec.iter() {
            if let Some(path) = self.word_map.get(word) {
                paths.push(path.as_str());
            } else {
                println!("{}: Not found", word);
            }
        }
        let samples = audio_processor::concat_audio(&paths)?;
        self.audio_data.replace(samples);
        Ok(())
    }
    pub fn get_audio_data(&self) -> Option<&Vec<i16>> {
        self.audio_data.as_ref()
    }
    pub fn save_wav(&self, path: &str) -> Result<()> {
        if let Some(audio_data) = &self.audio_data {
            audio_processor::write_wav(audio_data, path)?;
            return Ok(());
        }
        Err(anyhow::anyhow!("No data to save"))
    }
}

pub mod audio_processor;
