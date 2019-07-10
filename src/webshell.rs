use std::sync::mpsc::{Sender, Receiver, TryRecvError, channel};
use std::sync::Mutex;
use std::io::{Result as IOResult};
use std::sync::Arc;
use std::ffi::CStr;
use libc::c_char;

use pi_vm::shell::SHELL_MANAGER;

type ReqCb = Option<Box<FnOnce(Arc<Vec<u8>>)>>;

lazy_static! {
    static ref CONSOLE_OUTPUT_CHANNLE: Arc<Mutex<(Sender<String>, Receiver<String>)>> = Arc::new(Mutex::new(channel()));
}

#[no_mangle]
extern "C" fn console_output(buf: *const c_char) {
    println!("!!!!!!shell char output, {:?}", unsafe { CStr::from_ptr(buf).to_string_lossy().into_owned() });
    let console_output = unsafe { CStr::from_ptr(buf).to_string_lossy().into_owned().to_string() };
    CONSOLE_OUTPUT_CHANNLE.lock().unwrap().0.send(console_output);
}

pub struct WebShell {
    req_ch: (Sender<ReqCb>, Receiver<ReqCb>),
    resp_ch: (Sender<IOResult<Arc<Vec<u8>>>>, Receiver<IOResult<Arc<Vec<u8>>>>),
}

impl WebShell {
    pub fn new() -> WebShell {
        let (req_tx, req_rx) = channel();
        let (resp_tx, resp_rx) = channel();

        let shell_id = SHELL_MANAGER.write().unwrap().open().unwrap();
        SHELL_MANAGER.read().unwrap().init_char_output(shell_id, console_output);

        let req_tx_clone = req_tx.clone();
        let resp_tx_clone = resp_tx.clone();

        let resp = Arc::new(move |result: IOResult<Arc<Vec<u8>>>, req: Option<Box<FnOnce(Arc<Vec<u8>>)>>| {
            let _ = resp_tx_clone.send(result);
            let _ = req_tx_clone.send(req);
        });

        let req = SHELL_MANAGER.write().unwrap().connect(shell_id, resp);
        let _ = req_tx.send(req);

        WebShell {
            req_ch: (req_tx, req_rx),
            resp_ch: (resp_tx, resp_rx),
        }
    }

    pub fn exec(&self, cmd: String) -> String {
        match self.req_ch.1.recv() {
            Ok(req) => {
                if let Some(r) = req {
                    r(Arc::new(cmd.into_bytes()));
                }
            }
            Err(_) => {
                eprintln!("receive req error");
            }
        }

        match self.resp_ch.1.recv() {
            Ok(result) => {
                match result {
                    Err(e) => return e.to_string(),
                    Ok(res) => {
                        let output = String::from_utf8(res.to_vec());
                        println!(">>> shell output : {:?}", output);
                        match output {
                            Ok(r) => {
                                match CONSOLE_OUTPUT_CHANNLE.lock().unwrap().1.try_recv() {
                                    Ok(console) => return r + "\n" + &console,
                                    Err(TryRecvError::Empty) => return r,
                                    Err(TryRecvError::Disconnected) => return "Fatal error: conole output channel disconnected".to_string()
                                }
                            }
                            Err(e) => return e.to_string()
                        }
                    }
                }
            }
            Err(e) => return e.to_string()
        }
    }
}
