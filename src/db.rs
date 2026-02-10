use sqlx::{postgres::PgPoolOptions, PgPool};
// use crate::indexer::proposals::ProposalCreatedFilter;

pub async fn connect(url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .expect("DB connection failed")
}

pub async fn save_proposal(
    pool: &PgPool,
    proposal_id: i64,
    proposer: String,
    start_block: i64,
    end_block: i64,
    description: String,
) {
    sqlx::query!(
        r#"
        INSERT INTO proposals
        (proposal_id, proposer, start_block, end_block, description)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (proposal_id) DO NOTHING
        "#,
        proposal_id,
        proposer,
        start_block,
        end_block,
        description,
    )
    .execute(pool)
    .await
    .expect("Failed to insert proposal");
}

// // pub async fn save_proposal(
//     pool: &PgPool,
//     p: ProposalCreatedFilter,
// ) {
//     sqlx::query!(
//         r#"
//         INSERT INTO proposals
//         (proposal_id, proposer, start_block, end_block, description)
//         VALUES ($1, $2, $3, $4, $5)
//         ON CONFLICT (proposal_id) DO NOTHING
//         "#,
//         p.proposal_id.as_u64() as i64,
//         format!("{:?}", p.proposer),
//         p.start_block.as_u64() as i64,
//         p.end_block.as_u64() as i64,
//         p.description,
//     )
//     .execute(pool)
//     .await
//     .expect("Failed to insert proposal");
// }
