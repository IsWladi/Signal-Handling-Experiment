import signal
import subprocess
import os
import sys
import time


# Flag to control termination
terminate = False

# Signal handler to update the terminate flag when SIGINT is received
def signal_handler(sig, frame):
    global terminate
    terminate = True

# Register SIGINT handler
signal.signal(signal.SIGINT, signal_handler)

def main():
    i = 0

    # Execute a command in a infinite loop until SIGINT is received.
    # The SIGINT signal have to be ignored in the child process to avoid the abrupt termination of executed command.
    while not terminate:
        # Execute command
        try:
            process = subprocess.Popen(
                ["sh", "-c", f"sleep 2 && echo -n 'hello #{i} after 2 seconds'"],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                preexec_fn=os.setsid  # create new session detached from the parent process
            )

            # Get stdout and stderr
            stdout, stderr = process.communicate()

            # Decode and print output
            print(f"stdout: {stdout.decode().strip()}, stderr: {stderr.decode().strip()}")

            i += 1
        except Exception as e:
            print(f"Error: {e}", file=sys.stderr)
            break

if __name__ == "__main__":
    main()
