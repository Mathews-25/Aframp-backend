//! Prometheus metrics registry and all metric definitions for Aframp backend.
//!
//! All metrics are registered in a single global registry exposed at GET /metrics.
//! Metric names follow Prometheus naming conventions: snake_case, unit suffix where
//! applicable, and the `aframp_` namespace prefix.

pub mod handler;
pub mod tests;

use prometheus::{
    register_counter_vec_with_registry, register_gauge_vec_with_registry,
    register_histogram_vec_with_registry, CounterVec, GaugeVec, HistogramVec, Registry,
};
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Global registry
// ---------------------------------------------------------------------------

static REGISTRY: OnceLock<Registry> = OnceLock::new();

/// Returns the global Prometheus registry, initialising it on first call.
pub fn registry() -> &'static Registry {
    REGISTRY.get_or_init(|| {
        let r = Registry::new();
        register_all(&r);
        r
    })
}

/// Render all metrics in Prometheus text exposition format.
pub fn render() -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let mut buf = Vec::new();
    encoder
        .encode(&registry().gather(), &mut buf)
        .expect("encoding metrics failed");
    String::from_utf8(buf).expect("metrics output is not valid UTF-8")
}

// ---------------------------------------------------------------------------
// HTTP request metrics
// ---------------------------------------------------------------------------

pub mod http {
    use super::*;

    static HTTP_REQUESTS_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static HTTP_REQUEST_DURATION_SECONDS: OnceLock<HistogramVec> = OnceLock::new();
    static HTTP_REQUESTS_IN_FLIGHT: OnceLock<GaugeVec> = OnceLock::new();

    pub fn requests_total() -> &'static CounterVec {
        HTTP_REQUESTS_TOTAL.get().expect("metrics not initialised")
    }

    pub fn request_duration_seconds() -> &'static HistogramVec {
        HTTP_REQUEST_DURATION_SECONDS
            .get()
            .expect("metrics not initialised")
    }

    pub fn requests_in_flight() -> &'static GaugeVec {
        HTTP_REQUESTS_IN_FLIGHT
            .get()
            .expect("metrics not initialised")
    }

    pub(super) fn register(r: &Registry) {
        HTTP_REQUESTS_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_http_requests_total",
                    "Total number of HTTP requests",
                    &["method", "route", "status_code"],
                    r
                )
                .unwrap(),
            )
            .ok();

        HTTP_REQUEST_DURATION_SECONDS
            .set(
                register_histogram_vec_with_registry!(
                    "aframp_http_request_duration_seconds",
                    "HTTP request duration in seconds",
                    &["method", "route"],
                    vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0],
                    r
                )
                .unwrap(),
            )
            .ok();

        HTTP_REQUESTS_IN_FLIGHT
            .set(
                register_gauge_vec_with_registry!(
                    "aframp_http_requests_in_flight",
                    "Number of HTTP requests currently being processed",
                    &["route"],
                    r
                )
                .unwrap(),
            )
            .ok();
    }
}

// ---------------------------------------------------------------------------
// cNGN transaction metrics
// ---------------------------------------------------------------------------

pub mod cngn {
    use super::*;

    static CNGN_TRANSACTIONS_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static CNGN_TRANSACTION_VOLUME: OnceLock<HistogramVec> = OnceLock::new();
    static CNGN_TRANSACTION_DURATION_SECONDS: OnceLock<HistogramVec> = OnceLock::new();

    pub fn transactions_total() -> &'static CounterVec {
        CNGN_TRANSACTIONS_TOTAL
            .get()
            .expect("metrics not initialised")
    }

    pub fn transaction_volume() -> &'static HistogramVec {
        CNGN_TRANSACTION_VOLUME
            .get()
            .expect("metrics not initialised")
    }

    pub fn transaction_duration_seconds() -> &'static HistogramVec {
        CNGN_TRANSACTION_DURATION_SECONDS
            .get()
            .expect("metrics not initialised")
    }

    pub(super) fn register(r: &Registry) {
        CNGN_TRANSACTIONS_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_cngn_transactions_total",
                    "Total cNGN transactions by type and status",
                    &["tx_type", "status"],
                    r
                )
                .unwrap(),
            )
            .ok();

        CNGN_TRANSACTION_VOLUME
            .set(
                register_histogram_vec_with_registry!(
                    "aframp_cngn_transaction_volume_ngn",
                    "cNGN transaction amounts in NGN",
                    &["tx_type"],
                    vec![
                        100.0, 500.0, 1_000.0, 5_000.0, 10_000.0, 50_000.0, 100_000.0,
                        500_000.0, 1_000_000.0,
                    ],
                    r
                )
                .unwrap(),
            )
            .ok();

        CNGN_TRANSACTION_DURATION_SECONDS
            .set(
                register_histogram_vec_with_registry!(
                    "aframp_cngn_transaction_duration_seconds",
                    "cNGN transaction processing duration from initiation to completion",
                    &["tx_type"],
                    vec![1.0, 5.0, 15.0, 30.0, 60.0, 120.0, 300.0, 600.0, 1800.0],
                    r
                )
                .unwrap(),
            )
            .ok();
    }
}

// ---------------------------------------------------------------------------
// Payment provider metrics
// ---------------------------------------------------------------------------

pub mod payment {
    use super::*;

    static PAYMENT_PROVIDER_REQUESTS_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static PAYMENT_PROVIDER_REQUEST_DURATION_SECONDS: OnceLock<HistogramVec> = OnceLock::new();
    static PAYMENT_PROVIDER_FAILURES_TOTAL: OnceLock<CounterVec> = OnceLock::new();

    pub fn provider_requests_total() -> &'static CounterVec {
        PAYMENT_PROVIDER_REQUESTS_TOTAL
            .get()
            .expect("metrics not initialised")
    }

    pub fn provider_request_duration_seconds() -> &'static HistogramVec {
        PAYMENT_PROVIDER_REQUEST_DURATION_SECONDS
            .get()
            .expect("metrics not initialised")
    }

    pub fn provider_failures_total() -> &'static CounterVec {
        PAYMENT_PROVIDER_FAILURES_TOTAL
            .get()
            .expect("metrics not initialised")
    }

    pub(super) fn register(r: &Registry) {
        PAYMENT_PROVIDER_REQUESTS_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_payment_provider_requests_total",
                    "Total payment provider requests by provider and operation",
                    &["provider", "operation"],
                    r
                )
                .unwrap(),
            )
            .ok();

        PAYMENT_PROVIDER_REQUEST_DURATION_SECONDS
            .set(
                register_histogram_vec_with_registry!(
                    "aframp_payment_provider_request_duration_seconds",
                    "Payment provider request duration in seconds",
                    &["provider", "operation"],
                    vec![0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0],
                    r
                )
                .unwrap(),
            )
            .ok();

        PAYMENT_PROVIDER_FAILURES_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_payment_provider_failures_total",
                    "Total payment provider failures by provider and failure reason",
                    &["provider", "failure_reason"],
                    r
                )
                .unwrap(),
            )
            .ok();
    }
}

// ---------------------------------------------------------------------------
// Stellar service metrics
// ---------------------------------------------------------------------------

pub mod stellar {
    use super::*;

    static STELLAR_TX_SUBMISSIONS_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static STELLAR_TX_SUBMISSION_DURATION_SECONDS: OnceLock<HistogramVec> = OnceLock::new();
    static STELLAR_TRUSTLINE_ATTEMPTS_TOTAL: OnceLock<CounterVec> = OnceLock::new();

    pub fn tx_submissions_total() -> &'static CounterVec {
        STELLAR_TX_SUBMISSIONS_TOTAL
            .get()
            .expect("metrics not initialised")
    }

    pub fn tx_submission_duration_seconds() -> &'static HistogramVec {
        STELLAR_TX_SUBMISSION_DURATION_SECONDS
            .get()
            .expect("metrics not initialised")
    }

    pub fn trustline_attempts_total() -> &'static CounterVec {
        STELLAR_TRUSTLINE_ATTEMPTS_TOTAL
            .get()
            .expect("metrics not initialised")
    }

    pub(super) fn register(r: &Registry) {
        STELLAR_TX_SUBMISSIONS_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_stellar_tx_submissions_total",
                    "Total Stellar transaction submissions by status",
                    &["status"],
                    r
                )
                .unwrap(),
            )
            .ok();

        STELLAR_TX_SUBMISSION_DURATION_SECONDS
            .set(
                register_histogram_vec_with_registry!(
                    "aframp_stellar_tx_submission_duration_seconds",
                    "Stellar transaction submission duration in seconds",
                    &[],
                    vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 30.0],
                    r
                )
                .unwrap(),
            )
            .ok();

        STELLAR_TRUSTLINE_ATTEMPTS_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_stellar_trustline_attempts_total",
                    "Total Stellar trustline creation attempts by status",
                    &["status"],
                    r
                )
                .unwrap(),
            )
            .ok();
    }
}

// ---------------------------------------------------------------------------
// Background worker metrics
// ---------------------------------------------------------------------------

pub mod worker {
    use super::*;

    static WORKER_CYCLES_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static WORKER_CYCLE_DURATION_SECONDS: OnceLock<HistogramVec> = OnceLock::new();
    static WORKER_RECORDS_PROCESSED: OnceLock<GaugeVec> = OnceLock::new();
    static WORKER_ERRORS_TOTAL: OnceLock<CounterVec> = OnceLock::new();

    pub fn cycles_total() -> &'static CounterVec {
        WORKER_CYCLES_TOTAL
            .get()
            .expect("metrics not initialised")
    }

    pub fn cycle_duration_seconds() -> &'static HistogramVec {
        WORKER_CYCLE_DURATION_SECONDS
            .get()
            .expect("metrics not initialised")
    }

    pub fn records_processed() -> &'static GaugeVec {
        WORKER_RECORDS_PROCESSED
            .get()
            .expect("metrics not initialised")
    }

    pub fn errors_total() -> &'static CounterVec {
        WORKER_ERRORS_TOTAL
            .get()
            .expect("metrics not initialised")
    }

    pub(super) fn register(r: &Registry) {
        WORKER_CYCLES_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_worker_cycles_total",
                    "Total background worker processing cycles",
                    &["worker"],
                    r
                )
                .unwrap(),
            )
            .ok();

        WORKER_CYCLE_DURATION_SECONDS
            .set(
                register_histogram_vec_with_registry!(
                    "aframp_worker_cycle_duration_seconds",
                    "Background worker cycle duration in seconds",
                    &["worker"],
                    vec![0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0, 30.0, 60.0],
                    r
                )
                .unwrap(),
            )
            .ok();

        WORKER_RECORDS_PROCESSED
            .set(
                register_gauge_vec_with_registry!(
                    "aframp_worker_records_processed",
                    "Number of records processed in the last worker cycle",
                    &["worker"],
                    r
                )
                .unwrap(),
            )
            .ok();

        WORKER_ERRORS_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_worker_errors_total",
                    "Total background worker errors by worker and error type",
                    &["worker", "error_type"],
                    r
                )
                .unwrap(),
            )
            .ok();
    }
}

// ---------------------------------------------------------------------------
// Redis cache metrics
// ---------------------------------------------------------------------------

pub mod cache {
    use super::*;

    static CACHE_HITS_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static CACHE_MISSES_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static CACHE_OPERATION_DURATION_SECONDS: OnceLock<HistogramVec> = OnceLock::new();

    pub fn hits_total() -> &'static CounterVec {
        CACHE_HITS_TOTAL.get().expect("metrics not initialised")
    }

    pub fn misses_total() -> &'static CounterVec {
        CACHE_MISSES_TOTAL.get().expect("metrics not initialised")
    }

    pub fn operation_duration_seconds() -> &'static HistogramVec {
        CACHE_OPERATION_DURATION_SECONDS
            .get()
            .expect("metrics not initialised")
    }

    pub(super) fn register(r: &Registry) {
        CACHE_HITS_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_cache_hits_total",
                    "Total Redis cache hits by key prefix",
                    &["key_prefix"],
                    r
                )
                .unwrap(),
            )
            .ok();

        CACHE_MISSES_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_cache_misses_total",
                    "Total Redis cache misses by key prefix",
                    &["key_prefix"],
                    r
                )
                .unwrap(),
            )
            .ok();

        CACHE_OPERATION_DURATION_SECONDS
            .set(
                register_histogram_vec_with_registry!(
                    "aframp_cache_operation_duration_seconds",
                    "Redis cache operation duration in seconds",
                    &["operation"],
                    vec![0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1, 0.5],
                    r
                )
                .unwrap(),
            )
            .ok();
    }
}

// ---------------------------------------------------------------------------
// Database metrics
// ---------------------------------------------------------------------------

pub mod database {
    use super::*;

    static DB_QUERY_DURATION_SECONDS: OnceLock<HistogramVec> = OnceLock::new();
    static DB_CONNECTIONS_ACTIVE: OnceLock<GaugeVec> = OnceLock::new();
    static DB_ERRORS_TOTAL: OnceLock<CounterVec> = OnceLock::new();

    pub fn query_duration_seconds() -> &'static HistogramVec {
        DB_QUERY_DURATION_SECONDS
            .get()
            .expect("metrics not initialised")
    }

    pub fn connections_active() -> &'static GaugeVec {
        DB_CONNECTIONS_ACTIVE
            .get()
            .expect("metrics not initialised")
    }

    pub fn errors_total() -> &'static CounterVec {
        DB_ERRORS_TOTAL.get().expect("metrics not initialised")
    }

    pub(super) fn register(r: &Registry) {
        DB_QUERY_DURATION_SECONDS
            .set(
                register_histogram_vec_with_registry!(
                    "aframp_db_query_duration_seconds",
                    "Database query duration in seconds",
                    &["query_type", "table"],
                    vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 5.0],
                    r
                )
                .unwrap(),
            )
            .ok();

        DB_CONNECTIONS_ACTIVE
            .set(
                register_gauge_vec_with_registry!(
                    "aframp_db_connections_active",
                    "Active database connections in the pool",
                    &["pool"],
                    r
                )
                .unwrap(),
            )
            .ok();

        DB_ERRORS_TOTAL
            .set(
                register_counter_vec_with_registry!(
                    "aframp_db_errors_total",
                    "Total database errors by error type",
                    &["error_type"],
                    r
                )
                .unwrap(),
            )
            .ok();
    }
}

// ---------------------------------------------------------------------------
// Register all metrics
// ---------------------------------------------------------------------------

fn register_all(r: &Registry) {
    http::register(r);
    cngn::register(r);
    payment::register(r);
    stellar::register(r);
    worker::register(r);
    cache::register(r);
    database::register(r);
    analytics::register(r);
}

// ---------------------------------------------------------------------------
// Helper: extract key prefix from a Redis key (first colon-delimited segment)
// ---------------------------------------------------------------------------

pub fn key_prefix(key: &str) -> &str {
    key.find(':').map(|i| &key[..i]).unwrap_or(key)
}

// ---------------------------------------------------------------------------
// Analytics metrics (Issue #369)
// ---------------------------------------------------------------------------

pub mod analytics {
    use super::*;

    static SNAPSHOTS_GENERATED_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static ANOMALY_DETECTIONS_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static INSIGHT_DELIVERIES_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static ANALYTICS_CACHE_HITS_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static ANALYTICS_CACHE_MISSES_TOTAL: OnceLock<CounterVec> = OnceLock::new();
    static ACTIVE_WALLET_COUNT: OnceLock<GaugeVec> = OnceLock::new();
    static AVG_WALLET_RISK_SCORE: OnceLock<GaugeVec> = OnceLock::new();
    static ANOMALY_FLAGGED_WALLETS: OnceLock<GaugeVec> = OnceLock::new();

    pub fn snapshots_generated_total() -> &'static CounterVec {
        SNAPSHOTS_GENERATED_TOTAL.get().expect("metrics not initialised")
    }
    pub fn anomaly_detections_total() -> &'static CounterVec {
        ANOMALY_DETECTIONS_TOTAL.get().expect("metrics not initialised")
    }
    pub fn insight_deliveries_total() -> &'static CounterVec {
        INSIGHT_DELIVERIES_TOTAL.get().expect("metrics not initialised")
    }
    pub fn analytics_cache_hits_total() -> &'static CounterVec {
        ANALYTICS_CACHE_HITS_TOTAL.get().expect("metrics not initialised")
    }
    pub fn analytics_cache_misses_total() -> &'static CounterVec {
        ANALYTICS_CACHE_MISSES_TOTAL.get().expect("metrics not initialised")
    }
    pub fn active_wallet_count_gauge() -> &'static GaugeVec {
        ACTIVE_WALLET_COUNT.get().expect("metrics not initialised")
    }
    pub fn avg_wallet_risk_score_gauge() -> &'static GaugeVec {
        AVG_WALLET_RISK_SCORE.get().expect("metrics not initialised")
    }
    pub fn anomaly_flagged_wallets_gauge() -> &'static GaugeVec {
        ANOMALY_FLAGGED_WALLETS.get().expect("metrics not initialised")
    }

    // Convenience helpers called from service/worker code
    pub fn snapshot_generated(wallet: &str, period: &str) {
        if let Some(c) = SNAPSHOTS_GENERATED_TOTAL.get() {
            c.with_label_values(&[period]).inc();
        }
    }
    pub fn anomaly_detected(anomaly_type: &str) {
        if let Some(c) = ANOMALY_DETECTIONS_TOTAL.get() {
            c.with_label_values(&[anomaly_type]).inc();
        }
    }
    pub fn insight_delivered(period: &str) {
        if let Some(c) = INSIGHT_DELIVERIES_TOTAL.get() {
            c.with_label_values(&[period]).inc();
        }
    }
    pub fn cache_hit(endpoint: &str) {
        if let Some(c) = ANALYTICS_CACHE_HITS_TOTAL.get() {
            c.with_label_values(&[endpoint]).inc();
        }
    }
    pub fn cache_miss(endpoint: &str) {
        if let Some(c) = ANALYTICS_CACHE_MISSES_TOTAL.get() {
            c.with_label_values(&[endpoint]).inc();
        }
    }
    pub fn active_wallet_count(count: f64) {
        if let Some(g) = ACTIVE_WALLET_COUNT.get() {
            g.with_label_values(&["all"]).set(count);
        }
    }
    pub fn avg_risk_score(score: f64) {
        if let Some(g) = AVG_WALLET_RISK_SCORE.get() {
            g.with_label_values(&["all"]).set(score);
        }
    }
    pub fn anomaly_flagged_wallets(count: f64) {
        if let Some(g) = ANOMALY_FLAGGED_WALLETS.get() {
            g.with_label_values(&["open"]).set(count);
        }
    }

    pub(super) fn register(r: &Registry) {
        SNAPSHOTS_GENERATED_TOTAL
            .set(register_counter_vec_with_registry!(
                "aframp_analytics_snapshots_generated_total",
                "Total analytics snapshots generated by period",
                &["period"],
                r
            ).unwrap()).ok();

        ANOMALY_DETECTIONS_TOTAL
            .set(register_counter_vec_with_registry!(
                "aframp_analytics_anomaly_detections_total",
                "Total anomaly detections by type",
                &["anomaly_type"],
                r
            ).unwrap()).ok();

        INSIGHT_DELIVERIES_TOTAL
            .set(register_counter_vec_with_registry!(
                "aframp_analytics_insight_deliveries_total",
                "Total insight deliveries by period",
                &["period"],
                r
            ).unwrap()).ok();

        ANALYTICS_CACHE_HITS_TOTAL
            .set(register_counter_vec_with_registry!(
                "aframp_analytics_cache_hits_total",
                "Analytics endpoint cache hits",
                &["endpoint"],
                r
            ).unwrap()).ok();

        ANALYTICS_CACHE_MISSES_TOTAL
            .set(register_counter_vec_with_registry!(
                "aframp_analytics_cache_misses_total",
                "Analytics endpoint cache misses",
                &["endpoint"],
                r
            ).unwrap()).ok();

        ACTIVE_WALLET_COUNT
            .set(register_gauge_vec_with_registry!(
                "aframp_analytics_active_wallet_count",
                "Number of active wallets per activity tier",
                &["tier"],
                r
            ).unwrap()).ok();

        AVG_WALLET_RISK_SCORE
            .set(register_gauge_vec_with_registry!(
                "aframp_analytics_avg_wallet_risk_score",
                "Average wallet risk score",
                &["scope"],
                r
            ).unwrap()).ok();

        ANOMALY_FLAGGED_WALLETS
            .set(register_gauge_vec_with_registry!(
                "aframp_analytics_anomaly_flagged_wallets",
                "Number of wallets currently flagged for anomalies",
                &["status"],
                r
            ).unwrap()).ok();
    }
}
