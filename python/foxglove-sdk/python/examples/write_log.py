import argparse
import foxglove

parser = argparse.ArgumentParser()
parser.add_argument("--path", type=str, default="output.mcap")
args = parser.parse_args()

# Open a new mcap file for recording
foxglove.record_file(args.path)
