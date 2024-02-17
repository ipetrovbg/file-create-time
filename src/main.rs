use chrono;
use std::fs;
use std::io;
use std::ops::Sub;
use std::time::SystemTime;

const MINUTES: i64 = 30;
fn main() -> Result<(), io::Error> {
    let metadata = fs::metadata("foo.txt")?;
    let creation_time = metadata.created()?;

    match creation_time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(creation_time) => {
            let now = chrono::Local::now();
            let creation_time = chrono::NaiveDateTime::from_timestamp_opt(
                creation_time.as_secs() as i64,
                creation_time.subsec_nanos(),
            );

            if let Some(creation_time) = creation_time {
                let now = now.sub(chrono::Duration::hours(2));
                let past_time = now.sub(chrono::Duration::minutes(MINUTES));

                println!("File creation time: {}", creation_time);
                println!("Past time:          {}", past_time.naive_local());

                if creation_time <= past_time.naive_local() {
                    println!("\nFile is older than {MINUTES} minutes");
                } else {
                    println!("\nFile is not older than {MINUTES} minutes");
                }
            }
        }
        _ => println!("No creation time found"),
    };
    Ok(())
}
