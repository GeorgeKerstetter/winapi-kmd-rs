//! Kernel Mode pools.

use ::PVOID;

extern "system"
{
	/// Allocates pool memory of the specified type and tag.
	pub fn ExAllocatePoolWithTag(PoolType: POOL_TYPE, NumberOfBytes: usize, Tag: u32) -> PVOID;
	/// Deallocates a block of pool memory allocated with the specified tag.
	pub fn ExFreePoolWithTag(P: PVOID, Tag: u32);

	/// Allocates pool memory of the specified type.
	pub fn ExAllocatePool(PoolType: POOL_TYPE, NumberOfBytes: usize) -> PVOID;
	/// Deallocates a block of pool memory.
	pub fn ExFreePool(P: PVOID);
}


/// Specifies the type of system memory to allocate.
#[repr(C)]
pub enum POOL_TYPE
{
	/// Nonpageable system memory, can be accessed from any IRQL.
  NonPagedPool = 0,
  /// Pageable system memory, can only be allocated and accessed at IRQL < DISPATCH_LEVEL.
  PagedPool,
  NonPagedPoolMustSucceed,
  DontUseThisType,
  /// Nonpaged pool, aligned on processor cache boundaries.
  NonPagedPoolCacheAligned,
  /// Paged pool, aligned on processor cache boundaries.
  PagedPoolCacheAligned,
  NonPagedPoolCacheAlignedMustS,
  MaxPoolType,
  NonPagedPoolSession = 32,
  PagedPoolSession,
  NonPagedPoolMustSucceedSession,
  DontUseThisTypeSession,
  NonPagedPoolCacheAlignedSession,
  PagedPoolCacheAlignedSession,
  NonPagedPoolCacheAlignedMustSSession,
}
