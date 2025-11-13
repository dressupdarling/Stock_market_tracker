use cassandra_cpp::{Cluster, Session};
use tokio;

#[tokio::main]
async fn main() -> Result<(), cassandra_cpp::Error> {
    // Connect to Cassandra
    let mut cluster = Cluster::default();
    cluster.set_contact_points("127.0.0.1")?;
    let session: Session = cluster.connect().await?;

    println!("Connected to Cassandra ");

    // Create keyspace
    session.execute(
        "CREATE KEYSPACE IF NOT EXISTS demo WITH REPLICATION = {'class': 'SimpleStrategy', 'replication_factor': 1};"
    ).await?;

    // Create table
    session.execute(
        "CREATE TABLE IF NOT EXISTS demo.users (id UUID PRIMARY KEY, name text);"
    ).await?;

    println!("Keyspace and table created ");

    Ok(())
}


//brew install cmake openssl@3 libuv cassandra-cpp-driver
//
// export OPENSSL_DIR=/opt/homebrew/opt/openssl@3
// export CASSANDRA_CPP_DRIVER=/opt/homebrew/opt/cassandra-cpp-driver
// export LIBUV_DIR=/opt/homebrew/opt/libuv
//
// export LIBRARY_PATH=$CASSANDRA_CPP_DRIVER/lib:$OPENSSL_DIR/lib:$LIBUV_DIR/lib:$LIBRARY_PATH
// export CPATH=$CASSANDRA_CPP_DRIVER/include:$OPENSSL_DIR/include:$LIBUV_DIR/include:$CPATH
//
// cargo clean
// cargo build
// cargo run