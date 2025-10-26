use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "procon-cli")]
#[command(about = "Gera listas de bloqueio AdBlock e hosts a partir da base de dados de sites não confiáveis da Fundação Procon-SP")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Gera lista de bloqueio AdBlock
    Generate {
        /// Formato de saída: adblock ou hosts
        #[arg(short, long, default_value = "adblock")]
        format: String,

        /// Caminho do arquivo de saída (padrão: stdout)
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
    eprintln!("Buscando sites da base de dados da Fundação Procon-SP...");
    let sites = fetch_sites().await?;
    eprintln!("Encontrados {} sites", sites.len());

    // Gera saída baseada no formato
    let content = match format.as_str() {
        "adblock" => generate_adblock(&sites),
        "hosts" => generate_hosts(&sites),
        _ => anyhow::bail!("Formato não suportado: {}", format),
    };

    // Saída para arquivo ou stdout
    if let Some(path) = output {
        std::fs::write(&path, &content)?;
        eprintln!("Lista gerada e salva em {}", path.display());
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

    let client = reqwest::Client::builder().build()?;
    let response = client.get(url).query(&[("_", timestamp)]).send().await?;

    let json: serde_json::Value = response.json().await?;
    let records = json["Records"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Resposta JSON inválida: array Records ausente"))?;

    let sites: Vec<String> = records
        .iter()
        .filter_map(|record| record["strSite"].as_str().map(|s| s.to_string()))
        .collect();

    Ok(sites)
}

fn generate_adblock(sites: &[String]) -> String {
    let now = chrono::Utc::now();

    let header = format!(
        "! Title: Evite esses Sites - Fundação Procon-SP\n\
         ! Expires: 1 day\n\
         ! Description: Lista gerada a partir do site Evite esses Sites - https://sistemas.procon.sp.gov.br/evitesite/list/evitesites.php - Fundação Procon/SP\n\
         ! Homepage: https://github.com/glauberlima/procon-blocklist\n\
         ! Licence: https://github.com/glauberlima/procon-blocklist/blob/main/LICENSE\n\
         ! Updated: {} (GMT{})\n",
        now.format("%d %B %Y %H:%M:%S"),
        now.format("%:z")
    );

    let rules = sites
        .iter()
        .map(|site| format!("||{}^\n", site))
        .collect::<String>();

    header + &rules
}

fn generate_hosts(sites: &[String]) -> String {
    let now = chrono::Utc::now();

    let header = format!(
        "# Title: Evite esses Sites - Fundação Procon-SP\n\
         # Description: Lista gerada a partir do site Evite esses Sites - https://sistemas.procon.sp.gov.br/evitesite/list/evitesites.php - Fundação Procon/SP\n\
         # Homepage: https://github.com/glauberlima/procon-blocklist\n\
         # Licence: https://github.com/glauberlima/procon-blocklist/blob/main/LICENSE\n\
         # Updated: {} (GMT{})\n",
        now.format("%d %B %Y %H:%M:%S"),
        now.format("%:z")
    );

    let rules = sites
        .iter()
        .map(|site| format!("0.0.0.0 {}\n", site))
        .collect::<String>();

    header + &rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_sites_structure() {
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

        assert!(result.contains("! Title: Evite esses Sites - Fundação Procon-SP"));
        assert!(result.contains("||example.com^"));
        assert!(result.contains("||test.org^"));
    }

    #[test]
    fn test_generate_hosts() {
        let sites = vec!["example.com".to_string(), "test.org".to_string()];
        let result = generate_hosts(&sites);

        assert!(result.contains("# Title: Evite esses Sites - Fundação Procon-SP"));
        assert!(result.contains("0.0.0.0 example.com"));
        assert!(result.contains("0.0.0.0 test.org"));
    }
}
