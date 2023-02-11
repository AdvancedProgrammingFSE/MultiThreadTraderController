# MultiThreadTraderController

Advance Programming project 2022/2023.

Controller application to manage the execution of multiple Traders and Visualizer processes.

## Features

The controller should allow the selection of an arbitrary trader-visualize pair from the available ones, estabilish a communication channel between the two and let them run without blocking the ui.
Given this idea the requirement of the app are as follow.

### Functional requirements
1. Has a list of available traders and visualizers.
2. Allow to choose a pair trader-visualizer.
3. Can run a trader and a visualizer in a separate process.
4. Estabilish a communication channel between a trader and a visualizer of the same pair.
5. Allow to run the same trader multiple time, at the same moment.

---

## Dependencies
```rust
gtk4 = "0.5.5"
relm4 = "0.5.0-rc.1"
relm4-macros = "0.5.0-rc.1"
relm4-components = "0.5.0-rc.1"
tracker-macros = "0.2.0"
serde = { version = "1.0.104", features = ["derive"] }
serde_json = "1.0.48"
```
---

## Implementation

### Requirement 1

To provide the list of available traders and visualizer to the controller it has been decided to specify them in a configuration file, which is read at the start of the program.
The file has a json structure, described as follow.
```json
{
  "traders" : [
    "trader_name" : "path_to_the_executable",
   ],
   "visualizers" : [
    "visualizer_name" : "path_to_the_executable",
   ]
}
```
The controller at the start will search for the configuration file and parse it using the serde crate.
The failure at finding the file or parsing it will cause the program to run with empty list for each traders and visualizers.
The key for each trader/visualizers is it's name, and what will be used in the program to identify it.

### Requirement 2

To allow the user to choose a trader-visualizer pair the controller will show a dropdown selector with the available traders and a button to select one.
After selecting a trader it will be added to a list below where it will be possible to choose the corresponding visualizer from another dropdown.
Each entry in the list will provide a button to start the execution of both the trader and the visualizer.

### Requirement 3

To execute a trader-visualizer pair the controller uses `std::process::Command::new().spawn()` to create a new thread and execute the program specified in the path in the config file.

### Requirement 4

To make a trader communicate with its visualizer it has been used a one way pipe.
On the creation of the trader process the controller pipes its stdout and pass it as stdin for the visualizer process.

```rust
let trader_process = std::process::Command::new(self.trader.get_path())
  .stdout(Stdio::piped()).spawn();
						
if let Ok(process) = trader_process {
  if let Some(stdout_pipe) = process.stdout {
    std::process::Command::new(visualizer.get_path())
      .stdin(stdout_pipe);
  }
}
```

To formalize the communication we created a common format to express the events appening in the trader process, and wirtten a [common crate](https://github.com/AdvancedProgrammingFSE/MessageFormatter) to use.

### Requirement 5

Because of the solution to requirement 3 and 4, also requirement 5 is naturally achieved, as clicking the run button of a selected trader will just create a new process which will not interfere with the others.

