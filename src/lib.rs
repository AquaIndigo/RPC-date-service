use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use date_rpc::{DateResponse, DateRequest};
use tonic::{Request, Response, Status, Code};
use chrono::Local;
use date_rpc::date_service_server::DateService;

pub mod date_rpc {
    tonic::include_proto!("date_rpc");
}

#[derive(Debug, Default)]
pub struct MyDateService {
    auth: Authorization,
}

#[tonic::async_trait]
impl DateService for MyDateService {
    async fn get_date(
        &self,
        request: Request<DateRequest>,
    ) -> Result<Response<DateResponse>, Status> {
        // println!("Got a token: {:?}", request.get_ref().token);
        let token: &str = &request.get_ref().token;

        if !self.auth.contains(token) {
            return Err(Status::new(Code::from_i32(-1), "Invalid token!"));
        }

        let now = Local::now().naive_local();
        let reply = date_rpc::DateResponse {
            analogue_time: now.format("%a %b %e %T %H:%M:%S %Y").to_string().into(),
            digital_time: (now.timestamp() as u64).into(),
        };

        println!("The date is {}", reply.analogue_time);
        Ok(Response::new(reply))
    }
}

impl MyDateService {
    pub fn new(token_file: String) -> Self {
        Self {
            auth: Authorization::new(token_file)
        }
    }
}

#[derive(Debug, Default)]
struct Authorization {
    auth: HashSet<String>,
}

impl Authorization {
    fn new(token_file: String) -> Self {
        let mut auth_set = HashSet::new();
        let file = std::fs::File::open(token_file).unwrap();
        let lines = BufReader::new(file).lines();
        for line in lines {
            auth_set.insert(line.unwrap());
        }
        Self {
            auth: auth_set,
        }
    }

    fn contains(&self, token: &str) -> bool {
        self.auth.contains(token)
    }
}
