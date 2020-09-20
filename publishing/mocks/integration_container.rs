use std::sync::Arc;

use tokio_postgres::NoTls;

use common::config::Config;
use common::error::Error;
use common::mocks::FakeEventPublisher;
use common::result::Result;
use shared::infrastructure::persistence::inmem::InMemUserRepository;
use shared::mocks::FakeUserService;

use crate::container::Container;
use crate::infrastructure::persistence::inmem::{
    InMemInteractionRepository, InMemPublicationRepository, InMemReaderRepository,
};
use crate::infrastructure::persistence::postgres::{
    PostgresAuthorRepository, PostgresCategoryRepository, PostgresCollectionRepository,
};
use crate::mocks;

pub async fn integration_container() -> Result<Container<FakeEventPublisher>> {
    let config = Config::get();
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host={} user={} password={} dbname={}",
            config.postgres_host(),
            config.postgres_username(),
            config.postgres_password(),
            config.postgres_database()
        ),
        NoTls,
    )
    .await
    .map_err(|err| Error::new("postgres", "connection").wrap_raw(err))?;

    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("error: {}", err);
        }
    });

    let client = Arc::new(client);

    let container = Container::new(
        Arc::new(FakeEventPublisher::new()),
        Arc::new(PostgresAuthorRepository::new(client.clone())),
        Arc::new(PostgresCategoryRepository::new(client.clone())),
        Arc::new(PostgresCollectionRepository::new(client.clone())),
        Arc::new(InMemInteractionRepository::new()),
        Arc::new(InMemPublicationRepository::new()),
        Arc::new(InMemReaderRepository::new()),
        Arc::new(InMemUserRepository::new()),
        Arc::new(FakeUserService::new()),
    );

    mocks::populate(&container).await?;

    Ok(container)
}
