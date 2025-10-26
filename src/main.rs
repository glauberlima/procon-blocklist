use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "procon-cli")]
#[command(about = "Generate adblock lists from Procon-SP bad sites database")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate adblock list
    Generate {
        /// Output format: adblock or hosts
        #[arg(short, long, default_value = "adblock")]
        format: String,

        /// Output file path (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { format, output } => {
            generate_list(format, output).await?;
        }
    }

    Ok(())
}

async fn generate_list(format: String, output: Option<PathBuf>) -> anyhow::Result<()> {
    eprintln!("Fetching sites from Procon-SP database...");
    let sites = fetch_sites().await?;
    eprintln!("Found {} sites", sites.len());

    // Generate output based on format
    let content = match format.as_str() {
        "adblock" => generate_adblock(&sites),
        "hosts" => generate_hosts(&sites),
        _ => anyhow::bail!("Unsupported format: {}", format),
    };

    // Output to file or stdout
    if let Some(path) = output {
        std::fs::write(&path, &content)?;
        eprintln!("List generated and saved to {}", path.display());
    } else {
        println!("{}", content);
    }

    Ok(())
}

async fn fetch_sites() -> anyhow::Result<Vec<String>> {
    let url = "https://sistemas.procon.sp.gov.br/evitesite/list/evitesite.php?action=list&jtStartIndex=0&jtPageSize=600&jtSorting=strSite%20ASC";
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;
    let response = client
        .get(url)
        .query(&[("_", timestamp)])
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let records = json["Records"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid JSON response: missing Records array"))?;

    let mut sites = Vec::new();
    for record in records {
        if let Some(site) = record["strSite"].as_str() {
            sites.push(site.to_string());
        }
    }

    Ok(sites)
}

fn generate_adblock(sites: &[String]) -> String {
    let mut content = String::new();

    // Header
    content.push_str("! Title: Evite esses Sites - Procon-SP\n");
    content.push_str("! Expires: 1 day\n");
    content.push_str("! Description: Lista gerada a partir do site Evite esses Sites - https://sistemas.procon.sp.gov.br/evitesite/list/evitesites.php - Fundação Procon/SP\n");
    content.push_str("! Homepage: https://github.com/glauberlima/procon-badsites\n");
    content.push_str("! Licence: https://github.com/glauberlima/procon-badsites/blob/main/LICENSE\n");

    let now = chrono::Utc::now();
    content.push_str(&format!("! Updated: {} (GMT{})\n",
        now.format("%d %B %Y %H:%M:%S"),
        now.format("%:z")
    ));

    // Rules
    for site in sites {
        content.push_str(&format!("||{}^\n", site));
    }

    content
}

fn generate_hosts(sites: &[String]) -> String {
    let mut content = String::new();

    // Header
    content.push_str("# Title: Evite esses Sites - Procon-SP\n");
    content.push_str("# Description: Lista gerada a partir do site Evite esses Sites - https://sistemas.procon.sp.gov.br/evitesite/list/evitesites.php - Fundação Procon/SP\n");
    content.push_str("# Homepage: https://github.com/glauberlima/procon-badsites\n");
    content.push_str("# Licence: https://github.com/glauberlima/procon-badsites/blob/main/LICENSE\n");

    let now = chrono::Utc::now();
    content.push_str(&format!("# Updated: {} (GMT{})\n",
        now.format("%d %B %Y %H:%M:%S"),
        now.format("%:z")
    ));

    // Rules
    for site in sites {
        content.push_str(&format!("0.0.0.0 {}\n", site));
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_sites_structure() {
        // Test that our mock data has expected structure
        let sites = vec![
            "123importados.com".to_string(),
            "123multiofertas.com.br".to_string(),
            "123multiofertas.net".to_string(),
            "Acessivelmodasbras.com.br".to_string(),
            "agachecomercial.net".to_string(),
        ];
        assert!(!sites.is_empty());
        assert!(sites.contains(&"123importados.com".to_string()));
        assert_eq!(sites.len(), 5);
    }

    #[test]
    fn test_generate_adblock() {
        let sites = vec!["example.com".to_string(), "test.org".to_string()];
        let result = generate_adblock(&sites);

        assert!(result.contains("! Title: Evite esses Sites - Procon-SP"));
        assert!(result.contains("||example.com^"));
        assert!(result.contains("||test.org^"));
    }

    #[test]
    fn test_generate_hosts() {
        let sites = vec!["example.com".to_string(), "test.org".to_string()];
        let result = generate_hosts(&sites);

        assert!(result.contains("# Title: Evite esses Sites - Procon-SP"));
        assert!(result.contains("0.0.0.0 example.com"));
        assert!(result.contains("0.0.0.0 test.org"));
    }
}
