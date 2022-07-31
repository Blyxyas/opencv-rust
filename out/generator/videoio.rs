#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Video I/O
//! 
//! Read and write video or images sequence with OpenCV
//! 
//! ### See also:
//! - @ref videoio_overview
//! - Tutorials: @ref tutorial_table_of_content_app
//!   # Flags for video I/O
//!   # Additional flags for video I/O API backends
//!   # Hardware-accelerated video decoding and encoding
//!   # C API for video I/O
//!   # iOS glue for video I/O
//!   # WinRT glue for video I/O
//!   # Query I/O API backends registry
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::VideoCaptureTraitConst, super::VideoCaptureTrait, super::VideoWriterTraitConst, super::VideoWriterTrait };
}

// CAP_ANDROID /usr/include/opencv2/videoio.hpp:106
pub const CAP_ANDROID: i32 = 1000;
// CAP_ANY /usr/include/opencv2/videoio.hpp:91
pub const CAP_ANY: i32 = 0;
// CAP_ARAVIS /usr/include/opencv2/videoio.hpp:121
pub const CAP_ARAVIS: i32 = 2100;
// CAP_AVFOUNDATION /usr/include/opencv2/videoio.hpp:108
pub const CAP_AVFOUNDATION: i32 = 1200;
// CAP_CMU1394 /usr/include/opencv2/videoio.hpp:99
pub const CAP_CMU1394: i32 = 300;
// CAP_DC1394 /usr/include/opencv2/videoio.hpp:98
pub const CAP_DC1394: i32 = 300;
// CAP_DSHOW /usr/include/opencv2/videoio.hpp:102
pub const CAP_DSHOW: i32 = 700;
// CAP_FFMPEG /usr/include/opencv2/videoio.hpp:119
pub const CAP_FFMPEG: i32 = 1900;
// CAP_FIREWARE /usr/include/opencv2/videoio.hpp:96
pub const CAP_FIREWARE: i32 = 300;
// CAP_FIREWIRE /usr/include/opencv2/videoio.hpp:95
pub const CAP_FIREWIRE: i32 = 300;
// CAP_GIGANETIX /usr/include/opencv2/videoio.hpp:109
pub const CAP_GIGANETIX: i32 = 1300;
// CAP_GPHOTO2 /usr/include/opencv2/videoio.hpp:117
pub const CAP_GPHOTO2: i32 = 1700;
// CAP_GSTREAMER /usr/include/opencv2/videoio.hpp:118
pub const CAP_GSTREAMER: i32 = 1800;
// CAP_IEEE1394 /usr/include/opencv2/videoio.hpp:97
pub const CAP_IEEE1394: i32 = 300;
// CAP_IMAGES /usr/include/opencv2/videoio.hpp:120
pub const CAP_IMAGES: i32 = 2000;
// CAP_INTELPERC /usr/include/opencv2/videoio.hpp:112
pub const CAP_INTELPERC: i32 = 1500;
// CAP_INTELPERC_DEPTH_GENERATOR /usr/include/opencv2/videoio.hpp:604
pub const CAP_INTELPERC_DEPTH_GENERATOR: i32 = 536870912;
// CAP_INTELPERC_DEPTH_MAP /usr/include/opencv2/videoio.hpp:610
pub const CAP_INTELPERC_DEPTH_MAP: i32 = 0;
// CAP_INTELPERC_GENERATORS_MASK /usr/include/opencv2/videoio.hpp:607
pub const CAP_INTELPERC_GENERATORS_MASK: i32 = 939524096;
// CAP_INTELPERC_IMAGE /usr/include/opencv2/videoio.hpp:613
pub const CAP_INTELPERC_IMAGE: i32 = 3;
// CAP_INTELPERC_IMAGE_GENERATOR /usr/include/opencv2/videoio.hpp:605
pub const CAP_INTELPERC_IMAGE_GENERATOR: i32 = 268435456;
// CAP_INTELPERC_IR_GENERATOR /usr/include/opencv2/videoio.hpp:606
pub const CAP_INTELPERC_IR_GENERATOR: i32 = 134217728;
// CAP_INTELPERC_IR_MAP /usr/include/opencv2/videoio.hpp:612
pub const CAP_INTELPERC_IR_MAP: i32 = 2;
// CAP_INTELPERC_UVDEPTH_MAP /usr/include/opencv2/videoio.hpp:611
pub const CAP_INTELPERC_UVDEPTH_MAP: i32 = 1;
// CAP_INTEL_MFX /usr/include/opencv2/videoio.hpp:123
pub const CAP_INTEL_MFX: i32 = 2300;
// CAP_MSMF /usr/include/opencv2/videoio.hpp:110
pub const CAP_MSMF: i32 = 1400;
// CAP_OPENCV_MJPEG /usr/include/opencv2/videoio.hpp:122
pub const CAP_OPENCV_MJPEG: i32 = 2200;
// CAP_OPENNI /usr/include/opencv2/videoio.hpp:104
pub const CAP_OPENNI: i32 = 900;
// CAP_OPENNI2 /usr/include/opencv2/videoio.hpp:114
pub const CAP_OPENNI2: i32 = 1600;
// CAP_OPENNI2_ASTRA /usr/include/opencv2/videoio.hpp:116
pub const CAP_OPENNI2_ASTRA: i32 = 1620;
// CAP_OPENNI2_ASUS /usr/include/opencv2/videoio.hpp:115
pub const CAP_OPENNI2_ASUS: i32 = 1610;
// CAP_OPENNI_ASUS /usr/include/opencv2/videoio.hpp:105
pub const CAP_OPENNI_ASUS: i32 = 910;
// CAP_OPENNI_BGR_IMAGE /usr/include/opencv2/videoio.hpp:323
pub const CAP_OPENNI_BGR_IMAGE: i32 = 5;
// CAP_OPENNI_DEPTH_GENERATOR /usr/include/opencv2/videoio.hpp:281
pub const CAP_OPENNI_DEPTH_GENERATOR: i32 = -2147483648;
// CAP_OPENNI_DEPTH_GENERATOR_BASELINE /usr/include/opencv2/videoio.hpp:309
pub const CAP_OPENNI_DEPTH_GENERATOR_BASELINE: i32 = -2147483546;
// CAP_OPENNI_DEPTH_GENERATOR_FOCAL_LENGTH /usr/include/opencv2/videoio.hpp:310
pub const CAP_OPENNI_DEPTH_GENERATOR_FOCAL_LENGTH: i32 = -2147483545;
// CAP_OPENNI_DEPTH_GENERATOR_PRESENT /usr/include/opencv2/videoio.hpp:308
pub const CAP_OPENNI_DEPTH_GENERATOR_PRESENT: i32 = -2147483539;
// CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION /usr/include/opencv2/videoio.hpp:311
pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION: i32 = -2147483544;
// CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION_ON /usr/include/opencv2/videoio.hpp:312
pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION_ON: i32 = -2147483544;
// CAP_OPENNI_DEPTH_MAP /usr/include/opencv2/videoio.hpp:317
pub const CAP_OPENNI_DEPTH_MAP: i32 = 0;
// CAP_OPENNI_DISPARITY_MAP /usr/include/opencv2/videoio.hpp:319
pub const CAP_OPENNI_DISPARITY_MAP: i32 = 2;
// CAP_OPENNI_DISPARITY_MAP_32F /usr/include/opencv2/videoio.hpp:320
pub const CAP_OPENNI_DISPARITY_MAP_32F: i32 = 3;
// CAP_OPENNI_GENERATORS_MASK /usr/include/opencv2/videoio.hpp:284
pub const CAP_OPENNI_GENERATORS_MASK: i32 = -536870912;
// CAP_OPENNI_GRAY_IMAGE /usr/include/opencv2/videoio.hpp:324
pub const CAP_OPENNI_GRAY_IMAGE: i32 = 6;
// CAP_OPENNI_IMAGE_GENERATOR /usr/include/opencv2/videoio.hpp:282
pub const CAP_OPENNI_IMAGE_GENERATOR: i32 = 1073741824;
// CAP_OPENNI_IMAGE_GENERATOR_OUTPUT_MODE /usr/include/opencv2/videoio.hpp:307
pub const CAP_OPENNI_IMAGE_GENERATOR_OUTPUT_MODE: i32 = 1073741924;
// CAP_OPENNI_IMAGE_GENERATOR_PRESENT /usr/include/opencv2/videoio.hpp:306
pub const CAP_OPENNI_IMAGE_GENERATOR_PRESENT: i32 = 1073741933;
// CAP_OPENNI_IR_GENERATOR /usr/include/opencv2/videoio.hpp:283
pub const CAP_OPENNI_IR_GENERATOR: i32 = 536870912;
// CAP_OPENNI_IR_GENERATOR_PRESENT /usr/include/opencv2/videoio.hpp:313
pub const CAP_OPENNI_IR_GENERATOR_PRESENT: i32 = 536871021;
// CAP_OPENNI_IR_IMAGE /usr/include/opencv2/videoio.hpp:326
pub const CAP_OPENNI_IR_IMAGE: i32 = 7;
// CAP_OPENNI_POINT_CLOUD_MAP /usr/include/opencv2/videoio.hpp:318
pub const CAP_OPENNI_POINT_CLOUD_MAP: i32 = 1;
// CAP_OPENNI_QVGA_30HZ /usr/include/opencv2/videoio.hpp:333
pub const CAP_OPENNI_QVGA_30HZ: i32 = 3;
// CAP_OPENNI_QVGA_60HZ /usr/include/opencv2/videoio.hpp:334
pub const CAP_OPENNI_QVGA_60HZ: i32 = 4;
// CAP_OPENNI_SXGA_15HZ /usr/include/opencv2/videoio.hpp:331
pub const CAP_OPENNI_SXGA_15HZ: i32 = 1;
// CAP_OPENNI_SXGA_30HZ /usr/include/opencv2/videoio.hpp:332
pub const CAP_OPENNI_SXGA_30HZ: i32 = 2;
// CAP_OPENNI_VALID_DEPTH_MASK /usr/include/opencv2/videoio.hpp:321
pub const CAP_OPENNI_VALID_DEPTH_MASK: i32 = 4;
// CAP_OPENNI_VGA_30HZ /usr/include/opencv2/videoio.hpp:330
pub const CAP_OPENNI_VGA_30HZ: i32 = 0;
// CAP_PROP_APERTURE /usr/include/opencv2/videoio.hpp:635
pub const CAP_PROP_APERTURE: i32 = 17008;
// CAP_PROP_ARAVIS_AUTOTRIGGER /usr/include/opencv2/videoio.hpp:555
pub const CAP_PROP_ARAVIS_AUTOTRIGGER: i32 = 600;
// CAP_PROP_AUDIO_BASE_INDEX /usr/include/opencv2/videoio.hpp:199
pub const CAP_PROP_AUDIO_BASE_INDEX: i32 = 63;
// CAP_PROP_AUDIO_DATA_DEPTH /usr/include/opencv2/videoio.hpp:197
pub const CAP_PROP_AUDIO_DATA_DEPTH: i32 = 61;
// CAP_PROP_AUDIO_POS /usr/include/opencv2/videoio.hpp:195
pub const CAP_PROP_AUDIO_POS: i32 = 59;
// CAP_PROP_AUDIO_SAMPLES_PER_SECOND /usr/include/opencv2/videoio.hpp:198
pub const CAP_PROP_AUDIO_SAMPLES_PER_SECOND: i32 = 62;
// CAP_PROP_AUDIO_SHIFT_NSEC /usr/include/opencv2/videoio.hpp:196
pub const CAP_PROP_AUDIO_SHIFT_NSEC: i32 = 60;
// CAP_PROP_AUDIO_STREAM /usr/include/opencv2/videoio.hpp:194
pub const CAP_PROP_AUDIO_STREAM: i32 = 58;
// CAP_PROP_AUDIO_SYNCHRONIZE /usr/include/opencv2/videoio.hpp:202
pub const CAP_PROP_AUDIO_SYNCHRONIZE: i32 = 66;
// CAP_PROP_AUDIO_TOTAL_CHANNELS /usr/include/opencv2/videoio.hpp:200
pub const CAP_PROP_AUDIO_TOTAL_CHANNELS: i32 = 64;
// CAP_PROP_AUDIO_TOTAL_STREAMS /usr/include/opencv2/videoio.hpp:201
pub const CAP_PROP_AUDIO_TOTAL_STREAMS: i32 = 65;
// CAP_PROP_AUTOFOCUS /usr/include/opencv2/videoio.hpp:175
pub const CAP_PROP_AUTOFOCUS: i32 = 39;
// CAP_PROP_AUTO_EXPOSURE /usr/include/opencv2/videoio.hpp:158
pub const CAP_PROP_AUTO_EXPOSURE: i32 = 21;
// CAP_PROP_AUTO_WB /usr/include/opencv2/videoio.hpp:180
pub const CAP_PROP_AUTO_WB: i32 = 44;
// CAP_PROP_BACKEND /usr/include/opencv2/videoio.hpp:178
pub const CAP_PROP_BACKEND: i32 = 42;
// CAP_PROP_BACKLIGHT /usr/include/opencv2/videoio.hpp:168
pub const CAP_PROP_BACKLIGHT: i32 = 32;
// CAP_PROP_BITRATE /usr/include/opencv2/videoio.hpp:183
pub const CAP_PROP_BITRATE: i32 = 47;
// CAP_PROP_BRIGHTNESS /usr/include/opencv2/videoio.hpp:146
pub const CAP_PROP_BRIGHTNESS: i32 = 10;
// CAP_PROP_BUFFERSIZE /usr/include/opencv2/videoio.hpp:174
pub const CAP_PROP_BUFFERSIZE: i32 = 38;
// CAP_PROP_CHANNEL /usr/include/opencv2/videoio.hpp:179
pub const CAP_PROP_CHANNEL: i32 = 43;
// CAP_PROP_CODEC_EXTRADATA_INDEX /usr/include/opencv2/videoio.hpp:204
pub const CAP_PROP_CODEC_EXTRADATA_INDEX: i32 = 68;
// CAP_PROP_CODEC_PIXEL_FORMAT /usr/include/opencv2/videoio.hpp:182
pub const CAP_PROP_CODEC_PIXEL_FORMAT: i32 = 46;
// CAP_PROP_CONTRAST /usr/include/opencv2/videoio.hpp:147
pub const CAP_PROP_CONTRAST: i32 = 11;
// CAP_PROP_CONVERT_RGB /usr/include/opencv2/videoio.hpp:152
pub const CAP_PROP_CONVERT_RGB: i32 = 16;
// CAP_PROP_DC1394_MAX /usr/include/opencv2/videoio.hpp:271
pub const CAP_PROP_DC1394_MAX: i32 = 31;
// CAP_PROP_DC1394_MODE_AUTO /usr/include/opencv2/videoio.hpp:269
pub const CAP_PROP_DC1394_MODE_AUTO: i32 = -2;
// CAP_PROP_DC1394_MODE_MANUAL /usr/include/opencv2/videoio.hpp:268
pub const CAP_PROP_DC1394_MODE_MANUAL: i32 = -3;
// CAP_PROP_DC1394_MODE_ONE_PUSH_AUTO /usr/include/opencv2/videoio.hpp:270
pub const CAP_PROP_DC1394_MODE_ONE_PUSH_AUTO: i32 = -1;
// CAP_PROP_DC1394_OFF /usr/include/opencv2/videoio.hpp:267
pub const CAP_PROP_DC1394_OFF: i32 = -4;
// CAP_PROP_EXPOSURE /usr/include/opencv2/videoio.hpp:151
pub const CAP_PROP_EXPOSURE: i32 = 15;
// CAP_PROP_EXPOSUREPROGRAM /usr/include/opencv2/videoio.hpp:636
pub const CAP_PROP_EXPOSUREPROGRAM: i32 = 17009;
// CAP_PROP_FOCUS /usr/include/opencv2/videoio.hpp:165
pub const CAP_PROP_FOCUS: i32 = 28;
// CAP_PROP_FORMAT /usr/include/opencv2/videoio.hpp:143
pub const CAP_PROP_FORMAT: i32 = 8;
// CAP_PROP_FOURCC /usr/include/opencv2/videoio.hpp:141
pub const CAP_PROP_FOURCC: i32 = 6;
// CAP_PROP_FPS /usr/include/opencv2/videoio.hpp:140
pub const CAP_PROP_FPS: i32 = 5;
// CAP_PROP_FRAME_COUNT /usr/include/opencv2/videoio.hpp:142
pub const CAP_PROP_FRAME_COUNT: i32 = 7;
// CAP_PROP_FRAME_HEIGHT /usr/include/opencv2/videoio.hpp:139
pub const CAP_PROP_FRAME_HEIGHT: i32 = 4;
// CAP_PROP_FRAME_WIDTH /usr/include/opencv2/videoio.hpp:138
pub const CAP_PROP_FRAME_WIDTH: i32 = 3;
// CAP_PROP_GAIN /usr/include/opencv2/videoio.hpp:150
pub const CAP_PROP_GAIN: i32 = 14;
// CAP_PROP_GAMMA /usr/include/opencv2/videoio.hpp:159
pub const CAP_PROP_GAMMA: i32 = 22;
// CAP_PROP_GIGA_FRAME_HEIGH_MAX /usr/include/opencv2/videoio.hpp:584
pub const CAP_PROP_GIGA_FRAME_HEIGH_MAX: i32 = 10004;
// CAP_PROP_GIGA_FRAME_OFFSET_X /usr/include/opencv2/videoio.hpp:581
pub const CAP_PROP_GIGA_FRAME_OFFSET_X: i32 = 10001;
// CAP_PROP_GIGA_FRAME_OFFSET_Y /usr/include/opencv2/videoio.hpp:582
pub const CAP_PROP_GIGA_FRAME_OFFSET_Y: i32 = 10002;
// CAP_PROP_GIGA_FRAME_SENS_HEIGH /usr/include/opencv2/videoio.hpp:586
pub const CAP_PROP_GIGA_FRAME_SENS_HEIGH: i32 = 10006;
// CAP_PROP_GIGA_FRAME_SENS_WIDTH /usr/include/opencv2/videoio.hpp:585
pub const CAP_PROP_GIGA_FRAME_SENS_WIDTH: i32 = 10005;
// CAP_PROP_GIGA_FRAME_WIDTH_MAX /usr/include/opencv2/videoio.hpp:583
pub const CAP_PROP_GIGA_FRAME_WIDTH_MAX: i32 = 10003;
// CAP_PROP_GPHOTO2_COLLECT_MSGS /usr/include/opencv2/videoio.hpp:632
pub const CAP_PROP_GPHOTO2_COLLECT_MSGS: i32 = 17005;
// CAP_PROP_GPHOTO2_FLUSH_MSGS /usr/include/opencv2/videoio.hpp:633
pub const CAP_PROP_GPHOTO2_FLUSH_MSGS: i32 = 17006;
// CAP_PROP_GPHOTO2_PREVIEW /usr/include/opencv2/videoio.hpp:628
pub const CAP_PROP_GPHOTO2_PREVIEW: i32 = 17001;
// CAP_PROP_GPHOTO2_RELOAD_CONFIG /usr/include/opencv2/videoio.hpp:630
pub const CAP_PROP_GPHOTO2_RELOAD_CONFIG: i32 = 17003;
// CAP_PROP_GPHOTO2_RELOAD_ON_CHANGE /usr/include/opencv2/videoio.hpp:631
pub const CAP_PROP_GPHOTO2_RELOAD_ON_CHANGE: i32 = 17004;
// CAP_PROP_GPHOTO2_WIDGET_ENUMERATE /usr/include/opencv2/videoio.hpp:629
pub const CAP_PROP_GPHOTO2_WIDGET_ENUMERATE: i32 = 17002;
// CAP_PROP_GSTREAMER_QUEUE_LENGTH /usr/include/opencv2/videoio.hpp:343
pub const CAP_PROP_GSTREAMER_QUEUE_LENGTH: i32 = 200;
// CAP_PROP_GUID /usr/include/opencv2/videoio.hpp:166
pub const CAP_PROP_GUID: i32 = 29;
// CAP_PROP_HUE /usr/include/opencv2/videoio.hpp:149
pub const CAP_PROP_HUE: i32 = 13;
// CAP_PROP_HW_ACCELERATION /usr/include/opencv2/videoio.hpp:186
pub const CAP_PROP_HW_ACCELERATION: i32 = 50;
// CAP_PROP_HW_ACCELERATION_USE_OPENCL /usr/include/opencv2/videoio.hpp:188
pub const CAP_PROP_HW_ACCELERATION_USE_OPENCL: i32 = 52;
// CAP_PROP_HW_DEVICE /usr/include/opencv2/videoio.hpp:187
pub const CAP_PROP_HW_DEVICE: i32 = 51;
// CAP_PROP_IMAGES_BASE /usr/include/opencv2/videoio.hpp:650
pub const CAP_PROP_IMAGES_BASE: i32 = 18000;
// CAP_PROP_IMAGES_LAST /usr/include/opencv2/videoio.hpp:651
pub const CAP_PROP_IMAGES_LAST: i32 = 19000;
// CAP_PROP_INTELPERC_DEPTH_CONFIDENCE_THRESHOLD /usr/include/opencv2/videoio.hpp:598
pub const CAP_PROP_INTELPERC_DEPTH_CONFIDENCE_THRESHOLD: i32 = 11005;
// CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_HORZ /usr/include/opencv2/videoio.hpp:599
pub const CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_HORZ: i32 = 11006;
// CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_VERT /usr/include/opencv2/videoio.hpp:600
pub const CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_VERT: i32 = 11007;
// CAP_PROP_INTELPERC_DEPTH_LOW_CONFIDENCE_VALUE /usr/include/opencv2/videoio.hpp:596
pub const CAP_PROP_INTELPERC_DEPTH_LOW_CONFIDENCE_VALUE: i32 = 11003;
// CAP_PROP_INTELPERC_DEPTH_SATURATION_VALUE /usr/include/opencv2/videoio.hpp:597
pub const CAP_PROP_INTELPERC_DEPTH_SATURATION_VALUE: i32 = 11004;
// CAP_PROP_INTELPERC_PROFILE_COUNT /usr/include/opencv2/videoio.hpp:594
pub const CAP_PROP_INTELPERC_PROFILE_COUNT: i32 = 11001;
// CAP_PROP_INTELPERC_PROFILE_IDX /usr/include/opencv2/videoio.hpp:595
pub const CAP_PROP_INTELPERC_PROFILE_IDX: i32 = 11002;
// CAP_PROP_IOS_DEVICE_EXPOSURE /usr/include/opencv2/videoio.hpp:566
pub const CAP_PROP_IOS_DEVICE_EXPOSURE: i32 = 9002;
// CAP_PROP_IOS_DEVICE_FLASH /usr/include/opencv2/videoio.hpp:567
pub const CAP_PROP_IOS_DEVICE_FLASH: i32 = 9003;
// CAP_PROP_IOS_DEVICE_FOCUS /usr/include/opencv2/videoio.hpp:565
pub const CAP_PROP_IOS_DEVICE_FOCUS: i32 = 9001;
// CAP_PROP_IOS_DEVICE_TORCH /usr/include/opencv2/videoio.hpp:569
pub const CAP_PROP_IOS_DEVICE_TORCH: i32 = 9005;
// CAP_PROP_IOS_DEVICE_WHITEBALANCE /usr/include/opencv2/videoio.hpp:568
pub const CAP_PROP_IOS_DEVICE_WHITEBALANCE: i32 = 9004;
// CAP_PROP_IRIS /usr/include/opencv2/videoio.hpp:172
pub const CAP_PROP_IRIS: i32 = 36;
// CAP_PROP_ISO_SPEED /usr/include/opencv2/videoio.hpp:167
pub const CAP_PROP_ISO_SPEED: i32 = 30;
// CAP_PROP_LRF_HAS_KEY_FRAME /usr/include/opencv2/videoio.hpp:203
pub const CAP_PROP_LRF_HAS_KEY_FRAME: i32 = 67;
// CAP_PROP_MODE /usr/include/opencv2/videoio.hpp:145
pub const CAP_PROP_MODE: i32 = 9;
// CAP_PROP_MONOCHROME /usr/include/opencv2/videoio.hpp:156
pub const CAP_PROP_MONOCHROME: i32 = 19;
// CAP_PROP_OPENNI2_MIRROR /usr/include/opencv2/videoio.hpp:302
pub const CAP_PROP_OPENNI2_MIRROR: i32 = 111;
// CAP_PROP_OPENNI2_SYNC /usr/include/opencv2/videoio.hpp:301
pub const CAP_PROP_OPENNI2_SYNC: i32 = 110;
// CAP_PROP_OPENNI_APPROX_FRAME_SYNC /usr/include/opencv2/videoio.hpp:296
pub const CAP_PROP_OPENNI_APPROX_FRAME_SYNC: i32 = 105;
// CAP_PROP_OPENNI_BASELINE /usr/include/opencv2/videoio.hpp:290
pub const CAP_PROP_OPENNI_BASELINE: i32 = 102;
// CAP_PROP_OPENNI_CIRCLE_BUFFER /usr/include/opencv2/videoio.hpp:298
pub const CAP_PROP_OPENNI_CIRCLE_BUFFER: i32 = 107;
// CAP_PROP_OPENNI_FOCAL_LENGTH /usr/include/opencv2/videoio.hpp:291
pub const CAP_PROP_OPENNI_FOCAL_LENGTH: i32 = 103;
// CAP_PROP_OPENNI_FRAME_MAX_DEPTH /usr/include/opencv2/videoio.hpp:289
pub const CAP_PROP_OPENNI_FRAME_MAX_DEPTH: i32 = 101;
// CAP_PROP_OPENNI_GENERATOR_PRESENT /usr/include/opencv2/videoio.hpp:300
pub const CAP_PROP_OPENNI_GENERATOR_PRESENT: i32 = 109;
// CAP_PROP_OPENNI_MAX_BUFFER_SIZE /usr/include/opencv2/videoio.hpp:297
pub const CAP_PROP_OPENNI_MAX_BUFFER_SIZE: i32 = 106;
// CAP_PROP_OPENNI_MAX_TIME_DURATION /usr/include/opencv2/videoio.hpp:299
pub const CAP_PROP_OPENNI_MAX_TIME_DURATION: i32 = 108;
// CAP_PROP_OPENNI_OUTPUT_MODE /usr/include/opencv2/videoio.hpp:288
pub const CAP_PROP_OPENNI_OUTPUT_MODE: i32 = 100;
// CAP_PROP_OPENNI_REGISTRATION /usr/include/opencv2/videoio.hpp:292
pub const CAP_PROP_OPENNI_REGISTRATION: i32 = 104;
// CAP_PROP_OPENNI_REGISTRATION_ON /usr/include/opencv2/videoio.hpp:295
pub const CAP_PROP_OPENNI_REGISTRATION_ON: i32 = 104;
// CAP_PROP_OPEN_TIMEOUT_MSEC /usr/include/opencv2/videoio.hpp:189
pub const CAP_PROP_OPEN_TIMEOUT_MSEC: i32 = 53;
// CAP_PROP_ORIENTATION_AUTO /usr/include/opencv2/videoio.hpp:185
pub const CAP_PROP_ORIENTATION_AUTO: i32 = 49;
// CAP_PROP_ORIENTATION_META /usr/include/opencv2/videoio.hpp:184
pub const CAP_PROP_ORIENTATION_META: i32 = 48;
// CAP_PROP_PAN /usr/include/opencv2/videoio.hpp:169
pub const CAP_PROP_PAN: i32 = 33;
// CAP_PROP_POS_AVI_RATIO /usr/include/opencv2/videoio.hpp:137
pub const CAP_PROP_POS_AVI_RATIO: i32 = 2;
// CAP_PROP_POS_FRAMES /usr/include/opencv2/videoio.hpp:136
pub const CAP_PROP_POS_FRAMES: i32 = 1;
// CAP_PROP_POS_MSEC /usr/include/opencv2/videoio.hpp:135
pub const CAP_PROP_POS_MSEC: i32 = 0;
// CAP_PROP_PVAPI_BINNINGX /usr/include/opencv2/videoio.hpp:357
pub const CAP_PROP_PVAPI_BINNINGX: i32 = 304;
// CAP_PROP_PVAPI_BINNINGY /usr/include/opencv2/videoio.hpp:358
pub const CAP_PROP_PVAPI_BINNINGY: i32 = 305;
// CAP_PROP_PVAPI_DECIMATIONHORIZONTAL /usr/include/opencv2/videoio.hpp:355
pub const CAP_PROP_PVAPI_DECIMATIONHORIZONTAL: i32 = 302;
// CAP_PROP_PVAPI_DECIMATIONVERTICAL /usr/include/opencv2/videoio.hpp:356
pub const CAP_PROP_PVAPI_DECIMATIONVERTICAL: i32 = 303;
// CAP_PROP_PVAPI_FRAMESTARTTRIGGERMODE /usr/include/opencv2/videoio.hpp:354
pub const CAP_PROP_PVAPI_FRAMESTARTTRIGGERMODE: i32 = 301;
// CAP_PROP_PVAPI_MULTICASTIP /usr/include/opencv2/videoio.hpp:353
pub const CAP_PROP_PVAPI_MULTICASTIP: i32 = 300;
// CAP_PROP_PVAPI_PIXELFORMAT /usr/include/opencv2/videoio.hpp:359
pub const CAP_PROP_PVAPI_PIXELFORMAT: i32 = 306;
// CAP_PROP_READ_TIMEOUT_MSEC /usr/include/opencv2/videoio.hpp:190
pub const CAP_PROP_READ_TIMEOUT_MSEC: i32 = 54;
// CAP_PROP_RECTIFICATION /usr/include/opencv2/videoio.hpp:155
pub const CAP_PROP_RECTIFICATION: i32 = 18;
// CAP_PROP_ROLL /usr/include/opencv2/videoio.hpp:171
pub const CAP_PROP_ROLL: i32 = 35;
// CAP_PROP_SAR_DEN /usr/include/opencv2/videoio.hpp:177
pub const CAP_PROP_SAR_DEN: i32 = 41;
// CAP_PROP_SAR_NUM /usr/include/opencv2/videoio.hpp:176
pub const CAP_PROP_SAR_NUM: i32 = 40;
// CAP_PROP_SATURATION /usr/include/opencv2/videoio.hpp:148
pub const CAP_PROP_SATURATION: i32 = 12;
// CAP_PROP_SETTINGS /usr/include/opencv2/videoio.hpp:173
pub const CAP_PROP_SETTINGS: i32 = 37;
// CAP_PROP_SHARPNESS /usr/include/opencv2/videoio.hpp:157
pub const CAP_PROP_SHARPNESS: i32 = 20;
// CAP_PROP_SPEED /usr/include/opencv2/videoio.hpp:634
pub const CAP_PROP_SPEED: i32 = 17007;
// CAP_PROP_STREAM_OPEN_TIME_USEC /usr/include/opencv2/videoio.hpp:191
pub const CAP_PROP_STREAM_OPEN_TIME_USEC: i32 = 55;
// CAP_PROP_TEMPERATURE /usr/include/opencv2/videoio.hpp:160
pub const CAP_PROP_TEMPERATURE: i32 = 23;
// CAP_PROP_TILT /usr/include/opencv2/videoio.hpp:170
pub const CAP_PROP_TILT: i32 = 34;
// CAP_PROP_TRIGGER /usr/include/opencv2/videoio.hpp:161
pub const CAP_PROP_TRIGGER: i32 = 24;
// CAP_PROP_TRIGGER_DELAY /usr/include/opencv2/videoio.hpp:162
pub const CAP_PROP_TRIGGER_DELAY: i32 = 25;
// CAP_PROP_VIDEO_STREAM /usr/include/opencv2/videoio.hpp:193
pub const CAP_PROP_VIDEO_STREAM: i32 = 57;
// CAP_PROP_VIDEO_TOTAL_CHANNELS /usr/include/opencv2/videoio.hpp:192
pub const CAP_PROP_VIDEO_TOTAL_CHANNELS: i32 = 56;
// CAP_PROP_VIEWFINDER /usr/include/opencv2/videoio.hpp:637
pub const CAP_PROP_VIEWFINDER: i32 = 17010;
// CAP_PROP_WB_TEMPERATURE /usr/include/opencv2/videoio.hpp:181
pub const CAP_PROP_WB_TEMPERATURE: i32 = 45;
// CAP_PROP_WHITE_BALANCE_BLUE_U /usr/include/opencv2/videoio.hpp:154
pub const CAP_PROP_WHITE_BALANCE_BLUE_U: i32 = 17;
// CAP_PROP_WHITE_BALANCE_RED_V /usr/include/opencv2/videoio.hpp:163
pub const CAP_PROP_WHITE_BALANCE_RED_V: i32 = 26;
// CAP_PROP_XI_ACQ_BUFFER_SIZE /usr/include/opencv2/videoio.hpp:518
pub const CAP_PROP_XI_ACQ_BUFFER_SIZE: i32 = 548;
// CAP_PROP_XI_ACQ_BUFFER_SIZE_UNIT /usr/include/opencv2/videoio.hpp:519
pub const CAP_PROP_XI_ACQ_BUFFER_SIZE_UNIT: i32 = 549;
// CAP_PROP_XI_ACQ_FRAME_BURST_COUNT /usr/include/opencv2/videoio.hpp:484
pub const CAP_PROP_XI_ACQ_FRAME_BURST_COUNT: i32 = 499;
// CAP_PROP_XI_ACQ_TIMING_MODE /usr/include/opencv2/videoio.hpp:508
pub const CAP_PROP_XI_ACQ_TIMING_MODE: i32 = 538;
// CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_COMMIT /usr/include/opencv2/videoio.hpp:522
pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_COMMIT: i32 = 552;
// CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_SIZE /usr/include/opencv2/videoio.hpp:520
pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_SIZE: i32 = 550;
// CAP_PROP_XI_AEAG /usr/include/opencv2/videoio.hpp:410
pub const CAP_PROP_XI_AEAG: i32 = 415;
// CAP_PROP_XI_AEAG_LEVEL /usr/include/opencv2/videoio.hpp:414
pub const CAP_PROP_XI_AEAG_LEVEL: i32 = 419;
// CAP_PROP_XI_AEAG_ROI_HEIGHT /usr/include/opencv2/videoio.hpp:437
pub const CAP_PROP_XI_AEAG_ROI_HEIGHT: i32 = 442;
// CAP_PROP_XI_AEAG_ROI_OFFSET_X /usr/include/opencv2/videoio.hpp:434
pub const CAP_PROP_XI_AEAG_ROI_OFFSET_X: i32 = 439;
// CAP_PROP_XI_AEAG_ROI_OFFSET_Y /usr/include/opencv2/videoio.hpp:435
pub const CAP_PROP_XI_AEAG_ROI_OFFSET_Y: i32 = 440;
// CAP_PROP_XI_AEAG_ROI_WIDTH /usr/include/opencv2/videoio.hpp:436
pub const CAP_PROP_XI_AEAG_ROI_WIDTH: i32 = 441;
// CAP_PROP_XI_AE_MAX_LIMIT /usr/include/opencv2/videoio.hpp:412
pub const CAP_PROP_XI_AE_MAX_LIMIT: i32 = 417;
// CAP_PROP_XI_AG_MAX_LIMIT /usr/include/opencv2/videoio.hpp:413
pub const CAP_PROP_XI_AG_MAX_LIMIT: i32 = 418;
// CAP_PROP_XI_APPLY_CMS /usr/include/opencv2/videoio.hpp:460
pub const CAP_PROP_XI_APPLY_CMS: i32 = 471;
// CAP_PROP_XI_AUTO_BANDWIDTH_CALCULATION /usr/include/opencv2/videoio.hpp:537
pub const CAP_PROP_XI_AUTO_BANDWIDTH_CALCULATION: i32 = 573;
// CAP_PROP_XI_AUTO_WB /usr/include/opencv2/videoio.hpp:409
pub const CAP_PROP_XI_AUTO_WB: i32 = 414;
// CAP_PROP_XI_AVAILABLE_BANDWIDTH /usr/include/opencv2/videoio.hpp:509
pub const CAP_PROP_XI_AVAILABLE_BANDWIDTH: i32 = 539;
// CAP_PROP_XI_BINNING_HORIZONTAL /usr/include/opencv2/videoio.hpp:423
pub const CAP_PROP_XI_BINNING_HORIZONTAL: i32 = 429;
// CAP_PROP_XI_BINNING_PATTERN /usr/include/opencv2/videoio.hpp:424
pub const CAP_PROP_XI_BINNING_PATTERN: i32 = 430;
// CAP_PROP_XI_BINNING_SELECTOR /usr/include/opencv2/videoio.hpp:421
pub const CAP_PROP_XI_BINNING_SELECTOR: i32 = 427;
// CAP_PROP_XI_BINNING_VERTICAL /usr/include/opencv2/videoio.hpp:422
pub const CAP_PROP_XI_BINNING_VERTICAL: i32 = 428;
// CAP_PROP_XI_BPC /usr/include/opencv2/videoio.hpp:438
pub const CAP_PROP_XI_BPC: i32 = 445;
// CAP_PROP_XI_BUFFERS_QUEUE_SIZE /usr/include/opencv2/videoio.hpp:521
pub const CAP_PROP_XI_BUFFERS_QUEUE_SIZE: i32 = 551;
// CAP_PROP_XI_BUFFER_POLICY /usr/include/opencv2/videoio.hpp:510
pub const CAP_PROP_XI_BUFFER_POLICY: i32 = 540;
// CAP_PROP_XI_CC_MATRIX_00 /usr/include/opencv2/videoio.hpp:466
pub const CAP_PROP_XI_CC_MATRIX_00: i32 = 479;
// CAP_PROP_XI_CC_MATRIX_01 /usr/include/opencv2/videoio.hpp:467
pub const CAP_PROP_XI_CC_MATRIX_01: i32 = 480;
// CAP_PROP_XI_CC_MATRIX_02 /usr/include/opencv2/videoio.hpp:468
pub const CAP_PROP_XI_CC_MATRIX_02: i32 = 481;
// CAP_PROP_XI_CC_MATRIX_03 /usr/include/opencv2/videoio.hpp:469
pub const CAP_PROP_XI_CC_MATRIX_03: i32 = 482;
// CAP_PROP_XI_CC_MATRIX_10 /usr/include/opencv2/videoio.hpp:470
pub const CAP_PROP_XI_CC_MATRIX_10: i32 = 483;
// CAP_PROP_XI_CC_MATRIX_11 /usr/include/opencv2/videoio.hpp:471
pub const CAP_PROP_XI_CC_MATRIX_11: i32 = 484;
// CAP_PROP_XI_CC_MATRIX_12 /usr/include/opencv2/videoio.hpp:472
pub const CAP_PROP_XI_CC_MATRIX_12: i32 = 485;
// CAP_PROP_XI_CC_MATRIX_13 /usr/include/opencv2/videoio.hpp:473
pub const CAP_PROP_XI_CC_MATRIX_13: i32 = 486;
// CAP_PROP_XI_CC_MATRIX_20 /usr/include/opencv2/videoio.hpp:474
pub const CAP_PROP_XI_CC_MATRIX_20: i32 = 487;
// CAP_PROP_XI_CC_MATRIX_21 /usr/include/opencv2/videoio.hpp:475
pub const CAP_PROP_XI_CC_MATRIX_21: i32 = 488;
// CAP_PROP_XI_CC_MATRIX_22 /usr/include/opencv2/videoio.hpp:476
pub const CAP_PROP_XI_CC_MATRIX_22: i32 = 489;
// CAP_PROP_XI_CC_MATRIX_23 /usr/include/opencv2/videoio.hpp:477
pub const CAP_PROP_XI_CC_MATRIX_23: i32 = 490;
// CAP_PROP_XI_CC_MATRIX_30 /usr/include/opencv2/videoio.hpp:478
pub const CAP_PROP_XI_CC_MATRIX_30: i32 = 491;
// CAP_PROP_XI_CC_MATRIX_31 /usr/include/opencv2/videoio.hpp:479
pub const CAP_PROP_XI_CC_MATRIX_31: i32 = 492;
// CAP_PROP_XI_CC_MATRIX_32 /usr/include/opencv2/videoio.hpp:480
pub const CAP_PROP_XI_CC_MATRIX_32: i32 = 493;
// CAP_PROP_XI_CC_MATRIX_33 /usr/include/opencv2/videoio.hpp:481
pub const CAP_PROP_XI_CC_MATRIX_33: i32 = 494;
// CAP_PROP_XI_CHIP_TEMP /usr/include/opencv2/videoio.hpp:455
pub const CAP_PROP_XI_CHIP_TEMP: i32 = 468;
// CAP_PROP_XI_CMS /usr/include/opencv2/videoio.hpp:459
pub const CAP_PROP_XI_CMS: i32 = 470;
// CAP_PROP_XI_COLOR_FILTER_ARRAY /usr/include/opencv2/videoio.hpp:462
pub const CAP_PROP_XI_COLOR_FILTER_ARRAY: i32 = 475;
// CAP_PROP_XI_COLUMN_FPN_CORRECTION /usr/include/opencv2/videoio.hpp:525
pub const CAP_PROP_XI_COLUMN_FPN_CORRECTION: i32 = 555;
// CAP_PROP_XI_COOLING /usr/include/opencv2/videoio.hpp:453
pub const CAP_PROP_XI_COOLING: i32 = 466;
// CAP_PROP_XI_COUNTER_SELECTOR /usr/include/opencv2/videoio.hpp:506
pub const CAP_PROP_XI_COUNTER_SELECTOR: i32 = 536;
// CAP_PROP_XI_COUNTER_VALUE /usr/include/opencv2/videoio.hpp:507
pub const CAP_PROP_XI_COUNTER_VALUE: i32 = 537;
// CAP_PROP_XI_DATA_FORMAT /usr/include/opencv2/videoio.hpp:396
pub const CAP_PROP_XI_DATA_FORMAT: i32 = 401;
// CAP_PROP_XI_DEBOUNCE_EN /usr/include/opencv2/videoio.hpp:485
pub const CAP_PROP_XI_DEBOUNCE_EN: i32 = 507;
// CAP_PROP_XI_DEBOUNCE_POL /usr/include/opencv2/videoio.hpp:488
pub const CAP_PROP_XI_DEBOUNCE_POL: i32 = 510;
// CAP_PROP_XI_DEBOUNCE_T0 /usr/include/opencv2/videoio.hpp:486
pub const CAP_PROP_XI_DEBOUNCE_T0: i32 = 508;
// CAP_PROP_XI_DEBOUNCE_T1 /usr/include/opencv2/videoio.hpp:487
pub const CAP_PROP_XI_DEBOUNCE_T1: i32 = 509;
// CAP_PROP_XI_DEBUG_LEVEL /usr/include/opencv2/videoio.hpp:536
pub const CAP_PROP_XI_DEBUG_LEVEL: i32 = 572;
// CAP_PROP_XI_DECIMATION_HORIZONTAL /usr/include/opencv2/videoio.hpp:427
pub const CAP_PROP_XI_DECIMATION_HORIZONTAL: i32 = 433;
// CAP_PROP_XI_DECIMATION_PATTERN /usr/include/opencv2/videoio.hpp:428
pub const CAP_PROP_XI_DECIMATION_PATTERN: i32 = 434;
// CAP_PROP_XI_DECIMATION_SELECTOR /usr/include/opencv2/videoio.hpp:425
pub const CAP_PROP_XI_DECIMATION_SELECTOR: i32 = 431;
// CAP_PROP_XI_DECIMATION_VERTICAL /usr/include/opencv2/videoio.hpp:426
pub const CAP_PROP_XI_DECIMATION_VERTICAL: i32 = 432;
// CAP_PROP_XI_DEFAULT_CC_MATRIX /usr/include/opencv2/videoio.hpp:482
pub const CAP_PROP_XI_DEFAULT_CC_MATRIX: i32 = 495;
// CAP_PROP_XI_DEVICE_MODEL_ID /usr/include/opencv2/videoio.hpp:497
pub const CAP_PROP_XI_DEVICE_MODEL_ID: i32 = 521;
// CAP_PROP_XI_DEVICE_RESET /usr/include/opencv2/videoio.hpp:524
pub const CAP_PROP_XI_DEVICE_RESET: i32 = 554;
// CAP_PROP_XI_DEVICE_SN /usr/include/opencv2/videoio.hpp:498
pub const CAP_PROP_XI_DEVICE_SN: i32 = 522;
// CAP_PROP_XI_DOWNSAMPLING /usr/include/opencv2/videoio.hpp:395
pub const CAP_PROP_XI_DOWNSAMPLING: i32 = 400;
// CAP_PROP_XI_DOWNSAMPLING_TYPE /usr/include/opencv2/videoio.hpp:420
pub const CAP_PROP_XI_DOWNSAMPLING_TYPE: i32 = 426;
// CAP_PROP_XI_EXPOSURE /usr/include/opencv2/videoio.hpp:416
pub const CAP_PROP_XI_EXPOSURE: i32 = 421;
// CAP_PROP_XI_EXPOSURE_BURST_COUNT /usr/include/opencv2/videoio.hpp:417
pub const CAP_PROP_XI_EXPOSURE_BURST_COUNT: i32 = 422;
// CAP_PROP_XI_EXP_PRIORITY /usr/include/opencv2/videoio.hpp:411
pub const CAP_PROP_XI_EXP_PRIORITY: i32 = 416;
// CAP_PROP_XI_FFS_ACCESS_KEY /usr/include/opencv2/videoio.hpp:542
pub const CAP_PROP_XI_FFS_ACCESS_KEY: i32 = 583;
// CAP_PROP_XI_FFS_FILE_ID /usr/include/opencv2/videoio.hpp:538
pub const CAP_PROP_XI_FFS_FILE_ID: i32 = 594;
// CAP_PROP_XI_FFS_FILE_SIZE /usr/include/opencv2/videoio.hpp:539
pub const CAP_PROP_XI_FFS_FILE_SIZE: i32 = 580;
// CAP_PROP_XI_FRAMERATE /usr/include/opencv2/videoio.hpp:505
pub const CAP_PROP_XI_FRAMERATE: i32 = 535;
// CAP_PROP_XI_FREE_FFS_SIZE /usr/include/opencv2/videoio.hpp:540
pub const CAP_PROP_XI_FREE_FFS_SIZE: i32 = 581;
// CAP_PROP_XI_GAIN /usr/include/opencv2/videoio.hpp:419
pub const CAP_PROP_XI_GAIN: i32 = 424;
// CAP_PROP_XI_GAIN_SELECTOR /usr/include/opencv2/videoio.hpp:418
pub const CAP_PROP_XI_GAIN_SELECTOR: i32 = 423;
// CAP_PROP_XI_GAMMAC /usr/include/opencv2/videoio.hpp:464
pub const CAP_PROP_XI_GAMMAC: i32 = 477;
// CAP_PROP_XI_GAMMAY /usr/include/opencv2/videoio.hpp:463
pub const CAP_PROP_XI_GAMMAY: i32 = 476;
// CAP_PROP_XI_GPI_LEVEL /usr/include/opencv2/videoio.hpp:403
pub const CAP_PROP_XI_GPI_LEVEL: i32 = 408;
// CAP_PROP_XI_GPI_MODE /usr/include/opencv2/videoio.hpp:402
pub const CAP_PROP_XI_GPI_MODE: i32 = 407;
// CAP_PROP_XI_GPI_SELECTOR /usr/include/opencv2/videoio.hpp:401
pub const CAP_PROP_XI_GPI_SELECTOR: i32 = 406;
// CAP_PROP_XI_GPO_MODE /usr/include/opencv2/videoio.hpp:405
pub const CAP_PROP_XI_GPO_MODE: i32 = 410;
// CAP_PROP_XI_GPO_SELECTOR /usr/include/opencv2/videoio.hpp:404
pub const CAP_PROP_XI_GPO_SELECTOR: i32 = 409;
// CAP_PROP_XI_HDR /usr/include/opencv2/videoio.hpp:528
pub const CAP_PROP_XI_HDR: i32 = 559;
// CAP_PROP_XI_HDR_KNEEPOINT_COUNT /usr/include/opencv2/videoio.hpp:529
pub const CAP_PROP_XI_HDR_KNEEPOINT_COUNT: i32 = 560;
// CAP_PROP_XI_HDR_T1 /usr/include/opencv2/videoio.hpp:530
pub const CAP_PROP_XI_HDR_T1: i32 = 561;
// CAP_PROP_XI_HDR_T2 /usr/include/opencv2/videoio.hpp:531
pub const CAP_PROP_XI_HDR_T2: i32 = 562;
// CAP_PROP_XI_HEIGHT /usr/include/opencv2/videoio.hpp:443
pub const CAP_PROP_XI_HEIGHT: i32 = 452;
// CAP_PROP_XI_HOUS_BACK_SIDE_TEMP /usr/include/opencv2/videoio.hpp:457
pub const CAP_PROP_XI_HOUS_BACK_SIDE_TEMP: i32 = 590;
// CAP_PROP_XI_HOUS_TEMP /usr/include/opencv2/videoio.hpp:456
pub const CAP_PROP_XI_HOUS_TEMP: i32 = 469;
// CAP_PROP_XI_HW_REVISION /usr/include/opencv2/videoio.hpp:535
pub const CAP_PROP_XI_HW_REVISION: i32 = 571;
// CAP_PROP_XI_IMAGE_BLACK_LEVEL /usr/include/opencv2/videoio.hpp:534
pub const CAP_PROP_XI_IMAGE_BLACK_LEVEL: i32 = 565;
// CAP_PROP_XI_IMAGE_DATA_BIT_DEPTH /usr/include/opencv2/videoio.hpp:449
pub const CAP_PROP_XI_IMAGE_DATA_BIT_DEPTH: i32 = 462;
// CAP_PROP_XI_IMAGE_DATA_FORMAT /usr/include/opencv2/videoio.hpp:431
pub const CAP_PROP_XI_IMAGE_DATA_FORMAT: i32 = 435;
// CAP_PROP_XI_IMAGE_DATA_FORMAT_RGB32_ALPHA /usr/include/opencv2/videoio.hpp:499
pub const CAP_PROP_XI_IMAGE_DATA_FORMAT_RGB32_ALPHA: i32 = 529;
// CAP_PROP_XI_IMAGE_IS_COLOR /usr/include/opencv2/videoio.hpp:461
pub const CAP_PROP_XI_IMAGE_IS_COLOR: i32 = 474;
// CAP_PROP_XI_IMAGE_PAYLOAD_SIZE /usr/include/opencv2/videoio.hpp:500
pub const CAP_PROP_XI_IMAGE_PAYLOAD_SIZE: i32 = 530;
// CAP_PROP_XI_IS_COOLED /usr/include/opencv2/videoio.hpp:452
pub const CAP_PROP_XI_IS_COOLED: i32 = 465;
// CAP_PROP_XI_IS_DEVICE_EXIST /usr/include/opencv2/videoio.hpp:517
pub const CAP_PROP_XI_IS_DEVICE_EXIST: i32 = 547;
// CAP_PROP_XI_KNEEPOINT1 /usr/include/opencv2/videoio.hpp:532
pub const CAP_PROP_XI_KNEEPOINT1: i32 = 563;
// CAP_PROP_XI_KNEEPOINT2 /usr/include/opencv2/videoio.hpp:533
pub const CAP_PROP_XI_KNEEPOINT2: i32 = 564;
// CAP_PROP_XI_LED_MODE /usr/include/opencv2/videoio.hpp:407
pub const CAP_PROP_XI_LED_MODE: i32 = 412;
// CAP_PROP_XI_LED_SELECTOR /usr/include/opencv2/videoio.hpp:406
pub const CAP_PROP_XI_LED_SELECTOR: i32 = 411;
// CAP_PROP_XI_LENS_APERTURE_VALUE /usr/include/opencv2/videoio.hpp:490
pub const CAP_PROP_XI_LENS_APERTURE_VALUE: i32 = 512;
// CAP_PROP_XI_LENS_FEATURE /usr/include/opencv2/videoio.hpp:496
pub const CAP_PROP_XI_LENS_FEATURE: i32 = 518;
// CAP_PROP_XI_LENS_FEATURE_SELECTOR /usr/include/opencv2/videoio.hpp:495
pub const CAP_PROP_XI_LENS_FEATURE_SELECTOR: i32 = 517;
// CAP_PROP_XI_LENS_FOCAL_LENGTH /usr/include/opencv2/videoio.hpp:494
pub const CAP_PROP_XI_LENS_FOCAL_LENGTH: i32 = 516;
// CAP_PROP_XI_LENS_FOCUS_DISTANCE /usr/include/opencv2/videoio.hpp:493
pub const CAP_PROP_XI_LENS_FOCUS_DISTANCE: i32 = 515;
// CAP_PROP_XI_LENS_FOCUS_MOVE /usr/include/opencv2/videoio.hpp:492
pub const CAP_PROP_XI_LENS_FOCUS_MOVE: i32 = 514;
// CAP_PROP_XI_LENS_FOCUS_MOVEMENT_VALUE /usr/include/opencv2/videoio.hpp:491
pub const CAP_PROP_XI_LENS_FOCUS_MOVEMENT_VALUE: i32 = 513;
// CAP_PROP_XI_LENS_MODE /usr/include/opencv2/videoio.hpp:489
pub const CAP_PROP_XI_LENS_MODE: i32 = 511;
// CAP_PROP_XI_LIMIT_BANDWIDTH /usr/include/opencv2/videoio.hpp:446
pub const CAP_PROP_XI_LIMIT_BANDWIDTH: i32 = 459;
// CAP_PROP_XI_LUT_EN /usr/include/opencv2/videoio.hpp:511
pub const CAP_PROP_XI_LUT_EN: i32 = 541;
// CAP_PROP_XI_LUT_INDEX /usr/include/opencv2/videoio.hpp:512
pub const CAP_PROP_XI_LUT_INDEX: i32 = 542;
// CAP_PROP_XI_LUT_VALUE /usr/include/opencv2/videoio.hpp:513
pub const CAP_PROP_XI_LUT_VALUE: i32 = 543;
// CAP_PROP_XI_MANUAL_WB /usr/include/opencv2/videoio.hpp:408
pub const CAP_PROP_XI_MANUAL_WB: i32 = 413;
// CAP_PROP_XI_OFFSET_X /usr/include/opencv2/videoio.hpp:397
pub const CAP_PROP_XI_OFFSET_X: i32 = 402;
// CAP_PROP_XI_OFFSET_Y /usr/include/opencv2/videoio.hpp:398
pub const CAP_PROP_XI_OFFSET_Y: i32 = 403;
// CAP_PROP_XI_OUTPUT_DATA_BIT_DEPTH /usr/include/opencv2/videoio.hpp:448
pub const CAP_PROP_XI_OUTPUT_DATA_BIT_DEPTH: i32 = 461;
// CAP_PROP_XI_OUTPUT_DATA_PACKING /usr/include/opencv2/videoio.hpp:450
pub const CAP_PROP_XI_OUTPUT_DATA_PACKING: i32 = 463;
// CAP_PROP_XI_OUTPUT_DATA_PACKING_TYPE /usr/include/opencv2/videoio.hpp:451
pub const CAP_PROP_XI_OUTPUT_DATA_PACKING_TYPE: i32 = 464;
// CAP_PROP_XI_RECENT_FRAME /usr/include/opencv2/videoio.hpp:523
pub const CAP_PROP_XI_RECENT_FRAME: i32 = 553;
// CAP_PROP_XI_REGION_MODE /usr/include/opencv2/videoio.hpp:445
pub const CAP_PROP_XI_REGION_MODE: i32 = 595;
// CAP_PROP_XI_REGION_SELECTOR /usr/include/opencv2/videoio.hpp:444
pub const CAP_PROP_XI_REGION_SELECTOR: i32 = 589;
// CAP_PROP_XI_ROW_FPN_CORRECTION /usr/include/opencv2/videoio.hpp:526
pub const CAP_PROP_XI_ROW_FPN_CORRECTION: i32 = 591;
// CAP_PROP_XI_SENSOR_BOARD_TEMP /usr/include/opencv2/videoio.hpp:458
pub const CAP_PROP_XI_SENSOR_BOARD_TEMP: i32 = 596;
// CAP_PROP_XI_SENSOR_CLOCK_FREQ_HZ /usr/include/opencv2/videoio.hpp:502
pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_HZ: i32 = 532;
// CAP_PROP_XI_SENSOR_CLOCK_FREQ_INDEX /usr/include/opencv2/videoio.hpp:503
pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_INDEX: i32 = 533;
// CAP_PROP_XI_SENSOR_DATA_BIT_DEPTH /usr/include/opencv2/videoio.hpp:447
pub const CAP_PROP_XI_SENSOR_DATA_BIT_DEPTH: i32 = 460;
// CAP_PROP_XI_SENSOR_FEATURE_SELECTOR /usr/include/opencv2/videoio.hpp:543
pub const CAP_PROP_XI_SENSOR_FEATURE_SELECTOR: i32 = 585;
// CAP_PROP_XI_SENSOR_FEATURE_VALUE /usr/include/opencv2/videoio.hpp:544
pub const CAP_PROP_XI_SENSOR_FEATURE_VALUE: i32 = 586;
// CAP_PROP_XI_SENSOR_MODE /usr/include/opencv2/videoio.hpp:527
pub const CAP_PROP_XI_SENSOR_MODE: i32 = 558;
// CAP_PROP_XI_SENSOR_OUTPUT_CHANNEL_COUNT /usr/include/opencv2/videoio.hpp:504
pub const CAP_PROP_XI_SENSOR_OUTPUT_CHANNEL_COUNT: i32 = 534;
// CAP_PROP_XI_SENSOR_TAPS /usr/include/opencv2/videoio.hpp:433
pub const CAP_PROP_XI_SENSOR_TAPS: i32 = 437;
// CAP_PROP_XI_SHARPNESS /usr/include/opencv2/videoio.hpp:465
pub const CAP_PROP_XI_SHARPNESS: i32 = 478;
// CAP_PROP_XI_SHUTTER_TYPE /usr/include/opencv2/videoio.hpp:432
pub const CAP_PROP_XI_SHUTTER_TYPE: i32 = 436;
// CAP_PROP_XI_TARGET_TEMP /usr/include/opencv2/videoio.hpp:454
pub const CAP_PROP_XI_TARGET_TEMP: i32 = 467;
// CAP_PROP_XI_TEST_PATTERN /usr/include/opencv2/videoio.hpp:430
pub const CAP_PROP_XI_TEST_PATTERN: i32 = 588;
// CAP_PROP_XI_TEST_PATTERN_GENERATOR_SELECTOR /usr/include/opencv2/videoio.hpp:429
pub const CAP_PROP_XI_TEST_PATTERN_GENERATOR_SELECTOR: i32 = 587;
// CAP_PROP_XI_TIMEOUT /usr/include/opencv2/videoio.hpp:415
pub const CAP_PROP_XI_TIMEOUT: i32 = 420;
// CAP_PROP_XI_TRANSPORT_PIXEL_FORMAT /usr/include/opencv2/videoio.hpp:501
pub const CAP_PROP_XI_TRANSPORT_PIXEL_FORMAT: i32 = 531;
// CAP_PROP_XI_TRG_DELAY /usr/include/opencv2/videoio.hpp:514
pub const CAP_PROP_XI_TRG_DELAY: i32 = 544;
// CAP_PROP_XI_TRG_SELECTOR /usr/include/opencv2/videoio.hpp:483
pub const CAP_PROP_XI_TRG_SELECTOR: i32 = 498;
// CAP_PROP_XI_TRG_SOFTWARE /usr/include/opencv2/videoio.hpp:400
pub const CAP_PROP_XI_TRG_SOFTWARE: i32 = 405;
// CAP_PROP_XI_TRG_SOURCE /usr/include/opencv2/videoio.hpp:399
pub const CAP_PROP_XI_TRG_SOURCE: i32 = 404;
// CAP_PROP_XI_TS_RST_MODE /usr/include/opencv2/videoio.hpp:515
pub const CAP_PROP_XI_TS_RST_MODE: i32 = 545;
// CAP_PROP_XI_TS_RST_SOURCE /usr/include/opencv2/videoio.hpp:516
pub const CAP_PROP_XI_TS_RST_SOURCE: i32 = 546;
// CAP_PROP_XI_USED_FFS_SIZE /usr/include/opencv2/videoio.hpp:541
pub const CAP_PROP_XI_USED_FFS_SIZE: i32 = 582;
// CAP_PROP_XI_WB_KB /usr/include/opencv2/videoio.hpp:441
pub const CAP_PROP_XI_WB_KB: i32 = 450;
// CAP_PROP_XI_WB_KG /usr/include/opencv2/videoio.hpp:440
pub const CAP_PROP_XI_WB_KG: i32 = 449;
// CAP_PROP_XI_WB_KR /usr/include/opencv2/videoio.hpp:439
pub const CAP_PROP_XI_WB_KR: i32 = 448;
// CAP_PROP_XI_WIDTH /usr/include/opencv2/videoio.hpp:442
pub const CAP_PROP_XI_WIDTH: i32 = 451;
// CAP_PROP_ZOOM /usr/include/opencv2/videoio.hpp:164
pub const CAP_PROP_ZOOM: i32 = 27;
// CAP_PVAPI /usr/include/opencv2/videoio.hpp:103
pub const CAP_PVAPI: i32 = 800;
// CAP_PVAPI_DECIMATION_2OUTOF16 /usr/include/opencv2/videoio.hpp:374
pub const CAP_PVAPI_DECIMATION_2OUTOF16: i32 = 8;
// CAP_PVAPI_DECIMATION_2OUTOF4 /usr/include/opencv2/videoio.hpp:372
pub const CAP_PVAPI_DECIMATION_2OUTOF4: i32 = 2;
// CAP_PVAPI_DECIMATION_2OUTOF8 /usr/include/opencv2/videoio.hpp:373
pub const CAP_PVAPI_DECIMATION_2OUTOF8: i32 = 4;
// CAP_PVAPI_DECIMATION_OFF /usr/include/opencv2/videoio.hpp:371
pub const CAP_PVAPI_DECIMATION_OFF: i32 = 1;
// CAP_PVAPI_FSTRIGMODE_FIXEDRATE /usr/include/opencv2/videoio.hpp:366
pub const CAP_PVAPI_FSTRIGMODE_FIXEDRATE: i32 = 3;
// CAP_PVAPI_FSTRIGMODE_FREERUN /usr/include/opencv2/videoio.hpp:363
pub const CAP_PVAPI_FSTRIGMODE_FREERUN: i32 = 0;
// CAP_PVAPI_FSTRIGMODE_SOFTWARE /usr/include/opencv2/videoio.hpp:367
pub const CAP_PVAPI_FSTRIGMODE_SOFTWARE: i32 = 4;
// CAP_PVAPI_FSTRIGMODE_SYNCIN1 /usr/include/opencv2/videoio.hpp:364
pub const CAP_PVAPI_FSTRIGMODE_SYNCIN1: i32 = 1;
// CAP_PVAPI_FSTRIGMODE_SYNCIN2 /usr/include/opencv2/videoio.hpp:365
pub const CAP_PVAPI_FSTRIGMODE_SYNCIN2: i32 = 2;
// CAP_PVAPI_PIXELFORMAT_BAYER16 /usr/include/opencv2/videoio.hpp:381
pub const CAP_PVAPI_PIXELFORMAT_BAYER16: i32 = 4;
// CAP_PVAPI_PIXELFORMAT_BAYER8 /usr/include/opencv2/videoio.hpp:380
pub const CAP_PVAPI_PIXELFORMAT_BAYER8: i32 = 3;
// CAP_PVAPI_PIXELFORMAT_BGR24 /usr/include/opencv2/videoio.hpp:383
pub const CAP_PVAPI_PIXELFORMAT_BGR24: i32 = 6;
// CAP_PVAPI_PIXELFORMAT_BGRA32 /usr/include/opencv2/videoio.hpp:385
pub const CAP_PVAPI_PIXELFORMAT_BGRA32: i32 = 8;
// CAP_PVAPI_PIXELFORMAT_MONO16 /usr/include/opencv2/videoio.hpp:379
pub const CAP_PVAPI_PIXELFORMAT_MONO16: i32 = 2;
// CAP_PVAPI_PIXELFORMAT_MONO8 /usr/include/opencv2/videoio.hpp:378
pub const CAP_PVAPI_PIXELFORMAT_MONO8: i32 = 1;
// CAP_PVAPI_PIXELFORMAT_RGB24 /usr/include/opencv2/videoio.hpp:382
pub const CAP_PVAPI_PIXELFORMAT_RGB24: i32 = 5;
// CAP_PVAPI_PIXELFORMAT_RGBA32 /usr/include/opencv2/videoio.hpp:384
pub const CAP_PVAPI_PIXELFORMAT_RGBA32: i32 = 7;
// CAP_QT /usr/include/opencv2/videoio.hpp:100
pub const CAP_QT: i32 = 500;
// CAP_REALSENSE /usr/include/opencv2/videoio.hpp:113
pub const CAP_REALSENSE: i32 = 1500;
// CAP_UEYE /usr/include/opencv2/videoio.hpp:125
pub const CAP_UEYE: i32 = 2500;
// CAP_UNICAP /usr/include/opencv2/videoio.hpp:101
pub const CAP_UNICAP: i32 = 600;
// CAP_V4L /usr/include/opencv2/videoio.hpp:93
pub const CAP_V4L: i32 = 200;
// CAP_V4L2 /usr/include/opencv2/videoio.hpp:94
pub const CAP_V4L2: i32 = 200;
// CAP_VFW /usr/include/opencv2/videoio.hpp:92
pub const CAP_VFW: i32 = 200;
// CAP_WINRT /usr/include/opencv2/videoio.hpp:111
pub const CAP_WINRT: i32 = 1410;
// CAP_XIAPI /usr/include/opencv2/videoio.hpp:107
pub const CAP_XIAPI: i32 = 1100;
// CAP_XINE /usr/include/opencv2/videoio.hpp:124
pub const CAP_XINE: i32 = 2400;
// CV__CAP_PROP_LATEST /usr/include/opencv2/videoio.hpp:206
pub const CV__CAP_PROP_LATEST: i32 = 69;
// CV__VIDEOWRITER_PROP_LATEST /usr/include/opencv2/videoio.hpp:224
pub const CV__VIDEOWRITER_PROP_LATEST: i32 = 9;
// VIDEOWRITER_PROP_DEPTH /usr/include/opencv2/videoio.hpp:219
pub const VIDEOWRITER_PROP_DEPTH: i32 = 5;
// VIDEOWRITER_PROP_FRAMEBYTES /usr/include/opencv2/videoio.hpp:215
pub const VIDEOWRITER_PROP_FRAMEBYTES: i32 = 2;
// VIDEOWRITER_PROP_HW_ACCELERATION /usr/include/opencv2/videoio.hpp:220
pub const VIDEOWRITER_PROP_HW_ACCELERATION: i32 = 6;
// VIDEOWRITER_PROP_HW_ACCELERATION_USE_OPENCL /usr/include/opencv2/videoio.hpp:222
pub const VIDEOWRITER_PROP_HW_ACCELERATION_USE_OPENCL: i32 = 8;
// VIDEOWRITER_PROP_HW_DEVICE /usr/include/opencv2/videoio.hpp:221
pub const VIDEOWRITER_PROP_HW_DEVICE: i32 = 7;
// VIDEOWRITER_PROP_IS_COLOR /usr/include/opencv2/videoio.hpp:217
pub const VIDEOWRITER_PROP_IS_COLOR: i32 = 4;
// VIDEOWRITER_PROP_NSTRIPES /usr/include/opencv2/videoio.hpp:216
pub const VIDEOWRITER_PROP_NSTRIPES: i32 = 3;
// VIDEOWRITER_PROP_QUALITY /usr/include/opencv2/videoio.hpp:214
pub const VIDEOWRITER_PROP_QUALITY: i32 = 1;
// VIDEO_ACCELERATION_ANY /usr/include/opencv2/videoio.hpp:248
pub const VIDEO_ACCELERATION_ANY: i32 = 1;
// VIDEO_ACCELERATION_D3D11 /usr/include/opencv2/videoio.hpp:252
pub const VIDEO_ACCELERATION_D3D11: i32 = 2;
// VIDEO_ACCELERATION_MFX /usr/include/opencv2/videoio.hpp:254
pub const VIDEO_ACCELERATION_MFX: i32 = 4;
// VIDEO_ACCELERATION_NONE /usr/include/opencv2/videoio.hpp:245
pub const VIDEO_ACCELERATION_NONE: i32 = 0;
// VIDEO_ACCELERATION_VAAPI /usr/include/opencv2/videoio.hpp:253
pub const VIDEO_ACCELERATION_VAAPI: i32 = 3;
// VideoAccelerationType /usr/include/opencv2/videoio.hpp:243
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VideoAccelerationType {
	VIDEO_ACCELERATION_NONE = 0,
	VIDEO_ACCELERATION_ANY = 1,
	VIDEO_ACCELERATION_D3D11 = 2,
	VIDEO_ACCELERATION_VAAPI = 3,
	VIDEO_ACCELERATION_MFX = 4,
}

opencv_type_enum! { crate::videoio::VideoAccelerationType }

// VideoCaptureAPIs /usr/include/opencv2/videoio.hpp:90
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VideoCaptureAPIs {
	CAP_ANY = 0,
	CAP_VFW = 200,
	// CAP_V4L = 200 as isize, // duplicate discriminant
	// CAP_V4L2 = 200 as isize, // duplicate discriminant
	CAP_FIREWIRE = 300,
	// CAP_FIREWARE = 300 as isize, // duplicate discriminant
	// CAP_IEEE1394 = 300 as isize, // duplicate discriminant
	// CAP_DC1394 = 300 as isize, // duplicate discriminant
	// CAP_CMU1394 = 300 as isize, // duplicate discriminant
	CAP_QT = 500,
	CAP_UNICAP = 600,
	CAP_DSHOW = 700,
	CAP_PVAPI = 800,
	CAP_OPENNI = 900,
	CAP_OPENNI_ASUS = 910,
	CAP_ANDROID = 1000,
	CAP_XIAPI = 1100,
	CAP_AVFOUNDATION = 1200,
	CAP_GIGANETIX = 1300,
	CAP_MSMF = 1400,
	CAP_WINRT = 1410,
	CAP_INTELPERC = 1500,
	// CAP_REALSENSE = 1500 as isize, // duplicate discriminant
	CAP_OPENNI2 = 1600,
	CAP_OPENNI2_ASUS = 1610,
	CAP_OPENNI2_ASTRA = 1620,
	CAP_GPHOTO2 = 1700,
	CAP_GSTREAMER = 1800,
	CAP_FFMPEG = 1900,
	CAP_IMAGES = 2000,
	CAP_ARAVIS = 2100,
	CAP_OPENCV_MJPEG = 2200,
	CAP_INTEL_MFX = 2300,
	CAP_XINE = 2400,
	CAP_UEYE = 2500,
}

opencv_type_enum! { crate::videoio::VideoCaptureAPIs }

// VideoCaptureProperties /usr/include/opencv2/videoio.hpp:134
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VideoCaptureProperties {
	CAP_PROP_POS_MSEC = 0,
	CAP_PROP_POS_FRAMES = 1,
	CAP_PROP_POS_AVI_RATIO = 2,
	CAP_PROP_FRAME_WIDTH = 3,
	CAP_PROP_FRAME_HEIGHT = 4,
	CAP_PROP_FPS = 5,
	CAP_PROP_FOURCC = 6,
	CAP_PROP_FRAME_COUNT = 7,
	CAP_PROP_FORMAT = 8,
	CAP_PROP_MODE = 9,
	CAP_PROP_BRIGHTNESS = 10,
	CAP_PROP_CONTRAST = 11,
	CAP_PROP_SATURATION = 12,
	CAP_PROP_HUE = 13,
	CAP_PROP_GAIN = 14,
	CAP_PROP_EXPOSURE = 15,
	CAP_PROP_CONVERT_RGB = 16,
	CAP_PROP_WHITE_BALANCE_BLUE_U = 17,
	CAP_PROP_RECTIFICATION = 18,
	CAP_PROP_MONOCHROME = 19,
	CAP_PROP_SHARPNESS = 20,
	CAP_PROP_AUTO_EXPOSURE = 21,
	CAP_PROP_GAMMA = 22,
	CAP_PROP_TEMPERATURE = 23,
	CAP_PROP_TRIGGER = 24,
	CAP_PROP_TRIGGER_DELAY = 25,
	CAP_PROP_WHITE_BALANCE_RED_V = 26,
	CAP_PROP_ZOOM = 27,
	CAP_PROP_FOCUS = 28,
	CAP_PROP_GUID = 29,
	CAP_PROP_ISO_SPEED = 30,
	CAP_PROP_BACKLIGHT = 32,
	CAP_PROP_PAN = 33,
	CAP_PROP_TILT = 34,
	CAP_PROP_ROLL = 35,
	CAP_PROP_IRIS = 36,
	CAP_PROP_SETTINGS = 37,
	CAP_PROP_BUFFERSIZE = 38,
	CAP_PROP_AUTOFOCUS = 39,
	CAP_PROP_SAR_NUM = 40,
	CAP_PROP_SAR_DEN = 41,
	CAP_PROP_BACKEND = 42,
	CAP_PROP_CHANNEL = 43,
	CAP_PROP_AUTO_WB = 44,
	CAP_PROP_WB_TEMPERATURE = 45,
	CAP_PROP_CODEC_PIXEL_FORMAT = 46,
	CAP_PROP_BITRATE = 47,
	CAP_PROP_ORIENTATION_META = 48,
	CAP_PROP_ORIENTATION_AUTO = 49,
	CAP_PROP_HW_ACCELERATION = 50,
	CAP_PROP_HW_DEVICE = 51,
	CAP_PROP_HW_ACCELERATION_USE_OPENCL = 52,
	CAP_PROP_OPEN_TIMEOUT_MSEC = 53,
	CAP_PROP_READ_TIMEOUT_MSEC = 54,
	CAP_PROP_STREAM_OPEN_TIME_USEC = 55,
	CAP_PROP_VIDEO_TOTAL_CHANNELS = 56,
	CAP_PROP_VIDEO_STREAM = 57,
	CAP_PROP_AUDIO_STREAM = 58,
	CAP_PROP_AUDIO_POS = 59,
	CAP_PROP_AUDIO_SHIFT_NSEC = 60,
	CAP_PROP_AUDIO_DATA_DEPTH = 61,
	CAP_PROP_AUDIO_SAMPLES_PER_SECOND = 62,
	CAP_PROP_AUDIO_BASE_INDEX = 63,
	CAP_PROP_AUDIO_TOTAL_CHANNELS = 64,
	CAP_PROP_AUDIO_TOTAL_STREAMS = 65,
	CAP_PROP_AUDIO_SYNCHRONIZE = 66,
	CAP_PROP_LRF_HAS_KEY_FRAME = 67,
	CAP_PROP_CODEC_EXTRADATA_INDEX = 68,
	CV__CAP_PROP_LATEST = 69,
}

opencv_type_enum! { crate::videoio::VideoCaptureProperties }

// VideoWriterProperties /usr/include/opencv2/videoio.hpp:213
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VideoWriterProperties {
	VIDEOWRITER_PROP_QUALITY = 1,
	VIDEOWRITER_PROP_FRAMEBYTES = 2,
	VIDEOWRITER_PROP_NSTRIPES = 3,
	VIDEOWRITER_PROP_IS_COLOR = 4,
	VIDEOWRITER_PROP_DEPTH = 5,
	VIDEOWRITER_PROP_HW_ACCELERATION = 6,
	VIDEOWRITER_PROP_HW_DEVICE = 7,
	VIDEOWRITER_PROP_HW_ACCELERATION_USE_OPENCL = 8,
	CV__VIDEOWRITER_PROP_LATEST = 9,
}

opencv_type_enum! { crate::videoio::VideoWriterProperties }

// getBackendName(cv::VideoCaptureAPIs) /usr/include/opencv2/videoio/registry.hpp:27
#[inline]
pub fn get_backend_name(api: crate::videoio::VideoCaptureAPIs) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getBackendName_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getBackends() /usr/include/opencv2/videoio/registry.hpp:30
#[inline]
pub fn get_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

// getCameraBackendPluginVersion(cv::VideoCaptureAPIs, int &, int &) /usr/include/opencv2/videoio/registry.hpp:48
#[inline]
pub fn get_camera_backend_plugin_version(api: crate::videoio::VideoCaptureAPIs, version_abi: &mut i32, version_api: &mut i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getCameraBackendPluginVersion_VideoCaptureAPIs_intR_intR(api, version_abi, version_api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getCameraBackends() /usr/include/opencv2/videoio/registry.hpp:33
#[inline]
pub fn get_camera_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getCameraBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

// getStreamBackendPluginVersion(cv::VideoCaptureAPIs, int &, int &) /usr/include/opencv2/videoio/registry.hpp:55
#[inline]
pub fn get_stream_backend_plugin_version(api: crate::videoio::VideoCaptureAPIs, version_abi: &mut i32, version_api: &mut i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getStreamBackendPluginVersion_VideoCaptureAPIs_intR_intR(api, version_abi, version_api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getStreamBackends() /usr/include/opencv2/videoio/registry.hpp:36
#[inline]
pub fn get_stream_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getStreamBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

// getWriterBackendPluginVersion(cv::VideoCaptureAPIs, int &, int &) /usr/include/opencv2/videoio/registry.hpp:62
#[inline]
pub fn get_writer_backend_plugin_version(api: crate::videoio::VideoCaptureAPIs, version_abi: &mut i32, version_api: &mut i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getWriterBackendPluginVersion_VideoCaptureAPIs_intR_intR(api, version_abi, version_api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getWriterBackends() /usr/include/opencv2/videoio/registry.hpp:39
#[inline]
pub fn get_writer_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getWriterBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

// hasBackend(cv::VideoCaptureAPIs) /usr/include/opencv2/videoio/registry.hpp:42
#[inline]
pub fn has_backend(api: crate::videoio::VideoCaptureAPIs) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_hasBackend_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isBackendBuiltIn(cv::VideoCaptureAPIs) /usr/include/opencv2/videoio/registry.hpp:45
#[inline]
pub fn is_backend_built_in(api: crate::videoio::VideoCaptureAPIs) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_isBackendBuiltIn_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// VideoCapture /usr/include/opencv2/videoio.hpp:682
pub trait VideoCaptureTraitConst {
	fn as_raw_VideoCapture(&self) -> *const c_void;

	// isOpened() /usr/include/opencv2/videoio.hpp:796
	#[inline]
	fn is_opened(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_isOpened_const(self.as_raw_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// get(int) /usr/include/opencv2/videoio.hpp:900
	#[inline]
	fn get(&self, prop_id: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_get_const_int(self.as_raw_VideoCapture(), prop_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getBackendName() /usr/include/opencv2/videoio.hpp:906
	#[inline]
	fn get_backend_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_getBackendName_const(self.as_raw_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait VideoCaptureTrait: crate::videoio::VideoCaptureTraitConst {
	fn as_raw_mut_VideoCapture(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * api_preference: CAP_ANY
	// open(const cv::String &, int) /usr/include/opencv2/videoio.hpp:752
	#[inline]
	fn open_file(&mut self, filename: &str, api_preference: i32) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_const_StringR_int(self.as_raw_mut_VideoCapture(), filename.opencv_as_extern(), api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// open(const cv::String &, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:765
	#[inline]
	fn open(&mut self, filename: &str, api_preference: i32, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_const_StringR_int_const_vector_int_R(self.as_raw_mut_VideoCapture(), filename.opencv_as_extern(), api_preference, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * api_preference: CAP_ANY
	// open(int, int) /usr/include/opencv2/videoio.hpp:776
	#[inline]
	fn open_1(&mut self, index: i32, api_preference: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_int_int(self.as_raw_mut_VideoCapture(), index, api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// open(int, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:789
	#[inline]
	fn open_2(&mut self, index: i32, api_preference: i32, params: &core::Vector<i32>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_int_int_const_vector_int_R(self.as_raw_mut_VideoCapture(), index, api_preference, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/videoio.hpp:805
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_release(self.as_raw_mut_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// grab() /usr/include/opencv2/videoio.hpp:826
	#[inline]
	fn grab(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_grab(self.as_raw_mut_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flag: 0
	// retrieve(cv::OutputArray, int) /usr/include/opencv2/videoio.hpp:844
	#[inline]
	fn retrieve(&mut self, image: &mut dyn core::ToOutputArray, flag: i32) -> Result<bool> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_retrieve_const__OutputArrayR_int(self.as_raw_mut_VideoCapture(), image.as_raw__OutputArray(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// read(cv::OutputArray) /usr/include/opencv2/videoio.hpp:870
	#[inline]
	fn read(&mut self, image: &mut dyn core::ToOutputArray) -> Result<bool> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_read_const__OutputArrayR(self.as_raw_mut_VideoCapture(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// set(int, double) /usr/include/opencv2/videoio.hpp:881
	#[inline]
	fn set(&mut self, prop_id: i32, value: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_set_int_double(self.as_raw_mut_VideoCapture(), prop_id, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setExceptionMode(bool) /usr/include/opencv2/videoio.hpp:912
	#[inline]
	fn set_exception_mode(&mut self, enable: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_setExceptionMode_bool(self.as_raw_mut_VideoCapture(), enable, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getExceptionMode() /usr/include/opencv2/videoio.hpp:915
	#[inline]
	fn get_exception_mode(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_getExceptionMode(self.as_raw_mut_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// VideoCapture /usr/include/opencv2/videoio.hpp:682
pub struct VideoCapture {
	ptr: *mut c_void
}

opencv_type_boxed! { VideoCapture }

impl Drop for VideoCapture {
	fn drop(&mut self) {
		extern "C" { fn cv_VideoCapture_delete(instance: *mut c_void); }
		unsafe { cv_VideoCapture_delete(self.as_raw_mut_VideoCapture()) };
	}
}

unsafe impl Send for VideoCapture {}

impl crate::videoio::VideoCaptureTraitConst for VideoCapture {
	#[inline] fn as_raw_VideoCapture(&self) -> *const c_void { self.as_raw() }
}

impl crate::videoio::VideoCaptureTrait for VideoCapture {
	#[inline] fn as_raw_mut_VideoCapture(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl VideoCapture {
	// VideoCapture() /usr/include/opencv2/videoio.hpp:690
	#[inline]
	pub fn default() -> Result<crate::videoio::VideoCapture> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * api_preference: CAP_ANY
	// VideoCapture(const cv::String &, int) /usr/include/opencv2/videoio.hpp:707
	#[inline]
	pub fn from_file(filename: &str, api_preference: i32) -> Result<crate::videoio::VideoCapture> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture_const_StringR_int(filename.opencv_as_extern(), api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// VideoCapture(const cv::String &, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:715
	#[inline]
	pub fn from_file_with_params(filename: &str, api_preference: i32, params: &core::Vector<i32>) -> Result<crate::videoio::VideoCapture> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture_const_StringR_int_const_vector_int_R(filename.opencv_as_extern(), api_preference, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * api_preference: CAP_ANY
	// VideoCapture(int, int) /usr/include/opencv2/videoio.hpp:727
	#[inline]
	pub fn new(index: i32, api_preference: i32) -> Result<crate::videoio::VideoCapture> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture_int_int(index, api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// VideoCapture(int, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:735
	#[inline]
	pub fn new_with_params(index: i32, api_preference: i32, params: &core::Vector<i32>) -> Result<crate::videoio::VideoCapture> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture_int_int_const_vector_int_R(index, api_preference, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * timeout_ns: 0
	// waitAny(const std::vector<VideoCapture> &, std::vector<int> &, int64) /usr/include/opencv2/videoio.hpp:933
	#[inline]
	pub fn wait_any(streams: &core::Vector<crate::videoio::VideoCapture>, ready_index: &mut core::Vector<i32>, timeout_ns: i64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_waitAny_const_vector_VideoCapture_R_vector_int_R_int64_t(streams.as_raw_VectorOfVideoCapture(), ready_index.as_raw_mut_VectorOfi32(), timeout_ns, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// VideoWriter /usr/include/opencv2/videoio.hpp:960
pub trait VideoWriterTraitConst {
	fn as_raw_VideoWriter(&self) -> *const c_void;

	// isOpened() /usr/include/opencv2/videoio.hpp:1049
	#[inline]
	fn is_opened(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_isOpened_const(self.as_raw_VideoWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// get(int) /usr/include/opencv2/videoio.hpp:1095
	#[inline]
	fn get(&self, prop_id: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_get_const_int(self.as_raw_VideoWriter(), prop_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getBackendName() /usr/include/opencv2/videoio.hpp:1110
	#[inline]
	fn get_backend_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_getBackendName_const(self.as_raw_VideoWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait VideoWriterTrait: crate::videoio::VideoWriterTraitConst {
	fn as_raw_mut_VideoWriter(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * is_color: true
	// open(const cv::String &, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:1029
	#[inline]
	fn open(&mut self, filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_double_Size_bool(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), fourcc, fps, frame_size.opencv_as_extern(), is_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * is_color: true
	// open(const cv::String &, int, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:1034
	#[inline]
	fn open_with_backend(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), api_preference, fourcc, fps, frame_size.opencv_as_extern(), is_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// open(const cv::String &, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1039
	#[inline]
	fn open_1(&mut self, filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_double_const_SizeR_const_vector_int_R(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// open(const cv::String &, int, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1044
	#[inline]
	fn open_2(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_int_double_const_SizeR_const_vector_int_R(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), api_preference, fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/videoio.hpp:1056
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_release(self.as_raw_mut_VideoWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// write(cv::InputArray) /usr/include/opencv2/videoio.hpp:1075
	#[inline]
	fn write(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_write_const__InputArrayR(self.as_raw_mut_VideoWriter(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// set(int, double) /usr/include/opencv2/videoio.hpp:1085
	#[inline]
	fn set(&mut self, prop_id: i32, value: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_set_int_double(self.as_raw_mut_VideoWriter(), prop_id, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// VideoWriter /usr/include/opencv2/videoio.hpp:960
pub struct VideoWriter {
	ptr: *mut c_void
}

opencv_type_boxed! { VideoWriter }

impl Drop for VideoWriter {
	fn drop(&mut self) {
		extern "C" { fn cv_VideoWriter_delete(instance: *mut c_void); }
		unsafe { cv_VideoWriter_delete(self.as_raw_mut_VideoWriter()) };
	}
}

unsafe impl Send for VideoWriter {}

impl crate::videoio::VideoWriterTraitConst for VideoWriter {
	#[inline] fn as_raw_VideoWriter(&self) -> *const c_void { self.as_raw() }
}

impl crate::videoio::VideoWriterTrait for VideoWriter {
	#[inline] fn as_raw_mut_VideoWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl VideoWriter {
	// VideoWriter() /usr/include/opencv2/videoio.hpp:970
	#[inline]
	pub fn default() -> Result<crate::videoio::VideoWriter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * is_color: true
	// VideoWriter(const cv::String &, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:993
	#[inline]
	pub fn new(filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<crate::videoio::VideoWriter> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(filename.opencv_as_extern(), fourcc, fps, frame_size.opencv_as_extern(), is_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * is_color: true
	// VideoWriter(const cv::String &, int, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:1000
	#[inline]
	pub fn new_with_backend(filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<crate::videoio::VideoWriter> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(filename.opencv_as_extern(), api_preference, fourcc, fps, frame_size.opencv_as_extern(), is_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// VideoWriter(const cv::String &, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1007
	#[inline]
	pub fn new_1(filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<crate::videoio::VideoWriter> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_double_const_SizeR_const_vector_int_R(filename.opencv_as_extern(), fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// VideoWriter(const cv::String &, int, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1012
	#[inline]
	pub fn new_2(filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<crate::videoio::VideoWriter> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_int_double_const_SizeR_const_vector_int_R(filename.opencv_as_extern(), api_preference, fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// fourcc(char, char, char, char) /usr/include/opencv2/videoio.hpp:1104
	#[inline]
	pub fn fourcc(c1: i8, c2: i8, c3: i8, c4: i8) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_fourcc_char_char_char_char(c1, c2, c3, c4, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
