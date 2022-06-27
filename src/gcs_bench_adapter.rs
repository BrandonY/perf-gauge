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
}

#[derive(Builder, Deserialize, Clone, Debug)]
pub struct GcsBenchAdapter {
    gcp_project: String,
    api: String,
    bucket: String,
    objects: Vec<String>,
    scenario: GcsBenchScenario,

    #[builder(default)]
    upload_id: String,
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

        let (duration, call_result) = tokio::task::spawn_blocking(move || {
            let start_time = Instant::now();
            let call_result = client.read_object(bucket, object);
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
        }
    }
}

#[async_trait]
impl BenchmarkProtocolAdapter for GcsBenchAdapter {
    type Client = Arc<crate::mini_client_wrapper::GCSClient>;

    async fn build_client(&self) -> Result<Self::Client, String> {
        let api = map_client_api(&self.api);
        match GCSClient::new(api, self.gcp_project.clone()) {
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
        }
    }
}

impl fmt::Display for GcsBenchAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Some Gcs bench adapter")
    }
}
