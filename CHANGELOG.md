# Changelog
All notable changes to this project will be documented in this file.

## [0.1.0.dev] - 2022-06-14
------------------------
Fixed runtime issue caused by libtorch. Temporary solution for this issue is to change ownership of all the subdirectory
and files contained within `/var/system/openads`

The future trajectory of this project has been changed to using single-device cameras for surveillance. This has been done in order to minimize
the CPU utilization and repurpose the threads into processing models much efficiently.

#### Internal Changes
* Substituted Tokio channels with Crossbeams channels


