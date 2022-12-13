use crate::bench_run::BenchmarkProtocolAdapter;
use crate::metrics::{RequestStats, RequestStatsBuilder};
use crate::mini_client_wrapper::GCSClient;
use crate::mini_client_wrapper::GCSClientAPI;
use async_trait::async_trait;
use core::fmt;
use std::sync::Arc;
use std::time::Instant;
use rand::{thread_rng, Rng};


#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GcsBenchScenario {
    ReadObject,
    QueryWriteStatus,
    ResumableWriteObject,
    NonresumableWriteObject,
}

#[derive(Builder, Deserialize, Clone, Debug)]
pub struct GcsBenchAdapter {
    gcp_project: String,
    api: String,
    bucket: String,
    objects: Vec<String>,
    object_size: u64,
    scenario: GcsBenchScenario,
    universe: String,  // Usually "prod"

    #[builder(default)]
    upload_id: String,

    random_range_read_max_start:Option<u64>,
    random_range_read_max_len:Option<u64>,
}

fn map_client_api(name: &str) -> GCSClientAPI {
    if name == "json" {
        GCSClientAPI::Json
    } else if name == "grpc-no-directpath" {
        GCSClientAPI::GrpcNoDirectpath
    } else if name == "grpc-directpath" {
        GCSClientAPI::GrpcDirectpath
    } else {
        panic!("Oh no, don't recognize {}", name)
    }
}

impl GcsBenchAdapter {
    async fn read_object(
        &self,
        client: &Arc<crate::mini_client_wrapper::GCSClient>,
    ) -> RequestStats {
        let client = client.clone();
        let bucket = self.bucket.clone();
        let object = self.objects[thread_rng().gen_range(0..self.objects.len())].clone();

        let start_offset = self.random_range_read_max_start.map(|n| thread_rng().gen_range(1..n));
        let read_len = self.random_range_read_max_len.map(|n| thread_rng().gen_range(1..n));

        let (duration, call_result) = tokio::task::spawn_blocking(move || {
            let start_time = Instant::now();
            let call_result = match start_offset {
                Some(start) => client.range_read(bucket, object, start, read_len.unwrap()),
                None => client.read_object(bucket, object)
            };
            let end_time = Instant::now();
            (end_time.duration_since(start_time), call_result)
        })
        .await
        .unwrap();
        let byte_count = call_result.bytes_received as usize;

        if call_result.success {
            RequestStatsBuilder::default()
                .bytes_processed(byte_count)
                .status("OK".to_string())
                .is_success(true)
                .duration(duration)
                .fatal_error(false)
                .cell(call_result.bitpusher_cell())
                .build()
                .expect("RequestStatsBuilder failed")
        } else {
            RequestStatsBuilder::default()
                .bytes_processed(0)
                .status(call_result.error_code())
                .is_success(false)
                .fatal_error(false)
                .duration(duration)
                .cell(call_result.bitpusher_cell())
                .build()
                .expect("Error building RequestStats")
        }
    }

    async fn query_write_status(
        &self,
        client: &Arc<crate::mini_client_wrapper::GCSClient>,
    ) -> RequestStats {
        let client = client.clone();
        let upload_id = self.upload_id.clone();

        let (duration, call_result) = tokio::task::spawn_blocking(move || {
            let start_time = Instant::now();
            let call_result = client.query_write_status(upload_id);
            let end_time = Instant::now();
            (end_time.duration_since(start_time), call_result)
        })
        .await
        .unwrap();
        let byte_count = call_result.bytes_received as usize;

        if call_result.success {
            RequestStatsBuilder::default()
                .bytes_processed(byte_count)
                .status("OK".to_string())
                .is_success(true)
                .duration(duration)
                .fatal_error(false)
                .build()
                .expect("RequestStatsBuilder failed")
        } else {
            RequestStatsBuilder::default()
                .bytes_processed(0)
                .status(call_result.error_code())
                .is_success(false)
                .fatal_error(false)
                .duration(duration)
                .build()
                .expect("Error building RequestStats")
        } // gcshp-central1-storage. gcshp-east1-storage....
    }

    async fn write_object(
        &self,
        client: &Arc<crate::mini_client_wrapper::GCSClient>,
        resumable: bool
    ) -> RequestStats {
        let client = client.clone();
        let bucket = self.bucket.clone();
        let object = format!("{}_{}",
            self.objects[thread_rng().gen_range(0..self.objects.len())],
            thread_rng().gen_range(0..1000000));
        let num_bytes = self.object_size;

        let (duration, call_result) = tokio::task::spawn_blocking(move || {
            let start_time = Instant::now();
            let call_result = if resumable {
                client.write_object_resumable(bucket, object, num_bytes)
            } else {
                client.write_object_nonresumable(bucket, object, num_bytes)
            };
            let end_time = Instant::now();
            (end_time.duration_since(start_time), call_result)
        })
        .await
        .unwrap();
        let byte_count =num_bytes as usize;

        if call_result.success {
            RequestStatsBuilder::default()
                .bytes_processed(byte_count)
                .status("OK".to_string())
                .is_success(true)
                .duration(duration)
                .fatal_error(false)
                .build()
                .expect("RequestStatsBuilder failed")
        } else {
            RequestStatsBuilder::default()
                .bytes_processed(0)
                .status(call_result.error_code())
                .is_success(false)
                .fatal_error(false)
                .duration(duration)
                .build()
                .expect("Error building RequestStats")
        } // gcshp-central1-storage. gcshp-east1-storage....
    }
}

#[async_trait]
impl BenchmarkProtocolAdapter for GcsBenchAdapter {
    type Client = Arc<crate::mini_client_wrapper::GCSClient>;

    async fn build_client(&self) -> Result<Self::Client, String> {
        let api = map_client_api(&self.api);
        match GCSClient::new(api, self.gcp_project.clone(), self.universe.clone()) {
            Ok(c) => Ok(Arc::new(c)),
            Err(e) => panic!("Error creating client {:?}", e),
        }
    }

    async fn initialize_workload(&mut self, client: &Self::Client) -> Result<(), String> {
        if self.scenario == GcsBenchScenario::QueryWriteStatus {
            println!("Initializing workload");
            let client = client.clone();
            let bucket = self.bucket.clone();
            let object = self.objects[thread_rng().gen_range(0..self.objects.len())].clone();
            let call_result =
                tokio::task::spawn_blocking(move || client.start_resumable_write(bucket, object))
                    .await
                    .unwrap();
            if !call_result.success {
                Result::Err(String::from("Failed to initialize an upload, aborting"))
            } else {
                self.upload_id = call_result.upload_id();
                println!("Upload ID is {}", self.upload_id);
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    async fn send_request(&self, client: &Self::Client) -> RequestStats {
        match self.scenario {
            GcsBenchScenario::ReadObject => self.read_object(&client).await,
            GcsBenchScenario::QueryWriteStatus => self.query_write_status(&client).await,
            GcsBenchScenario::ResumableWriteObject => self.write_object(&client, true).await,
            GcsBenchScenario::NonresumableWriteObject => self.write_object(&client, false).await,
        }
    }
}

impl fmt::Display for GcsBenchAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Some Gcs bench adapter")
    }
}
