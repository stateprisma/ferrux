use limine::{
    request::{FramebufferRequest, HhdmRequest, MemoryMapRequest, RsdpRequest},
    BaseRevision,
};

pub static BASE_REVISION: BaseRevision = BaseRevision::new();

pub static PHYSMEM_MAP_REQUEST: HhdmRequest = HhdmRequest::new();

pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();
