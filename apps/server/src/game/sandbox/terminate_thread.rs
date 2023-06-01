extern crate libc;
use libc::{c_int, pthread_t};

#[link(name = "c")]
extern "C" {
    fn pthread_cancel(thread: pthread_t) -> c_int;
}

pub fn terminate_thread(thread_id: usize) -> i32 {
    return unsafe { pthread_cancel((thread_id as usize).try_into().unwrap()) };
}

#[cfg(test)]
mod tests {
    use super::terminate_thread;

    #[test]
    fn successfully_terminates_thread() {
        let (tx, rx) = std::sync::mpsc::channel::<usize>();
        std::thread::spawn(move || {
            tx.send(thread_id::get()).unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(5));
                tx.send(5).unwrap();
            }
        });
        let id = rx.recv().unwrap();
        let _ = rx.recv().unwrap();
        terminate_thread(id);
        match rx.recv_timeout(std::time::Duration::from_millis(10)) {
            Ok(_) => panic!("Thread was not terminated"),
            Err(_) => (),
        };
    }
}
