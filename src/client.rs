use std::env;
use std::thread::sleep;
use std::time::Duration;
use date_rpc::date_service_client::DateServiceClient;
use date_rpc::{DateRequest, DateResponse};

pub mod date_rpc {
    tonic::include_proto!("date_rpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("Usage: ./client -t {{token}}".into());
    }
    let mut client = DateServiceClient::connect("http://[::1]:19813").await?;
    let mut last_sec_per_min = 0;
    loop {
        let request = tonic::Request::new(DateRequest { token: args[2].clone().into() });

        let response: tonic::Response<DateResponse> = client.get_date(request).await?;
        let date_resp = response.get_ref();
        let digit_time: u64 = date_resp.digital_time;

        println!("Time on localhost is {}", date_resp.digital_time);
        if last_sec_per_min <= digit_time - 60 {
            last_sec_per_min = digit_time;
            println!("The date is {}", date_resp.analogue_time);
        }
        sleep(Duration::new(1, 0));
    }
}