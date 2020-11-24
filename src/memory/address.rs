use super::config::{KERNEL_MAP_OFFSET, PAGE_SIZE};
use bit_field::BitField;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VirtualAddress(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalAddress(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VirtualPageNumber(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalPageNumber(pub usize);

impl<T> From<*const T> for VirtualAddress {
    fn from(pointer: *const T) -> Self {
        Self(pointer as usize)
    }
}

impl<T> From<*mut T> for VirtualAddress {
    fn from(pointer: *mut T) -> Self {
        Self(pointer as usize)
    }
}

impl From<PhysicalPageNumber> for VirtualPageNumber {
    fn from(ppn: PhysicalPageNumber) -> Self {
        Self(ppn.0 + KERNEL_MAP_OFFSET / PAGE_SIZE)
    }
}

impl From<VirtualPageNumber> for PhysicalPageNumber {
    fn from(vpn: VirtualPageNumber) -> Self {
        Self(vpn.0 - KERNEL_MAP_OFFSET / PAGE_SIZE)
    }
}

impl From<PhysicalAddress> for VirtualAddress {
    fn from(pa: PhysicalAddress) -> Self {
        Self(pa.0 + KERNEL_MAP_OFFSET)
    }
}

impl From<VirtualAddress> for PhysicalAddress {
    fn from(va: VirtualAddress) -> Self {
        Self(va.0 - KERNEL_MAP_OFFSET)
    }
}

impl VirtualAddress {
    pub fn deref<T>(self) -> &'static mut T {
        unsafe { &mut *(self.0 as *mut T) }
    }

    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }
}

impl PhysicalAddress {
    pub fn deref_kernel<T>(self) -> &'static mut T {
        VirtualAddress::from(self).deref()
    }

    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }
}

impl VirtualPageNumber {
    pub fn deref(self) -> &'static mut [u8; PAGE_SIZE] {
        VirtualAddress::from(self).deref()
    }
}

impl PhysicalPageNumber {
    pub fn deref_kernel(self) -> &'static mut [u8; PAGE_SIZE] {
        PhysicalAddress::from(self).deref_kernel()
    }
}

impl VirtualPageNumber {
    pub fn levels(self) -> [usize; 3] {
        [
            self.0.get_bits(18..27),
            self.0.get_bits(9..18),
            self.0.get_bits(0..9),
        ]
    }
}

macro_rules! implement_address_to_page_number {
    ($address_type: ty, $page_number_type: ty) => {
        impl From<$page_number_type> for $address_type {
            fn from(page_number: $page_number_type) -> Self {
                Self(page_number.0 * PAGE_SIZE)
            }
        }

        impl From<$address_type> for $page_number_type {
            fn from(address: $address_type) -> Self {
                assert!(address.0 % PAGE_SIZE == 0);
                Self(address.0 / PAGE_SIZE)
            }
        }

        impl $page_number_type {
            pub const fn floor(address: $address_type) -> Self {
                Self(address.0 / PAGE_SIZE)
            }

            pub const fn ceil(address: $address_type) -> Self {
                Self(address.0 / PAGE_SIZE + (address.0 % PAGE_SIZE != 0) as usize)
            }
        }
    };
}
implement_address_to_page_number! {PhysicalAddress, PhysicalPageNumber}
implement_address_to_page_number! {VirtualAddress, VirtualPageNumber}

macro_rules! implement_usize_operations {
    ($type_name: ty) => {
        /// `+`
        impl core::ops::Add<usize> for $type_name {
            type Output = Self;
            fn add(self, other: usize) -> Self::Output {
                Self(self.0 + other)
            }
        }
        /// `+=`
        impl core::ops::AddAssign<usize> for $type_name {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs;
            }
        }
        /// `-`
        impl core::ops::Sub<usize> for $type_name {
            type Output = Self;
            fn sub(self, other: usize) -> Self::Output {
                Self(self.0 - other)
            }
        }
        /// `-`
        impl core::ops::Sub<$type_name> for $type_name {
            type Output = usize;
            fn sub(self, other: $type_name) -> Self::Output {
                self.0 - other.0
            }
        }
        /// `-=`
        impl core::ops::SubAssign<usize> for $type_name {
            fn sub_assign(&mut self, rhs: usize) {
                self.0 -= rhs;
            }
        }

        impl From<usize> for $type_name {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }

        impl From<$type_name> for usize {
            fn from(value: $type_name) -> Self {
                value.0
            }
        }
        impl $type_name {
            pub fn valid(&self) -> bool {
                self.0 != 0
            }
        }

        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(0x{:x})", stringify!($type_name), self.0)
            }
        }
    };
}

implement_usize_operations! {PhysicalAddress}
implement_usize_operations! {VirtualAddress}
implement_usize_operations! {PhysicalPageNumber}
implement_usize_operations! {VirtualPageNumber}
