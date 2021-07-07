mod cli;
mod services;
mod types;

use cli::{Action, Cli, CliError, Entity};
use httpclient::ApiError;
use services::{instance_service::InstanceService, workload_service::WorkloadService};

#[macro_use]
extern crate prettytable;

fn main() -> Result<(), ApiError> {
    let app = match Cli::new() {
        Ok(cli) => cli,
        Err(CliError::MissingArg(e)) => {
            println!("Error\nMissing argument : {}", e);
            std::process::exit(1);
        }
    };
    if app.entity == Entity::WORKLOAD {
        if app.action == Action::CREATE {
            WorkloadService::create(&app.file)?;
            println!("Workload created");
        } else if app.action == Action::DELETE {
            WorkloadService::delete(app.workload_id.clone())?;
            println!("Workload {} deleted.", &app.workload_id);
        } else if app.action == Action::GET {
            WorkloadService::list()?;
        }
    } else if app.entity == Entity::INSTANCE {
        if app.action == Action::CREATE {
            InstanceService::create(app.workload_id.clone(), Some("2".to_string()))?;
            println!("Instance created");
        } else if app.action == Action::DELETE {
            InstanceService::delete(app.instance_id.clone())?;
            println!("Instance {} deleted.", &app.instance_id);
        } else if app.action == Action::GET {
            InstanceService::list()?;
        }
    }
    Ok(())
}
