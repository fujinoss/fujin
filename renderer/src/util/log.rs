use std::collections::HashMap;
use std::fmt;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const MAX_MESSAGE_LEN: usize = 4096;
pub const MAX_RING_ENTRIES: usize = 2048;
pub const MAX_SINKS: usize = 16;
pub const DEFAULT_RATE_WINDOW_MS: u64 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

impl Level {
    pub const ALL: [Level; 6] = [
        Level::Trace,
        Level::Debug,
        Level::Info,
        Level::Warn,
        Level::Error,
        Level::Fatal,
    ];

    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL",
        }
    }

    #[inline]
    pub const fn short(self) -> &'static str {
        match self {
            Level::Trace => "T",
            Level::Debug => "D",
            Level::Info => "I",
            Level::Warn => "W",
            Level::Error => "E",
            Level::Fatal => "F",
        }
    }

    #[inline]
    pub const fn ansi(self) -> &'static str {
        match self {
            Level::Trace => "\x1b[90m",
            Level::Debug => "\x1b[36m",
            Level::Info => "\x1b[32m",
            Level::Warn => "\x1b[33m",
            Level::Error => "\x1b[31m",
            Level::Fatal => "\x1b[35;1m",
        }
    }

    #[inline]
    pub const fn ansi_reset(self) -> &'static str {
        "\x1b[0m"
    }

    #[inline]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Level::Trace),
            1 => Some(Level::Debug),
            2 => Some(Level::Info),
            3 => Some(Level::Warn),
            4 => Some(Level::Error),
            5 => Some(Level::Fatal),
            _ => None,
        }
    }

    #[inline]
    pub const fn to_log(self) -> log::Level {
        match self {
            Level::Trace => log::Level::Trace,
            Level::Debug => log::Level::Debug,
            Level::Info => log::Level::Info,
            Level::Warn => log::Level::Warn,
            Level::Error | Level::Fatal => log::Level::Error,
        }
    }

    #[inline]
    pub const fn to_filter(self) -> log::LevelFilter {
        match self {
            Level::Trace => log::LevelFilter::Trace,
            Level::Debug => log::LevelFilter::Debug,
            Level::Info => log::LevelFilter::Info,
            Level::Warn => log::LevelFilter::Warn,
            Level::Error | Level::Fatal => log::LevelFilter::Error,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Default for Level {
    fn default() -> Self {
        Level::Info
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Category {
    General = 0,
    Vulkan = 1,
    Gles = 2,
    Swapchain = 3,
    Pipeline = 4,
    Buffers = 5,
    Memory = 6,
    Atlas = 7,
    Font = 8,
    Shader = 9,
    Damage = 10,
    Cursor = 11,
    Selection = 12,
    Viewport = 13,
    Backend = 14,
    Jni = 15,
}

pub const ALL_CATEGORIES: [Category; 16] = [
    Category::General,
    Category::Vulkan,
    Category::Gles,
    Category::Swapchain,
    Category::Pipeline,
    Category::Buffers,
    Category::Memory,
    Category::Atlas,
    Category::Font,
    Category::Shader,
    Category::Damage,
    Category::Cursor,
    Category::Selection,
    Category::Viewport,
    Category::Backend,
    Category::Jni,
];

impl Category {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Category::General => "general",
            Category::Vulkan => "vulkan",
            Category::Gles => "gles",
            Category::Swapchain => "swapchain",
            Category::Pipeline => "pipeline",
            Category::Buffers => "buffers",
            Category::Memory => "memory",
            Category::Atlas => "atlas",
            Category::Font => "font",
            Category::Shader => "shader",
            Category::Damage => "damage",
            Category::Cursor => "cursor",
            Category::Selection => "selection",
            Category::Viewport => "viewport",
            Category::Backend => "backend",
            Category::Jni => "jni",
        }
    }

    #[inline]
    pub const fn bit(self) -> u32 {
        1u32 << (self as u32)
    }

    #[inline]
    pub const fn from_bit(b: u32) -> Option<Self> {
        match b {
            0 => Some(Category::General),
            1 => Some(Category::Vulkan),
            2 => Some(Category::Gles),
            3 => Some(Category::Swapchain),
            4 => Some(Category::Pipeline),
            5 => Some(Category::Buffers),
            6 => Some(Category::Memory),
            7 => Some(Category::Atlas),
            8 => Some(Category::Font),
            9 => Some(Category::Shader),
            10 => Some(Category::Damage),
            11 => Some(Category::Cursor),
            12 => Some(Category::Selection),
            13 => Some(Category::Viewport),
            14 => Some(Category::Backend),
            15 => Some(Category::Jni),
            _ => None,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CategoryMask(pub u32);

impl CategoryMask {
    pub const NONE: CategoryMask = CategoryMask(0);
    pub const ALL: CategoryMask = CategoryMask((1u32 << 16) - 1);

    #[inline]
    pub const fn contains(self, cat: Category) -> bool {
        (self.0 & cat.bit()) != 0
    }

    #[inline]
    pub const fn with(self, cat: Category) -> Self {
        Self(self.0 | cat.bit())
    }

    #[inline]
    pub const fn without(self, cat: Category) -> Self {
        Self(self.0 & !cat.bit())
    }

    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    pub const fn intersect(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn is_all(self) -> bool {
        self.0 == Self::ALL.0
    }

    #[inline]
    pub fn iter(self) -> impl Iterator<Item = Category> {
        (0..16u32).filter_map(move |i| {
            let bit = 1u32 << i;
            if (self.0 & bit) != 0 {
                Category::from_bit(i)
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub level: Level,
    pub category: Category,
    pub target: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub message: String,
    pub frame: Option<u64>,
    pub timestamp: SystemTime,
    pub thread_id: u64,
}

impl Record {
    #[inline]
    pub fn new(
        level: Level,
        category: Category,
        target: &'static str,
        file: &'static str,
        line: u32,
        message: String,
    ) -> Self {
        Self {
            level,
            category,
            target,
            file,
            line,
            message,
            frame: None,
            timestamp: SystemTime::now(),
            thread_id: current_thread_id(),
        }
    }

    #[inline]
    pub fn with_frame(mut self, frame: u64) -> Self {
        self.frame = Some(frame);
        self
    }

    #[inline]
    pub fn millis_since_epoch(&self) -> u128 {
        self.timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis()
    }

    #[inline]
    pub fn micros_since_epoch(&self) -> u128 {
        self.timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_micros()
    }

    pub fn format_hms_millis(&self) -> String {
        let d = self
            .timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);
        let total_secs = d.as_secs();
        let millis = d.subsec_millis();
        let h = (total_secs / 3600) % 24;
        let m = (total_secs / 60) % 60;
        let s = total_secs % 60;
        format!("{:02}:{:02}:{:02}.{:03}", h, m, s, millis)
    }

    pub fn short_file(&self) -> &'static str {
        match self.file.rfind('/') {
            Some(pos) => &self.file[pos + 1..],
            None => self.file,
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} [{}] {} ({}:{})",
            self.format_hms_millis(),
            self.level.short(),
            self.category,
            self.message,
            self.short_file(),
            self.line
        )
    }
}

#[cfg(target_os = "linux")]
fn current_thread_id() -> u64 {
    extern "C" {
        fn syscall(num: i64, ...) -> i64;
    }
    const SYS_GETTID: i64 = 178;
    unsafe { syscall(SYS_GETTID) as u64 }
}

#[cfg(not(target_os = "linux"))]
fn current_thread_id() -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    hasher.finish()
}

pub trait Sink: Send + Sync {
    fn write(&self, record: &Record);
    fn open(&self) {}
    fn close(&self) {}
    fn flush_frame(&self, _frame: u64) {}
    fn on_filter_changed(&self, _filter: &Filter) {}
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub trait Formatter: Send + Sync {
    fn format(&self, record: &Record) -> String;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlainFormatter;

impl PlainFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for PlainFormatter {
    fn format(&self, record: &Record) -> String {
        let frame = record.frame.map(|n| format!(" f{n}")).unwrap_or_default();
        format!(
            "{} {}{} [{}] {} ({}:{})\n",
            record.format_hms_millis(),
            record.level.as_str(),
            frame,
            record.category,
            record.message,
            record.short_file(),
            record.line
        )
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AnsiFormatter;

impl AnsiFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for AnsiFormatter {
    fn format(&self, record: &Record) -> String {
        let color = record.level.ansi();
        let reset = record.level.ansi_reset();
        let frame = record.frame.map(|n| format!(" f{n}")).unwrap_or_default();
        format!(
            "{color}{} {}{} {reset}[{}] {} ({}:{})\n",
            record.format_hms_millis(),
            record.level.as_str(),
            frame,
            record.category,
            record.message,
            record.short_file(),
            record.line
        )
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CompactFormatter;

impl CompactFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for CompactFormatter {
    fn format(&self, record: &Record) -> String {
        format!(
            "{} {} [{}] {}\n",
            record.level.short(),
            record.format_hms_millis(),
            record.category,
            record.message
        )
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct JsonFormatter;

impl JsonFormatter {
    pub fn new() -> Self {
        Self
    }
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

impl Formatter for JsonFormatter {
    fn format(&self, record: &Record) -> String {
        let frame = match record.frame {
            Some(n) => format!("{n}"),
            None => "null".to_string(),
        };
        format!(
            "{{\"t\":{},\"lvl\":\"{}\",\"cat\":\"{}\",\"target\":\"{}\",\"file\":\"{}\",\"line\":{},\"frame\":{},\"tid\":{},\"msg\":\"{}\"}}\n",
            record.millis_since_epoch(),
            record.level.as_str(),
            record.category,
            json_escape(record.target),
            json_escape(record.short_file()),
            record.line,
            frame,
            record.thread_id,
            json_escape(&record.message)
        )
    }
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub min_level: Level,
    pub enabled_categories: CategoryMask,
    pub module_levels: Vec<(String, Level)>,
    pub exclude_modules: Vec<String>,
}

impl Filter {
    pub fn new(min_level: Level) -> Self {
        Self {
            min_level,
            enabled_categories: CategoryMask::ALL,
            module_levels: Vec::new(),
            exclude_modules: Vec::new(),
        }
    }

    pub fn permissive() -> Self {
        Self::new(Level::Trace)
    }

    pub fn quiet() -> Self {
        Self::new(Level::Warn)
    }

    #[inline]
    pub fn allows(&self, level: Level, category: Category, module: &str) -> bool {
        if !self.enabled_categories.contains(category) {
            return false;
        }
        for excluded in &self.exclude_modules {
            if module.starts_with(excluded.as_str()) {
                return false;
            }
        }
        let threshold = self.module_level(module).unwrap_or(self.min_level);
        level >= threshold
    }

    fn module_level(&self, module: &str) -> Option<Level> {
        let mut best: Option<(usize, Level)> = None;
        for (prefix, lvl) in &self.module_levels {
            if module.starts_with(prefix.as_str()) {
                let len = prefix.len();
                if best.map_or(true, |(best_len, _)| len > best_len) {
                    best = Some((len, *lvl));
                }
            }
        }
        best.map(|(_, l)| l)
    }

    pub fn set_module_level(&mut self, prefix: impl Into<String>, level: Level) {
        let prefix = prefix.into();
        for entry in self.module_levels.iter_mut() {
            if entry.0 == prefix {
                entry.1 = level;
                return;
            }
        }
        self.module_levels.push((prefix, level));
    }

    pub fn exclude_module(&mut self, prefix: impl Into<String>) {
        let prefix = prefix.into();
        if !self.exclude_modules.contains(&prefix) {
            self.exclude_modules.push(prefix);
        }
    }

    pub fn clear_module_levels(&mut self) {
        self.module_levels.clear();
    }

    pub fn clear_excludes(&mut self) {
        self.exclude_modules.clear();
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new(Level::Info)
    }
}

pub struct StderrSink {
    formatter: Box<dyn Formatter>,
    counter: AtomicU64,
    lock: Mutex<()>,
}

impl StderrSink {
    pub fn plain() -> Self {
        Self {
            formatter: Box::new(PlainFormatter::new()),
            counter: AtomicU64::new(0),
            lock: Mutex::new(()),
        }
    }

    pub fn ansi() -> Self {
        Self {
            formatter: Box::new(AnsiFormatter::new()),
            counter: AtomicU64::new(0),
            lock: Mutex::new(()),
        }
    }

    pub fn compact() -> Self {
        Self {
            formatter: Box::new(CompactFormatter::new()),
            counter: AtomicU64::new(0),
            lock: Mutex::new(()),
        }
    }

    pub fn auto() -> Self {
        if is_tty_stderr() {
            Self::ansi()
        } else {
            Self::plain()
        }
    }

    pub fn count(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }
}

impl Sink for StderrSink {
    fn write(&self, record: &Record) {
        if let Ok(_guard) = self.lock.try_lock() {
            let s = self.formatter.format(record);
            let stderr = std::io::stderr();
            let mut handle = stderr.lock();
            let _ = handle.write_all(s.as_bytes());
            let _ = handle.flush();
            self.counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn name(&self) -> &'static str {
        "StderrSink"
    }
}

#[cfg(unix)]
fn is_tty_stderr() -> bool {
    extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    unsafe { isatty(2) == 1 }
}

#[cfg(not(unix))]
fn is_tty_stderr() -> bool {
    false
}

#[derive(Debug, Clone)]
pub struct RingEntry {
    pub frame: Option<u64>,
    pub timestamp_ms: u128,
    pub level: Level,
    pub category: Category,
    pub message: String,
    pub target: &'static str,
    pub file: &'static str,
    pub line: u32,
}

impl RingEntry {
    pub fn from_record(r: &Record) -> Self {
        Self {
            frame: r.frame,
            timestamp_ms: r.millis_since_epoch(),
            level: r.level,
            category: r.category,
            message: r.message.clone(),
            target: r.target,
            file: r.file,
            line: r.line,
        }
    }

    pub fn compact_line(&self) -> String {
        let frame = self.frame.map(|n| format!("f{n} ")).unwrap_or_default();
        format!(
            "{}{} [{}] {}",
            frame,
            self.level.short(),
            self.category,
            self.message
        )
    }
}

pub struct RingBufferSink {
    capacity: usize,
    entries: Mutex<Vec<RingEntry>>,
    dropped: AtomicU64,
    written: AtomicU64,
}

impl RingBufferSink {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "ring buffer capacity must be > 0");
        Self {
            capacity,
            entries: Mutex::new(Vec::with_capacity(capacity)),
            dropped: AtomicU64::new(0),
            written: AtomicU64::new(0),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.entries.lock().map(|v| v.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn dropped(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }

    pub fn written(&self) -> u64 {
        self.written.load(Ordering::Relaxed)
    }

    pub fn clear(&self) {
        if let Ok(mut guard) = self.entries.lock() {
            guard.clear();
        }
    }

    pub fn snapshot(&self) -> Vec<RingEntry> {
        self.entries
            .lock()
            .map(|v| v.clone())
            .unwrap_or_default()
    }

    pub fn recent(&self, n: usize) -> Vec<RingEntry> {
        self.entries
            .lock()
            .map(|v| {
                let start = v.len().saturating_sub(n);
                v[start..].to_vec()
            })
            .unwrap_or_default()
    }

    pub fn lines(&self, n: usize) -> Vec<String> {
        self.recent(n)
            .into_iter()
            .map(|e| e.compact_line())
            .collect()
    }
}

impl Sink for RingBufferSink {
    fn write(&self, record: &Record) {
        let entry = RingEntry::from_record(record);
        match self.entries.try_lock() {
            Ok(mut guard) => {
                if guard.len() >= self.capacity {
                    guard.remove(0);
                    self.dropped.fetch_add(1, Ordering::Relaxed);
                }
                guard.push(entry);
                self.written.fetch_add(1, Ordering::Relaxed);
            }
            Err(TryLockError::WouldBlock) => {
                self.dropped.fetch_add(1, Ordering::Relaxed);
            }
            Err(TryLockError::Poisoned(_)) => {
                self.dropped.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    fn name(&self) -> &'static str {
        "RingBufferSink"
    }
}

#[derive(Default)]
pub struct NullSink {
    count: AtomicU64,
}

impl NullSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }
}

impl Sink for NullSink {
    fn write(&self, _record: &Record) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "NullSink"
    }
}

pub struct RateLimiter {
    window_ms: u64,
    max_per_window: u32,
    state: Mutex<RateState>,
}

struct RateState {
    window_start: Instant,
    count: u32,
    total_suppressed: u64,
}

impl RateLimiter {
    pub fn new(window_ms: u64, max_per_window: u32) -> Self {
        Self {
            window_ms,
            max_per_window,
            state: Mutex::new(RateState {
                window_start: Instant::now(),
                count: 0,
                total_suppressed: 0,
            }),
        }
    }

    pub fn allow(&self) -> bool {
        let mut state = match self.state.try_lock() {
            Ok(g) => g,
            Err(_) => return false,
        };
        let now = Instant::now();
        if now.duration_since(state.window_start).as_millis() as u64 > self.window_ms {
            state.window_start = now;
            state.count = 0;
        }
        if state.count >= self.max_per_window {
            state.total_suppressed += 1;
            return false;
        }
        state.count += 1;
        true
    }

    pub fn suppressed(&self) -> u64 {
        self.state
            .lock()
            .map(|s| s.total_suppressed)
            .unwrap_or(0)
    }
}

#[derive(Debug)]
pub struct FrameStats {
    pub frame_number: AtomicU64,
    pub cpu_us: AtomicU64,
    pub gpu_us: AtomicU64,
    pub cells_pushed: AtomicU64,
    pub instances_drawn: AtomicU64,
    pub bytes_uploaded: AtomicU64,
    pub draw_calls: AtomicU64,
    pub pipeline_switches: AtomicU64,
    pub descriptor_binds: AtomicU64,
    pub dropped_frames: AtomicU64,
    pub acquired_images: AtomicU64,
    pub presented_images: AtomicU64,
}

impl FrameStats {
    pub const fn new() -> Self {
        Self {
            frame_number: AtomicU64::new(0),
            cpu_us: AtomicU64::new(0),
            gpu_us: AtomicU64::new(0),
            cells_pushed: AtomicU64::new(0),
            instances_drawn: AtomicU64::new(0),
            bytes_uploaded: AtomicU64::new(0),
            draw_calls: AtomicU64::new(0),
            pipeline_switches: AtomicU64::new(0),
            descriptor_binds: AtomicU64::new(0),
            dropped_frames: AtomicU64::new(0),
            acquired_images: AtomicU64::new(0),
            presented_images: AtomicU64::new(0),
        }
    }

    #[inline]
    pub fn set_frame(&self, n: u64) {
        self.frame_number.store(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_cpu_us(&self, us: u64) {
        self.cpu_us.store(us, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_gpu_us(&self, us: u64) {
        self.gpu_us.store(us, Ordering::Relaxed);
    }

    #[inline]
    pub fn add_cells_pushed(&self, n: u64) {
        self.cells_pushed.fetch_add(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn add_instances_drawn(&self, n: u64) {
        self.instances_drawn.fetch_add(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn add_bytes_uploaded(&self, n: u64) {
        self.bytes_uploaded.fetch_add(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn add_draw_calls(&self, n: u64) {
        self.draw_calls.fetch_add(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn add_pipeline_switches(&self, n: u64) {
        self.pipeline_switches.fetch_add(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn add_descriptor_binds(&self, n: u64) {
        self.descriptor_binds.fetch_add(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn add_dropped_frames(&self, n: u64) {
        self.dropped_frames.fetch_add(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn add_acquired_images(&self, n: u64) {
        self.acquired_images.fetch_add(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn add_presented_images(&self, n: u64) {
        self.presented_images.fetch_add(n, Ordering::Relaxed);
    }
}

impl Default for FrameStats {
    fn default() -> Self {
        Self::new()
    }
}

struct SinkEntry {
    name: &'static str,
    sink: Arc<dyn Sink>,
}

pub struct LoggerState {
    initialized: AtomicBool,
    filter: Mutex<Filter>,
    sinks: Mutex<Vec<SinkEntry>>,
    frame: AtomicU64,
    min_level_cached: AtomicU8,
    total_written: AtomicU64,
    total_filtered: AtomicU64,
    rate_limiter: Option<RateLimiter>,
    stats: Arc<FrameStats>,
    ring: Option<Arc<RingBufferSink>>,
}

impl LoggerState {
    fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            filter: Mutex::new(Filter::default()),
            sinks: Mutex::new(Vec::with_capacity(MAX_SINKS)),
            frame: AtomicU64::new(0),
            min_level_cached: AtomicU8::new(Level::Info as u8),
            total_written: AtomicU64::new(0),
            total_filtered: AtomicU64::new(0),
            rate_limiter: None,
            stats: Arc::new(FrameStats::new()),
            ring: None,
        }
    }

    #[inline]
    pub fn allows(&self, level: Level, category: Category) -> bool {
        if (level as u8) < self.min_level_cached.load(Ordering::Relaxed) {
            self.total_filtered.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        let filter = match self.filter.try_lock() {
            Ok(f) => f,
            Err(_) => return false,
        };
        if !filter.allows(level, category, "") {
            self.total_filtered.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        true
    }

    #[inline]
    pub fn allows_module(&self, level: Level, category: Category, module: &str) -> bool {
        if (level as u8) < self.min_level_cached.load(Ordering::Relaxed) {
            self.total_filtered.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        let filter = match self.filter.try_lock() {
            Ok(f) => f,
            Err(_) => return false,
        };
        if !filter.allows(level, category, module) {
            self.total_filtered.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        true
    }

    pub fn emit(&self, record: Record) {
        let sinks = match self.sinks.try_lock() {
            Ok(s) => s,
            Err(_) => return,
        };
        for entry in sinks.iter() {
            entry.sink.write(&record);
        }
        self.total_written.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_min_level(&self, level: Level) {
        let mut filter = self.filter.lock().unwrap();
        filter.min_level = level;
        self.min_level_cached.store(level as u8, Ordering::Relaxed);
        self.notify_filter_changed(&filter);
    }

    pub fn set_categories(&self, mask: CategoryMask) {
        let mut filter = self.filter.lock().unwrap();
        filter.enabled_categories = mask;
        self.notify_filter_changed(&filter);
    }

    pub fn set_module_level(&self, prefix: impl Into<String>, level: Level) {
        let mut filter = self.filter.lock().unwrap();
        filter.set_module_level(prefix, level);
        self.notify_filter_changed(&filter);
    }

    pub fn exclude_module(&self, prefix: impl Into<String>) {
        let mut filter = self.filter.lock().unwrap();
        filter.exclude_module(prefix);
        self.notify_filter_changed(&filter);
    }

    fn notify_filter_changed(&self, filter: &Filter) {
        if let Ok(sinks) = self.sinks.try_lock() {
            for entry in sinks.iter() {
                entry.sink.on_filter_changed(filter);
            }
        }
    }

    pub fn add_sink(&self, sink: Arc<dyn Sink>) {
        if let Ok(mut sinks) = self.sinks.lock() {
            if sinks.len() < MAX_SINKS {
                sinks.push(SinkEntry {
                    name: sink.name(),
                    sink,
                });
            }
        }
    }

    pub fn set_ring(&mut self, ring: Arc<RingBufferSink>) {
        self.ring = Some(ring.clone());
        self.add_sink(ring);
    }

    pub fn set_rate_limit(&mut self, window_ms: u64, max_per_window: u32) {
        self.rate_limiter = Some(RateLimiter::new(window_ms, max_per_window));
    }

    pub fn rate_allow(&self) -> bool {
        match &self.rate_limiter {
            Some(r) => r.allow(),
            None => true,
        }
    }

    pub fn begin_frame(&self, n: u64) {
        self.frame.store(n, Ordering::Relaxed);
        self.stats.set_frame(n);
    }

    pub fn flush_frame(&self) {
        let frame = self.frame.load(Ordering::Relaxed);
        if let Ok(sinks) = self.sinks.try_lock() {
            for entry in sinks.iter() {
                entry.sink.flush_frame(frame);
            }
        }
    }

    pub fn stats(&self) -> Arc<FrameStats> {
        self.stats.clone()
    }

    pub fn ring(&self) -> Option<&Arc<RingBufferSink>> {
        self.ring.as_ref()
    }

    pub fn total_written(&self) -> u64 {
        self.total_written.load(Ordering::Relaxed)
    }

    pub fn total_filtered(&self) -> u64 {
        self.total_filtered.load(Ordering::Relaxed)
    }

    pub fn filter_snapshot(&self) -> Filter {
        self.filter
            .lock()
            .map(|f| f.clone())
            .unwrap_or_else(|_| Filter::default())
    }

    pub fn sink_count(&self) -> usize {
        self.sinks.lock().map(|s| s.len()).unwrap_or(0)
    }

    pub fn sink_names(&self) -> Vec<&'static str> {
        self.sinks
            .lock()
            .map(|s| s.iter().map(|e| e.name).collect())
            .unwrap_or_default()
    }

    pub fn shutdown(&self) {
        if let Ok(sinks) = self.sinks.try_lock() {
            for entry in sinks.iter() {
                entry.sink.close();
            }
        }
    }
}

static LOGGER: once_cell::sync::Lazy<LoggerState> =
    once_cell::sync::Lazy::new(LoggerState::new);

pub fn global() -> &'static LoggerState {
    &LOGGER
}

pub fn is_initialized() -> bool {
    LOGGER.initialized.load(Ordering::Relaxed)
}

pub fn init() {
    if LOGGER.initialized.swap(true, Ordering::SeqCst) {
        return;
    }
    LOGGER.add_sink(Arc::new(StderrSink::auto()));
    let ring = Arc::new(RingBufferSink::new(MAX_RING_ENTRIES));
    unsafe {
        let logger_ptr = &LOGGER as *const LoggerState as *mut LoggerState;
        (*logger_ptr).ring = Some(ring.clone());
        (*logger_ptr).add_sink(ring);
        (*logger_ptr).set_rate_limit(DEFAULT_RATE_WINDOW_MS, 1000);
    }
}

pub fn init_with(filter: Filter) {
    if LOGGER.initialized.swap(true, Ordering::SeqCst) {
        return;
    }
    {
        let mut f = LOGGER.filter.lock().unwrap();
        *f = filter;
    }
    LOGGER.add_sink(Arc::new(StderrSink::auto()));
    let ring = Arc::new(RingBufferSink::new(MAX_RING_ENTRIES));
    unsafe {
        let logger_ptr = &LOGGER as *const LoggerState as *mut LoggerState;
        (*logger_ptr).ring = Some(ring.clone());
        (*logger_ptr).add_sink(ring);
        (*logger_ptr).set_rate_limit(DEFAULT_RATE_WINDOW_MS, 1000);
    }
}

pub fn shutdown() {
    LOGGER.shutdown();
}

pub fn set_level(level: Level) {
    LOGGER.set_min_level(level);
}

pub fn set_categories(mask: CategoryMask) {
    LOGGER.set_categories(mask);
}

pub fn begin_frame(n: u64) {
    LOGGER.begin_frame(n);
}

pub fn flush_frame() {
    LOGGER.flush_frame();
}

pub fn stats() -> Arc<FrameStats> {
    LOGGER.stats()
}

pub fn ring_lines(n: usize) -> Vec<String> {
    LOGGER.ring().map(|r| r.lines(n)).unwrap_or_default()
}

pub fn ring_clear() {
    if let Some(r) = LOGGER.ring() {
        r.clear();
    }
}

pub fn total_written() -> u64 {
    LOGGER.total_written()
}

pub fn total_filtered() -> u64 {
    LOGGER.total_filtered()
}

#[macro_export]
macro_rules! log_emit {
    ($level:expr, $cat:expr, $($arg:tt)*) => {{
        let __msg = format!($($arg)*);
        if $crate::util::log::global().allows_module($level, $cat, module_path!()) {
            let __rec = $crate::util::log::Record::new(
                $level,
                $cat,
                module_path!(),
                file!(),
                line!(),
                __msg,
            );
            $crate::util::log::global().emit(__rec);
        }
    }};
}

#[macro_export]
macro_rules! log_trace {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log_emit!($crate::util::log::Level::Trace, $cat, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log_emit!($crate::util::log::Level::Debug, $cat, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_info {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log_emit!($crate::util::log::Level::Info, $cat, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log_emit!($crate::util::log::Level::Warn, $cat, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_error {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log_emit!($crate::util::log::Level::Error, $cat, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log_emit!($crate::util::log::Level::Fatal, $cat, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_vk {
    ($lvl:expr, $($arg:tt)*) => {
        $crate::log_emit!($lvl, $crate::util::log::Category::Vulkan, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_gles {
    ($lvl:expr, $($arg:tt)*) => {
        $crate::log_emit!($lvl, $crate::util::log::Category::Gles, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_atlas {
    ($lvl:expr, $($arg:tt)*) => {
        $crate::log_emit!($lvl, $crate::util::log::Category::Atlas, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_font {
    ($lvl:expr, $($arg:tt)*) => {
        $crate::log_emit!($lvl, $crate::util::log::Category::Font, $($arg)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_ordering() {
        assert!(Level::Trace < Level::Debug);
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info < Level::Warn);
        assert!(Level::Warn < Level::Error);
        assert!(Level::Error < Level::Fatal);
    }

    #[test]
    fn level_from_u8() {
        assert_eq!(Level::from_u8(0), Some(Level::Trace));
        assert_eq!(Level::from_u8(5), Some(Level::Fatal));
        assert_eq!(Level::from_u8(6), None);
        assert_eq!(Level::from_u8(255), None);
    }

    #[test]
    fn level_as_str() {
        assert_eq!(Level::Info.as_str(), "INFO");
        assert_eq!(Level::Fatal.as_str(), "FATAL");
    }

    #[test]
    fn category_bits_distinct() {
        let mut seen: u32 = 0;
        for cat in ALL_CATEGORIES {
            let bit = cat.bit();
            assert_eq!(seen & bit, 0, "duplicate bit for {cat}");
            seen |= bit;
        }
        assert_eq!(seen, CategoryMask::ALL.0);
    }

    #[test]
    fn category_mask_ops() {
        let a = CategoryMask::NONE.with(Category::Vulkan).with(Category::Gles);
        assert!(a.contains(Category::Vulkan));
        assert!(a.contains(Category::Gles));
        assert!(!a.contains(Category::Atlas));

        let b = a.without(Category::Gles);
        assert!(!b.contains(Category::Gles));

        let c = a.intersect(b);
        assert!(c.contains(Category::Vulkan));
        assert!(!c.contains(Category::Gles));
    }

    #[test]
    fn category_mask_iter() {
        let m = CategoryMask::NONE.with(Category::Vulkan).with(Category::Atlas);
        let cats: Vec<Category> = m.iter().collect();
        assert_eq!(cats.len(), 2);
        assert!(cats.contains(&Category::Vulkan));
        assert!(cats.contains(&Category::Atlas));
    }

    #[test]
    fn record_construction() {
        let r = Record::new(
            Level::Info,
            Category::Vulkan,
            "test",
            "test.rs",
            42,
            "hello".to_string(),
        );
        assert_eq!(r.level, Level::Info);
        assert_eq!(r.category, Category::Vulkan);
        assert_eq!(r.line, 42);
        assert_eq!(r.message, "hello");
    }

    #[test]
    fn record_with_frame() {
        let r = Record::new(
            Level::Debug,
            Category::General,
            "t",
            "t.rs",
            1,
            "x".into(),
        )
        .with_frame(99);
        assert_eq!(r.frame, Some(99));
    }

    #[test]
    fn record_short_file() {
        let r = Record::new(
            Level::Debug,
            Category::General,
            "t",
            "src/vulkan/instance/instance.rs",
            1,
            "x".into(),
        );
        assert_eq!(r.short_file(), "instance.rs");
    }

    #[test]
    fn record_short_file_no_path() {
        let r = Record::new(Level::Debug, Category::General, "t", "main.rs", 1, "x".into());
        assert_eq!(r.short_file(), "main.rs");
    }

    #[test]
    fn filter_default_allows_info() {
        let f = Filter::default();
        assert!(f.allows(Level::Info, Category::Vulkan, ""));
        assert!(f.allows(Level::Error, Category::Vulkan, ""));
        assert!(!f.allows(Level::Debug, Category::Vulkan, ""));
    }

    #[test]
    fn filter_module_specific() {
        let mut f = Filter::default();
        f.set_module_level("arashi::vulkan", Level::Trace);
        assert!(f.allows(Level::Trace, Category::Vulkan, "arashi::vulkan::instance"));
        assert!(!f.allows(Level::Trace, Category::General, "arashi::color"));
    }

    #[test]
    fn filter_longest_prefix_wins() {
        let mut f = Filter::default();
        f.set_module_level("arashi", Level::Warn);
        f.set_module_level("arashi::vulkan", Level::Trace);
        assert!(f.allows(Level::Trace, Category::Vulkan, "arashi::vulkan::instance"));
        assert!(!f.allows(Level::Debug, Category::General, "arashi::color"));
    }

    #[test]
    fn filter_exclude_module() {
        let mut f = Filter::permissive();
        f.exclude_module("arashi::noisy");
        assert!(!f.allows(Level::Error, Category::General, "arashi::noisy::thing"));
        assert!(f.allows(Level::Error, Category::General, "arashi::quiet"));
    }

    #[test]
    fn filter_category_restriction() {
        let mut f = Filter::permissive();
        f.enabled_categories = CategoryMask::NONE.with(Category::Vulkan);
        assert!(f.allows(Level::Error, Category::Vulkan, ""));
        assert!(!f.allows(Level::Error, Category::Gles, ""));
    }

    #[test]
    fn plain_formatter_output() {
        let r = Record::new(Level::Info, Category::General, "t", "test.rs", 5, "msg".into());
        let f = PlainFormatter::new();
        let s = f.format(&r);
        assert!(s.contains("INFO"));
        assert!(s.contains("msg"));
        assert!(s.contains("test.rs:5"));
    }

    #[test]
    fn json_formatter_escaping() {
        let r = Record::new(
            Level::Error,
            Category::Vulkan,
            "t",
            "x.rs",
            1,
            "has \"quotes\" and\nnewline".into(),
        );
        let f = JsonFormatter::new();
        let s = f.format(&r);
        assert!(s.contains("\\\"quotes\\\""));
        assert!(s.contains("\\n"));
    }

    #[test]
    fn null_sink_counts() {
        let sink = NullSink::new();
        let r = Record::new(Level::Info, Category::General, "t", "t.rs", 1, "x".into());
        sink.write(&r);
        sink.write(&r);
        assert_eq!(sink.count(), 2);
    }

    #[test]capacity() {
        let ring = RingBufferSink::new(3);
        for i in 0..5 {
            let r = Record::new(
                Level::Info,
                Category::General,
                "t",
                "t.rs",
                i,
                format!("msg{i}"),
            );
            ring.write(&r);
        }
        assert_eq!(ring.len(), 3);
        assert_eq!(ring.written(), 5);
        assert_eq!(ring.dropped(), 2);
    }

    #[test]
    fn ring_buffer_recent() {
        let ring = RingBufferSink::new(10);
        for i in 0..5 {
            let r = Record::new(
                Level::Info,
                Category::General,
                "t",
                "t.rs",
                i,
                format!("m{i}"),
            );
            ring.write(&r);
        }
        let recent = ring.recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].message, "m3");
        assert_eq!(recent[1].message, "m4");
    }

    #[test]
    fn ring_buffer_clear() {
        let ring = RingBufferSink::new(4);
        let r = Record::new(Level::Info, Category::General, "t", "t.rs", 1, "x".into());
        ring.write(&r);
        assert_eq!(ring.len(), 1);
        ring.clear();
        assert_eq!(ring.len(), 0);
    }

    #[test]
    fn rate_limiter_within_window() {
        let rl = RateLimiter::new(1000, 3);
        assert!(rl.allow());
        assert!(rl.allow());
        assert!(rl.allow());
        assert!(!rl.allow());
        assert!(!rl.allow());
        assert_eq!(rl.suppressed(), 2);
    }

    #[test]
    fn rate_limiter_new_window() {
        let rl = RateLimiter::new(1, 1);
        assert!(rl.allow());
        assert!(!rl.allow());
        std::thread::sleep(std::time::Duration::from_millis(5));
        assert!(rl.allow());
    }

    #[test]
    fn frame_stats_defaults() {
        let s = FrameStats::new();
        assert_eq!(s.frame_number.load(Ordering::Relaxed), 0);
        assert_eq!(s.draw_calls.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn frame_stats_add() {
        let s = FrameStats::new();
        s.add_draw_calls(3);
        s.add_draw_calls(4);
        assert_eq!(s.draw_calls.load(Ordering::Relaxed), 7);
    }

    #[test]
    fn frame_stats_set() {
        let s = FrameStats::new();
        s.set_frame(42);
        assert_eq!(s.frame_number.load(Ordering::Relaxed), 42);
        s.set_cpu_us(1234);
        assert_eq!(s.cpu_us.load(Ordering::Relaxed), 1234);
    }

    #[test]
    fn logger_state_allows_info() {
        let state = LoggerState::new();
        assert!(state.allows(Level::Info, Category::General));
    }

    #[test]
    fn logger_state_rejects_trace() {
        let state = LoggerState::new();
        assert!(!state.allows(Level::Trace, Category::General));
    }

    #[test]
    fn logger_state_set_level() {
        let state = LoggerState::new();
        state.set_min_level(Level::Trace);
        assert!(state.allows(Level::Trace, Category::General));
    }

    #[test]
    fn logger_add_sink() {
        let state = LoggerState::new();
        let n = NullSink::new();
        state.add_sink(Arc::new(n));
        assert_eq!(state.sink_count(), 1);
    }

    #[test]
    fn logger_emit_to_null_sink() {
        let state = LoggerState::new();
        let n = Arc::new(NullSink::new());
        state.add_sink(n.clone());
        let r = Record::new(Level::Info, Category::General, "t", "t.rs", 1, "x".into());
        state.emit(r);
        assert_eq!(n.count(), 1);
    }

    #[test]
    fn logger_set_categories() {
        let state = LoggerState::new();
        state.set_categories(CategoryMask::NONE);
        assert!(!state.allows(Level::Error, Category::General));
    }

    #[test]
    fn logger_begin_frame() {
        let state = LoggerState::new();
        state.begin_frame(10);
        assert_eq!(state.stats().frame_number.load(Ordering::Relaxed), 10);
    }

    #[test]
    fn logger_shutdown_idempotent() {
        let state = LoggerState::new();
        state.shutdown();
        state.shutdown();
    }
}
    fn ring_buffer_
