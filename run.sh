if [ $1 == "server" ];then
    cargo build --features server --bin dorea-$1 && clear && ./target/debug/dorea-$1
elif [ $1 == "cli" ];then
    cargo build --bin dorea-$1 && clear && ./target/debug/dorea-$1
else
    ECHO "unknown operation: $1 [ server , cli ]"
fi
