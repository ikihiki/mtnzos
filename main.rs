#![no_std]
#![no_main]
//#![feature(asm)]
#![feature(abi_efiapi)]
//  //extern crate rlibc;
// // (...)
extern crate alloc;
// // (...)
use crate::alloc::vec::Vec;
// extern crate uefi;
// extern crate uefi_services;
// extern crate log;


use uefi::table::boot::*;

use uefi::prelude::*;

use log::info;


#[entry]
fn uefi_start(_handle: Handle, mut system_table: uefi::table::SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap_success();

    // reset console before doing anything else
    system_table
        .stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");

    // Print out UEFI revision number
    {
        let rev = system_table.uefi_revision();
        let (major, minor) = (rev.major(), rev.minor());

        info!("UEFI {}.{}", major, minor);

        memory_map(&system_table.boot_services());
    }
    loop {}
    Status::SUCCESS
}


const EFI_PAGE_SIZE: u64 = 0x1000;
fn memory_map(bt: &BootServices) {
    // Get the estimated map size
    let map_size = bt.memory_map_size();

    // Build a buffer bigger enough to handle the memory map
    let mut buffer = Vec::with_capacity(map_size);
    unsafe {
        buffer.set_len(map_size);
    }

    let (_k, desc_iter) = bt
        .memory_map(&mut buffer)
        .expect_success("Failed to retrieve UEFI memory map");

    let descriptors = desc_iter.copied().collect::<Vec<_>>();

    assert!(!descriptors.is_empty(), "Memory map is empty");

    // Print out a list of all the usable memory we see in the memory map.
    // Don't print out everything, the memory map is probably pretty big
    // (e.g. OVMF under QEMU returns a map with nearly 50 entries here).

    info!("efi: usable memory ranges ({} total)", descriptors.len());
    descriptors
        .iter()
        .for_each(|descriptor| match descriptor.ty {
            MemoryType::CONVENTIONAL => {
                let size = descriptor.page_count * EFI_PAGE_SIZE;
                let end_address = descriptor.phys_start + size;
                info!(
                    "> {:#x} - {:#x} ({} KiB)",
                    descriptor.phys_start, end_address, size
                );
            }
            _ => {}
        })
}