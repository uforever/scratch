use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// Rust中的静态变量需要在编译时初始化
// 使用lazy_static定义延迟初始化的静态变量
lazy_static! {
    // pub static ref WRITER: Writer = Writer {
    //     column_position: 0,
    //     color_code: ColorCode::new(Color::Yellow, Color::Black),
    //     buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    // };
    // 但是这样还是无法使用 因为我们需要的是mut变量
    // 这里需要使用自旋互斥锁
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// allow dead_code 不使用也不会发出警告
// 生成几个trait以实现 复制 比较 调试 打印
// 其实4个bit就够了 但没有u4类型
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// repr transparent 指示类型与其单个字段的布局相同
// 可以使得在一些场景下 某些特定类型之间的转换更加方便和高效
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// 指示编译器按照C语言的布局规则对某个类型进行内存布局
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // self.buffer.chars[row][col] = ScreenChar {
                //     ascii_character: byte,
                //     color_code,
                // };
                // 改为使用volatile库 指示 易失操作
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 可以是能打印的 ASCII 码字节，也可以是换行符
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 不包含在上述范围之内的字节
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// pub fn print_something() {
//     use core::fmt::Write;
//     let mut writer = Writer {
//         column_position: 0,
//         color_code: ColorCode::new(Color::Yellow, Color::Black),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//     };

//     writer.write_byte(b'H');
//     writer.write_string("ello! ");
//     write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
// }

#[macro_export]
macro_rules! print {
    // ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    // 匹配任意数量的token tree
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

// 防止生成的文档中出现
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
