//! Architecture dependent interface to initialize a task

use alloc::rc::Rc;
use core::cell::RefCell;
use core::{mem, ptr};

use align_address::Align;

use crate::arch::aarch64::kernel::core_local::*;
use crate::arch::aarch64::kernel::processor;
use crate::arch::aarch64::mm::paging::{BasePageSize, PageSize, PageTableEntryFlags};
use crate::arch::aarch64::mm::{PhysAddr, VirtAddr};
use crate::scheduler::task::{Task, TaskFrame};
use crate::{env, DEFAULT_STACK_SIZE, KERNEL_STACK_SIZE};

extern "C" {
	static tls_start: u8;
	static tls_end: u8;
}

pub struct BootStack {
	/// stack for kernel tasks
	stack: VirtAddr,
	/// stack to handle interrupts
	ist0: VirtAddr,
}

pub struct CommonStack {
	/// start address of allocated virtual memory region
	virt_addr: VirtAddr,
	/// start address of allocated virtual memory region
	phys_addr: PhysAddr,
	/// total size of all stacks
	total_size: usize,
}

pub enum TaskStacks {
	Boot(BootStack),
	Common(CommonStack),
}

impl TaskStacks {
	/// Size of the debug marker at the very top of each stack.
	///
	/// We have a marker at the very top of the stack for debugging (`0xdeadbeef`), which should not be overridden.
	pub const MARKER_SIZE: usize = 0x10;

	pub fn new(size: usize) -> Self {
		let user_stack_size = if size < KERNEL_STACK_SIZE {
			KERNEL_STACK_SIZE
		} else {
			size.align_up(BasePageSize::SIZE as usize)
		};
		let total_size = user_stack_size + DEFAULT_STACK_SIZE + KERNEL_STACK_SIZE;
		let virt_addr =
			crate::arch::mm::virtualmem::allocate(total_size + 4 * BasePageSize::SIZE as usize)
				.expect("Failed to allocate Virtual Memory for TaskStacks");
		let phys_addr = crate::arch::mm::physicalmem::allocate(total_size)
			.expect("Failed to allocate Physical Memory for TaskStacks");

		debug!(
			"Create stacks at {:#X} with a size of {} KB",
			virt_addr,
			total_size >> 10
		);

		let mut flags = PageTableEntryFlags::empty();
		flags.normal().writable().execute_disable();

		// map IST0 into the address space
		crate::arch::mm::paging::map::<BasePageSize>(
			virt_addr + BasePageSize::SIZE,
			phys_addr,
			KERNEL_STACK_SIZE / BasePageSize::SIZE as usize,
			flags,
		);

		// map kernel stack into the address space
		crate::arch::mm::paging::map::<BasePageSize>(
			virt_addr + KERNEL_STACK_SIZE + 2 * BasePageSize::SIZE,
			phys_addr + KERNEL_STACK_SIZE,
			DEFAULT_STACK_SIZE / BasePageSize::SIZE as usize,
			flags,
		);

		// map user stack into the address space
		crate::arch::mm::paging::map::<BasePageSize>(
			virt_addr + KERNEL_STACK_SIZE + DEFAULT_STACK_SIZE + 3 * BasePageSize::SIZE,
			phys_addr + KERNEL_STACK_SIZE + DEFAULT_STACK_SIZE,
			user_stack_size / BasePageSize::SIZE as usize,
			flags,
		);

		// clear user stack
		unsafe {
			ptr::write_bytes(
				(virt_addr
					+ KERNEL_STACK_SIZE + DEFAULT_STACK_SIZE
					+ 3 * BasePageSize::SIZE as usize)
					.as_mut_ptr::<u8>(),
				0xAC,
				user_stack_size,
			);
		}

		TaskStacks::Common(CommonStack {
			virt_addr,
			phys_addr,
			total_size,
		})
	}

	pub fn from_boot_stacks() -> TaskStacks {
		//let tss = unsafe { &(*CORE_LOCAL.tss.get()) };
		/*let stack = VirtAddr::from_usize(tss.rsp[0] as usize + 0x10 - KERNEL_STACK_SIZE);
		debug!("Using boot stack {:#X}", stack);
		let ist0 = VirtAddr::from_usize(tss.ist[0] as usize + 0x10 - KERNEL_STACK_SIZE);
		debug!("IST0 is located at {:#X}", ist0);*/
		let stack = VirtAddr::zero();
		let ist0 = VirtAddr::zero();

		TaskStacks::Boot(BootStack { stack, ist0 })
	}

	pub fn get_user_stack_size(&self) -> usize {
		match self {
			TaskStacks::Boot(_) => 0,
			TaskStacks::Common(stacks) => {
				stacks.total_size - DEFAULT_STACK_SIZE - KERNEL_STACK_SIZE
			}
		}
	}

	pub fn get_user_stack(&self) -> VirtAddr {
		match self {
			TaskStacks::Boot(_) => VirtAddr::zero(),
			TaskStacks::Common(stacks) => {
				stacks.virt_addr
					+ KERNEL_STACK_SIZE + DEFAULT_STACK_SIZE
					+ 3 * BasePageSize::SIZE as usize
			}
		}
	}

	pub fn get_kernel_stack(&self) -> VirtAddr {
		match self {
			TaskStacks::Boot(stacks) => stacks.stack,
			TaskStacks::Common(stacks) => {
				stacks.virt_addr + KERNEL_STACK_SIZE + 2 * BasePageSize::SIZE as usize
			}
		}
	}

	pub fn get_kernel_stack_size(&self) -> usize {
		match self {
			TaskStacks::Boot(_) => KERNEL_STACK_SIZE,
			TaskStacks::Common(_) => DEFAULT_STACK_SIZE,
		}
	}

	pub fn get_interrupt_stack(&self) -> VirtAddr {
		match self {
			TaskStacks::Boot(stacks) => stacks.ist0,
			TaskStacks::Common(stacks) => stacks.virt_addr + BasePageSize::SIZE as usize,
		}
	}

	pub fn get_interrupt_stack_size(&self) -> usize {
		KERNEL_STACK_SIZE
	}
}

impl Drop for TaskStacks {
	fn drop(&mut self) {
		// we should never deallocate a boot stack
		match self {
			TaskStacks::Boot(_) => {}
			TaskStacks::Common(stacks) => {
				debug!(
					"Deallocating stacks at {:#X} with a size of {} KB",
					stacks.virt_addr,
					stacks.total_size >> 10,
				);

				crate::arch::mm::paging::unmap::<BasePageSize>(
					stacks.virt_addr,
					stacks.total_size / BasePageSize::SIZE as usize + 4,
				);
				crate::arch::mm::virtualmem::deallocate(
					stacks.virt_addr,
					stacks.total_size + 4 * BasePageSize::SIZE as usize,
				);
				crate::arch::mm::physicalmem::deallocate(stacks.phys_addr, stacks.total_size);
			}
		}
	}
}

pub struct TaskTLS {
	address: VirtAddr,
	//fs: VirtAddr,
	//layout: Layout,
}

impl TaskTLS {
	fn from_environment() -> Self {
		Self {
			address: VirtAddr::zero(),
		}
	}
}

impl Drop for TaskTLS {
	fn drop(&mut self) {
		/*debug!(
				"Deallocate TLS at {:#x} (layout {:?})",
				self.address, self.layout,
		);

		unsafe {
				dealloc(self.address.as_mut_ptr::<u8>(), self.layout);
		}*/
	}
}

extern "C" fn leave_task() -> ! {
	core_scheduler().exit(0)
}

extern "C" fn task_entry(func: extern "C" fn(usize), arg: usize) {
	// Check if the task (process or thread) uses Thread-Local-Storage.
	/*let tls_size = unsafe { &tls_end as *const u8 as usize - &tls_start as *const u8 as usize };
	if tls_size > 0 {
		// Yes, it does, so we have to allocate TLS memory.
		// Allocate enough space for the given size and one more variable of type usize, which holds the tls_pointer.
		let tls_allocation_size = tls_size + mem::size_of::<usize>();
		let tls = TaskTLS::from_environment();

		// The tls_pointer is the address to the end of the TLS area requested by the task.
		let tls_pointer = tls.address + tls_size;

		// TODO: Implement AArch64 TLS

		// Associate the TLS memory to the current task.
		let mut current_task_borrowed = core_scheduler().current_task.borrow_mut();
		debug!(
			"Set up TLS for task {} at address {:#X}",
			current_task_borrowed.id,
			tls.address
		);
		current_task_borrowed.tls = Some(tls);
	}*/

	// Call the actual entry point of the task.
	func(arg);
}

impl TaskFrame for Task {
	fn create_stack_frame(&mut self, func: extern "C" fn(usize), arg: usize) {
		// TODO: Implement AArch64 stack frame
	}
}
