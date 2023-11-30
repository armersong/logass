use crate::plugins::{LOWEST_ORDER, TextFilter};

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

}

impl TextFilter for Color {
    fn name(&self) -> &'static str {
        &"color"
    }

    fn order(&self) -> u32 {
        LOWEST_ORDER -1
    }

    fn init(&mut self, _config: &str) -> std::io::Result<()> {
        todo!()
    }

    fn filter(&mut self, input: String) -> Option<String> {
        todo!()
    }
}