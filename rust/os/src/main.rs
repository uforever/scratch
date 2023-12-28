#![no_std]
// 不使用标准库
// println函数不再可以使用
#![no_main]
// 自定义测试框架
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

// 引入VGA Buffer
// mod vga_buffer;
use os::{print, println};

// 出现两个Error
// 需要panic_handler函数 和 一个语言项
use core::panic::PanicInfo;

// Cargo.toml中禁用栈展开
// 会产生一个新的报错 缺少start语言项
// 这是因为大多数语言都有一个需要在main函数前启动的运行时系统
// 如Java的GC Go的goroutine Rust中是crt0
// 添加#![no_main]属性 不使用预定义的入口点

// static HELLO: &[u8] = b"Hello World!";

// 大多数系统使用_start()作为入口点名称
// 也是发散函数 直接被bootloader调用
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xa;
    //     }
    // }

    // vga_buffer::print_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(
    //     vga_buffer::WRITER.lock(),
    //     ", some numbers: {} {}",
    //     42,
    //     1.337
    // )
    // .unwrap();

    println!("Hello World{}", "!");
    // panic!("Some panic message");

    #[cfg(test)]
    test_main();

    loop {}
}

// Rust编译器默认以当前系统的三元组编译

// BIOS启动 先进入一个16位兼容的实模式
// 引导程序 存储在存储介质开头512字节
// 第一阶段引导程序长度不超过512字节
// 第二阶段引导程序长度可能较长 可以存储在其它位置
// 16位实模式 -> 32位保护模式 -> 64位长模式

// 添加目标target的json配置文件
// rustup override add nightly
// cargo install bootimage
// rustup component add llvm-tools-preview

// 发散函数 从不返回 用!表示Never类型
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    // print!("trivial assertion... ");
    // serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    // println!("[ok]");
    // serial_println!("[ok]");
}
