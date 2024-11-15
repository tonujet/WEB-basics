use sqlx::Pool;

pub async fn connect_and_migrate<T>(name: &str) -> Pool<T>
where
    <T as sqlx::Database>::Connection: sqlx::migrate::Migrate, T: sqlx::Database
{
    let pool = Pool::<T>::connect(name)
        .await
        .expect(&format!("Couldn't connect to database: {name}"));
    println!("Connected to database successfully");
    sqlx::migrate!()
        .run(
            &mut pool
                .acquire()
                .await
                .expect("Can't acquire connection in the pool"),
        )
        .await
        .expect(&format!("could not run migrations for database {name}"));
    println!("Migrations were run successfully");
    pool
}
