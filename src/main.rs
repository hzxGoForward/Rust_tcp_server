/*
by hzx 2020-08-01
一个简易的echo tcp server实现
*/

use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time;

/// 该函数处理接入的连接
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    // 512字节的缓冲区读取数据
    let mut buf = [0; 512];
    // 设置read函数超时时间为60秒
    stream
        .set_read_timeout(Some(time::Duration::from_secs(60 as u64)))
        .expect("set_read_timeout call failed");
    // 获取远程节点的ip地址和端口号
    let addr = stream.peer_addr().unwrap();

    // 打印远程节点的ip地址和端口号
    println!("accept a new connection from {}", addr);

    // loop无限循环，直到read函数超时或者远程客户端关闭连接
    loop {

        // 以阻塞模式尝试从stea中接收数据
        let res = stream.read(&mut buf);

        // match语句判定读取结果
        match res {
            // 读取数据成功
            Ok(bytes_read) => {
                // 如果读取数据长度大于0
                if bytes_read > 0 {
                    // 输出远程客户端ip地址、端口号以及接收的数据
                    println!(
                        "{}: {}",
                        addr,
                        str::from_utf8(&buf[..bytes_read])
                            .expect("Could not write buffer as string")
                    );
                    // echo 服务器将原数据返回
                    stream.write(&buf[..bytes_read])?;
                }
            }
            // 读取过程出现错误
            Err(e) => {
                // 打印错误
                println!("{}, error: {:?}", addr, e);

                // 跳出loop循环
                break;
            }
        }
        // 线程休息100毫秒
        thread::sleep(time::Duration::from_millis(100 as u64));
    }
    // 关闭连接
    stream
        .shutdown(Shutdown::Both)
        .expect("shutdown call failed");
    // 返回
    return Ok(());
}

fn main() -> std::io::Result<()> {
    // 创建一个TcpListener并绑定至本地8080端口
    let listener = TcpListener::bind("127.0.0.1:8080").expect("bind error");
    // 创建一个线程Vec管理线程
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    // 持续监听
    for stream in listener.incoming() {
        // 获取监听到的TcpStream，否则报错。
        let stream = stream.expect("failed!");

        // 创建一个新的线程并返回线程句柄
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        // 将该线程句柄放入thread_vec中
        thread_vec.push(handle);
    }

    // 遍历thread_vec中所有线程，等待线程执行完毕
    for handle in thread_vec {
        handle.join().unwrap();
    }
    // 返回结果
    Ok(())
}
