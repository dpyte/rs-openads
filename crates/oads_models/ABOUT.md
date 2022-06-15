
## How the pipeline works

* A 'server in from this model' will actively look for incoming frames from vision.rs. From there on, the frames will pass through the feature extractor
* Feature extractor will try to filter out People and Everyday Objects. 

