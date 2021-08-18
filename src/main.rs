/**
 * A spotify i3bar plugin
 * This is based on the spec from: https://i3wm.org/docs/i3bar-protocol.html
 */

use dbus::{blocking::Connection, arg};
use std::time::Duration;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use std::io;
use json::object;
use json::array;
use json::JsonValue::Array;


fn read_line() -> Result<String, std::io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            return Ok(input);
        }
        Err(error) => Err(error)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let props = conn.with_proxy(
        "org.mpris.MediaPlayer2.spotify",
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(5000)
    );

    let mut line: String;
    let _ = read_line()?; // Read version header
    let _ = read_line()?; // Begin endless array

    // Send back version and start data
    println!("{}", json::stringify(object!{"version": 1}));
    println!("[");
    println!("[]");

    loop {
        line = read_line()?;
        if line.starts_with(",") {
            line = line[1..].to_string();
        }

        let mut parsed = match json::parse(&line)? {
            Array(parsed) => parsed,
            _ => vec![]
        };

        // Check for playing status
        let status: String = props.get("org.mpris.MediaPlayer2.Player", "PlaybackStatus")?;
        if status != "Playing" {
            println!(",{}", json::stringify(parsed));
            continue
        }

        // Read in spotify information
        let metadata: arg::PropMap = props.get("org.mpris.MediaPlayer2.Player", "Metadata")?;
        let title: Option<&String> = arg::prop_cast(&metadata, "xesam:title");
        let title: String = match title {
            Some(title) => title.to_string(),
            None => "".to_string()
        };
        let artists: Option<&Vec<String>> = arg::prop_cast(&metadata, "xesam:artist");
        let artists: String = match artists {
            Some(artists) => artists.join("-"),
            None => "".to_string()
        };

        parsed.insert(0, object!{
            "color" : "#9ec600",
            "full_text" : format!(" {} - {}", artists, title),
            "name" : "spotify"
        });

        println!(",{}", json::stringify(parsed));
    }
}
