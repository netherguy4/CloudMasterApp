mod instances;
mod utils;
use crate::instances::{instance_get, instance_start, instance_stop, instances_list};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage: {} <project_id> [command] [instance_name] [zone]",
            args[0]
        );
        return Ok(());
    }

    let project_id = &args[1];
    let command = args.get(2).map(|s| s.as_str());

    match command {
        Some("list") => {
            let output = instances_list(project_id).await?;
            println!("{}", output);
        }

        Some("get") | Some("start") | Some("stop") => {
            // Check that instance name and zone are provided
            let instance = match args.get(3) {
                Some(i) => i,
                None => {
                    println!("Error: instance name is missing.");
                    println!(
                        "Usage: {} {} <instance_name> <zone>",
                        args[0],
                        command.unwrap()
                    );
                    return Ok(());
                }
            };

            let zone = match args.get(4) {
                Some(z) => z,
                None => {
                    println!("Error: zone is missing.");
                    println!(
                        "Usage: {} {} <instance_name> <zone>",
                        args[0],
                        command.unwrap()
                    );
                    return Ok(());
                }
            };

            let output = match command.unwrap() {
                "get" => instance_get(project_id, instance, zone).await?,
                "start" => instance_start(project_id, instance, zone).await?,
                "stop" => instance_stop(project_id, instance, zone).await?,
                _ => unreachable!(),
            };

            println!("{}", output);
        }

        Some(other) => {
            println!("No such command: {other}");
            println!("Available commands: list, get, start, stop");
        }

        None => {
            println!(
                "No command provided. Usage: {} <project_id> <command> [instance_name] [zone]",
                args[0]
            );
        }
    }

    Ok(())
}
