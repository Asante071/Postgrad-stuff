import sys
import time
from datetime import datetime

SESSION_FILE = "sessions.txt"

def start_session():
    print("Study session started...")
    print("Timer: 25 minutes")

    time.sleep(5)  # shortened for demo

    print("Session completed!")

    with open(SESSION_FILE, "a") as f:
        f.write(f"{datetime.now()}\n")

def show_stats():
    try:
        with open(SESSION_FILE, "r") as f:
            sessions = f.readlines()

        print(f"Total sessions: {len(sessions)}")
        print(f"Total study time: {len(sessions) * 25} minutes")

    except FileNotFoundError:
        print("No study sessions recorded yet.")

def main():
    if len(sys.argv) < 2:
        print("Commands:")
        print(" start  → start study timer")
        print(" stats  → view study statistics")
        return

    command = sys.argv[1]

    if command == "start":
        start_session()
    elif command == "stats":
        show_stats()
    else:
        print("Unknown command")

if __name__ == "__main__":
    main()