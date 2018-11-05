extern crate gen_network;
extern crate gen_processor;
use gen_network::service::download::SyncController;
use gen_processor::ThreadService;

fn main() {
    let mut controller = SyncController::new().unwrap();
    let ten_millis = std::time::Duration::from_millis(10);
    std::thread::sleep(ten_millis);
    controller.start();

    loop {}
}