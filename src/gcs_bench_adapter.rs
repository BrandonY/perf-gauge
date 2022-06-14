use crate::bench_run::BenchmarkProtocolAdapter;
use crate::metrics::{RequestStats, RequestStatsBuilder};
use async_trait::async_trait;
use rand::{thread_rng, Rng};
use std::time::Instant;
use crate::mini_client_wrapper::GCSClient;
use crate::mini_client_wrapper::GCSClientAPI;

#[derive(Builder, Deserialize, Clone, Debug)]
pub struct GcsBenchAdapter {
    gcp_project: String,
    api: String,
    bucket: String,
    objects: Vec<String>,
}

fn map_client_api(name: &str) -> GCSClientAPI {
    if name == "json" { GCSClientAPI::Json }
    else if name == "grpc_no_directpath" { GCSClientAPI::GrpcNoDirectpath }
    else if name == "grpc_directpath" { GCSClientAPI::GrpcDirectpath }
    else { panic!("Oh no, don't recognize {}", name) }
}

#[async_trait]
impl BenchmarkProtocolAdapter for GcsBenchAdapter {
    type Client = crate::mini_client_wrapper::GCSClient;

    async fn build_client(&self) -> Result<Self::Client, String> {
        let api = map_client_api(&self.api);
        GCSClient::new(api, self.gcp_project.clone())
    }

    async fn send_request(&self, client: &Self::Client) -> RequestStats {
        let start = Instant::now();
        let client = client.clone();
        let bucket = self.bucket.clone();
        
        let call_result = client.read_object(bucket, String::from("test.txt"));
        let byte_count = call_result.bytes_received as usize;


        if call_result.success {
                // let data = r.get().await.unwrap_or(vec![]);
                // bytes_processed += 100; // data.len();

                RequestStatsBuilder::default()
                    .bytes_processed(byte_count)
                    .status("OK".to_string())
                    .is_success(true)
                    .duration(Instant::now().duration_since(start))
                    .build()
                    .expect("RequestStatsBuilder failed")
        } else {
            RequestStatsBuilder::default()
                .bytes_processed(0)
                .status(format!("Unexpected error reading object")) //format!(
                    //"Unexpected error getting bucket {}. Error: {:?}",
                    // self.bucket, e
                //))
                .is_success(false)
                .duration(Instant::now().duration_since(start))
                .build()
                .expect("Error building RequestStats")
        }
    }
}
