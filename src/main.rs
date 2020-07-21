use std::collections::HashMap;
use std::marker::Send;
use std::pin::Pin;

use futures::future::{join_all, Shared};
use futures::prelude::*;
use futures::Future;
use once_cell::sync::OnceCell;
use tokio::{
    sync::Mutex,
    time::{delay_for, Duration},
};
use uuid::Uuid;

/// SpecialData
/// Represents some data returned from a remote system
#[derive(Debug, Clone)]
struct SpecialData {
    pub key: String,
    pub data: Uuid,
}

/// Request Registry
/// This global mutex holds on to in-flight futures (requests) to the remote system
fn request_registry(
) -> &'static Mutex<HashMap<String, Shared<Pin<Box<dyn Future<Output = SpecialData> + Send>>>>> {
    static INSTANCE: OnceCell<
        Mutex<HashMap<String, Shared<Pin<Box<dyn Future<Output = SpecialData> + Send>>>>>,
    > = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let registry = HashMap::new();
        Mutex::new(registry)
    })
}

/// Request spy
/// Used just for this example to log requests to the remote system, showing that only
/// one request per client is in flight at a time
fn request_spy() -> &'static Mutex<Vec<String>> {
    static INSTANCE: OnceCell<Mutex<Vec<String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let spy = Vec::new();
        Mutex::new(spy)
    })
}

/// Make Request to Remote System
/// This is a dummy function simulating a call to a remote system
/// to fetch the special data
async fn make_request_to_remote_system(key: String) -> SpecialData {
    let data = Uuid::new_v4();

    // Artificial delay simulating a network request
    delay_for(Duration::from_secs(2)).await;

    // Log the request for this example to show it is only called once per client
    request_spy()
        .lock()
        .await
        .push(format!("Request from client {}, returning {}", key, data));

    SpecialData { key, data }
}

/// Get Special Data
/// This function abstracts the request to the remote system
/// making sure there's only one in flight request at a time
async fn get_special_data(key: &str) -> SpecialData {
    // Get a lock on the request registry
    let mut requests = request_registry().lock().await;
    // Check to see if there is already a request for that key
    let maybe_in_flight_request = requests.get(key).map(|inner| inner.clone());

    // If there is, use the future associated with that request
    let fut = if maybe_in_flight_request.is_some() {
        maybe_in_flight_request.unwrap()
    } else {
        // If not, create a new request. The call to shared() creates a
        // shared future that can be cloned and awaited by other tasks
        let request = make_request_to_remote_system(key.to_owned())
            .boxed()
            .shared();

        // Store the created future so others can await on it
        requests.insert(key.into(), request.clone());

        request
    };

    // Release the lock on the request registry so other requests can
    // register to wait on a remote resource
    drop(requests);

    // await the future - it will now start fetching the remote data
    let result = fut.await;

    // When the future resolves, get a lock on the registry again
    let mut requests = request_registry().lock().await;
    // Remove the request from the registry
    requests.remove(key);

    result

    // `requests` goes out of scope, releasing the lock on the registry
}

#[tokio::main]
async fn main() {
    // key1 and key2 represent two clients making requests to the system
    // at the same time. They each make multiple concurrent requests
    let key1 = "Some key";
    let key2 = "Some other key";

    let all = join_all(vec![
        get_special_data(key1),
        get_special_data(key1),
        get_special_data(key2),
        get_special_data(key2),
        get_special_data(key1),
        get_special_data(key2),
        get_special_data(key2),
        get_special_data(key2),
    ])
    .await;

    assert_keys_have_same_return_value(&all);

    let spy = request_spy().lock().await;
    dbg!(&all);
    dbg!(&spy);
}

fn assert_keys_have_same_return_value(items: &[SpecialData]) {
    let mut m = HashMap::new();
    for item in items {
        m.entry(&item.key).or_insert_with(Vec::new).push(item.data)
    }

    for (_, values) in m {
        let got = values.windows(2).all(|w| w[0] == w[1]);
        let want = true;
        assert_eq!(got, want);
    }
}