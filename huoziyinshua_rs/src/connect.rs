use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;
// 这个模块负责连接音频文件并生成最终的输出，完全由chatgpt生成，未经过修改
// 因为本人对rust的音频处理不太熟悉，所以这个模块的代码质量可能不太高，欢迎大家提出改进建议
use std::fs::File;
use anyhow::Result;

fn decode_file(path: &str) -> Result<Vec<i16>> {
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe().format(
        &Hint::new(),
        mss,
        &Default::default(),
        &MetadataOptions::default(),
    )?;

    let mut format = probed.format;

    let track = format.default_track().unwrap();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())?;

    let mut samples = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(_) => break,
        };

        let decoded = decoder.decode(&packet)?;

        if let AudioBufferRef::S16(buf) = decoded {
            samples.extend_from_slice(buf.chan(0));
        } 
    }

    Ok(samples)
}

pub fn concat_audio(files: &[&String]) -> Result<Vec<i16>> {
    let mut result = Vec::new();

    for file in files {
        let samples = decode_file(file)?;
        result.extend(samples);
    }

    Ok(result)
}

pub fn write_wav(samples: &[i16], path: &str) -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;

    for s in samples {
        writer.write_sample(*s)?;
    }

    writer.finalize()?;

    Ok(())
}