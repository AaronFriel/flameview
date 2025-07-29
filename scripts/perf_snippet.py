#!/usr/bin/env python3
import sys

def main(report_path):
    with open(report_path, 'r') as f:
        lines = [next(f) for _ in range(10)]
    for line in lines:
        print(line.rstrip())

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: perf_snippet.py <perf_report>")
        sys.exit(1)
    main(sys.argv[1])
