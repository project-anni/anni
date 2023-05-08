use crate::route::user::AudioQuality;
use std::process::Stdio;
use tokio::process::Child;

pub trait Transcode {
    fn content_type(&self) -> &'static str;

    fn quality(&self) -> AudioQuality;

    fn need_transcode(&self) -> bool {
        self.quality().need_transcode()
    }

    fn spawn(&self) -> Child;
}

pub struct AacTranscoder(AudioQuality);

impl AacTranscoder {
    pub fn new(quality: AudioQuality) -> Self {
        if let AudioQuality::Lossless = quality {
            panic!("AacTranscoder cannot be lossless");
        }

        Self(quality)
    }
}

impl Transcode for AacTranscoder {
    fn content_type(&self) -> &'static str {
        "audio/aac"
    }

    fn quality(&self) -> AudioQuality {
        self.0
    }

    fn spawn(&self) -> Child {
        let bitrate = match self.quality() {
            AudioQuality::Low => "128k",
            AudioQuality::Medium => "192k",
            AudioQuality::High => "256k",
            AudioQuality::Lossless => unreachable!(),
        };

        tokio::process::Command::new("ffmpeg")
            .args(&[
                "-i", "pipe:0", "-map", "0:0", "-b:a", bitrate, "-f", "adts", "-",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
    }
}

pub struct OpusTranscoder(AudioQuality);

impl OpusTranscoder {
    pub fn new(quality: AudioQuality) -> Self {
        if let AudioQuality::Lossless = quality {
            panic!("OpusTranscoder cannot be lossless");
        }

        Self(quality)
    }
}

impl Transcode for OpusTranscoder {
    fn content_type(&self) -> &'static str {
        "audio/opus"
    }

    fn quality(&self) -> AudioQuality {
        self.0
    }

    fn spawn(&self) -> Child {
        let bitrate = match self.quality() {
            AudioQuality::Low => "128",
            AudioQuality::Medium => "192",
            AudioQuality::High => "256",
            AudioQuality::Lossless => unreachable!(),
        };

        #[rustfmt::skip]
        let args = &[
            "--bitrate", bitrate,
            "--hard-cbr",
            "--music",
            "--framesize", "20",
            "--comp", "0",
            "--discard-comments",
            "--discard-pictures",
            "-", // input from stdin
            "-", // output to stdout
        ];

        tokio::process::Command::new("opusenc")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
    }
}

pub struct FlacTranscoder;

impl FlacTranscoder {
    pub fn new(quality: AudioQuality) -> Self {
        if let AudioQuality::Lossless = quality {
            Self
        } else {
            panic!("FlacTranscoder can only be lossless");
        }
    }
}

impl Transcode for FlacTranscoder {
    fn content_type(&self) -> &'static str {
        "audio/flac"
    }

    fn quality(&self) -> AudioQuality {
        AudioQuality::Lossless
    }

    fn spawn(&self) -> Child {
        panic!("FlacTranscoder cannot transcode")
    }
}
