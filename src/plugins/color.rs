use crate::plugins::model::{ColorConfig, ColorLogConfig};
use crate::plugins::{Context, TextFilter, LOWEST_ORDER};
use owo_colors::OwoColorize;
use regex::Regex;
use std::io::Result;
use crate::log::log;
/*
使用方式

\033[显示方式;前景色;背景色m

显示方式
0（默认值）、1（高亮）、22（非粗体）、4（下划线）、24（非下划线）、5（闪烁）、25（非闪烁）、7（反显）、27（非反显）

前景色
30（黑色）、31（红色）、32（绿色）、 33（黄色）、34（蓝色）、35（洋红）、36（青色）、37（白色）

背景色
40（黑色）、41（红色）、42（绿色）、 43（黄色）、44（蓝色）、45（洋红）、46（青色）、47（白色）

控制码

控制字符是打开某种样式，输出完成时需要再关闭样式才能使terminal恢复到原来状态：

printf("\e[32m%s\e[0m\n", "hello world");

\033[0m                     关闭所有属性
\033[1m                       设置高亮度
\033[4m                          下划线
\033[5m                            闪烁
\033[7m                            反显
\033[8m                            消隐
\033[30m----\33[37m           设置前景色
\033[40m----\33[47m           设置背景色
 */

pub struct Color {
    parsers: Vec<Box<dyn ParseLog>>,
}

impl Color {
    pub fn new() -> Self {
        Self { parsers: vec![] }
    }
}

impl TextFilter for Color {
    fn name(&self) -> &'static str {
        &"color"
    }

    fn order(&self) -> u32 {
        LOWEST_ORDER - 1
    }

    fn init(&mut self, config: &str) -> std::io::Result<()> {
        let cfg: ColorConfig =
            serde_json::from_str(config).map_err(|e| std::io::Error::other(e.to_string()))?;
        self.parsers.push(IotLoggerDynParserV1::new(cfg.log));
        if self.parsers.is_empty() {
            self.parsers.push(Box::new(IotLoggerParserV1::new()));
        }
        Ok(())
    }

    fn filter(&mut self, _ctx: &mut Context, input: String) -> Option<String> {
        for p in self.parsers.iter_mut() {
            match p.parse(input.as_str()) {
                LogLevel::Unknown => continue,
                LogLevel::Debug => {
                    return Some(input);
                }
                LogLevel::Info => {
                    return Some(format!("{}", input.yellow()));
                    // return Some(format!("{}", input.fg_rgb::<255,215,0>()));
                }
                LogLevel::Warn => {
                    return Some(format!("{}", input.red()));
                    // return Some(format!("{}", input.fg_rgb::<255, 69, 0>()));
                }
                LogLevel::Error => {
                    return Some(format!("{}", input.bright_red()));
                    // return Some(format!("{}", input.fg_rgb::<255, 0, 0>()));
                }
                LogLevel::Pass => {
                    return Some(format!("{}", input.bright_green()));
                }
            }
        }
        Some(input)
    }
}

#[derive(Debug, Clone)]
enum LogLevel {
    Unknown,
    Debug,
    Info,
    Warn,
    Error,
    Pass,
}

impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        match value {
            "D" => LogLevel::Debug,
            "I" => LogLevel::Info,
            "W" => LogLevel::Warn,
            "E" => LogLevel::Error,
            "P" => LogLevel::Pass,
            _ => LogLevel::Unknown,
        }
    }
}

trait ParseLog {
    fn parse(&mut self, txt: &str) -> LogLevel;
}

struct IotLoggerParserV1 {}

impl IotLoggerParserV1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl ParseLog for IotLoggerParserV1 {
    fn parse(&mut self, txt: &str) -> LogLevel {
        if txt.len() < 3 {
            return LogLevel::Unknown;
        }
        let prefix = &txt[..3];
        match prefix {
            "[I]" => LogLevel::Info,
            "[D]" => LogLevel::Debug,
            "[W]" => LogLevel::Warn,
            "[E]" => LogLevel::Error,
            "[P]" => LogLevel::Pass,
            _ => LogLevel::Unknown,
        }
    }
}

struct IotLogRule {
    reg: Regex,
    level: LogLevel,
}

impl IotLogRule {
    pub fn new(cfg: ColorLogConfig) -> Result<Self> {
        let reg =
            Regex::new(cfg.match_.as_str()).map_err(|e| std::io::Error::other(e.to_string()))?;
        Ok(Self {
            reg,
            level: LogLevel::from(cfg.level.as_str()),
        })
    }
}

struct IotLoggerDynParserV1 {
    log_rules: Vec<IotLogRule>,
}

impl IotLoggerDynParserV1 {
    pub fn new(rules: Vec<ColorLogConfig>) -> Box<Self> {
        let mut rs = Vec::new();
        for r in rules {
            match IotLogRule::new(r.clone()) {
                Ok(r) => rs.push(r),
                Err(e) => {
                    log(format!("rule [{:?}] invalid [{}]", r, e));
                }
            }
        }
        Box::new(Self { log_rules: rs })
    }
}

impl ParseLog for IotLoggerDynParserV1 {
    fn parse(&mut self, txt: &str) -> LogLevel {
        for rule in self.log_rules.iter() {
            if rule.reg.is_match(txt) {
                return rule.level.clone();
            }
        }
        LogLevel::Unknown
    }
}
