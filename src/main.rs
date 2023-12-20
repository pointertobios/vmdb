use vmdb::Config;

fn main() {
    // 自己的预设，实际上应该在内核项目中写上配置文件，由这个程序读取
    let config = Config {
        host: "localhost".to_string(),
        port: 1234,
        kernel_elf: "../Metaverse/src/metaverse.elf".to_string(),
    };
    vmdb::run(config);
}
