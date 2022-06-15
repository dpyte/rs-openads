
// Siamese network requires two images inorder to perform cross-match analysis
// performing this task in real-time is a bit challenging therefore, the approach used in this
// model is to first register data -> analyze -> write result to the file with added annotations/ metadata
static ELEPHANT_TS_MODEL: &str = "/var/system/openads/config/models/model/scad_elephant_model.pt";
