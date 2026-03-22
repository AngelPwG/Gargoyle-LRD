use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

pub struct Umbral {
    pub valor: f64,
}

/// Crea el pool y lee configuracion_umbral al arrancar.
/// Falla en startup si no puede conectar — comportamiento intencional:
/// el detector no debe arrancar sin saber cuál es su umbral.
pub async fn init(url: &str) -> anyhow::Result<(MySqlPool, Umbral)> {
    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .connect(url)
        .await?;

    let umbral = leer_umbral(&pool).await?;
    Ok((pool, umbral))
}

async fn leer_umbral(pool: &MySqlPool) -> anyhow::Result<Umbral> {
    let row = sqlx::query!(
        "SELECT valor FROM configuracion_umbral WHERE parametro = 'ENTROPIA_UMBRAL' LIMIT 1"
    )
    .fetch_one(pool)
    .await?;

    let valor: f64 = row.valor
        .to_string()
        .parse()
        .unwrap_or(7.5); // fallback si el parse falla

    Ok(Umbral { valor })
}
