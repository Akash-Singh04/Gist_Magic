use clap::{Parser, Subcommand};
use crate::requests::{fetch_gists , view_gist};
use textwrap::fill;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

pub struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "List all gists")]
    List,
    #[command(about = "Create a new gist")]
    Create,
    #[command(about = "Edit a gist")]
    Edit{
    #[arg(long, short = 'g' , help = "ID of the gist to list stargazers for")]
    gistid: String,
},
    #[command(about = "Delete a gist")]
    Delete,
    #[command(about = "View a gist")]
    View{
    #[arg(long, short = 'g' , help = "ID of the gist to list stargazers for")]
    gistid: String,
},
    #[command(about = "Star a gist")]
    Star,
    #[command(about = "Unstar a gist")]
    Unstar,
}

pub async fn  parse_cmd(args: Args , github_token: &str) {
    let mut request_url: &str = "https://api.github.com/gists";

    match args.cmd{
        Commands::List => {
            match fetch_gists(&request_url  , &github_token).await {
                Ok(gists) => {
                    for gist in gists {
                      println!("ID: {}", gist.id);
                      println!("Description: {:?}", gist.description);
                      println!("Owner: {}", gist.owner.login);
                      println!("------------------------");
                    }
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
          }
        },
        Commands::Create => {
            println!("Create a new gist");
        },
        Commands::Edit{gistid} => {
            println!("Edit a gist {}" , gistid);
        },
        Commands::Delete => {
            println!("Delete a gist");
        },
        Commands::View{gistid}  => {
            let formatted_url = format!("https://api.github.com/gists/{}", gistid);
            request_url = &formatted_url;
            match view_gist(&request_url , &github_token).await {
                Ok(gist) => {
                    println!("ID: {}", gist.id);
                    println!("Description: {:?}", gist.description);
                    println!("Owner: {}", gist.owner.login);
                    println!("Gist Files: ");
                    // Iterate over each file in the gist
                    if let Some(files) = gist.files.as_object() {
                        for (filename, file) in files {
                            println!("Filename: {}", filename);
                            println!("Language: {}", file["language"]);
                            println!("Size: {} bytes", file["size"]);
                            println!("Raw URL: {}", file["raw_url"]);
                            println!("-------------------------");
                            println!("Content:");
                            // let content = file["content"].as_str().unwrap_or("");
                            // Wrap the content of the file before printing
                            let wrapped_content = fill(file["content"].as_str().unwrap_or(""), 80);
                            println!("{}", wrapped_content);
                            println!("-------------------------");
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        },
        Commands::Star => {
            println!("Star a gist");
        },
        Commands::Unstar => {
            println!("Unstar a gist");
        }
    }
}