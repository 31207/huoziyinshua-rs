use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;
// 这个模块负责连接音频文件并生成最终的输出，完全由chatgpt生成，未经过修改
// 因为本人对音频处理不太熟悉，所以这个模块的代码质量可能不太高，欢迎大家提出改进建议
use std::fs::File;
use std::io::{Seek, Write};
use anyhow::Result;

fn decode_file(path: &str) -> Result<Vec<i16>> {
    let file = File::open(path)?;
    // 创建一个MediaSourceStream来读取音频文件
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    // 使用symphonia的probe功能来自动检测音频格式并解码
    let probed = get_probe().format(
        &Hint::new(),
        mss,
        &Default::default(),
        &MetadataOptions::default(),
    )?;
    // 获取默认的音频轨道并创建一个解码器
    let mut format = probed.format;
    
    let track = format.default_track().unwrap();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())?;
    // 解码音频数据并将其存储在一个Vec<i16>中
    let mut samples: Vec<i16> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(_) => break,
        };

        let decoded = decoder.decode(&packet)?;
        // 根据解码后的PCM 音频数据类型，将其转换为i16并存储在samples中
        // 这里的代码可能不太高效，因为它需要根据不同的音频格式进行不同的处理，但它应该能够正确地处理大多数常见的音频格式
        match decoded {
            AudioBufferRef::S8(buf) => {
                for &s in buf.chan(0) {
                    samples.push((s as i16) * 256);
                }
            }
            AudioBufferRef::S16(buf) => {
                samples.extend_from_slice(buf.chan(0));
            }
            AudioBufferRef::S24(buf) => {
                for &s in buf.chan(0) {
                    samples.push((s.0 >> 8) as i16);
                }
            }
            AudioBufferRef::S32(buf) => {
                for &s in buf.chan(0) {
                    samples.push((s >> 16) as i16);
                }
            }
            AudioBufferRef::F32(buf) => {
                for &s in buf.chan(0) {
                    samples.push((s * 32767.0) as i16);
                }
            }
            AudioBufferRef::F64(buf) => {
                for &s in buf.chan(0) {
                    samples.push((s * 32767.0) as i16);
                }
            }
            _ => continue,
        }
    }

    Ok(samples)
}

pub fn concat_audio(files: &[&str]) -> Result<Vec<i16>> {
    let mut result = Vec::new();

    for file in files {
        let samples = decode_file(file)?;
        result.extend(samples);
    }

    Ok(result)
}

pub fn write_file<W: Write + Seek>(samples: &[i16], file: &mut W) -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::new(file, spec)?;

    for s in samples {
        writer.write_sample(*s)?;
    }

    writer.finalize()?;

    Ok(())
}
