use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    // 特殊字符 、数字 、大写字母、小写字母
    #[arg(short, long, default_value_t = 12)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = false)]
    pub special: bool,
}
