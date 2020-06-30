pub mod hosting;

// Exposing paths with pub
mod serving {
    fn take_order() {}
    
    fn serve_order() {}

    fn take_payment() {}
}
