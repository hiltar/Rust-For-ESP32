use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let conf_date : String = "9.10.2022 - 13:11".to_string();

    println!("Hello, world!");
    println!("Made by Tarmo\nat {}", conf_date);

    
}
