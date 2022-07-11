#!/bin/env python3
from storage import storage_check_sequence


# Start sequence:
# 	- StorageCap
# 	- openads


def startup_sequence():
	storage_check_sequence()


def main():
	try:
		startup_sequence()
	except:
		raise Exception("Failed to launch the system")


if __name__ == '__main__':
	main()
