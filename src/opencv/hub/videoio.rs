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

pub const CAP_ANDROID: i32 = 1000;
pub const CAP_ANY: i32 = 0;
pub const CAP_ARAVIS: i32 = 2100;
pub const CAP_AVFOUNDATION: i32 = 1200;
pub const CAP_CMU1394: i32 = 300;
pub const CAP_DC1394: i32 = 300;
pub const CAP_DSHOW: i32 = 700;
pub const CAP_FFMPEG: i32 = 1900;
pub const CAP_FIREWARE: i32 = 300;
pub const CAP_FIREWIRE: i32 = 300;
pub const CAP_GIGANETIX: i32 = 1300;
pub const CAP_GPHOTO2: i32 = 1700;
pub const CAP_GSTREAMER: i32 = 1800;
pub const CAP_IEEE1394: i32 = 300;
pub const CAP_IMAGES: i32 = 2000;
pub const CAP_INTELPERC: i32 = 1500;
pub const CAP_INTELPERC_DEPTH_GENERATOR: i32 = 536870912;
pub const CAP_INTELPERC_DEPTH_MAP: i32 = 0;
pub const CAP_INTELPERC_GENERATORS_MASK: i32 = 939524096;
pub const CAP_INTELPERC_IMAGE: i32 = 3;
pub const CAP_INTELPERC_IMAGE_GENERATOR: i32 = 268435456;
pub const CAP_INTELPERC_IR_GENERATOR: i32 = 134217728;
pub const CAP_INTELPERC_IR_MAP: i32 = 2;
pub const CAP_INTELPERC_UVDEPTH_MAP: i32 = 1;
pub const CAP_INTEL_MFX: i32 = 2300;
pub const CAP_MSMF: i32 = 1400;
pub const CAP_OPENCV_MJPEG: i32 = 2200;
pub const CAP_OPENNI: i32 = 900;
pub const CAP_OPENNI2: i32 = 1600;
pub const CAP_OPENNI2_ASTRA: i32 = 1620;
pub const CAP_OPENNI2_ASUS: i32 = 1610;
pub const CAP_OPENNI_ASUS: i32 = 910;
pub const CAP_OPENNI_BGR_IMAGE: i32 = 5;
pub const CAP_OPENNI_DEPTH_GENERATOR: i32 = -2147483648;
pub const CAP_OPENNI_DEPTH_GENERATOR_BASELINE: i32 = -2147483546;
pub const CAP_OPENNI_DEPTH_GENERATOR_FOCAL_LENGTH: i32 = -2147483545;
pub const CAP_OPENNI_DEPTH_GENERATOR_PRESENT: i32 = -2147483539;
pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION: i32 = -2147483544;
pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION_ON: i32 = -2147483544;
pub const CAP_OPENNI_DEPTH_MAP: i32 = 0;
pub const CAP_OPENNI_DISPARITY_MAP: i32 = 2;
pub const CAP_OPENNI_DISPARITY_MAP_32F: i32 = 3;
pub const CAP_OPENNI_GENERATORS_MASK: i32 = -536870912;
pub const CAP_OPENNI_GRAY_IMAGE: i32 = 6;
pub const CAP_OPENNI_IMAGE_GENERATOR: i32 = 1073741824;
pub const CAP_OPENNI_IMAGE_GENERATOR_OUTPUT_MODE: i32 = 1073741924;
pub const CAP_OPENNI_IMAGE_GENERATOR_PRESENT: i32 = 1073741933;
pub const CAP_OPENNI_IR_GENERATOR: i32 = 536870912;
pub const CAP_OPENNI_IR_GENERATOR_PRESENT: i32 = 536871021;
pub const CAP_OPENNI_IR_IMAGE: i32 = 7;
pub const CAP_OPENNI_POINT_CLOUD_MAP: i32 = 1;
pub const CAP_OPENNI_QVGA_30HZ: i32 = 3;
pub const CAP_OPENNI_QVGA_60HZ: i32 = 4;
pub const CAP_OPENNI_SXGA_15HZ: i32 = 1;
pub const CAP_OPENNI_SXGA_30HZ: i32 = 2;
pub const CAP_OPENNI_VALID_DEPTH_MASK: i32 = 4;
pub const CAP_OPENNI_VGA_30HZ: i32 = 0;
pub const CAP_PROP_APERTURE: i32 = 17008;
pub const CAP_PROP_ARAVIS_AUTOTRIGGER: i32 = 600;
pub const CAP_PROP_AUDIO_BASE_INDEX: i32 = 63;
pub const CAP_PROP_AUDIO_DATA_DEPTH: i32 = 61;
pub const CAP_PROP_AUDIO_POS: i32 = 59;
pub const CAP_PROP_AUDIO_SAMPLES_PER_SECOND: i32 = 62;
pub const CAP_PROP_AUDIO_SHIFT_NSEC: i32 = 60;
pub const CAP_PROP_AUDIO_STREAM: i32 = 58;
pub const CAP_PROP_AUDIO_SYNCHRONIZE: i32 = 66;
pub const CAP_PROP_AUDIO_TOTAL_CHANNELS: i32 = 64;
pub const CAP_PROP_AUDIO_TOTAL_STREAMS: i32 = 65;
pub const CAP_PROP_AUTOFOCUS: i32 = 39;
pub const CAP_PROP_AUTO_EXPOSURE: i32 = 21;
pub const CAP_PROP_AUTO_WB: i32 = 44;
pub const CAP_PROP_BACKEND: i32 = 42;
pub const CAP_PROP_BACKLIGHT: i32 = 32;
pub const CAP_PROP_BITRATE: i32 = 47;
pub const CAP_PROP_BRIGHTNESS: i32 = 10;
pub const CAP_PROP_BUFFERSIZE: i32 = 38;
pub const CAP_PROP_CHANNEL: i32 = 43;
pub const CAP_PROP_CODEC_EXTRADATA_INDEX: i32 = 68;
pub const CAP_PROP_CODEC_PIXEL_FORMAT: i32 = 46;
pub const CAP_PROP_CONTRAST: i32 = 11;
pub const CAP_PROP_CONVERT_RGB: i32 = 16;
pub const CAP_PROP_DC1394_MAX: i32 = 31;
pub const CAP_PROP_DC1394_MODE_AUTO: i32 = -2;
pub const CAP_PROP_DC1394_MODE_MANUAL: i32 = -3;
pub const CAP_PROP_DC1394_MODE_ONE_PUSH_AUTO: i32 = -1;
pub const CAP_PROP_DC1394_OFF: i32 = -4;
pub const CAP_PROP_EXPOSURE: i32 = 15;
pub const CAP_PROP_EXPOSUREPROGRAM: i32 = 17009;
pub const CAP_PROP_FOCUS: i32 = 28;
pub const CAP_PROP_FORMAT: i32 = 8;
pub const CAP_PROP_FOURCC: i32 = 6;
pub const CAP_PROP_FPS: i32 = 5;
pub const CAP_PROP_FRAME_COUNT: i32 = 7;
pub const CAP_PROP_FRAME_HEIGHT: i32 = 4;
pub const CAP_PROP_FRAME_WIDTH: i32 = 3;
pub const CAP_PROP_GAIN: i32 = 14;
pub const CAP_PROP_GAMMA: i32 = 22;
pub const CAP_PROP_GIGA_FRAME_HEIGH_MAX: i32 = 10004;
pub const CAP_PROP_GIGA_FRAME_OFFSET_X: i32 = 10001;
pub const CAP_PROP_GIGA_FRAME_OFFSET_Y: i32 = 10002;
pub const CAP_PROP_GIGA_FRAME_SENS_HEIGH: i32 = 10006;
pub const CAP_PROP_GIGA_FRAME_SENS_WIDTH: i32 = 10005;
pub const CAP_PROP_GIGA_FRAME_WIDTH_MAX: i32 = 10003;
pub const CAP_PROP_GPHOTO2_COLLECT_MSGS: i32 = 17005;
pub const CAP_PROP_GPHOTO2_FLUSH_MSGS: i32 = 17006;
pub const CAP_PROP_GPHOTO2_PREVIEW: i32 = 17001;
pub const CAP_PROP_GPHOTO2_RELOAD_CONFIG: i32 = 17003;
pub const CAP_PROP_GPHOTO2_RELOAD_ON_CHANGE: i32 = 17004;
pub const CAP_PROP_GPHOTO2_WIDGET_ENUMERATE: i32 = 17002;
pub const CAP_PROP_GSTREAMER_QUEUE_LENGTH: i32 = 200;
pub const CAP_PROP_GUID: i32 = 29;
pub const CAP_PROP_HUE: i32 = 13;
pub const CAP_PROP_HW_ACCELERATION: i32 = 50;
pub const CAP_PROP_HW_ACCELERATION_USE_OPENCL: i32 = 52;
pub const CAP_PROP_HW_DEVICE: i32 = 51;
pub const CAP_PROP_IMAGES_BASE: i32 = 18000;
pub const CAP_PROP_IMAGES_LAST: i32 = 19000;
pub const CAP_PROP_INTELPERC_DEPTH_CONFIDENCE_THRESHOLD: i32 = 11005;
pub const CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_HORZ: i32 = 11006;
pub const CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_VERT: i32 = 11007;
pub const CAP_PROP_INTELPERC_DEPTH_LOW_CONFIDENCE_VALUE: i32 = 11003;
pub const CAP_PROP_INTELPERC_DEPTH_SATURATION_VALUE: i32 = 11004;
pub const CAP_PROP_INTELPERC_PROFILE_COUNT: i32 = 11001;
pub const CAP_PROP_INTELPERC_PROFILE_IDX: i32 = 11002;
pub const CAP_PROP_IOS_DEVICE_EXPOSURE: i32 = 9002;
pub const CAP_PROP_IOS_DEVICE_FLASH: i32 = 9003;
pub const CAP_PROP_IOS_DEVICE_FOCUS: i32 = 9001;
pub const CAP_PROP_IOS_DEVICE_TORCH: i32 = 9005;
pub const CAP_PROP_IOS_DEVICE_WHITEBALANCE: i32 = 9004;
pub const CAP_PROP_IRIS: i32 = 36;
pub const CAP_PROP_ISO_SPEED: i32 = 30;
pub const CAP_PROP_LRF_HAS_KEY_FRAME: i32 = 67;
pub const CAP_PROP_MODE: i32 = 9;
pub const CAP_PROP_MONOCHROME: i32 = 19;
pub const CAP_PROP_OPENNI2_MIRROR: i32 = 111;
pub const CAP_PROP_OPENNI2_SYNC: i32 = 110;
pub const CAP_PROP_OPENNI_APPROX_FRAME_SYNC: i32 = 105;
pub const CAP_PROP_OPENNI_BASELINE: i32 = 102;
pub const CAP_PROP_OPENNI_CIRCLE_BUFFER: i32 = 107;
pub const CAP_PROP_OPENNI_FOCAL_LENGTH: i32 = 103;
pub const CAP_PROP_OPENNI_FRAME_MAX_DEPTH: i32 = 101;
pub const CAP_PROP_OPENNI_GENERATOR_PRESENT: i32 = 109;
pub const CAP_PROP_OPENNI_MAX_BUFFER_SIZE: i32 = 106;
pub const CAP_PROP_OPENNI_MAX_TIME_DURATION: i32 = 108;
pub const CAP_PROP_OPENNI_OUTPUT_MODE: i32 = 100;
pub const CAP_PROP_OPENNI_REGISTRATION: i32 = 104;
pub const CAP_PROP_OPENNI_REGISTRATION_ON: i32 = 104;
pub const CAP_PROP_OPEN_TIMEOUT_MSEC: i32 = 53;
pub const CAP_PROP_ORIENTATION_AUTO: i32 = 49;
pub const CAP_PROP_ORIENTATION_META: i32 = 48;
pub const CAP_PROP_PAN: i32 = 33;
pub const CAP_PROP_POS_AVI_RATIO: i32 = 2;
pub const CAP_PROP_POS_FRAMES: i32 = 1;
pub const CAP_PROP_POS_MSEC: i32 = 0;
pub const CAP_PROP_PVAPI_BINNINGX: i32 = 304;
pub const CAP_PROP_PVAPI_BINNINGY: i32 = 305;
pub const CAP_PROP_PVAPI_DECIMATIONHORIZONTAL: i32 = 302;
pub const CAP_PROP_PVAPI_DECIMATIONVERTICAL: i32 = 303;
pub const CAP_PROP_PVAPI_FRAMESTARTTRIGGERMODE: i32 = 301;
pub const CAP_PROP_PVAPI_MULTICASTIP: i32 = 300;
pub const CAP_PROP_PVAPI_PIXELFORMAT: i32 = 306;
pub const CAP_PROP_READ_TIMEOUT_MSEC: i32 = 54;
pub const CAP_PROP_RECTIFICATION: i32 = 18;
pub const CAP_PROP_ROLL: i32 = 35;
pub const CAP_PROP_SAR_DEN: i32 = 41;
pub const CAP_PROP_SAR_NUM: i32 = 40;
pub const CAP_PROP_SATURATION: i32 = 12;
pub const CAP_PROP_SETTINGS: i32 = 37;
pub const CAP_PROP_SHARPNESS: i32 = 20;
pub const CAP_PROP_SPEED: i32 = 17007;
pub const CAP_PROP_STREAM_OPEN_TIME_USEC: i32 = 55;
pub const CAP_PROP_TEMPERATURE: i32 = 23;
pub const CAP_PROP_TILT: i32 = 34;
pub const CAP_PROP_TRIGGER: i32 = 24;
pub const CAP_PROP_TRIGGER_DELAY: i32 = 25;
pub const CAP_PROP_VIDEO_STREAM: i32 = 57;
pub const CAP_PROP_VIDEO_TOTAL_CHANNELS: i32 = 56;
pub const CAP_PROP_VIEWFINDER: i32 = 17010;
pub const CAP_PROP_WB_TEMPERATURE: i32 = 45;
pub const CAP_PROP_WHITE_BALANCE_BLUE_U: i32 = 17;
pub const CAP_PROP_WHITE_BALANCE_RED_V: i32 = 26;
pub const CAP_PROP_XI_ACQ_BUFFER_SIZE: i32 = 548;
pub const CAP_PROP_XI_ACQ_BUFFER_SIZE_UNIT: i32 = 549;
pub const CAP_PROP_XI_ACQ_FRAME_BURST_COUNT: i32 = 499;
pub const CAP_PROP_XI_ACQ_TIMING_MODE: i32 = 538;
pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_COMMIT: i32 = 552;
pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_SIZE: i32 = 550;
pub const CAP_PROP_XI_AEAG: i32 = 415;
pub const CAP_PROP_XI_AEAG_LEVEL: i32 = 419;
pub const CAP_PROP_XI_AEAG_ROI_HEIGHT: i32 = 442;
pub const CAP_PROP_XI_AEAG_ROI_OFFSET_X: i32 = 439;
pub const CAP_PROP_XI_AEAG_ROI_OFFSET_Y: i32 = 440;
pub const CAP_PROP_XI_AEAG_ROI_WIDTH: i32 = 441;
pub const CAP_PROP_XI_AE_MAX_LIMIT: i32 = 417;
pub const CAP_PROP_XI_AG_MAX_LIMIT: i32 = 418;
pub const CAP_PROP_XI_APPLY_CMS: i32 = 471;
pub const CAP_PROP_XI_AUTO_BANDWIDTH_CALCULATION: i32 = 573;
pub const CAP_PROP_XI_AUTO_WB: i32 = 414;
pub const CAP_PROP_XI_AVAILABLE_BANDWIDTH: i32 = 539;
pub const CAP_PROP_XI_BINNING_HORIZONTAL: i32 = 429;
pub const CAP_PROP_XI_BINNING_PATTERN: i32 = 430;
pub const CAP_PROP_XI_BINNING_SELECTOR: i32 = 427;
pub const CAP_PROP_XI_BINNING_VERTICAL: i32 = 428;
pub const CAP_PROP_XI_BPC: i32 = 445;
pub const CAP_PROP_XI_BUFFERS_QUEUE_SIZE: i32 = 551;
pub const CAP_PROP_XI_BUFFER_POLICY: i32 = 540;
pub const CAP_PROP_XI_CC_MATRIX_00: i32 = 479;
pub const CAP_PROP_XI_CC_MATRIX_01: i32 = 480;
pub const CAP_PROP_XI_CC_MATRIX_02: i32 = 481;
pub const CAP_PROP_XI_CC_MATRIX_03: i32 = 482;
pub const CAP_PROP_XI_CC_MATRIX_10: i32 = 483;
pub const CAP_PROP_XI_CC_MATRIX_11: i32 = 484;
pub const CAP_PROP_XI_CC_MATRIX_12: i32 = 485;
pub const CAP_PROP_XI_CC_MATRIX_13: i32 = 486;
pub const CAP_PROP_XI_CC_MATRIX_20: i32 = 487;
pub const CAP_PROP_XI_CC_MATRIX_21: i32 = 488;
pub const CAP_PROP_XI_CC_MATRIX_22: i32 = 489;
pub const CAP_PROP_XI_CC_MATRIX_23: i32 = 490;
pub const CAP_PROP_XI_CC_MATRIX_30: i32 = 491;
pub const CAP_PROP_XI_CC_MATRIX_31: i32 = 492;
pub const CAP_PROP_XI_CC_MATRIX_32: i32 = 493;
pub const CAP_PROP_XI_CC_MATRIX_33: i32 = 494;
pub const CAP_PROP_XI_CHIP_TEMP: i32 = 468;
pub const CAP_PROP_XI_CMS: i32 = 470;
pub const CAP_PROP_XI_COLOR_FILTER_ARRAY: i32 = 475;
pub const CAP_PROP_XI_COLUMN_FPN_CORRECTION: i32 = 555;
pub const CAP_PROP_XI_COOLING: i32 = 466;
pub const CAP_PROP_XI_COUNTER_SELECTOR: i32 = 536;
pub const CAP_PROP_XI_COUNTER_VALUE: i32 = 537;
pub const CAP_PROP_XI_DATA_FORMAT: i32 = 401;
pub const CAP_PROP_XI_DEBOUNCE_EN: i32 = 507;
pub const CAP_PROP_XI_DEBOUNCE_POL: i32 = 510;
pub const CAP_PROP_XI_DEBOUNCE_T0: i32 = 508;
pub const CAP_PROP_XI_DEBOUNCE_T1: i32 = 509;
pub const CAP_PROP_XI_DEBUG_LEVEL: i32 = 572;
pub const CAP_PROP_XI_DECIMATION_HORIZONTAL: i32 = 433;
pub const CAP_PROP_XI_DECIMATION_PATTERN: i32 = 434;
pub const CAP_PROP_XI_DECIMATION_SELECTOR: i32 = 431;
pub const CAP_PROP_XI_DECIMATION_VERTICAL: i32 = 432;
pub const CAP_PROP_XI_DEFAULT_CC_MATRIX: i32 = 495;
pub const CAP_PROP_XI_DEVICE_MODEL_ID: i32 = 521;
pub const CAP_PROP_XI_DEVICE_RESET: i32 = 554;
pub const CAP_PROP_XI_DEVICE_SN: i32 = 522;
pub const CAP_PROP_XI_DOWNSAMPLING: i32 = 400;
pub const CAP_PROP_XI_DOWNSAMPLING_TYPE: i32 = 426;
pub const CAP_PROP_XI_EXPOSURE: i32 = 421;
pub const CAP_PROP_XI_EXPOSURE_BURST_COUNT: i32 = 422;
pub const CAP_PROP_XI_EXP_PRIORITY: i32 = 416;
pub const CAP_PROP_XI_FFS_ACCESS_KEY: i32 = 583;
pub const CAP_PROP_XI_FFS_FILE_ID: i32 = 594;
pub const CAP_PROP_XI_FFS_FILE_SIZE: i32 = 580;
pub const CAP_PROP_XI_FRAMERATE: i32 = 535;
pub const CAP_PROP_XI_FREE_FFS_SIZE: i32 = 581;
pub const CAP_PROP_XI_GAIN: i32 = 424;
pub const CAP_PROP_XI_GAIN_SELECTOR: i32 = 423;
pub const CAP_PROP_XI_GAMMAC: i32 = 477;
pub const CAP_PROP_XI_GAMMAY: i32 = 476;
pub const CAP_PROP_XI_GPI_LEVEL: i32 = 408;
pub const CAP_PROP_XI_GPI_MODE: i32 = 407;
pub const CAP_PROP_XI_GPI_SELECTOR: i32 = 406;
pub const CAP_PROP_XI_GPO_MODE: i32 = 410;
pub const CAP_PROP_XI_GPO_SELECTOR: i32 = 409;
pub const CAP_PROP_XI_HDR: i32 = 559;
pub const CAP_PROP_XI_HDR_KNEEPOINT_COUNT: i32 = 560;
pub const CAP_PROP_XI_HDR_T1: i32 = 561;
pub const CAP_PROP_XI_HDR_T2: i32 = 562;
pub const CAP_PROP_XI_HEIGHT: i32 = 452;
pub const CAP_PROP_XI_HOUS_BACK_SIDE_TEMP: i32 = 590;
pub const CAP_PROP_XI_HOUS_TEMP: i32 = 469;
pub const CAP_PROP_XI_HW_REVISION: i32 = 571;
pub const CAP_PROP_XI_IMAGE_BLACK_LEVEL: i32 = 565;
pub const CAP_PROP_XI_IMAGE_DATA_BIT_DEPTH: i32 = 462;
pub const CAP_PROP_XI_IMAGE_DATA_FORMAT: i32 = 435;
pub const CAP_PROP_XI_IMAGE_DATA_FORMAT_RGB32_ALPHA: i32 = 529;
pub const CAP_PROP_XI_IMAGE_IS_COLOR: i32 = 474;
pub const CAP_PROP_XI_IMAGE_PAYLOAD_SIZE: i32 = 530;
pub const CAP_PROP_XI_IS_COOLED: i32 = 465;
pub const CAP_PROP_XI_IS_DEVICE_EXIST: i32 = 547;
pub const CAP_PROP_XI_KNEEPOINT1: i32 = 563;
pub const CAP_PROP_XI_KNEEPOINT2: i32 = 564;
pub const CAP_PROP_XI_LED_MODE: i32 = 412;
pub const CAP_PROP_XI_LED_SELECTOR: i32 = 411;
pub const CAP_PROP_XI_LENS_APERTURE_VALUE: i32 = 512;
pub const CAP_PROP_XI_LENS_FEATURE: i32 = 518;
pub const CAP_PROP_XI_LENS_FEATURE_SELECTOR: i32 = 517;
pub const CAP_PROP_XI_LENS_FOCAL_LENGTH: i32 = 516;
pub const CAP_PROP_XI_LENS_FOCUS_DISTANCE: i32 = 515;
pub const CAP_PROP_XI_LENS_FOCUS_MOVE: i32 = 514;
pub const CAP_PROP_XI_LENS_FOCUS_MOVEMENT_VALUE: i32 = 513;
pub const CAP_PROP_XI_LENS_MODE: i32 = 511;
pub const CAP_PROP_XI_LIMIT_BANDWIDTH: i32 = 459;
pub const CAP_PROP_XI_LUT_EN: i32 = 541;
pub const CAP_PROP_XI_LUT_INDEX: i32 = 542;
pub const CAP_PROP_XI_LUT_VALUE: i32 = 543;
pub const CAP_PROP_XI_MANUAL_WB: i32 = 413;
pub const CAP_PROP_XI_OFFSET_X: i32 = 402;
pub const CAP_PROP_XI_OFFSET_Y: i32 = 403;
pub const CAP_PROP_XI_OUTPUT_DATA_BIT_DEPTH: i32 = 461;
pub const CAP_PROP_XI_OUTPUT_DATA_PACKING: i32 = 463;
pub const CAP_PROP_XI_OUTPUT_DATA_PACKING_TYPE: i32 = 464;
pub const CAP_PROP_XI_RECENT_FRAME: i32 = 553;
pub const CAP_PROP_XI_REGION_MODE: i32 = 595;
pub const CAP_PROP_XI_REGION_SELECTOR: i32 = 589;
pub const CAP_PROP_XI_ROW_FPN_CORRECTION: i32 = 591;
pub const CAP_PROP_XI_SENSOR_BOARD_TEMP: i32 = 596;
pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_HZ: i32 = 532;
pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_INDEX: i32 = 533;
pub const CAP_PROP_XI_SENSOR_DATA_BIT_DEPTH: i32 = 460;
pub const CAP_PROP_XI_SENSOR_FEATURE_SELECTOR: i32 = 585;
pub const CAP_PROP_XI_SENSOR_FEATURE_VALUE: i32 = 586;
pub const CAP_PROP_XI_SENSOR_MODE: i32 = 558;
pub const CAP_PROP_XI_SENSOR_OUTPUT_CHANNEL_COUNT: i32 = 534;
pub const CAP_PROP_XI_SENSOR_TAPS: i32 = 437;
pub const CAP_PROP_XI_SHARPNESS: i32 = 478;
pub const CAP_PROP_XI_SHUTTER_TYPE: i32 = 436;
pub const CAP_PROP_XI_TARGET_TEMP: i32 = 467;
pub const CAP_PROP_XI_TEST_PATTERN: i32 = 588;
pub const CAP_PROP_XI_TEST_PATTERN_GENERATOR_SELECTOR: i32 = 587;
pub const CAP_PROP_XI_TIMEOUT: i32 = 420;
pub const CAP_PROP_XI_TRANSPORT_PIXEL_FORMAT: i32 = 531;
pub const CAP_PROP_XI_TRG_DELAY: i32 = 544;
pub const CAP_PROP_XI_TRG_SELECTOR: i32 = 498;
pub const CAP_PROP_XI_TRG_SOFTWARE: i32 = 405;
pub const CAP_PROP_XI_TRG_SOURCE: i32 = 404;
pub const CAP_PROP_XI_TS_RST_MODE: i32 = 545;
pub const CAP_PROP_XI_TS_RST_SOURCE: i32 = 546;
pub const CAP_PROP_XI_USED_FFS_SIZE: i32 = 582;
pub const CAP_PROP_XI_WB_KB: i32 = 450;
pub const CAP_PROP_XI_WB_KG: i32 = 449;
pub const CAP_PROP_XI_WB_KR: i32 = 448;
pub const CAP_PROP_XI_WIDTH: i32 = 451;
pub const CAP_PROP_ZOOM: i32 = 27;
pub const CAP_PVAPI: i32 = 800;
pub const CAP_PVAPI_DECIMATION_2OUTOF16: i32 = 8;
pub const CAP_PVAPI_DECIMATION_2OUTOF4: i32 = 2;
pub const CAP_PVAPI_DECIMATION_2OUTOF8: i32 = 4;
pub const CAP_PVAPI_DECIMATION_OFF: i32 = 1;
pub const CAP_PVAPI_FSTRIGMODE_FIXEDRATE: i32 = 3;
pub const CAP_PVAPI_FSTRIGMODE_FREERUN: i32 = 0;
pub const CAP_PVAPI_FSTRIGMODE_SOFTWARE: i32 = 4;
pub const CAP_PVAPI_FSTRIGMODE_SYNCIN1: i32 = 1;
pub const CAP_PVAPI_FSTRIGMODE_SYNCIN2: i32 = 2;
pub const CAP_PVAPI_PIXELFORMAT_BAYER16: i32 = 4;
pub const CAP_PVAPI_PIXELFORMAT_BAYER8: i32 = 3;
pub const CAP_PVAPI_PIXELFORMAT_BGR24: i32 = 6;
pub const CAP_PVAPI_PIXELFORMAT_BGRA32: i32 = 8;
pub const CAP_PVAPI_PIXELFORMAT_MONO16: i32 = 2;
pub const CAP_PVAPI_PIXELFORMAT_MONO8: i32 = 1;
pub const CAP_PVAPI_PIXELFORMAT_RGB24: i32 = 5;
pub const CAP_PVAPI_PIXELFORMAT_RGBA32: i32 = 7;
pub const CAP_QT: i32 = 500;
pub const CAP_REALSENSE: i32 = 1500;
pub const CAP_UEYE: i32 = 2500;
pub const CAP_UNICAP: i32 = 600;
pub const CAP_V4L: i32 = 200;
pub const CAP_V4L2: i32 = 200;
pub const CAP_VFW: i32 = 200;
pub const CAP_WINRT: i32 = 1410;
pub const CAP_XIAPI: i32 = 1100;
pub const CAP_XINE: i32 = 2400;
pub const CV__CAP_PROP_LATEST: i32 = 69;
pub const CV__VIDEOWRITER_PROP_LATEST: i32 = 9;
pub const VIDEOWRITER_PROP_DEPTH: i32 = 5;
pub const VIDEOWRITER_PROP_FRAMEBYTES: i32 = 2;
pub const VIDEOWRITER_PROP_HW_ACCELERATION: i32 = 6;
pub const VIDEOWRITER_PROP_HW_ACCELERATION_USE_OPENCL: i32 = 8;
pub const VIDEOWRITER_PROP_HW_DEVICE: i32 = 7;
pub const VIDEOWRITER_PROP_IS_COLOR: i32 = 4;
pub const VIDEOWRITER_PROP_NSTRIPES: i32 = 3;
pub const VIDEOWRITER_PROP_QUALITY: i32 = 1;
pub const VIDEO_ACCELERATION_ANY: i32 = 1;
pub const VIDEO_ACCELERATION_D3D11: i32 = 2;
pub const VIDEO_ACCELERATION_MFX: i32 = 4;
pub const VIDEO_ACCELERATION_NONE: i32 = 0;
pub const VIDEO_ACCELERATION_VAAPI: i32 = 3;
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

#[inline]
pub fn get_backend_name(api: crate::videoio::VideoCaptureAPIs) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getBackendName_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_camera_backend_plugin_version(api: crate::videoio::VideoCaptureAPIs, version_abi: &mut i32, version_api: &mut i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getCameraBackendPluginVersion_VideoCaptureAPIs_intR_intR(api, version_abi, version_api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_camera_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getCameraBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_stream_backend_plugin_version(api: crate::videoio::VideoCaptureAPIs, version_abi: &mut i32, version_api: &mut i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getStreamBackendPluginVersion_VideoCaptureAPIs_intR_intR(api, version_abi, version_api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_stream_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getStreamBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_writer_backend_plugin_version(api: crate::videoio::VideoCaptureAPIs, version_abi: &mut i32, version_api: &mut i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getWriterBackendPluginVersion_VideoCaptureAPIs_intR_intR(api, version_abi, version_api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_writer_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getWriterBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn has_backend(api: crate::videoio::VideoCaptureAPIs) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_hasBackend_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn is_backend_built_in(api: crate::videoio::VideoCaptureAPIs) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_isBackendBuiltIn_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub trait VideoCaptureTraitConst {
	fn as_raw_VideoCapture(&self) -> *const c_void;

	#[inline]
	fn is_opened(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_isOpened_const(self.as_raw_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get(&self, prop_id: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_get_const_int(self.as_raw_VideoCapture(), prop_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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
	#[inline]
	fn open_file(&mut self, filename: &str, api_preference: i32) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_const_StringR_int(self.as_raw_mut_VideoCapture(), filename.opencv_as_extern(), api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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
	#[inline]
	fn open_1(&mut self, index: i32, api_preference: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_int_int(self.as_raw_mut_VideoCapture(), index, api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn open_2(&mut self, index: i32, api_preference: i32, params: &core::Vector<i32>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_int_int_const_vector_int_R(self.as_raw_mut_VideoCapture(), index, api_preference, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_release(self.as_raw_mut_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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
	#[inline]
	fn retrieve(&mut self, image: &mut dyn core::ToOutputArray, flag: i32) -> Result<bool> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_retrieve_const__OutputArrayR_int(self.as_raw_mut_VideoCapture(), image.as_raw__OutputArray(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, image: &mut dyn core::ToOutputArray) -> Result<bool> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_read_const__OutputArrayR(self.as_raw_mut_VideoCapture(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set(&mut self, prop_id: i32, value: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_set_int_double(self.as_raw_mut_VideoCapture(), prop_id, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_exception_mode(&mut self, enable: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_setExceptionMode_bool(self.as_raw_mut_VideoCapture(), enable, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_exception_mode(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_getExceptionMode(self.as_raw_mut_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

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
	#[inline]
	pub fn new(index: i32, api_preference: i32) -> Result<crate::videoio::VideoCapture> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture_int_int(index, api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}
	
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
	#[inline]
	pub fn wait_any(streams: &core::Vector<crate::videoio::VideoCapture>, ready_index: &mut core::Vector<i32>, timeout_ns: i64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_waitAny_const_vector_VideoCapture_R_vector_int_R_int64_t(streams.as_raw_VectorOfVideoCapture(), ready_index.as_raw_mut_VectorOfi32(), timeout_ns, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait VideoWriterTraitConst {
	fn as_raw_VideoWriter(&self) -> *const c_void;

	#[inline]
	fn is_opened(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_isOpened_const(self.as_raw_VideoWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get(&self, prop_id: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_get_const_int(self.as_raw_VideoWriter(), prop_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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
	#[inline]
	fn open_with_backend(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), api_preference, fourcc, fps, frame_size.opencv_as_extern(), is_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn open_1(&mut self, filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_double_const_SizeR_const_vector_int_R(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn open_2(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_int_double_const_SizeR_const_vector_int_R(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), api_preference, fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_release(self.as_raw_mut_VideoWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn write(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_write_const__InputArrayR(self.as_raw_mut_VideoWriter(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set(&mut self, prop_id: i32, value: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_set_int_double(self.as_raw_mut_VideoWriter(), prop_id, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

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
	
	#[inline]
	pub fn fourcc(c1: i8, c2: i8, c3: i8, c4: i8) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_fourcc_char_char_char_char(c1, c2, c3, c4, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
