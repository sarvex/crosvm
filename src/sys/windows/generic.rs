// Copyright 2022 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::BTreeMap;
use std::thread::JoinHandle;

use anyhow::Result;
use arch::RunnableLinuxVm;
use arch::VcpuArch;
use arch::VirtioDeviceStub;
use arch::VmArch;
use base::AsRawDescriptor;
use base::Event;
use base::EventToken;
use base::ProtoTube;
use base::SendTube;
use base::Tube;
use base::WaitContext;
use crosvm_cli::sys::windows::exit::Exit;
use crosvm_cli::sys::windows::exit::ExitContext;
use devices::virtio;
#[cfg(feature = "gpu")]
use devices::virtio::vhost::user::gpu::sys::windows::product::GpuBackendConfig as GpuBackendConfigProduct;
#[cfg(feature = "gpu")]
use devices::virtio::vhost::user::gpu::sys::windows::product::GpuVmmConfig as GpuVmmConfigProduct;
#[cfg(feature = "gpu")]
use devices::virtio::vhost::user::gpu::sys::windows::GpuVmmConfig;
#[cfg(feature = "audio")]
use devices::virtio::vhost::user::snd::sys::windows::product::SndBackendConfig as SndBackendConfigProduct;
#[cfg(feature = "audio")]
use devices::virtio::vhost::user::snd::sys::windows::product::SndVmmConfig as SndVmmConfigProduct;
#[cfg(feature = "audio")]
use devices::virtio::vhost::user::snd::sys::windows::SndVmmConfig;
use devices::virtio::DisplayBackend;
use devices::virtio::EventDevice;
use devices::virtio::Gpu;
use devices::virtio::GpuParameters;
pub(crate) use metrics::log_descriptor;
pub(crate) use metrics::MetricEventType;
use sync::Mutex;
use vm_control::PvClockCommand;

use super::run_vcpu::VcpuRunMode;
use crate::crosvm::config::Config;
use crate::crosvm::sys::cmdline::RunMetricsCommand;
use crate::sys::platform::TaggedControlTube as SharedTaggedControlTube;

pub struct MessageFromService {}

pub struct ServiceVmState {}

impl ServiceVmState {
    pub fn set_memory_size(&mut self, _size: u64) {}
    pub fn generate_send_state_message(&self) {}
}

pub(super) struct RunControlArgs {}

#[derive(Debug)]
pub(super) enum TaggedControlTube {
    Unused,
}

impl TaggedControlTube {
    pub fn get_read_notifier(&self) -> &dyn AsRawDescriptor {
        panic!(
            "get_read_notifier called on generic tagged control: {:?}",
            self
        )
    }
}

#[derive(EventToken, Debug)]
pub(super) enum Token {
    VmEvent,
    BrokerShutdown,
    VmControl { index: usize },
}

pub(super) fn handle_hungup_event(token: &Token) {
    panic!(
        "Unable to handle hungup on a shared token in product specific handler: {:?}",
        token
    )
}

pub(super) fn setup_common_metric_invariants(cfg: &Config) {}

// Sets package name to the name contained in `msg`.
pub(super) fn set_package_name(msg: &MessageFromService) {}

pub(super) fn get_run_control_args(cfg: &mut Config) -> RunControlArgs {
    RunControlArgs {}
}
// Merges session invariants.
pub(super) fn merge_session_invariants(serialized_session_invariants: &[u8]) {}

// Handles sending command to pvclock device.
pub(super) fn handle_pvclock_request(tube: &Option<Tube>, command: PvClockCommand) -> Result<()> {
    Ok(())
}

// Run ime thread.
pub(super) fn run_ime_thread(
    product_args: RunControlArgs,
    exit_evt: &Event,
) -> Result<Option<JoinHandle<Result<()>>>> {
    Ok(None)
}

pub(super) fn create_snd_state_tube(
    control_tubes: &mut [SharedTaggedControlTube],
) -> Result<Option<Tube>> {
    Ok(None)
}

pub(super) fn create_snd_mute_tube_pair() -> Result<(Option<Tube>, Option<Tube>)> {
    Ok((None, None))
}

// Returns two tubes and a handle to service_ipc. One for ipc_main_loop and another
// for proto_main_loop.
pub(super) fn start_service_ipc_listener(
    service_pipe_name: Option<String>,
) -> Result<(Option<Tube>, Option<ProtoTube>, Option<()>)> {
    Ok((None, None, None))
}

pub(super) fn handle_tagged_control_tube_event(
    product_tube: &TaggedControlTube,
    virtio_snd_host_mute_tube: &mut Option<Tube>,
    service_vm_state: &mut ServiceVmState,
    ipc_main_loop_tube: Option<&Tube>,
    ac97_host_tubes: &[Tube],
) {
}

pub(super) fn push_triggers<'a>(
    _triggers: &mut [(&'a dyn AsRawDescriptor, Token)],
    ipc_tube: &'a Option<Tube>,
    proto_tube: &'a Option<ProtoTube>,
) {
    if ipc_tube.is_some() {
        panic!("trying to push non-none ipc tube in generic product");
    }
    if proto_tube.is_some() {
        panic!("trying to push non-none proto tube in generic product");
    }
}

pub(super) fn handle_received_token<V: VmArch + 'static, Vcpu: VcpuArch + 'static>(
    token: &Token,
    _ac97_host_tubes: &[Tube],
    _anti_tamper_main_thread_tube: &Option<ProtoTube>,
    _balloon_host_tube: &Option<Tube>,
    _control_tubes: &[SharedTaggedControlTube],
    _guest_os: &mut RunnableLinuxVm<V, Vcpu>,
    _ipc_main_loop_tube: Option<&Tube>,
    _memory_size_mb: u64,
    _proto_main_loop_tube: Option<&ProtoTube>,
    _pvclock_host_tube: &Option<Tube>,
    _run_mode_arc: &VcpuRunMode,
    _service_vm_state: &mut ServiceVmState,
    _vcpu_boxes: &Mutex<Vec<Box<dyn VcpuArch>>>,
    _virtio_snd_host_mute_tube: &mut Option<Tube>,
) {
    panic!(
        "Received an unrecognized shared token to product specific handler: {:?}",
        token
    )
}

pub(super) fn spawn_anti_tamper_thread(wait_ctx: &WaitContext<Token>) -> Option<ProtoTube> {
    None
}

pub(super) fn create_service_vm_state(_memory_size_mb: u64) -> ServiceVmState {
    ServiceVmState {}
}

pub(super) fn create_gpu(
    vm_evt_wrtube: &SendTube,
    resource_bridges: Vec<Tube>,
    display_backends: Vec<DisplayBackend>,
    gpu_parameters: &GpuParameters,
    event_devices: Vec<EventDevice>,
    features: u64,
    _product_args: GpuBackendConfigProduct,
) -> Result<Gpu> {
    let wndproc_thread =
        virtio::gpu::start_wndproc_thread(None).expect("Failed to start wndproc_thread!");

    Ok(Gpu::new(
        vm_evt_wrtube
            .try_clone()
            .exit_context(Exit::CloneTube, "failed to clone tube")?,
        resource_bridges,
        display_backends,
        gpu_parameters,
        None,
        event_devices,
        /* external_blob= */ false,
        /* system_blob= */ false,
        features,
        BTreeMap::new(),
        wndproc_thread,
    ))
}

#[cfg(feature = "gpu")]
pub(super) fn push_gpu_control_tubes(
    _control_tubes: &mut [SharedTaggedControlTube],
    _gpu_vmm_config: &mut GpuVmmConfig,
) {
}

#[cfg(feature = "audio")]
pub(super) fn push_snd_control_tubes(
    _control_tubes: &mut [SharedTaggedControlTube],
    _snd_vmm_config: &mut SndVmmConfig,
) {
}

#[cfg(feature = "audio")]
pub(crate) fn num_input_sound_devices(_cfg: &Config) -> u32 {
    0
}

#[cfg(feature = "audio")]
pub(crate) fn num_input_sound_streams(_cfg: &Config) -> u32 {
    0
}

#[cfg(feature = "gpu")]
pub(crate) fn get_gpu_product_configs(
    cfg: &Config,
    alias_pid: u32,
) -> Result<(GpuBackendConfigProduct, GpuVmmConfigProduct)> {
    Ok((GpuBackendConfigProduct {}, GpuVmmConfigProduct {}))
}

#[cfg(feature = "audio")]
pub(crate) fn get_snd_product_configs(
    _cfg: &Config,
    _alias_pid: u32,
) -> Result<(SndBackendConfigProduct, SndVmmConfigProduct)> {
    Ok((SndBackendConfigProduct {}, SndVmmConfigProduct {}))
}

#[cfg(feature = "audio")]
pub(super) fn virtio_sound_enabled() -> bool {
    false
}

pub(crate) fn run_metrics(_args: RunMetricsCommand) -> Result<()> {
    Ok(())
}

pub(crate) fn setup_metrics_reporting() -> Result<()> {
    Ok(())
}

pub(super) fn push_mouse_device(
    cfg: &Config,
    _gpu_vmm_config: &mut GpuVmmConfig,
    _devs: &mut [VirtioDeviceStub],
) -> Result<()> {
    Ok(())
}

pub(super) fn push_pvclock_device(
    cfg: &Config,
    devs: &mut [VirtioDeviceStub],
    tsc_frequency: u64,
    tube: Tube,
) {
}
