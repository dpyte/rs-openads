
import os


# Figure out how much space is left in the partition
def storage_size() -> int:
	target_dir = '/var/system/openads/storage'
	t_size = 0
	for dirpath, dirnames, filenames, in os.walk(target_dir):
		for fn in filenames:
			fp = os.path.join(dirpath, fn)
			if not os.path.islink(fp):
				t_size += os.path.getsize(fp)
	return t_size


def read_storage_capacity() -> int:
	lines = []
	capacity_file = '/var/system/openads/config/storage/capacity'
	with open(capacity_file, 'r') as file:
		line = file.readlines()
		line = [ln.rstrip() for ln in line]
		for ln in line:
			if ln[0] != '#':
				lines.append(ln)
	# Raise an error if total line count > 1
	if len(lines) > 1:
		raise Exception(f"total value in {capacity_file} should not exceed 1 i.e., it should contain only one value")
	return int(lines[0].split('=')[-1])


def storage_check_sequence():
	init_capacity = read_storage_capacity()
	target_dir_size = storage_size()

	print('occupied size is {:2f} GB'.format(target_dir_size / 1024 / 1024 / 1204))
	print(f'configured storage size: {init_capacity} GB')

