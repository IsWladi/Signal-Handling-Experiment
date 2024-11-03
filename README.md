# Signal Handling Experiment
In this experiment, I compared Rust and Python for executing system commands in a loop with SIGINT signal handling (CTRL+C). The goal was to achieve a graceful shutdown without abruptly terminating in-progress command executions.

## Rust Example Execution
![image](https://github.com/user-attachments/assets/7f7eb9dd-621f-424a-a64d-c740fa35edc6)

## Python Example Execution
![image](https://github.com/user-attachments/assets/eaca3499-b4ec-4096-ba35-ca61513338db)

## Conclusion
To prevent signals from propagating to the child process and causing abrupt termination, it is essential to detach the child process from the main program. This ensures that only the main process handles signals, allowing for a controlled and graceful shutdown.
