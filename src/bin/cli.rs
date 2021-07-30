use clap::clap_app;
use dorea::client::{DoreaClient, InfoType};
use rustyline::Editor;

#[tokio::main]
pub async fn main() {

    let matches = clap_app!(dorea =>
        (version: "0.2.1")
        (author: "ZhuoEr Liu <mrxzx@qq.com>")
        (about: "Does awesome things")
        (@arg HOSTNAME: -h --hostname +takes_value "Set the server hostname")
        (@arg PORT: -p --port +takes_value "Set the server port")
        (@arg PASSWORD: -a --password +takes_value "Connect password")
    ).get_matches();

    let hostname = match matches.value_of("HOSTNAME") {
        None => "127.0.0.1",
        Some(v) => v
    }.to_string();

    let port = match matches.value_of("PORT") {
        None => 3450,
        Some(v) => {
            match v.parse::<u16>() {
                Ok(n) => n,
                Err(_) => 3450
            }
        }
    };

    let password = match matches.value_of("PASSWORD") {
        None => "",
        Some(v) => v
    };

    let password = password.clone();

    // 获取数据库客户端连接
    let c =DoreaClient::connect(
        (
            Box::leak(hostname.clone().into_boxed_str()),
            port
        ),
        password
    ).await;

    let c = match c {
        Ok(c) => c,
        Err(err) => {
            panic!("{:?}", err);
        }
    };


    let prompt = format!("{}:{} ~> ",hostname,port);
    let mut readline = Editor::<()>::new();
    loop {
        let cmd = readline.readline(&prompt);
        match cmd {
            Ok(s) => {}
            Err(_) => {}
        }
    }
}