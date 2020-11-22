use super::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

// ダイナミックメモリの割り当てに使用されるヒープ空間
///
// サイズを [`KERNEL_HEAP_SIZE`] に変更します。
// このスペースはコンパイルされ、オペレーティングシステムの実行ファイルのbssセクションに配置されます。
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

// [global_allocator
// [ `LockedHeap`] は [`alloc::alloc::GlobalAlloc`] trait を実装しています。
// グローバルにヒープが必要な場所にスペースを割り当てることができます。 例えば、`Box` `Arc` などです。
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

// OSのランタイムヒープ空間を初期化する
pub fn init() {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

// スペース確保エラーのコールバック、ダイレクトパニック終了
#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!("alloc error")
}
