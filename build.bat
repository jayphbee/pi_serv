set PI_JS_PROXY_EXT_CRATES=..\pi_serv_lib
set PI_JS_PROXY_TS_PATH=..\pi_pt

cargo clean -p pi_serv
cargo b
pause;
