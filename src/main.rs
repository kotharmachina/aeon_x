mod memory;

fn main() {
    println!("Booting Æon-X...");

    let mut ram = memory::Memory::new();

    let test_address: u64 = 0x0200;
    let test_value: u8 = 0xFF;

    ram.write(test_address, test_value);

    let read_value = ram.read(test_address);

    println!("Ram test - Address: {:#018X} | Read/written value: {:#04X}", test_address, read_value);

    if read_value == test_value {
        println!("Hardware Status: OK! Memory operating normally.");
    } else {
        println!("Hardware Status: FAIL! Parity error.");
    }
}
