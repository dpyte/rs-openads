#!bin/env python3

import os
import shutil
from pathlib import Path


def check_for_openads():
	root_dir = '/var/system/openads'
	config_dir = os.path.join(root_dir, 'config', 'camera')
	storage_dir = os.path.join(root_dir, 'storage')

	rpath = Path(root_dir)
	rpath.parent.mkdir(parents=True, exist_ok=True)

	cpath = Path(config_dir)
	cpath.parent.mkdir(parents=True, exist_ok=True)

	spath = Path(storage_dir)
	spath.parent.mkdir(parents=True, exist_ok=True)

	config_src = os.path.abspath('../../configuration/camera/info.xml')
	shutil.copyfile(config_src, os.path.join(config_dir, 'info.xml'))


def main():
	check_for_openads()
	pass


if __name__ == '__main__':
	main()
