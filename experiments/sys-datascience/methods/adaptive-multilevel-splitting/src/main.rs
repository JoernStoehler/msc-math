use adaptive_multilevel_splitting::{
    file_sha256, run_packet, run_synthetic_packet, synthetic_observation, ArtifactSink, Config,
    EvaluationStatus, Manifest, Observation, Oracle, OracleOutcome, OracleRequest, RunOutcome,
    SourceIdentity, TargetDiagnostics, TerminalErrorKind, ADAPTIVE_BUDGET, GENERATION_SCHEDULE,
    IID_BUDGET, MUTATION_KERNEL,
};
use exp_sys_landscape::{
    compute_sys_computation, exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache,
};
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::io::{Read, Write};
#[cfg(target_os = "linux")]
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use symplectic::OrbitAdmissibility;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TargetMode {
    Synthetic,
    Production,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Synthetic,
    Production,
}

struct Args {
    mode: Mode,
    config: PathBuf,
    artifacts: PathBuf,
    reviewed_commit: Option<String>,
    force_synthetic_hit: bool,
    synthetic_hit_call: Option<usize>,
    synthetic_fail_call: Option<usize>,
    synthetic_child_delay_ms: u64,
    synthetic_call_timeout_ms: Option<u64>,
    synthetic_response_padding_bytes: usize,
}

#[derive(Serialize, Deserialize)]
struct TargetOnceRequest {
    mode: TargetMode,
    exact_geometry_key: String,
    dual_vertices_f64: Vec<[f64; 4]>,
    synthetic_force_hit: bool,
    synthetic_force_failure: bool,
    synthetic_validate_constructor: bool,
    synthetic_delay_ms: u64,
    synthetic_response_padding_bytes: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
enum TargetOnceResponse {
    Success {
        observation: Observation,
        synthetic_padding: Option<String>,
    },
    Failure {
        evaluation_status: EvaluationStatus,
        reason: String,
    },
}

#[derive(Serialize)]
struct PrintedOutcome {
    artifact_kind: &'static str,
    readiness_complete: bool,
    adaptive_attempts: usize,
    iid_attempts: usize,
    stopped_on_sys_gt_one: bool,
    artifacts: PathBuf,
}

struct ChildOracle {
    executable: PathBuf,
    mode: TargetMode,
    calls: usize,
    force_first_hit: bool,
    synthetic_hit_call: Option<usize>,
    synthetic_fail_call: Option<usize>,
    synthetic_delay_ms: u64,
    call_timeout_override: Option<Duration>,
    synthetic_response_padding_bytes: usize,
}

impl ChildOracle {
    #[allow(clippy::too_many_arguments)]
    fn new(
        executable: PathBuf,
        mode: TargetMode,
        force_first_hit: bool,
        synthetic_hit_call: Option<usize>,
        synthetic_fail_call: Option<usize>,
        synthetic_delay_ms: u64,
        call_timeout_override: Option<Duration>,
        synthetic_response_padding_bytes: usize,
    ) -> Self {
        Self {
            executable,
            mode,
            calls: 0,
            force_first_hit,
            synthetic_hit_call,
            synthetic_fail_call,
            synthetic_delay_ms,
            call_timeout_override,
            synthetic_response_padding_bytes,
        }
    }
}

impl Oracle for ChildOracle {
    fn compute(&mut self, request: OracleRequest<'_>, remaining: Duration) -> OracleOutcome {
        self.calls += 1;
        let timeout = self
            .call_timeout_override
            .map_or(remaining, |override_timeout| {
                remaining.min(override_timeout)
            });
        let payload = TargetOnceRequest {
            mode: self.mode,
            exact_geometry_key: request.exact_geometry_key.to_owned(),
            dual_vertices_f64: request.dual_vertices_f64.to_vec(),
            synthetic_force_hit: (self.force_first_hit && self.calls == 1)
                || self.synthetic_hit_call == Some(self.calls),
            synthetic_force_failure: self.synthetic_fail_call == Some(self.calls),
            synthetic_validate_constructor: self.calls == 1,
            synthetic_delay_ms: self.synthetic_delay_ms,
            synthetic_response_padding_bytes: self.synthetic_response_padding_bytes,
        };
        match invoke_target_child(&self.executable, &payload, timeout) {
            Ok(TargetOnceResponse::Success { observation, .. }) => {
                OracleOutcome::Success(observation)
            }
            Ok(TargetOnceResponse::Failure {
                evaluation_status,
                reason,
            }) => OracleOutcome::Failure {
                status: evaluation_status,
                reason,
            },
            Err(ChildInvokeError::Timeout(reason)) => OracleOutcome::Failure {
                status: EvaluationStatus::Timeout,
                reason,
            },
            Err(ChildInvokeError::Failure(reason)) => OracleOutcome::Failure {
                status: EvaluationStatus::ChildFailure,
                reason,
            },
        }
    }
}

enum ChildInvokeError {
    Timeout(String),
    Failure(String),
}

const MAX_TARGET_RESPONSE_BYTES: u64 = 64 * 1024 * 1024;
const MAX_TARGET_STDERR_BYTES: u64 = 1024 * 1024;

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn real_main() -> Result<(), String> {
    if env::args().nth(1).as_deref() == Some("target-once") {
        return target_once();
    }
    let args = parse_args()?;
    let config = Config::from_path(&args.config)?;
    let executable = env::current_exe().map_err(|e| format!("locate current executable: {e}"))?;
    let source = source_identity(args.mode, args.reviewed_commit.as_deref(), &executable)?;
    validate_launch(&args, &source)?;
    if args.mode == Mode::Production
        && env::var_os("AMS_TEST_REFUSAL_ONLY").is_some()
        && env::var_os("AMS_TEST_VALIDATE_DIRTY_SOURCE").is_none()
    {
        return Err(
            "production target disabled by the private AMS_TEST_REFUSAL_ONLY safety guard".into(),
        );
    }
    if args.mode == Mode::Production && !source.source_tree_clean {
        return Err("production target execution refuses a dirty or untracked source tree".into());
    }
    if args.mode == Mode::Production && env::var_os("AMS_TEST_REFUSAL_ONLY").is_some() {
        return Err(
            "production target disabled by the private AMS_TEST_REFUSAL_ONLY safety guard".into(),
        );
    }
    let artifact_kind = match args.mode {
        Mode::Synthetic => "synthetic_target_free",
        Mode::Production => "production_target",
    };
    let started = Instant::now();
    let start_unix_ms = unix_time_ms()?;
    let run_id = run_id(start_unix_ms, &source.git_revision, &args.artifacts);
    let manifest = Manifest {
        artifact_kind: artifact_kind.into(),
        run_id,
        start_unix_ms,
        launch_process_id: std::process::id(),
        artifact_directory: args.artifacts.display().to_string(),
        config_identity: config.identity(),
        exact_config: config.clone(),
        source,
        adaptive_budget: ADAPTIVE_BUDGET,
        iid_budget: IID_BUDGET,
        target_probability_estimate: None,
        tail_probability_supported: false,
        mutation_kernel: MUTATION_KERNEL.into(),
        generation_schedule: GENERATION_SCHEDULE.into(),
        factor_exchange_quotiented: false,
    };
    let sink = ArtifactSink::create(&args.artifacts, &manifest)?;
    let source_revision = manifest.source.git_revision.clone();
    let child_mode = match args.mode {
        Mode::Synthetic => TargetMode::Synthetic,
        Mode::Production => TargetMode::Production,
    };
    let timeout_override = args.synthetic_call_timeout_ms.map(Duration::from_millis);
    let mut adaptive = ChildOracle::new(
        executable.clone(),
        child_mode,
        args.force_synthetic_hit,
        args.synthetic_hit_call,
        args.synthetic_fail_call,
        args.synthetic_child_delay_ms,
        timeout_override,
        args.synthetic_response_padding_bytes,
    );
    let mut iid = ChildOracle::new(
        executable,
        child_mode,
        false,
        None,
        None,
        0,
        timeout_override,
        0,
    );
    let run_result = match args.mode {
        Mode::Synthetic => {
            run_synthetic_packet(&config, &source_revision, &mut adaptive, &mut iid, &sink)
        }
        Mode::Production => run_packet(&config, &source_revision, &mut adaptive, &mut iid, &sink),
    };
    let terminal_error = match &run_result {
        Ok(_) => None,
        Err(error) => Some(sink.terminal_error_evidence(error)?),
    };
    let (disposition, status_error) = match (&run_result, terminal_error.as_ref()) {
        (Ok(outcome), _) if outcome.stopped.is_some() => ("sys_gt_one_stop", None),
        (Ok(_), _) => ("complete", None),
        (Err(error), Some(evidence))
            if matches!(evidence.kind, TerminalErrorKind::FailedTarget)
                && evidence.evaluation_status == Some(EvaluationStatus::Timeout) =>
        {
            ("timeout", Some(error.clone()))
        }
        (Err(error), _) => ("error", Some(error.clone())),
    };
    sink.finalize(disposition, status_error, terminal_error, started)?;
    let outcome = run_result?;
    print_outcome(args.mode, &args.artifacts, &outcome)?;
    Ok(())
}

fn validate_launch(args: &Args, source: &SourceIdentity) -> Result<(), String> {
    match args.mode {
        Mode::Synthetic => {
            if args.reviewed_commit.is_some() {
                return Err("--reviewed-commit is production-only".into());
            }
        }
        Mode::Production => {
            if args.force_synthetic_hit
                || args.synthetic_child_delay_ms != 0
                || args.synthetic_call_timeout_ms.is_some()
                || args.synthetic_response_padding_bytes != 0
                || args.synthetic_hit_call.is_some()
                || args.synthetic_fail_call.is_some()
            {
                return Err("synthetic test flags are prohibited in production mode".into());
            }
            let reviewed = source
                .reviewed_revision
                .as_deref()
                .ok_or("production requires --reviewed-commit FULL_40_HEX_REVISION")?;
            if source.git_revision != reviewed {
                return Err(format!(
                    "production HEAD {} does not equal reviewed commit {reviewed}",
                    source.git_revision
                ));
            }
        }
    }
    Ok(())
}

fn print_outcome(mode: Mode, artifacts: &Path, outcome: &RunOutcome) -> Result<(), String> {
    let printed = PrintedOutcome {
        artifact_kind: match mode {
            Mode::Synthetic => "synthetic_target_free",
            Mode::Production => "production_target",
        },
        readiness_complete: outcome.stopped.is_none(),
        adaptive_attempts: outcome.adaptive_attempts,
        iid_attempts: outcome.iid_attempts,
        stopped_on_sys_gt_one: outcome.stopped.is_some(),
        artifacts: artifacts.to_owned(),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&printed).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn target_once() -> Result<(), String> {
    if env::args().len() != 2 {
        return Err("private target-once subcommand accepts its request only on stdin".into());
    }
    let mut bytes = Vec::new();
    std::io::stdin()
        .read_to_end(&mut bytes)
        .map_err(|e| format!("read target-once request: {e}"))?;
    let request: TargetOnceRequest =
        serde_json::from_slice(&bytes).map_err(|e| format!("parse target-once request: {e}"))?;
    if request.mode == TargetMode::Production && env::var_os("AMS_TEST_REFUSAL_ONLY").is_some() {
        return Err(
            "production target child disabled by the private AMS_TEST_REFUSAL_ONLY safety guard"
                .into(),
        );
    }
    if request.synthetic_delay_ms > 0 {
        if request.mode != TargetMode::Synthetic {
            return Err("synthetic delay is prohibited for a production target child".into());
        }
        thread::sleep(Duration::from_millis(request.synthetic_delay_ms));
    }
    let response = match request.mode {
        TargetMode::Synthetic => {
            let volume = if request.synthetic_validate_constructor {
                let polytope = match reconstruct_target_geometry(&request.dual_vertices_f64) {
                    Ok(polytope) => polytope,
                    Err(reason) => {
                        return write_target_once_response(TargetOnceResponse::Failure {
                            evaluation_status: EvaluationStatus::TargetUnavailable,
                            reason,
                        })
                    }
                };
                exact_volume_from_incidence_as_f64(
                    &polytope.vertices,
                    &polytope.vertex_facet_incidence,
                )
            } else {
                match synthetic_product_volume(&request.dual_vertices_f64) {
                    Some(volume) => volume,
                    None => {
                        return write_target_once_response(TargetOnceResponse::Failure {
                            evaluation_status: EvaluationStatus::TargetUnavailable,
                            reason: "synthetic product-volume reconstruction rejected geometry"
                                .into(),
                        })
                    }
                }
            };
            if request.synthetic_force_failure {
                return write_target_once_response(TargetOnceResponse::Failure {
                    evaluation_status: EvaluationStatus::TargetUnavailable,
                    reason: "synthetic structured target failure fixture".into(),
                });
            }
            let digest = Sha256::digest(request.exact_geometry_key.as_bytes());
            let fraction = u64::from_be_bytes(digest[..8].try_into().expect("eight digest bytes"))
                as f64
                / u64::MAX as f64;
            let sys = if request.synthetic_force_hit {
                1.01
            } else {
                0.72 + 0.2 * fraction
            };
            let capacity = (2.0 * volume * sys).sqrt();
            TargetOnceResponse::Success {
                observation: synthetic_observation(capacity, sys),
                synthetic_padding: (request.synthetic_response_padding_bytes != 0)
                    .then(|| "x".repeat(request.synthetic_response_padding_bytes)),
            }
        }
        TargetMode::Production => production_target_once(request.dual_vertices_f64),
    };
    write_target_once_response(response)
}

fn write_target_once_response(response: TargetOnceResponse) -> Result<(), String> {
    serde_json::to_writer(std::io::stdout(), &response)
        .map_err(|e| format!("write target-once response: {e}"))?;
    std::io::stdout()
        .write_all(b"\n")
        .map_err(|e| format!("flush target-once response: {e}"))?;
    Ok(())
}

fn production_target_once(vertices: Vec<[f64; 4]>) -> TargetOnceResponse {
    let polytope = match reconstruct_target_geometry(&vertices) {
        Ok(polytope) => polytope,
        Err(reason) => {
            return TargetOnceResponse::Failure {
                evaluation_status: EvaluationStatus::TargetUnavailable,
                reason,
            }
        }
    };
    let Some(computation) = compute_sys_computation(&polytope) else {
        return TargetOnceResponse::Failure {
            evaluation_status: EvaluationStatus::TargetUnavailable,
            reason: "current automatic capacity route returned no computation".into(),
        };
    };
    let capacity_result = match serde_json::to_value(&computation.capacity) {
        Ok(value) => value,
        Err(error) => {
            return TargetOnceResponse::Failure {
                evaluation_status: EvaluationStatus::ChildFailure,
                reason: format!("serialize OrbitSearchResult: {error}"),
            }
        }
    };
    let exact_admissible_count = computation
        .capacity
        .orbits
        .iter()
        .filter(|orbit| matches!(orbit.admissibility, OrbitAdmissibility::AdmissibleExact))
        .count();
    let indeterminate_count = computation
        .capacity
        .orbits
        .iter()
        .filter(|orbit| matches!(orbit.admissibility, OrbitAdmissibility::IndeterminateF64))
        .count();
    TargetOnceResponse::Success {
        observation: Observation {
            capacity: computation.capacity.min_action,
            volume: computation.vol,
            sys: computation.sys,
            diagnostics: TargetDiagnostics {
                iterations: computation.capacity.iterations,
                returned_orbit_count: computation.capacity.orbits.len(),
                action_lower: computation.capacity.min_action_lower,
                action_upper: computation.capacity.min_action_upper,
                exact_admissible_count,
                indeterminate_count,
            },
            capacity_result: Some(capacity_result),
            audit_kind: "full_orbit_search_result".into(),
        },
        synthetic_padding: None,
    }
}

fn reconstruct_target_geometry(vertices: &[[f64; 4]]) -> Result<SysLandscapePolytopeCache, String> {
    if vertices.len() != 10 || vertices.iter().flatten().any(|value| !value.is_finite()) {
        return Err("target child received invalid 5 x 5 f64 dual geometry".into());
    }
    let vectors = vertices
        .iter()
        .map(|row| Vector4::new(row[0], row[1], row[2], row[3]))
        .collect();
    SysLandscapePolytopeCache::from_f64_dual_vertices(vectors)
        .ok_or_else(|| "SysLandscapePolytopeCache::from_f64_dual_vertices rejected geometry".into())
}

fn synthetic_product_volume(vertices: &[[f64; 4]]) -> Option<f64> {
    if vertices.len() != 10 || vertices.iter().flatten().any(|value| !value.is_finite()) {
        return None;
    }
    let q: Vec<[f64; 2]> = vertices[..5].iter().map(|row| [row[0], row[1]]).collect();
    let p: Vec<[f64; 2]> = vertices[5..].iter().map(|row| [row[2], row[3]]).collect();
    Some(synthetic_primal_area(q)? * synthetic_primal_area(p)?)
}

fn synthetic_primal_area(mut duals: Vec<[f64; 2]>) -> Option<f64> {
    duals.sort_by(|left, right| left[1].atan2(left[0]).total_cmp(&right[1].atan2(right[0])));
    let mut primal = Vec::with_capacity(duals.len());
    for index in 0..duals.len() {
        let left = duals[index];
        let right = duals[(index + 1) % duals.len()];
        let determinant = left[0] * right[1] - left[1] * right[0];
        if !determinant.is_finite() || determinant.abs() <= f64::EPSILON {
            return None;
        }
        primal.push([
            (right[1] - left[1]) / determinant,
            (left[0] - right[0]) / determinant,
        ]);
    }
    let twice_area: f64 = (0..primal.len())
        .map(|index| {
            let left = primal[index];
            let right = primal[(index + 1) % primal.len()];
            left[0] * right[1] - left[1] * right[0]
        })
        .sum();
    let area = twice_area.abs() / 2.0;
    (area.is_finite() && area > 0.0).then_some(area)
}

fn invoke_target_child(
    executable: &Path,
    request: &TargetOnceRequest,
    timeout: Duration,
) -> Result<TargetOnceResponse, ChildInvokeError> {
    let mut command = Command::new(executable);
    command
        .arg("target-once")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    configure_parent_death(&mut command)?;
    let mut child = command
        .spawn()
        .map_err(|e| ChildInvokeError::Failure(format!("spawn target child: {e}")))?;
    let readers = start_child_pipe_readers(&mut child)?;
    let payload = serde_json::to_vec(request)
        .map_err(|e| ChildInvokeError::Failure(format!("serialize target request: {e}")))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(&payload)
            .map_err(|e| ChildInvokeError::Failure(format!("feed target child: {e}")))?;
    }
    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let completed_elapsed = started.elapsed();
                let (stdout, stderr) = finish_child_pipe_readers(readers)?;
                if !status.success() {
                    return Err(ChildInvokeError::Failure(format!(
                        "target child exited {status}: {}",
                        String::from_utf8_lossy(&stderr).trim()
                    )));
                }
                let response: TargetOnceResponse =
                    serde_json::from_slice(&stdout).map_err(|e| {
                        ChildInvokeError::Failure(format!("parse target child response: {e}"))
                    })?;
                return classify_completed_response(response, completed_elapsed, timeout);
            }
            Ok(None) if started.elapsed() < timeout => {
                thread::sleep(Duration::from_millis(2));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = finish_child_pipe_readers(readers);
                return Err(ChildInvokeError::Timeout(format!(
                    "target child exceeded remaining {:.3} ms deadline and was killed",
                    timeout.as_secs_f64() * 1_000.0
                )));
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = finish_child_pipe_readers(readers);
                return Err(ChildInvokeError::Failure(format!(
                    "poll target child: {error}"
                )));
            }
        }
    }
}

fn response_is_sys_hit(response: &TargetOnceResponse) -> bool {
    matches!(
        response,
        TargetOnceResponse::Success { observation, .. } if observation.sys > 1.0
    )
}

fn classify_completed_response(
    response: TargetOnceResponse,
    elapsed: Duration,
    timeout: Duration,
) -> Result<TargetOnceResponse, ChildInvokeError> {
    if elapsed > timeout && !response_is_sys_hit(&response) {
        return Err(ChildInvokeError::Timeout(format!(
            "target child completed after {:.3} ms deadline; late non-hit was charged as timeout",
            timeout.as_secs_f64() * 1_000.0
        )));
    }
    Ok(response)
}

#[cfg(target_os = "linux")]
fn configure_parent_death(command: &mut Command) -> Result<(), ChildInvokeError> {
    let expected_parent = unsafe { libc::getpid() };
    // SAFETY: only async-signal-safe libc calls occur between fork and exec.
    unsafe {
        command.pre_exec(move || {
            if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            if libc::getppid() != expected_parent {
                libc::kill(libc::getpid(), libc::SIGKILL);
                libc::_exit(125);
            }
            Ok(())
        });
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn configure_parent_death(_command: &mut Command) -> Result<(), ChildInvokeError> {
    Err(ChildInvokeError::Failure(
        "target child parent-death enforcement is available only on Linux".into(),
    ))
}

struct ChildPipeReaders {
    stdout: JoinHandle<Result<Vec<u8>, String>>,
    stderr: JoinHandle<Result<Vec<u8>, String>>,
}

fn start_child_pipe_readers(child: &mut Child) -> Result<ChildPipeReaders, ChildInvokeError> {
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| ChildInvokeError::Failure("target child stdout was not piped".into()))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| ChildInvokeError::Failure("target child stderr was not piped".into()))?;
    Ok(ChildPipeReaders {
        stdout: thread::spawn(move || {
            read_limited(stdout, MAX_TARGET_RESPONSE_BYTES, "target stdout response")
        }),
        stderr: thread::spawn(move || {
            read_limited(stderr, MAX_TARGET_STDERR_BYTES, "target stderr")
        }),
    })
}

fn read_limited(reader: impl Read, limit: u64, label: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    reader
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("read {label}: {error}"))?;
    if bytes.len() as u64 > limit {
        return Err(format!("{label} exceeded {limit} byte bound"));
    }
    Ok(bytes)
}

fn finish_child_pipe_readers(
    readers: ChildPipeReaders,
) -> Result<(Vec<u8>, Vec<u8>), ChildInvokeError> {
    let stdout = readers
        .stdout
        .join()
        .map_err(|_| ChildInvokeError::Failure("target stdout reader panicked".into()))?
        .map_err(ChildInvokeError::Failure)?;
    let stderr = readers
        .stderr
        .join()
        .map_err(|_| ChildInvokeError::Failure("target stderr reader panicked".into()))?
        .map_err(ChildInvokeError::Failure)?;
    Ok((stdout, stderr))
}

fn parse_args() -> Result<Args, String> {
    let mut values = env::args().skip(1);
    let mode = match values.next().as_deref() {
        Some("synthetic") => Mode::Synthetic,
        Some("production") => Mode::Production,
        _ => return Err(usage()),
    };
    let mut config = None;
    let mut artifacts = None;
    let mut reviewed_commit = None;
    let mut force_synthetic_hit = false;
    let mut synthetic_hit_call = None;
    let mut synthetic_fail_call = None;
    let mut synthetic_child_delay_ms = 0;
    let mut synthetic_call_timeout_ms = None;
    let mut synthetic_response_padding_bytes = 0;
    while let Some(argument) = values.next() {
        match argument.as_str() {
            "--config" => config = Some(next_path(&mut values, "--config")?),
            "--artifacts" => artifacts = Some(next_path(&mut values, "--artifacts")?),
            "--reviewed-commit" => {
                reviewed_commit = Some(
                    values
                        .next()
                        .ok_or_else(|| "--reviewed-commit needs a revision".to_owned())?,
                )
            }
            "--force-synthetic-hit" => force_synthetic_hit = true,
            "--synthetic-hit-call" => {
                synthetic_hit_call = Some(next_usize(&mut values, &argument)?)
            }
            "--synthetic-fail-call" => {
                synthetic_fail_call = Some(next_usize(&mut values, &argument)?)
            }
            "--synthetic-child-delay-ms" => {
                synthetic_child_delay_ms = next_u64(&mut values, &argument)?
            }
            "--synthetic-call-timeout-ms" => {
                synthetic_call_timeout_ms = Some(next_u64(&mut values, &argument)?)
            }
            "--synthetic-response-padding-bytes" => {
                synthetic_response_padding_bytes = next_u64(&mut values, &argument)?
                    .try_into()
                    .map_err(|_| "synthetic response padding does not fit usize".to_owned())?
            }
            _ => return Err(format!("unknown argument {argument:?}\n{}", usage())),
        }
    }
    let args = Args {
        mode,
        config: config.ok_or_else(usage)?,
        artifacts: artifacts.ok_or_else(usage)?,
        reviewed_commit,
        force_synthetic_hit,
        synthetic_hit_call,
        synthetic_fail_call,
        synthetic_child_delay_ms,
        synthetic_call_timeout_ms,
        synthetic_response_padding_bytes,
    };
    if args.mode == Mode::Production && args.reviewed_commit.is_none() {
        return Err("production requires --reviewed-commit FULL_40_HEX_REVISION".into());
    }
    if args.force_synthetic_hit && args.synthetic_hit_call.is_some() {
        return Err("use either --force-synthetic-hit or --synthetic-hit-call, not both".into());
    }
    if let Some(revision) = args.reviewed_commit.as_deref() {
        if revision.len() != 40 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("--reviewed-commit must be a full 40-hex commit".into());
        }
    }
    Ok(args)
}

fn next_path(values: &mut impl Iterator<Item = String>, flag: &str) -> Result<PathBuf, String> {
    values
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| format!("{flag} needs a path"))
}

fn next_u64(values: &mut impl Iterator<Item = String>, flag: &str) -> Result<u64, String> {
    values
        .next()
        .ok_or_else(|| format!("{flag} needs an integer"))?
        .parse()
        .map_err(|e| format!("invalid {flag}: {e}"))
}

fn next_usize(values: &mut impl Iterator<Item = String>, flag: &str) -> Result<usize, String> {
    values
        .next()
        .ok_or_else(|| format!("{flag} needs an integer"))?
        .parse()
        .map_err(|e| format!("invalid {flag}: {e}"))
}

fn usage() -> String {
    "usage: adaptive-multilevel-splitting synthetic --config PATH --artifacts NEW_DIRECTORY [--force-synthetic-hit|--synthetic-hit-call N] [--synthetic-fail-call N] [--synthetic-child-delay-ms N --synthetic-call-timeout-ms N] [--synthetic-response-padding-bytes N]\n       adaptive-multilevel-splitting production --config PATH --artifacts NEW_DIRECTORY --reviewed-commit FULL_40_HEX_REVISION".into()
}

fn source_identity(
    mode: Mode,
    reviewed_revision: Option<&str>,
    executable: &Path,
) -> Result<SourceIdentity, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let revision = git_output(root, &["rev-parse", "HEAD"])?;
    let status = git_output(root, &["status", "--porcelain", "--untracked-files=normal"])?;
    Ok(SourceIdentity {
        git_revision: revision,
        reviewed_revision: reviewed_revision.map(str::to_owned),
        source_tree_clean: status.is_empty(),
        executable_sha256: file_sha256(executable)?,
        cargo_lock_sha256: file_sha256(&root.join("Cargo.lock"))?,
        production_target: mode == Mode::Production,
    })
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .map_err(|e| format!("run git {arguments:?}: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {arguments:?} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map(|value| value.trim().to_owned())
        .map_err(|e| format!("git output was not UTF-8: {e}"))
}

fn run_id(start_unix_ms: u128, revision: &str, artifacts: &Path) -> String {
    let material = format!(
        "ams-readiness-run-v1\n{start_unix_ms}\n{}\n{revision}\n{}\n",
        std::process::id(),
        artifacts.display()
    );
    format!(
        "amsrun-{}",
        &format!("{:x}", Sha256::digest(material.as_bytes()))[..24]
    )
}

fn unix_time_ms() -> Result<u128, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("system clock before Unix epoch: {e}"))
        .map(|duration| duration.as_millis())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(sys: f64) -> TargetOnceResponse {
        TargetOnceResponse::Success {
            observation: adaptive_multilevel_splitting::synthetic_observation(1.0, sys),
            synthetic_padding: None,
        }
    }

    #[test]
    fn late_completed_non_hit_is_timeout() {
        let result = classify_completed_response(
            response(0.9),
            Duration::from_millis(11),
            Duration::from_millis(10),
        );
        assert!(matches!(result, Err(ChildInvokeError::Timeout(_))));
    }

    #[test]
    fn late_completed_sys_hit_is_returned_for_flush_and_stop() {
        let result = classify_completed_response(
            response(1.01),
            Duration::from_millis(11),
            Duration::from_millis(10),
        );
        assert!(matches!(result, Ok(TargetOnceResponse::Success { .. })));
    }
}
