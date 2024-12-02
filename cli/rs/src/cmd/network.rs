/* Import modules. */
use std::io::{BufReader, BufRead};
use std::io::{self, Write};
use std::process::{Command, Stdio};

/**
 * Ping
 * 
 * Starts a long-lived ping process on the provided destination.
 */
pub fn ping() {
    let mut child = Command::new("ping")
        .arg("www.yahoo.com")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Oops! Failed to execute child.");

    /* Initialize output for child. */ 
    let stdout = child.stdout
        .as_mut()
        .expect("Oops! Failed to initialize output for child.");
    
    /* Initialize intput buffer. */
    let stdout_reader = BufReader::new(stdout);
    
    /* Handle line inputs. */
    let stdout_lines = stdout_reader
        .lines();

    /* Handle output reader buffer. */
    for line in stdout_lines {
        println!("Read -> {:?}", line);
    }

    /* Wait for child. */
    // TODO: How can we retrieve the final output?
    let output = child
        .wait_with_output()
        .expect("Oops! Failed to wait for child.");
    assert!(output.status.success());
    println!("Final output -> {:?}\n", output);
}

pub fn ping2() {
    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    let mut stdout = child.stdout.as_mut().unwrap();
    let mut stderr = child.stderr.as_mut().unwrap();

    // let stdout = String::from_utf8(stdout).unwrap();
    println!("{:?}", stdout);

    // io::stdout().write_all(&mut stdout).unwrap();
    // io::stderr().write_all(&mut stderr).unwrap();
}

pub fn avalanche() -> Result<String, Box<dyn std::error::Error>> {
    /* Initialize locals. */
    let mut response;

    let output = Command::new("avalanche")
        .arg("--help")
        .output();
    
    match output {
        Ok(ref out) => {
            response = String::from_utf8_lossy(&output.unwrap().stdout).to_string();
        },
        Err(ref err) => {
            response = format!("ERROR: {:?}", err.to_string());
        },
    };

    Ok(response)
}

pub fn install_avalanche() -> Result<String, Box<dyn std::error::Error>> {
    /* Initialize locals. */
    let mut response;

    let cmd1 = Command::new("cd")
        .arg("/tmp");

    let cmd2 = Command::new("mkdir")
        .arg("noderunr");

    let cmd3 = Command::new("cd")
        .arg("noderunr");

    let cmd4 = Command::new("git")
        .arg("clone")
        .arg("https://github.com/ava-labs/avalanchego.git");

    let cmd = cmd1
        .command("&&").unwrap()
        .join(cmd2)
        .command("&&").unwrap()
        .join(cmd3)
        .command("&&").unwrap()
        .join(cmd4);

    let output = cmd.output();
    
    match output {
        Ok(ref out) => {
            response = String::from_utf8_lossy(&output.unwrap().stdout).to_string();
        },
        Err(ref err) => {
            response = format!("ERROR! {:?}", err.to_string());
        },
    };

    Ok(response)
}