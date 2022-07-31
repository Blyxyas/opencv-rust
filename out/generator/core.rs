#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Core functionality
//!    # Basic structures
//!    # C structures and operations
//!        # Connections with C++
//!    # Operations on arrays
//!    # Asynchronous API
//!    # XML/YAML Persistence
//!    # Clustering
//!    # Utility and system functions and macros
//!        # Logging facilities
//!        # SSE utilities
//!        # NEON utilities
//!        # VSX utilities
//!        # Softfloat support
//!        # Utility functions for OpenCV samples
//!    # OpenGL interoperability
//!    # Intel IPP Asynchronous C/C++ Converters
//!    # Optimization Algorithms
//!    # DirectX interoperability
//!    # Eigen support
//!    # OpenCL support
//!    # Intel VA-API/OpenCL (CL-VA) interoperability
//!    # Hardware Acceleration Layer
//!        # Functions
//!        # Interface
//!        # Universal intrinsics
//!            # Private implementation helpers
//!        # Low-level API for external libraries / plugins
//!    # Parallel Processing
//!        # Parallel backends API
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::HammingTraitConst, super::HammingTrait, super::Detail_CheckContextTraitConst, super::Detail_CheckContextTrait, super::Matx_AddOpTraitConst, super::Matx_AddOpTrait, super::Matx_SubOpTraitConst, super::Matx_SubOpTrait, super::Matx_ScaleOpTraitConst, super::Matx_ScaleOpTrait, super::Matx_MulOpTraitConst, super::Matx_MulOpTrait, super::Matx_DivOpTraitConst, super::Matx_DivOpTrait, super::Matx_MatMulOpTraitConst, super::Matx_MatMulOpTrait, super::Matx_TOpTraitConst, super::Matx_TOpTrait, super::RotatedRectTraitConst, super::RotatedRectTrait, super::RangeTraitConst, super::RangeTrait, super::_InputArrayTraitConst, super::_InputArrayTrait, super::_OutputArrayTraitConst, super::_OutputArrayTrait, super::_InputOutputArrayTraitConst, super::_InputOutputArrayTrait, super::UMatDataTraitConst, super::UMatDataTrait, super::MatSizeTraitConst, super::MatSizeTrait, super::MatStepTraitConst, super::MatStepTrait, super::MatTraitConst, super::MatTrait, super::UMatTraitConst, super::UMatTrait, super::SparseMat_HdrTraitConst, super::SparseMat_HdrTrait, super::SparseMat_NodeTraitConst, super::SparseMat_NodeTrait, super::SparseMatTraitConst, super::SparseMatTrait, super::MatConstIteratorTraitConst, super::MatConstIteratorTrait, super::SparseMatConstIteratorTraitConst, super::SparseMatConstIteratorTrait, super::SparseMatIteratorTraitConst, super::SparseMatIteratorTrait, super::MatOpConst, super::MatOp, super::MatExprTraitConst, super::MatExprTrait, super::FileStorageTraitConst, super::FileStorageTrait, super::FileNodeTraitConst, super::FileNodeTrait, super::FileNodeIteratorTraitConst, super::FileNodeIteratorTrait, super::WriteStructContextTraitConst, super::WriteStructContextTrait, super::ExceptionTraitConst, super::ExceptionTrait, super::PCATraitConst, super::PCATrait, super::LDATraitConst, super::LDATrait, super::SVDTraitConst, super::SVDTrait, super::RNGTraitConst, super::RNGTrait, super::RNG_MT19937TraitConst, super::RNG_MT19937Trait, super::FormattedConst, super::Formatted, super::FormatterConst, super::Formatter, super::AlgorithmTraitConst, super::AlgorithmTrait, super::TickMeterTraitConst, super::TickMeterTrait, super::ParallelLoopBodyConst, super::ParallelLoopBody, super::CommandLineParserTraitConst, super::CommandLineParserTrait, super::TLSDataContainerConst, super::TLSDataContainer, super::NodeDataTraitConst, super::NodeDataTrait, super::MinProblemSolver_FunctionConst, super::MinProblemSolver_Function, super::MinProblemSolverConst, super::MinProblemSolver, super::DownhillSolverConst, super::DownhillSolver, super::ConjGradSolverConst, super::ConjGradSolver, super::DeviceTraitConst, super::DeviceTrait, super::Context_UserContextTraitConst, super::Context_UserContextTrait, super::ContextTraitConst, super::ContextTrait, super::PlatformTraitConst, super::PlatformTrait, super::QueueTraitConst, super::QueueTrait, super::KernelArgTraitConst, super::KernelArgTrait, super::KernelTraitConst, super::KernelTrait, super::ProgramTraitConst, super::ProgramTrait, super::ProgramSourceTraitConst, super::ProgramSourceTrait, super::PlatformInfoTraitConst, super::PlatformInfoTrait, super::Image2DTraitConst, super::Image2DTrait, super::TimerTraitConst, super::TimerTrait, super::OpenCLExecutionContextTraitConst, super::OpenCLExecutionContextTrait, super::GpuMat_AllocatorConst, super::GpuMat_Allocator, super::GpuMatTraitConst, super::GpuMatTrait, super::GpuDataTraitConst, super::GpuDataTrait, super::GpuMatNDTraitConst, super::GpuMatNDTrait, super::BufferPoolTraitConst, super::BufferPoolTrait, super::HostMemTraitConst, super::HostMemTrait, super::StreamTraitConst, super::StreamTrait, super::EventTraitConst, super::EventTrait, super::TargetArchsTraitConst, super::TargetArchsTrait, super::DeviceInfoTraitConst, super::DeviceInfoTrait, super::BufferTraitConst, super::BufferTrait, super::Texture2DTraitConst, super::Texture2DTrait, super::ArraysTraitConst, super::ArraysTrait, super::AsyncArrayTraitConst, super::AsyncArrayTrait, super::AsyncPromiseTraitConst, super::AsyncPromiseTrait, super::LogTagTraitConst, super::LogTagTrait, super::OriginalClassNameTraitConst, super::OriginalClassNameTrait };
}

// ACCESS_FAST /usr/include/opencv2/core/mat.hpp:65
pub const ACCESS_FAST: i32 = 67108864;
// ACCESS_MASK /usr/include/opencv2/core/mat.hpp:65
pub const ACCESS_MASK: i32 = 50331648;
// ACCESS_READ /usr/include/opencv2/core/mat.hpp:64
pub const ACCESS_READ: i32 = 16777216;
// ACCESS_RW /usr/include/opencv2/core/mat.hpp:65
pub const ACCESS_RW: i32 = 50331648;
// ACCESS_WRITE /usr/include/opencv2/core/mat.hpp:64
pub const ACCESS_WRITE: i32 = 33554432;
// BORDER_CONSTANT /usr/include/opencv2/core/base.hpp:269
pub const BORDER_CONSTANT: i32 = 0;
// BORDER_DEFAULT /usr/include/opencv2/core/base.hpp:277
pub const BORDER_DEFAULT: i32 = 4;
// BORDER_ISOLATED /usr/include/opencv2/core/base.hpp:278
pub const BORDER_ISOLATED: i32 = 16;
// BORDER_REFLECT /usr/include/opencv2/core/base.hpp:271
pub const BORDER_REFLECT: i32 = 2;
// BORDER_REFLECT101 /usr/include/opencv2/core/base.hpp:276
pub const BORDER_REFLECT101: i32 = 4;
// BORDER_REFLECT_101 /usr/include/opencv2/core/base.hpp:273
pub const BORDER_REFLECT_101: i32 = 4;
// BORDER_REPLICATE /usr/include/opencv2/core/base.hpp:270
pub const BORDER_REPLICATE: i32 = 1;
// BORDER_TRANSPARENT /usr/include/opencv2/core/base.hpp:274
pub const BORDER_TRANSPARENT: i32 = 5;
// BORDER_WRAP /usr/include/opencv2/core/base.hpp:272
pub const BORDER_WRAP: i32 = 3;
// BadAlign /usr/include/opencv2/core/base.hpp:90
pub const BadAlign: i32 = -21;
// BadAlphaChannel /usr/include/opencv2/core/base.hpp:87
pub const BadAlphaChannel: i32 = -18;
// BadCOI /usr/include/opencv2/core/base.hpp:93
pub const BadCOI: i32 = -24;
// BadCallBack /usr/include/opencv2/core/base.hpp:91
pub const BadCallBack: i32 = -22;
// BadDataPtr /usr/include/opencv2/core/base.hpp:81
pub const BadDataPtr: i32 = -12;
// BadDepth /usr/include/opencv2/core/base.hpp:86
pub const BadDepth: i32 = -17;
// BadImageSize /usr/include/opencv2/core/base.hpp:79
pub const BadImageSize: i32 = -10;
// BadModelOrChSeq /usr/include/opencv2/core/base.hpp:83
pub const BadModelOrChSeq: i32 = -14;
// BadNumChannel1U /usr/include/opencv2/core/base.hpp:85
pub const BadNumChannel1U: i32 = -16;
// BadNumChannels /usr/include/opencv2/core/base.hpp:84
pub const BadNumChannels: i32 = -15;
// BadOffset /usr/include/opencv2/core/base.hpp:80
pub const BadOffset: i32 = -11;
// BadOrder /usr/include/opencv2/core/base.hpp:88
pub const BadOrder: i32 = -19;
// BadOrigin /usr/include/opencv2/core/base.hpp:89
pub const BadOrigin: i32 = -20;
// BadROISize /usr/include/opencv2/core/base.hpp:94
pub const BadROISize: i32 = -25;
// BadStep /usr/include/opencv2/core/base.hpp:82
pub const BadStep: i32 = -13;
// BadTileSize /usr/include/opencv2/core/base.hpp:92
pub const BadTileSize: i32 = -23;
// CMP_EQ /usr/include/opencv2/core/base.hpp:211
pub const CMP_EQ: i32 = 0;
// CMP_GE /usr/include/opencv2/core/base.hpp:213
pub const CMP_GE: i32 = 2;
// CMP_GT /usr/include/opencv2/core/base.hpp:212
pub const CMP_GT: i32 = 1;
// CMP_LE /usr/include/opencv2/core/base.hpp:215
pub const CMP_LE: i32 = 4;
// CMP_LT /usr/include/opencv2/core/base.hpp:214
pub const CMP_LT: i32 = 3;
// CMP_NE /usr/include/opencv2/core/base.hpp:216
pub const CMP_NE: i32 = 5;
// COVAR_COLS /usr/include/opencv2/core.hpp:206
pub const COVAR_COLS: i32 = 16;
// COVAR_NORMAL /usr/include/opencv2/core.hpp:187
pub const COVAR_NORMAL: i32 = 1;
// COVAR_ROWS /usr/include/opencv2/core.hpp:202
pub const COVAR_ROWS: i32 = 8;
// COVAR_SCALE /usr/include/opencv2/core.hpp:198
pub const COVAR_SCALE: i32 = 4;
// COVAR_SCRAMBLED /usr/include/opencv2/core.hpp:182
pub const COVAR_SCRAMBLED: i32 = 0;
// COVAR_USE_AVG /usr/include/opencv2/core.hpp:193
pub const COVAR_USE_AVG: i32 = 2;
// CPU_AVX /usr/include/opencv2/core/cvdef.h:305
pub const CPU_AVX: i32 = 10;
// CPU_AVX2 /usr/include/opencv2/core/cvdef.h:306
pub const CPU_AVX2: i32 = 11;
// CPU_AVX512_CLX /usr/include/opencv2/core/cvdef.h:342
pub const CPU_AVX512_CLX: i32 = 261;
// CPU_AVX512_CNL /usr/include/opencv2/core/cvdef.h:341
pub const CPU_AVX512_CNL: i32 = 260;
// CPU_AVX512_COMMON /usr/include/opencv2/core/cvdef.h:338
pub const CPU_AVX512_COMMON: i32 = 257;
// CPU_AVX512_ICL /usr/include/opencv2/core/cvdef.h:343
pub const CPU_AVX512_ICL: i32 = 262;
// CPU_AVX512_KNL /usr/include/opencv2/core/cvdef.h:339
pub const CPU_AVX512_KNL: i32 = 258;
// CPU_AVX512_KNM /usr/include/opencv2/core/cvdef.h:340
pub const CPU_AVX512_KNM: i32 = 259;
// CPU_AVX512_SKX /usr/include/opencv2/core/cvdef.h:337
pub const CPU_AVX512_SKX: i32 = 256;
// CPU_AVX_5124FMAPS /usr/include/opencv2/core/cvdef.h:324
pub const CPU_AVX_5124FMAPS: i32 = 27;
// CPU_AVX_5124VNNIW /usr/include/opencv2/core/cvdef.h:323
pub const CPU_AVX_5124VNNIW: i32 = 26;
// CPU_AVX_512BITALG /usr/include/opencv2/core/cvdef.h:321
pub const CPU_AVX_512BITALG: i32 = 24;
// CPU_AVX_512BW /usr/include/opencv2/core/cvdef.h:310
pub const CPU_AVX_512BW: i32 = 14;
// CPU_AVX_512CD /usr/include/opencv2/core/cvdef.h:311
pub const CPU_AVX_512CD: i32 = 15;
// CPU_AVX_512DQ /usr/include/opencv2/core/cvdef.h:312
pub const CPU_AVX_512DQ: i32 = 16;
// CPU_AVX_512ER /usr/include/opencv2/core/cvdef.h:313
pub const CPU_AVX_512ER: i32 = 17;
// CPU_AVX_512F /usr/include/opencv2/core/cvdef.h:309
pub const CPU_AVX_512F: i32 = 13;
// CPU_AVX_512IFMA /usr/include/opencv2/core/cvdef.h:315
pub const CPU_AVX_512IFMA: i32 = 18;
// CPU_AVX_512IFMA512 /usr/include/opencv2/core/cvdef.h:314
pub const CPU_AVX_512IFMA512: i32 = 18;
// CPU_AVX_512PF /usr/include/opencv2/core/cvdef.h:316
pub const CPU_AVX_512PF: i32 = 19;
// CPU_AVX_512VBMI /usr/include/opencv2/core/cvdef.h:317
pub const CPU_AVX_512VBMI: i32 = 20;
// CPU_AVX_512VBMI2 /usr/include/opencv2/core/cvdef.h:319
pub const CPU_AVX_512VBMI2: i32 = 22;
// CPU_AVX_512VL /usr/include/opencv2/core/cvdef.h:318
pub const CPU_AVX_512VL: i32 = 21;
// CPU_AVX_512VNNI /usr/include/opencv2/core/cvdef.h:320
pub const CPU_AVX_512VNNI: i32 = 23;
// CPU_AVX_512VPOPCNTDQ /usr/include/opencv2/core/cvdef.h:322
pub const CPU_AVX_512VPOPCNTDQ: i32 = 25;
// CPU_FMA3 /usr/include/opencv2/core/cvdef.h:307
pub const CPU_FMA3: i32 = 12;
// CPU_FP16 /usr/include/opencv2/core/cvdef.h:304
pub const CPU_FP16: i32 = 9;
// CPU_MAX_FEATURE /usr/include/opencv2/core/cvdef.h:345
pub const CPU_MAX_FEATURE: i32 = 512;
// CPU_MMX /usr/include/opencv2/core/cvdef.h:296
pub const CPU_MMX: i32 = 1;
// CPU_MSA /usr/include/opencv2/core/cvdef.h:328
pub const CPU_MSA: i32 = 150;
// CPU_NEON /usr/include/opencv2/core/cvdef.h:326
pub const CPU_NEON: i32 = 100;
// CPU_POPCNT /usr/include/opencv2/core/cvdef.h:303
pub const CPU_POPCNT: i32 = 8;
// CPU_RISCVV /usr/include/opencv2/core/cvdef.h:330
pub const CPU_RISCVV: i32 = 170;
// CPU_RVV /usr/include/opencv2/core/cvdef.h:335
pub const CPU_RVV: i32 = 210;
// CPU_SSE /usr/include/opencv2/core/cvdef.h:297
pub const CPU_SSE: i32 = 2;
// CPU_SSE2 /usr/include/opencv2/core/cvdef.h:298
pub const CPU_SSE2: i32 = 3;
// CPU_SSE3 /usr/include/opencv2/core/cvdef.h:299
pub const CPU_SSE3: i32 = 4;
// CPU_SSE4_1 /usr/include/opencv2/core/cvdef.h:301
pub const CPU_SSE4_1: i32 = 6;
// CPU_SSE4_2 /usr/include/opencv2/core/cvdef.h:302
pub const CPU_SSE4_2: i32 = 7;
// CPU_SSSE3 /usr/include/opencv2/core/cvdef.h:300
pub const CPU_SSSE3: i32 = 5;
// CPU_VSX /usr/include/opencv2/core/cvdef.h:332
pub const CPU_VSX: i32 = 200;
// CPU_VSX3 /usr/include/opencv2/core/cvdef.h:333
pub const CPU_VSX3: i32 = 201;
// CV_16F /usr/include/opencv2/core/hal/interface.h:80
pub const CV_16F: i32 = 7;
// CV_16FC1 /usr/include/opencv2/core/hal/interface.h:130
pub const CV_16FC1: i32 = CV_MAKETYPE(CV_16F,1);
// CV_16FC2 /usr/include/opencv2/core/hal/interface.h:131
pub const CV_16FC2: i32 = CV_MAKETYPE(CV_16F,2);
// CV_16FC3 /usr/include/opencv2/core/hal/interface.h:132
pub const CV_16FC3: i32 = CV_MAKETYPE(CV_16F,3);
// CV_16FC4 /usr/include/opencv2/core/hal/interface.h:133
pub const CV_16FC4: i32 = CV_MAKETYPE(CV_16F,4);
// CV_16S /usr/include/opencv2/core/hal/interface.h:76
pub const CV_16S: i32 = 3;
// CV_16SC1 /usr/include/opencv2/core/hal/interface.h:106
pub const CV_16SC1: i32 = CV_MAKETYPE(CV_16S,1);
// CV_16SC2 /usr/include/opencv2/core/hal/interface.h:107
pub const CV_16SC2: i32 = CV_MAKETYPE(CV_16S,2);
// CV_16SC3 /usr/include/opencv2/core/hal/interface.h:108
pub const CV_16SC3: i32 = CV_MAKETYPE(CV_16S,3);
// CV_16SC4 /usr/include/opencv2/core/hal/interface.h:109
pub const CV_16SC4: i32 = CV_MAKETYPE(CV_16S,4);
// CV_16U /usr/include/opencv2/core/hal/interface.h:75
pub const CV_16U: i32 = 2;
// CV_16UC1 /usr/include/opencv2/core/hal/interface.h:100
pub const CV_16UC1: i32 = CV_MAKETYPE(CV_16U,1);
// CV_16UC2 /usr/include/opencv2/core/hal/interface.h:101
pub const CV_16UC2: i32 = CV_MAKETYPE(CV_16U,2);
// CV_16UC3 /usr/include/opencv2/core/hal/interface.h:102
pub const CV_16UC3: i32 = CV_MAKETYPE(CV_16U,3);
// CV_16UC4 /usr/include/opencv2/core/hal/interface.h:103
pub const CV_16UC4: i32 = CV_MAKETYPE(CV_16U,4);
// CV_2PI /usr/include/opencv2/core/cvdef.h:361
pub const CV_2PI: f64 = 6.283185307179586476925286766559;
// CV_32F /usr/include/opencv2/core/hal/interface.h:78
pub const CV_32F: i32 = 5;
// CV_32FC1 /usr/include/opencv2/core/hal/interface.h:118
pub const CV_32FC1: i32 = CV_MAKETYPE(CV_32F,1);
// CV_32FC2 /usr/include/opencv2/core/hal/interface.h:119
pub const CV_32FC2: i32 = CV_MAKETYPE(CV_32F,2);
// CV_32FC3 /usr/include/opencv2/core/hal/interface.h:120
pub const CV_32FC3: i32 = CV_MAKETYPE(CV_32F,3);
// CV_32FC4 /usr/include/opencv2/core/hal/interface.h:121
pub const CV_32FC4: i32 = CV_MAKETYPE(CV_32F,4);
// CV_32S /usr/include/opencv2/core/hal/interface.h:77
pub const CV_32S: i32 = 4;
// CV_32SC1 /usr/include/opencv2/core/hal/interface.h:112
pub const CV_32SC1: i32 = CV_MAKETYPE(CV_32S,1);
// CV_32SC2 /usr/include/opencv2/core/hal/interface.h:113
pub const CV_32SC2: i32 = CV_MAKETYPE(CV_32S,2);
// CV_32SC3 /usr/include/opencv2/core/hal/interface.h:114
pub const CV_32SC3: i32 = CV_MAKETYPE(CV_32S,3);
// CV_32SC4 /usr/include/opencv2/core/hal/interface.h:115
pub const CV_32SC4: i32 = CV_MAKETYPE(CV_32S,4);
// CV_64F /usr/include/opencv2/core/hal/interface.h:79
pub const CV_64F: i32 = 6;
// CV_64FC1 /usr/include/opencv2/core/hal/interface.h:124
pub const CV_64FC1: i32 = CV_MAKETYPE(CV_64F,1);
// CV_64FC2 /usr/include/opencv2/core/hal/interface.h:125
pub const CV_64FC2: i32 = CV_MAKETYPE(CV_64F,2);
// CV_64FC3 /usr/include/opencv2/core/hal/interface.h:126
pub const CV_64FC3: i32 = CV_MAKETYPE(CV_64F,3);
// CV_64FC4 /usr/include/opencv2/core/hal/interface.h:127
pub const CV_64FC4: i32 = CV_MAKETYPE(CV_64F,4);
// CV_8S /usr/include/opencv2/core/hal/interface.h:74
pub const CV_8S: i32 = 1;
// CV_8SC1 /usr/include/opencv2/core/hal/interface.h:94
pub const CV_8SC1: i32 = CV_MAKETYPE(CV_8S,1);
// CV_8SC2 /usr/include/opencv2/core/hal/interface.h:95
pub const CV_8SC2: i32 = CV_MAKETYPE(CV_8S,2);
// CV_8SC3 /usr/include/opencv2/core/hal/interface.h:96
pub const CV_8SC3: i32 = CV_MAKETYPE(CV_8S,3);
// CV_8SC4 /usr/include/opencv2/core/hal/interface.h:97
pub const CV_8SC4: i32 = CV_MAKETYPE(CV_8S,4);
// CV_8U /usr/include/opencv2/core/hal/interface.h:73
pub const CV_8U: i32 = 0;
// CV_8UC1 /usr/include/opencv2/core/hal/interface.h:88
pub const CV_8UC1: i32 = CV_MAKETYPE(CV_8U,1);
// CV_8UC2 /usr/include/opencv2/core/hal/interface.h:89
pub const CV_8UC2: i32 = CV_MAKETYPE(CV_8U,2);
// CV_8UC3 /usr/include/opencv2/core/hal/interface.h:90
pub const CV_8UC3: i32 = CV_MAKETYPE(CV_8U,3);
// CV_8UC4 /usr/include/opencv2/core/hal/interface.h:91
pub const CV_8UC4: i32 = CV_MAKETYPE(CV_8U,4);
// CV_AVX /usr/include/opencv2/core/cv_cpu_dispatch.h:263
pub const CV_AVX: i32 = 0;
// CV_AVX2 /usr/include/opencv2/core/cv_cpu_dispatch.h:269
pub const CV_AVX2: i32 = 0;
// CV_AVX512_CLX /usr/include/opencv2/core/cv_cpu_dispatch.h:336
pub const CV_AVX512_CLX: i32 = 0;
// CV_AVX512_CNL /usr/include/opencv2/core/cv_cpu_dispatch.h:333
pub const CV_AVX512_CNL: i32 = 0;
// CV_AVX512_COMMON /usr/include/opencv2/core/cv_cpu_dispatch.h:321
pub const CV_AVX512_COMMON: i32 = 0;
// CV_AVX512_ICL /usr/include/opencv2/core/cv_cpu_dispatch.h:339
pub const CV_AVX512_ICL: i32 = 0;
// CV_AVX512_KNL /usr/include/opencv2/core/cv_cpu_dispatch.h:324
pub const CV_AVX512_KNL: i32 = 0;
// CV_AVX512_KNM /usr/include/opencv2/core/cv_cpu_dispatch.h:327
pub const CV_AVX512_KNM: i32 = 0;
// CV_AVX512_SKX /usr/include/opencv2/core/cv_cpu_dispatch.h:330
pub const CV_AVX512_SKX: i32 = 0;
// CV_AVX_5124FMAPS /usr/include/opencv2/core/cv_cpu_dispatch.h:303
pub const CV_AVX_5124FMAPS: i32 = 0;
// CV_AVX_5124VNNIW /usr/include/opencv2/core/cv_cpu_dispatch.h:306
pub const CV_AVX_5124VNNIW: i32 = 0;
// CV_AVX_512BITALG /usr/include/opencv2/core/cv_cpu_dispatch.h:318
pub const CV_AVX_512BITALG: i32 = 0;
// CV_AVX_512BW /usr/include/opencv2/core/cv_cpu_dispatch.h:278
pub const CV_AVX_512BW: i32 = 0;
// CV_AVX_512CD /usr/include/opencv2/core/cv_cpu_dispatch.h:281
pub const CV_AVX_512CD: i32 = 0;
// CV_AVX_512DQ /usr/include/opencv2/core/cv_cpu_dispatch.h:284
pub const CV_AVX_512DQ: i32 = 0;
// CV_AVX_512ER /usr/include/opencv2/core/cv_cpu_dispatch.h:287
pub const CV_AVX_512ER: i32 = 0;
// CV_AVX_512F /usr/include/opencv2/core/cv_cpu_dispatch.h:275
pub const CV_AVX_512F: i32 = 0;
// CV_AVX_512IFMA /usr/include/opencv2/core/cv_cpu_dispatch.h:290
pub const CV_AVX_512IFMA: i32 = 0;
// CV_AVX_512IFMA512 /usr/include/opencv2/core/cv_cpu_dispatch.h:292
pub const CV_AVX_512IFMA512: i32 = CV_AVX_512IFMA;
// CV_AVX_512PF /usr/include/opencv2/core/cv_cpu_dispatch.h:294
pub const CV_AVX_512PF: i32 = 0;
// CV_AVX_512VBMI /usr/include/opencv2/core/cv_cpu_dispatch.h:297
pub const CV_AVX_512VBMI: i32 = 0;
// CV_AVX_512VBMI2 /usr/include/opencv2/core/cv_cpu_dispatch.h:315
pub const CV_AVX_512VBMI2: i32 = 0;
// CV_AVX_512VL /usr/include/opencv2/core/cv_cpu_dispatch.h:300
pub const CV_AVX_512VL: i32 = 0;
// CV_AVX_512VNNI /usr/include/opencv2/core/cv_cpu_dispatch.h:312
pub const CV_AVX_512VNNI: i32 = 0;
// CV_AVX_512VPOPCNTDQ /usr/include/opencv2/core/cv_cpu_dispatch.h:309
pub const CV_AVX_512VPOPCNTDQ: i32 = 0;
// CV_CN_MAX /usr/include/opencv2/core/hal/interface.h:69
pub const CV_CN_MAX: i32 = 512;
// CV_CN_SHIFT /usr/include/opencv2/core/hal/interface.h:70
pub const CV_CN_SHIFT: i32 = 3;
// CV_CPU_AVX /usr/include/opencv2/core/cvdef.h:249
pub const CV_CPU_AVX: i32 = 10;
// CV_CPU_AVX2 /usr/include/opencv2/core/cvdef.h:250
pub const CV_CPU_AVX2: i32 = 11;
// CV_CPU_AVX512_CLX /usr/include/opencv2/core/cvdef.h:287
pub const CV_CPU_AVX512_CLX: i32 = 261;
// CV_CPU_AVX512_CNL /usr/include/opencv2/core/cvdef.h:286
pub const CV_CPU_AVX512_CNL: i32 = 260;
// CV_CPU_AVX512_COMMON /usr/include/opencv2/core/cvdef.h:283
pub const CV_CPU_AVX512_COMMON: i32 = 257;
// CV_CPU_AVX512_ICL /usr/include/opencv2/core/cvdef.h:288
pub const CV_CPU_AVX512_ICL: i32 = 262;
// CV_CPU_AVX512_KNL /usr/include/opencv2/core/cvdef.h:284
pub const CV_CPU_AVX512_KNL: i32 = 258;
// CV_CPU_AVX512_KNM /usr/include/opencv2/core/cvdef.h:285
pub const CV_CPU_AVX512_KNM: i32 = 259;
// CV_CPU_AVX512_SKX /usr/include/opencv2/core/cvdef.h:282
pub const CV_CPU_AVX512_SKX: i32 = 256;
// CV_CPU_AVX_5124FMAPS /usr/include/opencv2/core/cvdef.h:268
pub const CV_CPU_AVX_5124FMAPS: i32 = 27;
// CV_CPU_AVX_5124VNNIW /usr/include/opencv2/core/cvdef.h:267
pub const CV_CPU_AVX_5124VNNIW: i32 = 26;
// CV_CPU_AVX_512BITALG /usr/include/opencv2/core/cvdef.h:265
pub const CV_CPU_AVX_512BITALG: i32 = 24;
// CV_CPU_AVX_512BW /usr/include/opencv2/core/cvdef.h:254
pub const CV_CPU_AVX_512BW: i32 = 14;
// CV_CPU_AVX_512CD /usr/include/opencv2/core/cvdef.h:255
pub const CV_CPU_AVX_512CD: i32 = 15;
// CV_CPU_AVX_512DQ /usr/include/opencv2/core/cvdef.h:256
pub const CV_CPU_AVX_512DQ: i32 = 16;
// CV_CPU_AVX_512ER /usr/include/opencv2/core/cvdef.h:257
pub const CV_CPU_AVX_512ER: i32 = 17;
// CV_CPU_AVX_512F /usr/include/opencv2/core/cvdef.h:253
pub const CV_CPU_AVX_512F: i32 = 13;
// CV_CPU_AVX_512IFMA /usr/include/opencv2/core/cvdef.h:259
pub const CV_CPU_AVX_512IFMA: i32 = 18;
// CV_CPU_AVX_512IFMA512 /usr/include/opencv2/core/cvdef.h:258
pub const CV_CPU_AVX_512IFMA512: i32 = 18;
// CV_CPU_AVX_512PF /usr/include/opencv2/core/cvdef.h:260
pub const CV_CPU_AVX_512PF: i32 = 19;
// CV_CPU_AVX_512VBMI /usr/include/opencv2/core/cvdef.h:261
pub const CV_CPU_AVX_512VBMI: i32 = 20;
// CV_CPU_AVX_512VBMI2 /usr/include/opencv2/core/cvdef.h:263
pub const CV_CPU_AVX_512VBMI2: i32 = 22;
// CV_CPU_AVX_512VL /usr/include/opencv2/core/cvdef.h:262
pub const CV_CPU_AVX_512VL: i32 = 21;
// CV_CPU_AVX_512VNNI /usr/include/opencv2/core/cvdef.h:264
pub const CV_CPU_AVX_512VNNI: i32 = 23;
// CV_CPU_AVX_512VPOPCNTDQ /usr/include/opencv2/core/cvdef.h:266
pub const CV_CPU_AVX_512VPOPCNTDQ: i32 = 25;
// CV_CPU_FMA3 /usr/include/opencv2/core/cvdef.h:251
pub const CV_CPU_FMA3: i32 = 12;
// CV_CPU_FP16 /usr/include/opencv2/core/cvdef.h:248
pub const CV_CPU_FP16: i32 = 9;
// CV_CPU_MMX /usr/include/opencv2/core/cvdef.h:240
pub const CV_CPU_MMX: i32 = 1;
// CV_CPU_MSA /usr/include/opencv2/core/cvdef.h:272
pub const CV_CPU_MSA: i32 = 150;
// CV_CPU_NEON /usr/include/opencv2/core/cvdef.h:270
pub const CV_CPU_NEON: i32 = 100;
// CV_CPU_NONE /usr/include/opencv2/core/cvdef.h:239
pub const CV_CPU_NONE: i32 = 0;
// CV_CPU_POPCNT /usr/include/opencv2/core/cvdef.h:247
pub const CV_CPU_POPCNT: i32 = 8;
// CV_CPU_RISCVV /usr/include/opencv2/core/cvdef.h:274
pub const CV_CPU_RISCVV: i32 = 170;
// CV_CPU_RVV /usr/include/opencv2/core/cvdef.h:279
pub const CV_CPU_RVV: i32 = 210;
// CV_CPU_SSE /usr/include/opencv2/core/cvdef.h:241
pub const CV_CPU_SSE: i32 = 2;
// CV_CPU_SSE2 /usr/include/opencv2/core/cvdef.h:242
pub const CV_CPU_SSE2: i32 = 3;
// CV_CPU_SSE3 /usr/include/opencv2/core/cvdef.h:243
pub const CV_CPU_SSE3: i32 = 4;
// CV_CPU_SSE4_1 /usr/include/opencv2/core/cvdef.h:245
pub const CV_CPU_SSE4_1: i32 = 6;
// CV_CPU_SSE4_2 /usr/include/opencv2/core/cvdef.h:246
pub const CV_CPU_SSE4_2: i32 = 7;
// CV_CPU_SSSE3 /usr/include/opencv2/core/cvdef.h:244
pub const CV_CPU_SSSE3: i32 = 5;
// CV_CPU_VSX /usr/include/opencv2/core/cvdef.h:276
pub const CV_CPU_VSX: i32 = 200;
// CV_CPU_VSX3 /usr/include/opencv2/core/cvdef.h:277
pub const CV_CPU_VSX3: i32 = 201;
// CV_CXX11 /usr/include/opencv2/core/cvdef.h:767
pub const CV_CXX11: i32 = 1;
// CV_CXX_MOVE_SEMANTICS /usr/include/opencv2/core/cvdef.h:778
pub const CV_CXX_MOVE_SEMANTICS: i32 = 1;
// CV_CXX_STD_ARRAY /usr/include/opencv2/core/cvdef.h:780
pub const CV_CXX_STD_ARRAY: i32 = 1;
// CV_DEPTH_MAX /usr/include/opencv2/core/hal/interface.h:71
pub const CV_DEPTH_MAX: i32 = (1<<CV_CN_SHIFT);
// CV_ENABLE_UNROLLED /usr/include/opencv2/core/cvdef.h:227
pub const CV_ENABLE_UNROLLED: i32 = 1;
// CV_FMA3 /usr/include/opencv2/core/cv_cpu_dispatch.h:272
pub const CV_FMA3: i32 = 0;
// CV_FP16 /usr/include/opencv2/core/cv_cpu_dispatch.h:266
pub const CV_FP16: i32 = 0;
// CV_FP16_TYPE /usr/include/opencv2/core/cvdef.h:368
pub const CV_FP16_TYPE: i32 = 0;
// CV_HAL_BORDER_CONSTANT /usr/include/opencv2/core/hal/interface.h:151
pub const CV_HAL_BORDER_CONSTANT: i32 = 0;
// CV_HAL_BORDER_ISOLATED /usr/include/opencv2/core/hal/interface.h:157
pub const CV_HAL_BORDER_ISOLATED: i32 = 16;
// CV_HAL_BORDER_REFLECT /usr/include/opencv2/core/hal/interface.h:153
pub const CV_HAL_BORDER_REFLECT: i32 = 2;
// CV_HAL_BORDER_REFLECT_101 /usr/include/opencv2/core/hal/interface.h:155
pub const CV_HAL_BORDER_REFLECT_101: i32 = 4;
// CV_HAL_BORDER_REPLICATE /usr/include/opencv2/core/hal/interface.h:152
pub const CV_HAL_BORDER_REPLICATE: i32 = 1;
// CV_HAL_BORDER_TRANSPARENT /usr/include/opencv2/core/hal/interface.h:156
pub const CV_HAL_BORDER_TRANSPARENT: i32 = 5;
// CV_HAL_BORDER_WRAP /usr/include/opencv2/core/hal/interface.h:154
pub const CV_HAL_BORDER_WRAP: i32 = 3;
// CV_HAL_CMP_EQ /usr/include/opencv2/core/hal/interface.h:140
pub const CV_HAL_CMP_EQ: i32 = 0;
// CV_HAL_CMP_GE /usr/include/opencv2/core/hal/interface.h:142
pub const CV_HAL_CMP_GE: i32 = 2;
// CV_HAL_CMP_GT /usr/include/opencv2/core/hal/interface.h:141
pub const CV_HAL_CMP_GT: i32 = 1;
// CV_HAL_CMP_LE /usr/include/opencv2/core/hal/interface.h:144
pub const CV_HAL_CMP_LE: i32 = 4;
// CV_HAL_CMP_LT /usr/include/opencv2/core/hal/interface.h:143
pub const CV_HAL_CMP_LT: i32 = 3;
// CV_HAL_CMP_NE /usr/include/opencv2/core/hal/interface.h:145
pub const CV_HAL_CMP_NE: i32 = 5;
// CV_HAL_DFT_COMPLEX_OUTPUT /usr/include/opencv2/core/hal/interface.h:165
pub const CV_HAL_DFT_COMPLEX_OUTPUT: i32 = 16;
// CV_HAL_DFT_INVERSE /usr/include/opencv2/core/hal/interface.h:162
pub const CV_HAL_DFT_INVERSE: i32 = 1;
// CV_HAL_DFT_IS_CONTINUOUS /usr/include/opencv2/core/hal/interface.h:169
pub const CV_HAL_DFT_IS_CONTINUOUS: i32 = 512;
// CV_HAL_DFT_IS_INPLACE /usr/include/opencv2/core/hal/interface.h:170
pub const CV_HAL_DFT_IS_INPLACE: i32 = 1024;
// CV_HAL_DFT_REAL_OUTPUT /usr/include/opencv2/core/hal/interface.h:166
pub const CV_HAL_DFT_REAL_OUTPUT: i32 = 32;
// CV_HAL_DFT_ROWS /usr/include/opencv2/core/hal/interface.h:164
pub const CV_HAL_DFT_ROWS: i32 = 4;
// CV_HAL_DFT_SCALE /usr/include/opencv2/core/hal/interface.h:163
pub const CV_HAL_DFT_SCALE: i32 = 2;
// CV_HAL_DFT_STAGE_COLS /usr/include/opencv2/core/hal/interface.h:168
pub const CV_HAL_DFT_STAGE_COLS: i32 = 128;
// CV_HAL_DFT_TWO_STAGE /usr/include/opencv2/core/hal/interface.h:167
pub const CV_HAL_DFT_TWO_STAGE: i32 = 64;
// CV_HAL_ERROR_NOT_IMPLEMENTED /usr/include/opencv2/core/hal/interface.h:10
pub const CV_HAL_ERROR_NOT_IMPLEMENTED: i32 = 1;
// CV_HAL_ERROR_OK /usr/include/opencv2/core/hal/interface.h:9
pub const CV_HAL_ERROR_OK: i32 = 0;
// CV_HAL_ERROR_UNKNOWN /usr/include/opencv2/core/hal/interface.h:11
pub const CV_HAL_ERROR_UNKNOWN: i32 = -1;
// CV_HAL_GEMM_1_T /usr/include/opencv2/core/hal/interface.h:183
pub const CV_HAL_GEMM_1_T: i32 = 1;
// CV_HAL_GEMM_2_T /usr/include/opencv2/core/hal/interface.h:184
pub const CV_HAL_GEMM_2_T: i32 = 2;
// CV_HAL_GEMM_3_T /usr/include/opencv2/core/hal/interface.h:185
pub const CV_HAL_GEMM_3_T: i32 = 4;
// CV_HAL_SVD_FULL_UV /usr/include/opencv2/core/hal/interface.h:178
pub const CV_HAL_SVD_FULL_UV: i32 = 8;
// CV_HAL_SVD_MODIFY_A /usr/include/opencv2/core/hal/interface.h:177
pub const CV_HAL_SVD_MODIFY_A: i32 = 4;
// CV_HAL_SVD_NO_UV /usr/include/opencv2/core/hal/interface.h:175
pub const CV_HAL_SVD_NO_UV: i32 = 1;
// CV_HAL_SVD_SHORT_UV /usr/include/opencv2/core/hal/interface.h:176
pub const CV_HAL_SVD_SHORT_UV: i32 = 2;
// CV_HARDWARE_MAX_FEATURE /usr/include/opencv2/core/cvdef.h:291
pub const CV_HARDWARE_MAX_FEATURE: i32 = 512;
// CV_IMPL_IPP /usr/include/opencv2/core/utils/instrumentation.hpp:28
pub const CV_IMPL_IPP: i32 = 0x04;
// CV_IMPL_MT /usr/include/opencv2/core/utils/instrumentation.hpp:29
pub const CV_IMPL_MT: i32 = 0x10;
// CV_IMPL_OCL /usr/include/opencv2/core/utils/instrumentation.hpp:27
pub const CV_IMPL_OCL: i32 = 0x02;
// CV_IMPL_PLAIN /usr/include/opencv2/core/utils/instrumentation.hpp:26
pub const CV_IMPL_PLAIN: i32 = 0x01;
// CV_LOG2 /usr/include/opencv2/core/cvdef.h:362
pub const CV_LOG2: f64 = 0.69314718055994530941723212145818;
// CV_LOG_LEVEL_DEBUG /usr/include/opencv2/core/utils/logger.defines.hpp:17
pub const CV_LOG_LEVEL_DEBUG: i32 = 5;
// CV_LOG_LEVEL_ERROR /usr/include/opencv2/core/utils/logger.defines.hpp:14
pub const CV_LOG_LEVEL_ERROR: i32 = 2;
// CV_LOG_LEVEL_FATAL /usr/include/opencv2/core/utils/logger.defines.hpp:13
pub const CV_LOG_LEVEL_FATAL: i32 = 1;
// CV_LOG_LEVEL_INFO /usr/include/opencv2/core/utils/logger.defines.hpp:16
pub const CV_LOG_LEVEL_INFO: i32 = 4;
// CV_LOG_LEVEL_SILENT /usr/include/opencv2/core/utils/logger.defines.hpp:12
pub const CV_LOG_LEVEL_SILENT: i32 = 0;
// CV_LOG_LEVEL_VERBOSE /usr/include/opencv2/core/utils/logger.defines.hpp:18
pub const CV_LOG_LEVEL_VERBOSE: i32 = 6;
// CV_LOG_LEVEL_WARN /usr/include/opencv2/core/utils/logger.defines.hpp:15
pub const CV_LOG_LEVEL_WARN: i32 = 3;
// CV_LOG_STRIP_LEVEL /usr/include/opencv2/core/utils/logger.hpp:67
pub const CV_LOG_STRIP_LEVEL: i32 = CV_LOG_LEVEL_VERBOSE;
// CV_MAJOR_VERSION /usr/include/opencv2/core/version.hpp:22
pub const CV_MAJOR_VERSION: i32 = CV_VERSION_MAJOR;
// CV_MAT_CN_MASK /usr/include/opencv2/core/cvdef.h:470
pub const CV_MAT_CN_MASK: i32 = ((CV_CN_MAX-1)<<CV_CN_SHIFT);
// CV_MAT_CONT_FLAG /usr/include/opencv2/core/cvdef.h:475
pub const CV_MAT_CONT_FLAG: i32 = (1<<CV_MAT_CONT_FLAG_SHIFT);
// CV_MAT_CONT_FLAG_SHIFT /usr/include/opencv2/core/cvdef.h:474
pub const CV_MAT_CONT_FLAG_SHIFT: i32 = 14;
// CV_MAT_DEPTH_MASK /usr/include/opencv2/core/hal/interface.h:82
pub const CV_MAT_DEPTH_MASK: i32 = (CV_DEPTH_MAX-1);
// CV_MAT_TYPE_MASK /usr/include/opencv2/core/cvdef.h:472
pub const CV_MAT_TYPE_MASK: i32 = (CV_DEPTH_MAX*CV_CN_MAX-1);
// CV_MINOR_VERSION /usr/include/opencv2/core/version.hpp:23
pub const CV_MINOR_VERSION: i32 = CV_VERSION_MINOR;
// CV_MMX /usr/include/opencv2/core/cv_cpu_dispatch.h:211
pub const CV_MMX: i32 = 1;
// CV_MSA /usr/include/opencv2/core/cv_cpu_dispatch.h:359
pub const CV_MSA: i32 = 0;
// CV_NEON /usr/include/opencv2/core/cv_cpu_dispatch.h:343
pub const CV_NEON: i32 = 0;
// CV_PI /usr/include/opencv2/core/cvdef.h:360
pub const CV_PI: f64 = 3.1415926535897932384626433832795;
// CV_POPCNT /usr/include/opencv2/core/cv_cpu_dispatch.h:260
pub const CV_POPCNT: i32 = 0;
// CV_RVV /usr/include/opencv2/core/cv_cpu_dispatch.h:367
pub const CV_RVV: i32 = 0;
// CV_RVV071 /usr/include/opencv2/core/cv_cpu_dispatch.h:347
pub const CV_RVV071: i32 = 0;
// CV_SSE /usr/include/opencv2/core/cv_cpu_dispatch.h:212
pub const CV_SSE: i32 = 1;
// CV_SSE2 /usr/include/opencv2/core/cv_cpu_dispatch.h:213
pub const CV_SSE2: i32 = 1;
// CV_SSE3 /usr/include/opencv2/core/cv_cpu_dispatch.h:248
pub const CV_SSE3: i32 = 0;
// CV_SSE4_1 /usr/include/opencv2/core/cv_cpu_dispatch.h:254
pub const CV_SSE4_1: i32 = 0;
// CV_SSE4_2 /usr/include/opencv2/core/cv_cpu_dispatch.h:257
pub const CV_SSE4_2: i32 = 0;
// CV_SSSE3 /usr/include/opencv2/core/cv_cpu_dispatch.h:251
pub const CV_SSSE3: i32 = 0;
// CV_STRONG_ALIGNMENT /usr/include/opencv2/core/cvdef.h:356
pub const CV_STRONG_ALIGNMENT: i32 = 0;
// CV_SUBMAT_FLAG /usr/include/opencv2/core/cvdef.h:479
pub const CV_SUBMAT_FLAG: i32 = (1<<CV_SUBMAT_FLAG_SHIFT);
// CV_SUBMAT_FLAG_SHIFT /usr/include/opencv2/core/cvdef.h:478
pub const CV_SUBMAT_FLAG_SHIFT: i32 = 15;
// CV_SUBMINOR_VERSION /usr/include/opencv2/core/version.hpp:24
pub const CV_SUBMINOR_VERSION: i32 = CV_VERSION_REVISION;
// CV_VERSION_OCVRS_OVERRIDE /home/alex/git/rust/opencv-rust/src_cpp/core.hpp:22
pub const CV_VERSION: &str = "4.6.0";
// CV_VERSION_MAJOR /usr/include/opencv2/core/version.hpp:8
pub const CV_VERSION_MAJOR: i32 = 4;
// CV_VERSION_MINOR /usr/include/opencv2/core/version.hpp:9
pub const CV_VERSION_MINOR: i32 = 6;
// CV_VERSION_REVISION /usr/include/opencv2/core/version.hpp:10
pub const CV_VERSION_REVISION: i32 = 0;
// CV_VERSION_STATUS /usr/include/opencv2/core/version.hpp:11
pub const CV_VERSION_STATUS: &str = "";
// CV_VSX /usr/include/opencv2/core/cv_cpu_dispatch.h:351
pub const CV_VSX: i32 = 0;
// CV_VSX3 /usr/include/opencv2/core/cv_cpu_dispatch.h:355
pub const CV_VSX3: i32 = 0;
// CV_WASM_SIMD /usr/include/opencv2/core/cv_cpu_dispatch.h:363
pub const CV_WASM_SIMD: i32 = 0;
// CV__EXCEPTION_PTR /usr/include/opencv2/core/detail/exception_ptr.hpp:12
pub const CV__EXCEPTION_PTR: i32 = 1;
// DCT_INVERSE /usr/include/opencv2/core/base.hpp:258
pub const DCT_INVERSE: i32 = 1;
// DCT_ROWS /usr/include/opencv2/core/base.hpp:263
pub const DCT_ROWS: i32 = 4;
// DECOMP_CHOLESKY /usr/include/opencv2/core/base.hpp:143
pub const DECOMP_CHOLESKY: i32 = 3;
// DECOMP_EIG /usr/include/opencv2/core/base.hpp:140
pub const DECOMP_EIG: i32 = 2;
// DECOMP_LU /usr/include/opencv2/core/base.hpp:135
pub const DECOMP_LU: i32 = 0;
// DECOMP_NORMAL /usr/include/opencv2/core/base.hpp:151
pub const DECOMP_NORMAL: i32 = 16;
// DECOMP_QR /usr/include/opencv2/core/base.hpp:145
pub const DECOMP_QR: i32 = 4;
// DECOMP_SVD /usr/include/opencv2/core/base.hpp:138
pub const DECOMP_SVD: i32 = 1;
// DFT_COMPLEX_INPUT /usr/include/opencv2/core/base.hpp:256
pub const DFT_COMPLEX_INPUT: i32 = 64;
// DFT_COMPLEX_OUTPUT /usr/include/opencv2/core/base.hpp:243
pub const DFT_COMPLEX_OUTPUT: i32 = 16;
// DFT_INVERSE /usr/include/opencv2/core/base.hpp:228
pub const DFT_INVERSE: i32 = 1;
// DFT_REAL_OUTPUT /usr/include/opencv2/core/base.hpp:252
pub const DFT_REAL_OUTPUT: i32 = 32;
// DFT_ROWS /usr/include/opencv2/core/base.hpp:236
pub const DFT_ROWS: i32 = 4;
// DFT_SCALE /usr/include/opencv2/core/base.hpp:231
pub const DFT_SCALE: i32 = 2;
// DYNAMIC_PARALLELISM /usr/include/opencv2/core/cuda.hpp:1006
pub const DYNAMIC_PARALLELISM: i32 = 35;
// CV__LAST_TEST_OP /usr/include/opencv2/core/check.hpp:36
pub const Detail_CV__LAST_TEST_OP: i32 = 7;
// TEST_CUSTOM /usr/include/opencv2/core/check.hpp:29
pub const Detail_TEST_CUSTOM: i32 = 0;
// TEST_EQ /usr/include/opencv2/core/check.hpp:30
pub const Detail_TEST_EQ: i32 = 1;
// TEST_GE /usr/include/opencv2/core/check.hpp:34
pub const Detail_TEST_GE: i32 = 5;
// TEST_GT /usr/include/opencv2/core/check.hpp:35
pub const Detail_TEST_GT: i32 = 6;
// TEST_LE /usr/include/opencv2/core/check.hpp:32
pub const Detail_TEST_LE: i32 = 3;
// TEST_LT /usr/include/opencv2/core/check.hpp:33
pub const Detail_TEST_LT: i32 = 4;
// TEST_NE /usr/include/opencv2/core/check.hpp:31
pub const Detail_TEST_NE: i32 = 2;
// EXEC_KERNEL /usr/include/opencv2/core/ocl.hpp:135
pub const Device_EXEC_KERNEL: i32 = 1;
// EXEC_NATIVE_KERNEL /usr/include/opencv2/core/ocl.hpp:136
pub const Device_EXEC_NATIVE_KERNEL: i32 = 2;
// FP_CORRECTLY_ROUNDED_DIVIDE_SQRT /usr/include/opencv2/core/ocl.hpp:124
pub const Device_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT: i32 = 128;
// FP_DENORM /usr/include/opencv2/core/ocl.hpp:117
pub const Device_FP_DENORM: i32 = 1;
// FP_FMA /usr/include/opencv2/core/ocl.hpp:122
pub const Device_FP_FMA: i32 = 32;
// FP_INF_NAN /usr/include/opencv2/core/ocl.hpp:118
pub const Device_FP_INF_NAN: i32 = 2;
// FP_ROUND_TO_INF /usr/include/opencv2/core/ocl.hpp:121
pub const Device_FP_ROUND_TO_INF: i32 = 16;
// FP_ROUND_TO_NEAREST /usr/include/opencv2/core/ocl.hpp:119
pub const Device_FP_ROUND_TO_NEAREST: i32 = 4;
// FP_ROUND_TO_ZERO /usr/include/opencv2/core/ocl.hpp:120
pub const Device_FP_ROUND_TO_ZERO: i32 = 8;
// FP_SOFT_FLOAT /usr/include/opencv2/core/ocl.hpp:123
pub const Device_FP_SOFT_FLOAT: i32 = 64;
// LOCAL_IS_GLOBAL /usr/include/opencv2/core/ocl.hpp:157
pub const Device_LOCAL_IS_GLOBAL: i32 = 2;
// LOCAL_IS_LOCAL /usr/include/opencv2/core/ocl.hpp:156
pub const Device_LOCAL_IS_LOCAL: i32 = 1;
// NO_CACHE /usr/include/opencv2/core/ocl.hpp:144
pub const Device_NO_CACHE: i32 = 0;
// NO_LOCAL_MEM /usr/include/opencv2/core/ocl.hpp:155
pub const Device_NO_LOCAL_MEM: i32 = 0;
// READ_ONLY_CACHE /usr/include/opencv2/core/ocl.hpp:145
pub const Device_READ_ONLY_CACHE: i32 = 1;
// READ_WRITE_CACHE /usr/include/opencv2/core/ocl.hpp:146
pub const Device_READ_WRITE_CACHE: i32 = 2;
// TYPE_ACCELERATOR /usr/include/opencv2/core/ocl.hpp:90
pub const Device_TYPE_ACCELERATOR: i32 = 8;
// TYPE_ALL /usr/include/opencv2/core/ocl.hpp:93
pub const Device_TYPE_ALL: i32 = -1;
// TYPE_CPU /usr/include/opencv2/core/ocl.hpp:88
pub const Device_TYPE_CPU: i32 = 2;
// TYPE_DEFAULT /usr/include/opencv2/core/ocl.hpp:87
pub const Device_TYPE_DEFAULT: i32 = 1;
// TYPE_DGPU /usr/include/opencv2/core/ocl.hpp:91
pub const Device_TYPE_DGPU: i32 = 65540;
// TYPE_GPU /usr/include/opencv2/core/ocl.hpp:89
pub const Device_TYPE_GPU: i32 = 4;
// TYPE_IGPU /usr/include/opencv2/core/ocl.hpp:92
pub const Device_TYPE_IGPU: i32 = 131076;
// UNKNOWN_VENDOR /usr/include/opencv2/core/ocl.hpp:183
pub const Device_UNKNOWN_VENDOR: i32 = 0;
// VENDOR_AMD /usr/include/opencv2/core/ocl.hpp:184
pub const Device_VENDOR_AMD: i32 = 1;
// VENDOR_INTEL /usr/include/opencv2/core/ocl.hpp:185
pub const Device_VENDOR_INTEL: i32 = 2;
// VENDOR_NVIDIA /usr/include/opencv2/core/ocl.hpp:186
pub const Device_VENDOR_NVIDIA: i32 = 3;
// ENUM_LOG_LEVEL_FORCE_INT /usr/include/opencv2/core/utils/logger.defines.hpp:34
pub const ENUM_LOG_LEVEL_FORCE_INT: i32 = 2147483647;
// FEATURE_SET_COMPUTE_10 /usr/include/opencv2/core/cuda.hpp:991
pub const FEATURE_SET_COMPUTE_10: i32 = 10;
// FEATURE_SET_COMPUTE_11 /usr/include/opencv2/core/cuda.hpp:992
pub const FEATURE_SET_COMPUTE_11: i32 = 11;
// FEATURE_SET_COMPUTE_12 /usr/include/opencv2/core/cuda.hpp:993
pub const FEATURE_SET_COMPUTE_12: i32 = 12;
// FEATURE_SET_COMPUTE_13 /usr/include/opencv2/core/cuda.hpp:994
pub const FEATURE_SET_COMPUTE_13: i32 = 13;
// FEATURE_SET_COMPUTE_20 /usr/include/opencv2/core/cuda.hpp:995
pub const FEATURE_SET_COMPUTE_20: i32 = 20;
// FEATURE_SET_COMPUTE_21 /usr/include/opencv2/core/cuda.hpp:996
pub const FEATURE_SET_COMPUTE_21: i32 = 21;
// FEATURE_SET_COMPUTE_30 /usr/include/opencv2/core/cuda.hpp:997
pub const FEATURE_SET_COMPUTE_30: i32 = 30;
// FEATURE_SET_COMPUTE_32 /usr/include/opencv2/core/cuda.hpp:998
pub const FEATURE_SET_COMPUTE_32: i32 = 32;
// FEATURE_SET_COMPUTE_35 /usr/include/opencv2/core/cuda.hpp:999
pub const FEATURE_SET_COMPUTE_35: i32 = 35;
// FEATURE_SET_COMPUTE_50 /usr/include/opencv2/core/cuda.hpp:1000
pub const FEATURE_SET_COMPUTE_50: i32 = 50;
// FLAGS_EXPAND_SAME_NAMES /usr/include/opencv2/core/utils/instrumentation.hpp:112
pub const FLAGS_EXPAND_SAME_NAMES: i32 = 2;
// FLAGS_MAPPING /usr/include/opencv2/core/utils/instrumentation.hpp:111
pub const FLAGS_MAPPING: i32 = 1;
// FLAGS_NONE /usr/include/opencv2/core/utils/instrumentation.hpp:110
pub const FLAGS_NONE: i32 = 0;
// EMPTY /usr/include/opencv2/core/persistence.hpp:500
pub const FileNode_EMPTY: i32 = 16;
// FLOAT /usr/include/opencv2/core/persistence.hpp:490
pub const FileNode_FLOAT: i32 = 2;
// FLOW /usr/include/opencv2/core/persistence.hpp:497
pub const FileNode_FLOW: i32 = 8;
// INT /usr/include/opencv2/core/persistence.hpp:488
pub const FileNode_INT: i32 = 1;
// MAP /usr/include/opencv2/core/persistence.hpp:494
pub const FileNode_MAP: i32 = 5;
// NAMED /usr/include/opencv2/core/persistence.hpp:501
pub const FileNode_NAMED: i32 = 32;
// NONE /usr/include/opencv2/core/persistence.hpp:487
pub const FileNode_NONE: i32 = 0;
// REAL /usr/include/opencv2/core/persistence.hpp:489
pub const FileNode_REAL: i32 = 2;
// SEQ /usr/include/opencv2/core/persistence.hpp:493
pub const FileNode_SEQ: i32 = 4;
// STR /usr/include/opencv2/core/persistence.hpp:491
pub const FileNode_STR: i32 = 3;
// STRING /usr/include/opencv2/core/persistence.hpp:492
pub const FileNode_STRING: i32 = 3;
// TYPE_MASK /usr/include/opencv2/core/persistence.hpp:495
pub const FileNode_TYPE_MASK: i32 = 7;
// UNIFORM /usr/include/opencv2/core/persistence.hpp:498
pub const FileNode_UNIFORM: i32 = 8;
// GEMM_1_T /usr/include/opencv2/core/base.hpp:220
pub const GEMM_1_T: i32 = 1;
// GEMM_2_T /usr/include/opencv2/core/base.hpp:221
pub const GEMM_2_T: i32 = 2;
// GEMM_3_T /usr/include/opencv2/core/base.hpp:222
pub const GEMM_3_T: i32 = 4;
// GLOBAL_ATOMICS /usr/include/opencv2/core/cuda.hpp:1002
pub const GLOBAL_ATOMICS: i32 = 11;
// GpuApiCallError /usr/include/opencv2/core/base.hpp:117
pub const GpuApiCallError: i32 = -217;
// GpuNotSupported /usr/include/opencv2/core/base.hpp:116
pub const GpuNotSupported: i32 = -216;
// HeaderIsNull /usr/include/opencv2/core/base.hpp:78
pub const HeaderIsNull: i32 = -9;
// IMPL_IPP /usr/include/opencv2/core/utils/instrumentation.hpp:56
pub const IMPL_IPP: i32 = 1;
// IMPL_OPENCL /usr/include/opencv2/core/utils/instrumentation.hpp:57
pub const IMPL_OPENCL: i32 = 2;
// IMPL_PLAIN /usr/include/opencv2/core/utils/instrumentation.hpp:55
pub const IMPL_PLAIN: i32 = 0;
// KMEANS_PP_CENTERS /usr/include/opencv2/core.hpp:217
pub const KMEANS_PP_CENTERS: i32 = 2;
// KMEANS_RANDOM_CENTERS /usr/include/opencv2/core.hpp:215
pub const KMEANS_RANDOM_CENTERS: i32 = 0;
// KMEANS_USE_INITIAL_LABELS /usr/include/opencv2/core.hpp:222
pub const KMEANS_USE_INITIAL_LABELS: i32 = 1;
// CONSTANT /usr/include/opencv2/core/ocl.hpp:420
pub const KernelArg_CONSTANT: i32 = 8;
// LOCAL /usr/include/opencv2/core/ocl.hpp:420
pub const KernelArg_LOCAL: i32 = 1;
// NO_SIZE /usr/include/opencv2/core/ocl.hpp:420
pub const KernelArg_NO_SIZE: i32 = 256;
// PTR_ONLY /usr/include/opencv2/core/ocl.hpp:420
pub const KernelArg_PTR_ONLY: i32 = 16;
// READ_ONLY /usr/include/opencv2/core/ocl.hpp:420
pub const KernelArg_READ_ONLY: i32 = 2;
// READ_WRITE /usr/include/opencv2/core/ocl.hpp:420
pub const KernelArg_READ_WRITE: i32 = 6;
// WRITE_ONLY /usr/include/opencv2/core/ocl.hpp:420
pub const KernelArg_WRITE_ONLY: i32 = 4;
// LINES /usr/include/opencv2/core/opengl.hpp:480
pub const LINES: i32 = 1;
// LINE_LOOP /usr/include/opencv2/core/opengl.hpp:481
pub const LINE_LOOP: i32 = 2;
// LINE_STRIP /usr/include/opencv2/core/opengl.hpp:482
pub const LINE_STRIP: i32 = 3;
// LOG_LEVEL_DEBUG /usr/include/opencv2/core/utils/logger.defines.hpp:31
pub const LOG_LEVEL_DEBUG: i32 = 5;
// LOG_LEVEL_ERROR /usr/include/opencv2/core/utils/logger.defines.hpp:28
pub const LOG_LEVEL_ERROR: i32 = 2;
// LOG_LEVEL_FATAL /usr/include/opencv2/core/utils/logger.defines.hpp:27
pub const LOG_LEVEL_FATAL: i32 = 1;
// LOG_LEVEL_INFO /usr/include/opencv2/core/utils/logger.defines.hpp:30
pub const LOG_LEVEL_INFO: i32 = 4;
// LOG_LEVEL_SILENT /usr/include/opencv2/core/utils/logger.defines.hpp:26
pub const LOG_LEVEL_SILENT: i32 = 0;
// LOG_LEVEL_VERBOSE /usr/include/opencv2/core/utils/logger.defines.hpp:32
pub const LOG_LEVEL_VERBOSE: i32 = 6;
// LOG_LEVEL_WARNING /usr/include/opencv2/core/utils/logger.defines.hpp:29
pub const LOG_LEVEL_WARNING: i32 = 3;
// MaskIsTiled /usr/include/opencv2/core/base.hpp:95
pub const MaskIsTiled: i32 = -26;
// AUTO_STEP /usr/include/opencv2/core/mat.hpp:2103
pub const Mat_AUTO_STEP: usize = 0;
// CONTINUOUS_FLAG /usr/include/opencv2/core/mat.hpp:2103
pub const Mat_CONTINUOUS_FLAG: i32 = 16384;
// DEPTH_MASK /usr/include/opencv2/core/mat.hpp:2104
pub const Mat_DEPTH_MASK: i32 = 7;
// MAGIC_MASK /usr/include/opencv2/core/mat.hpp:2104
pub const Mat_MAGIC_MASK: i32 = -65536;
// MAGIC_VAL /usr/include/opencv2/core/mat.hpp:2103
pub const Mat_MAGIC_VAL: i32 = 1124007936;
// SUBMATRIX_FLAG /usr/include/opencv2/core/mat.hpp:2103
pub const Mat_SUBMATRIX_FLAG: i32 = 32768;
// TYPE_MASK /usr/include/opencv2/core/mat.hpp:2104
pub const Mat_TYPE_MASK: i32 = 4095;
// NATIVE_DOUBLE /usr/include/opencv2/core/cuda.hpp:1004
pub const NATIVE_DOUBLE: i32 = 13;
// NORM_HAMMING /usr/include/opencv2/core/base.hpp:199
pub const NORM_HAMMING: i32 = 6;
// NORM_HAMMING2 /usr/include/opencv2/core/base.hpp:204
pub const NORM_HAMMING2: i32 = 7;
// NORM_INF /usr/include/opencv2/core/base.hpp:168
pub const NORM_INF: i32 = 1;
// NORM_L1 /usr/include/opencv2/core/base.hpp:176
pub const NORM_L1: i32 = 2;
// NORM_L2 /usr/include/opencv2/core/base.hpp:185
pub const NORM_L2: i32 = 4;
// NORM_L2SQR /usr/include/opencv2/core/base.hpp:194
pub const NORM_L2SQR: i32 = 5;
// NORM_MINMAX /usr/include/opencv2/core/base.hpp:207
pub const NORM_MINMAX: i32 = 32;
// NORM_RELATIVE /usr/include/opencv2/core/base.hpp:206
pub const NORM_RELATIVE: i32 = 8;
// NORM_TYPE_MASK /usr/include/opencv2/core/base.hpp:205
pub const NORM_TYPE_MASK: i32 = 7;
// OCL_VECTOR_DEFAULT /usr/include/opencv2/core/ocl.hpp:718
pub const OCL_VECTOR_DEFAULT: i32 = 0;
// OCL_VECTOR_MAX /usr/include/opencv2/core/ocl.hpp:715
pub const OCL_VECTOR_MAX: i32 = 1;
// OCL_VECTOR_OWN /usr/include/opencv2/core/ocl.hpp:712
pub const OCL_VECTOR_OWN: i32 = 0;
// OPENCV_ABI_COMPATIBILITY /usr/include/opencv2/core/cvdef.h:398
pub const OPENCV_ABI_COMPATIBILITY: i32 = 400;
// OPENCV_USE_FASTMATH_BUILTINS /usr/include/opencv2/core/fast_math.hpp:139
pub const OPENCV_USE_FASTMATH_BUILTINS: i32 = 1;
// OpenCLApiCallError /usr/include/opencv2/core/base.hpp:120
pub const OpenCLApiCallError: i32 = -220;
// OpenCLDoubleNotSupported /usr/include/opencv2/core/base.hpp:121
pub const OpenCLDoubleNotSupported: i32 = -221;
// OpenCLInitError /usr/include/opencv2/core/base.hpp:122
pub const OpenCLInitError: i32 = -222;
// OpenCLNoAMDBlasFft /usr/include/opencv2/core/base.hpp:123
pub const OpenCLNoAMDBlasFft: i32 = -223;
// OpenGlApiCallError /usr/include/opencv2/core/base.hpp:119
pub const OpenGlApiCallError: i32 = -219;
// OpenGlNotSupported /usr/include/opencv2/core/base.hpp:118
pub const OpenGlNotSupported: i32 = -218;
// POINTS /usr/include/opencv2/core/opengl.hpp:479
pub const POINTS: i32 = 0;
// POLYGON /usr/include/opencv2/core/opengl.hpp:488
pub const POLYGON: i32 = 9;
// ALGORITHM /usr/include/opencv2/core.hpp:3233
pub const Param_ALGORITHM: i32 = 6;
// BOOLEAN /usr/include/opencv2/core.hpp:3233
pub const Param_BOOLEAN: i32 = 1;
// FLOAT /usr/include/opencv2/core.hpp:3233
pub const Param_FLOAT: i32 = 7;
// INT /usr/include/opencv2/core.hpp:3233
pub const Param_INT: i32 = 0;
// MAT /usr/include/opencv2/core.hpp:3233
pub const Param_MAT: i32 = 4;
// MAT_VECTOR /usr/include/opencv2/core.hpp:3233
pub const Param_MAT_VECTOR: i32 = 5;
// REAL /usr/include/opencv2/core.hpp:3233
pub const Param_REAL: i32 = 2;
// SCALAR /usr/include/opencv2/core.hpp:3234
pub const Param_SCALAR: i32 = 12;
// STRING /usr/include/opencv2/core.hpp:3233
pub const Param_STRING: i32 = 3;
// UCHAR /usr/include/opencv2/core.hpp:3234
pub const Param_UCHAR: i32 = 11;
// UINT64 /usr/include/opencv2/core.hpp:3234
pub const Param_UINT64: i32 = 9;
// UNSIGNED_INT /usr/include/opencv2/core.hpp:3234
pub const Param_UNSIGNED_INT: i32 = 8;
// QUADS /usr/include/opencv2/core/opengl.hpp:486
pub const QUADS: i32 = 7;
// QUAD_STRIP /usr/include/opencv2/core/opengl.hpp:487
pub const QUAD_STRIP: i32 = 8;
// REDUCE_AVG /usr/include/opencv2/core.hpp:231
pub const REDUCE_AVG: i32 = 1;
// REDUCE_MAX /usr/include/opencv2/core.hpp:232
pub const REDUCE_MAX: i32 = 2;
// REDUCE_MIN /usr/include/opencv2/core.hpp:233
pub const REDUCE_MIN: i32 = 3;
// REDUCE_SUM /usr/include/opencv2/core.hpp:230
pub const REDUCE_SUM: i32 = 0;
// NORMAL /usr/include/opencv2/core.hpp:2829
pub const RNG_NORMAL: i32 = 1;
// UNIFORM /usr/include/opencv2/core.hpp:2828
pub const RNG_UNIFORM: i32 = 0;
// ROTATE_180 /usr/include/opencv2/core.hpp:1107
pub const ROTATE_180: i32 = 1;
// ROTATE_90_CLOCKWISE /usr/include/opencv2/core.hpp:1106
pub const ROTATE_90_CLOCKWISE: i32 = 0;
// ROTATE_90_COUNTERCLOCKWISE /usr/include/opencv2/core.hpp:1108
pub const ROTATE_90_COUNTERCLOCKWISE: i32 = 2;
// SHARED_ATOMICS /usr/include/opencv2/core/cuda.hpp:1003
pub const SHARED_ATOMICS: i32 = 12;
// SOLVELP_MULTI /usr/include/opencv2/core/optim.hpp:262
pub const SOLVELP_MULTI: i32 = 1;
// SOLVELP_SINGLE /usr/include/opencv2/core/optim.hpp:261
pub const SOLVELP_SINGLE: i32 = 0;
// SOLVELP_UNBOUNDED /usr/include/opencv2/core/optim.hpp:259
pub const SOLVELP_UNBOUNDED: i32 = -2;
// SOLVELP_UNFEASIBLE /usr/include/opencv2/core/optim.hpp:260
pub const SOLVELP_UNFEASIBLE: i32 = -1;
// SORT_ASCENDING /usr/include/opencv2/core.hpp:161
pub const SORT_ASCENDING: i32 = 0;
// SORT_DESCENDING /usr/include/opencv2/core.hpp:163
pub const SORT_DESCENDING: i32 = 16;
// SORT_EVERY_COLUMN /usr/include/opencv2/core.hpp:158
pub const SORT_EVERY_COLUMN: i32 = 1;
// SORT_EVERY_ROW /usr/include/opencv2/core.hpp:157
pub const SORT_EVERY_ROW: i32 = 0;
// HASH_BIT /usr/include/opencv2/core/mat.hpp:2718
pub const SparseMat_HASH_BIT: i32 = -2147483648;
// HASH_SCALE /usr/include/opencv2/core/mat.hpp:2718
pub const SparseMat_HASH_SCALE: i32 = 1540483477;
// MAGIC_VAL /usr/include/opencv2/core/mat.hpp:2718
pub const SparseMat_MAGIC_VAL: i32 = 1123876864;
// MAX_DIM /usr/include/opencv2/core/mat.hpp:2718
pub const SparseMat_MAX_DIM: i32 = 32;
// StsAssert /usr/include/opencv2/core/base.hpp:115
pub const StsAssert: i32 = -215;
// StsAutoTrace /usr/include/opencv2/core/base.hpp:77
pub const StsAutoTrace: i32 = -8;
// StsBackTrace /usr/include/opencv2/core/base.hpp:70
pub const StsBackTrace: i32 = -1;
// StsBadArg /usr/include/opencv2/core/base.hpp:74
pub const StsBadArg: i32 = -5;
// StsBadFlag /usr/include/opencv2/core/base.hpp:106
pub const StsBadFlag: i32 = -206;
// StsBadFunc /usr/include/opencv2/core/base.hpp:75
pub const StsBadFunc: i32 = -6;
// StsBadMask /usr/include/opencv2/core/base.hpp:108
pub const StsBadMask: i32 = -208;
// StsBadMemBlock /usr/include/opencv2/core/base.hpp:114
pub const StsBadMemBlock: i32 = -214;
// StsBadPoint /usr/include/opencv2/core/base.hpp:107
pub const StsBadPoint: i32 = -207;
// StsBadSize /usr/include/opencv2/core/base.hpp:101
pub const StsBadSize: i32 = -201;
// StsDivByZero /usr/include/opencv2/core/base.hpp:102
pub const StsDivByZero: i32 = -202;
// StsError /usr/include/opencv2/core/base.hpp:71
pub const StsError: i32 = -2;
// StsFilterOffsetErr /usr/include/opencv2/core/base.hpp:100
pub const StsFilterOffsetErr: i32 = -31;
// StsFilterStructContentErr /usr/include/opencv2/core/base.hpp:98
pub const StsFilterStructContentErr: i32 = -29;
// StsInplaceNotSupported /usr/include/opencv2/core/base.hpp:103
pub const StsInplaceNotSupported: i32 = -203;
// StsInternal /usr/include/opencv2/core/base.hpp:72
pub const StsInternal: i32 = -3;
// StsKernelStructContentErr /usr/include/opencv2/core/base.hpp:99
pub const StsKernelStructContentErr: i32 = -30;
// StsNoConv /usr/include/opencv2/core/base.hpp:76
pub const StsNoConv: i32 = -7;
// StsNoMem /usr/include/opencv2/core/base.hpp:73
pub const StsNoMem: i32 = -4;
// StsNotImplemented /usr/include/opencv2/core/base.hpp:113
pub const StsNotImplemented: i32 = -213;
// StsNullPtr /usr/include/opencv2/core/base.hpp:96
pub const StsNullPtr: i32 = -27;
// StsObjectNotFound /usr/include/opencv2/core/base.hpp:104
pub const StsObjectNotFound: i32 = -204;
// StsOk /usr/include/opencv2/core/base.hpp:69
pub const StsOk: i32 = 0;
// StsOutOfRange /usr/include/opencv2/core/base.hpp:111
pub const StsOutOfRange: i32 = -211;
// StsParseError /usr/include/opencv2/core/base.hpp:112
pub const StsParseError: i32 = -212;
// StsUnmatchedFormats /usr/include/opencv2/core/base.hpp:105
pub const StsUnmatchedFormats: i32 = -205;
// StsUnmatchedSizes /usr/include/opencv2/core/base.hpp:109
pub const StsUnmatchedSizes: i32 = -209;
// StsUnsupportedFormat /usr/include/opencv2/core/base.hpp:110
pub const StsUnsupportedFormat: i32 = -210;
// StsVecLengthErr /usr/include/opencv2/core/base.hpp:97
pub const StsVecLengthErr: i32 = -28;
// TRIANGLES /usr/include/opencv2/core/opengl.hpp:483
pub const TRIANGLES: i32 = 4;
// TRIANGLE_FAN /usr/include/opencv2/core/opengl.hpp:485
pub const TRIANGLE_FAN: i32 = 6;
// TRIANGLE_STRIP /usr/include/opencv2/core/opengl.hpp:484
pub const TRIANGLE_STRIP: i32 = 5;
// TYPE_FUN /usr/include/opencv2/core/utils/instrumentation.hpp:50
pub const TYPE_FUN: i32 = 3;
// TYPE_GENERAL /usr/include/opencv2/core/utils/instrumentation.hpp:47
pub const TYPE_GENERAL: i32 = 0;
// TYPE_MARKER /usr/include/opencv2/core/utils/instrumentation.hpp:48
pub const TYPE_MARKER: i32 = 1;
// TYPE_WRAPPER /usr/include/opencv2/core/utils/instrumentation.hpp:49
pub const TYPE_WRAPPER: i32 = 2;
// AUTO_STEP /usr/include/opencv2/core/mat.hpp:2583
pub const UMat_AUTO_STEP: i32 = 0;
// CONTINUOUS_FLAG /usr/include/opencv2/core/mat.hpp:2583
pub const UMat_CONTINUOUS_FLAG: i32 = 16384;
// DEPTH_MASK /usr/include/opencv2/core/mat.hpp:2584
pub const UMat_DEPTH_MASK: i32 = 7;
// MAGIC_MASK /usr/include/opencv2/core/mat.hpp:2584
pub const UMat_MAGIC_MASK: i32 = -65536;
// MAGIC_VAL /usr/include/opencv2/core/mat.hpp:2583
pub const UMat_MAGIC_VAL: i32 = 1124007936;
// SUBMATRIX_FLAG /usr/include/opencv2/core/mat.hpp:2583
pub const UMat_SUBMATRIX_FLAG: i32 = 32768;
// TYPE_MASK /usr/include/opencv2/core/mat.hpp:2584
pub const UMat_TYPE_MASK: i32 = 4095;
// USAGE_ALLOCATE_DEVICE_MEMORY /usr/include/opencv2/core/mat.hpp:468
pub const USAGE_ALLOCATE_DEVICE_MEMORY: i32 = 2;
// USAGE_ALLOCATE_HOST_MEMORY /usr/include/opencv2/core/mat.hpp:467
pub const USAGE_ALLOCATE_HOST_MEMORY: i32 = 1;
// USAGE_ALLOCATE_SHARED_MEMORY /usr/include/opencv2/core/mat.hpp:469
pub const USAGE_ALLOCATE_SHARED_MEMORY: i32 = 4;
// USAGE_DEFAULT /usr/include/opencv2/core/mat.hpp:464
pub const USAGE_DEFAULT: i32 = 0;
// WARP_SHUFFLE_FUNCTIONS /usr/include/opencv2/core/cuda.hpp:1005
pub const WARP_SHUFFLE_FUNCTIONS: i32 = 30;
// __UMAT_USAGE_FLAGS_32BIT /usr/include/opencv2/core/mat.hpp:471
pub const __UMAT_USAGE_FLAGS_32BIT: i32 = 2147483647;
// AccessFlag /usr/include/opencv2/core/mat.hpp:64
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AccessFlag {
	ACCESS_READ = 16777216,
	ACCESS_WRITE = 33554432,
	ACCESS_RW = 50331648,
	// ACCESS_MASK = 50331648 as isize, // duplicate discriminant
	ACCESS_FAST = 67108864,
}

opencv_type_enum! { core::AccessFlag }

// BorderTypes /usr/include/opencv2/core/base.hpp:268
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BorderTypes {
	BORDER_CONSTANT = 0,
	BORDER_REPLICATE = 1,
	BORDER_REFLECT = 2,
	BORDER_WRAP = 3,
	BORDER_REFLECT_101 = 4,
	BORDER_TRANSPARENT = 5,
	// BORDER_REFLECT101 = 4 as isize, // duplicate discriminant
	// BORDER_DEFAULT = 4 as isize, // duplicate discriminant
	BORDER_ISOLATED = 16,
}

opencv_type_enum! { core::BorderTypes }

// Access /usr/include/opencv2/core/opengl.hpp:92
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Buffer_Access {
	READ_ONLY = 35000,
	WRITE_ONLY = 35001,
	READ_WRITE = 35002,
}

opencv_type_enum! { core::Buffer_Access }

// Target /usr/include/opencv2/core/opengl.hpp:84
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Buffer_Target {
	ARRAY_BUFFER = 34962,
	ELEMENT_ARRAY_BUFFER = 34963,
	PIXEL_PACK_BUFFER = 35051,
	PIXEL_UNPACK_BUFFER = 35052,
}

opencv_type_enum! { core::Buffer_Target }

// CmpTypes /usr/include/opencv2/core/base.hpp:211
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CmpTypes {
	CMP_EQ = 0,
	CMP_GT = 1,
	CMP_GE = 2,
	CMP_LT = 3,
	CMP_LE = 4,
	CMP_NE = 5,
}

opencv_type_enum! { core::CmpTypes }

// Code /usr/include/opencv2/core/base.hpp:68
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Code {
	StsOk = 0,
	StsBackTrace = -1,
	StsError = -2,
	StsInternal = -3,
	StsNoMem = -4,
	StsBadArg = -5,
	StsBadFunc = -6,
	StsNoConv = -7,
	StsAutoTrace = -8,
	HeaderIsNull = -9,
	BadImageSize = -10,
	BadOffset = -11,
	BadDataPtr = -12,
	BadStep = -13,
	BadModelOrChSeq = -14,
	BadNumChannels = -15,
	BadNumChannel1U = -16,
	BadDepth = -17,
	BadAlphaChannel = -18,
	BadOrder = -19,
	BadOrigin = -20,
	BadAlign = -21,
	BadCallBack = -22,
	BadTileSize = -23,
	BadCOI = -24,
	BadROISize = -25,
	MaskIsTiled = -26,
	StsNullPtr = -27,
	StsVecLengthErr = -28,
	StsFilterStructContentErr = -29,
	StsKernelStructContentErr = -30,
	StsFilterOffsetErr = -31,
	StsBadSize = -201,
	StsDivByZero = -202,
	StsInplaceNotSupported = -203,
	StsObjectNotFound = -204,
	StsUnmatchedFormats = -205,
	StsBadFlag = -206,
	StsBadPoint = -207,
	StsBadMask = -208,
	StsUnmatchedSizes = -209,
	StsUnsupportedFormat = -210,
	StsOutOfRange = -211,
	StsParseError = -212,
	StsNotImplemented = -213,
	StsBadMemBlock = -214,
	StsAssert = -215,
	GpuNotSupported = -216,
	GpuApiCallError = -217,
	OpenGlNotSupported = -218,
	OpenGlApiCallError = -219,
	OpenCLApiCallError = -220,
	OpenCLDoubleNotSupported = -221,
	OpenCLInitError = -222,
	OpenCLNoAMDBlasFft = -223,
}

opencv_type_enum! { core::Code }

// CovarFlags /usr/include/opencv2/core.hpp:174
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CovarFlags {
	COVAR_SCRAMBLED = 0,
	COVAR_NORMAL = 1,
	COVAR_USE_AVG = 2,
	COVAR_SCALE = 4,
	COVAR_ROWS = 8,
	COVAR_COLS = 16,
}

opencv_type_enum! { core::CovarFlags }

// CpuFeatures /usr/include/opencv2/core/cvdef.h:295
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CpuFeatures {
	CPU_MMX = 1,
	CPU_SSE = 2,
	CPU_SSE2 = 3,
	CPU_SSE3 = 4,
	CPU_SSSE3 = 5,
	CPU_SSE4_1 = 6,
	CPU_SSE4_2 = 7,
	CPU_POPCNT = 8,
	CPU_FP16 = 9,
	CPU_AVX = 10,
	CPU_AVX2 = 11,
	CPU_FMA3 = 12,
	CPU_AVX_512F = 13,
	CPU_AVX_512BW = 14,
	CPU_AVX_512CD = 15,
	CPU_AVX_512DQ = 16,
	CPU_AVX_512ER = 17,
	CPU_AVX_512IFMA512 = 18,
	// CPU_AVX_512IFMA = 18 as isize, // duplicate discriminant
	CPU_AVX_512PF = 19,
	CPU_AVX_512VBMI = 20,
	CPU_AVX_512VL = 21,
	CPU_AVX_512VBMI2 = 22,
	CPU_AVX_512VNNI = 23,
	CPU_AVX_512BITALG = 24,
	CPU_AVX_512VPOPCNTDQ = 25,
	CPU_AVX_5124VNNIW = 26,
	CPU_AVX_5124FMAPS = 27,
	CPU_NEON = 100,
	CPU_MSA = 150,
	CPU_RISCVV = 170,
	CPU_VSX = 200,
	CPU_VSX3 = 201,
	CPU_RVV = 210,
	CPU_AVX512_SKX = 256,
	CPU_AVX512_COMMON = 257,
	CPU_AVX512_KNL = 258,
	CPU_AVX512_KNM = 259,
	CPU_AVX512_CNL = 260,
	CPU_AVX512_CLX = 261,
	CPU_AVX512_ICL = 262,
	CPU_MAX_FEATURE = 512,
}

opencv_type_enum! { core::CpuFeatures }

// DecompTypes /usr/include/opencv2/core/base.hpp:133
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DecompTypes {
	DECOMP_LU = 0,
	DECOMP_SVD = 1,
	DECOMP_EIG = 2,
	DECOMP_CHOLESKY = 3,
	DECOMP_QR = 4,
	DECOMP_NORMAL = 16,
}

opencv_type_enum! { core::DecompTypes }

// TestOp /usr/include/opencv2/core/check.hpp:28
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Detail_TestOp {
	TEST_CUSTOM = 0,
	TEST_EQ = 1,
	TEST_NE = 2,
	TEST_LE = 3,
	TEST_LT = 4,
	TEST_GE = 5,
	TEST_GT = 6,
	CV__LAST_TEST_OP = 7,
}

opencv_type_enum! { core::Detail_TestOp }

// ComputeMode /usr/include/opencv2/core/cuda.hpp:1121
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeviceInfo_ComputeMode {
	ComputeModeDefault = 0,
	ComputeModeExclusive = 1,
	ComputeModeProhibited = 2,
	ComputeModeExclusiveProcess = 3,
}

opencv_type_enum! { core::DeviceInfo_ComputeMode }

// DftFlags /usr/include/opencv2/core/base.hpp:225
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DftFlags {
	DFT_INVERSE = 1,
	DFT_SCALE = 2,
	DFT_ROWS = 4,
	DFT_COMPLEX_OUTPUT = 16,
	DFT_REAL_OUTPUT = 32,
	DFT_COMPLEX_INPUT = 64,
	// DCT_INVERSE = 1 as isize, // duplicate discriminant
	// DCT_ROWS = 4 as isize, // duplicate discriminant
}

opencv_type_enum! { core::DftFlags }

// CreateFlags /usr/include/opencv2/core/cuda.hpp:919
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Event_CreateFlags {
	DEFAULT = 0,
	BLOCKING_SYNC = 1,
	DISABLE_TIMING = 2,
	INTERPROCESS = 4,
}

opencv_type_enum! { core::Event_CreateFlags }

// FLAGS /usr/include/opencv2/core/utils/instrumentation.hpp:108
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FLAGS {
	FLAGS_NONE = 0,
	FLAGS_MAPPING = 1,
	FLAGS_EXPAND_SAME_NAMES = 2,
}

opencv_type_enum! { core::FLAGS }

// FeatureSet /usr/include/opencv2/core/cuda.hpp:989
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FeatureSet {
	FEATURE_SET_COMPUTE_10 = 10,
	FEATURE_SET_COMPUTE_11 = 11,
	FEATURE_SET_COMPUTE_12 = 12,
	FEATURE_SET_COMPUTE_13 = 13,
	FEATURE_SET_COMPUTE_20 = 20,
	FEATURE_SET_COMPUTE_21 = 21,
	FEATURE_SET_COMPUTE_30 = 30,
	FEATURE_SET_COMPUTE_32 = 32,
	FEATURE_SET_COMPUTE_35 = 35,
	FEATURE_SET_COMPUTE_50 = 50,
	// GLOBAL_ATOMICS = 11 as isize, // duplicate discriminant
	// SHARED_ATOMICS = 12 as isize, // duplicate discriminant
	// NATIVE_DOUBLE = 13 as isize, // duplicate discriminant
	// WARP_SHUFFLE_FUNCTIONS = 30 as isize, // duplicate discriminant
	// DYNAMIC_PARALLELISM = 35 as isize, // duplicate discriminant
}

opencv_type_enum! { core::FeatureSet }

// Mode /usr/include/opencv2/core/persistence.hpp:307
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FileStorage_Mode {
	READ = 0,
	WRITE = 1,
	APPEND = 2,
	MEMORY = 4,
	FORMAT_MASK = 56,
	// FORMAT_AUTO = 0 as isize, // duplicate discriminant
	FORMAT_XML = 8,
	FORMAT_YAML = 16,
	FORMAT_JSON = 24,
	BASE64 = 64,
	WRITE_BASE64 = 65,
}

opencv_type_enum! { core::FileStorage_Mode }

// State /usr/include/opencv2/core/persistence.hpp:323
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FileStorage_State {
	UNDEFINED = 0,
	VALUE_EXPECTED = 1,
	NAME_EXPECTED = 2,
	INSIDE_MAP = 4,
}

opencv_type_enum! { core::FileStorage_State }

// FormatType /usr/include/opencv2/core.hpp:3078
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Formatter_FormatType {
	FMT_DEFAULT = 0,
	FMT_MATLAB = 1,
	FMT_CSV = 2,
	FMT_PYTHON = 3,
	FMT_NUMPY = 4,
	FMT_C = 5,
}

opencv_type_enum! { core::Formatter_FormatType }

// GemmFlags /usr/include/opencv2/core/base.hpp:220
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GemmFlags {
	GEMM_1_T = 1,
	GEMM_2_T = 2,
	GEMM_3_T = 4,
}

opencv_type_enum! { core::GemmFlags }

// AllocType /usr/include/opencv2/core/cuda.hpp:733
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HostMem_AllocType {
	PAGE_LOCKED = 1,
	SHARED = 2,
	WRITE_COMBINED = 4,
}

opencv_type_enum! { core::HostMem_AllocType }

// IMPL /usr/include/opencv2/core/utils/instrumentation.hpp:53
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IMPL {
	IMPL_PLAIN = 0,
	IMPL_IPP = 1,
	IMPL_OPENCL = 2,
}

opencv_type_enum! { core::IMPL }

// KmeansFlags /usr/include/opencv2/core.hpp:213
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KmeansFlags {
	KMEANS_RANDOM_CENTERS = 0,
	KMEANS_PP_CENTERS = 2,
	KMEANS_USE_INITIAL_LABELS = 1,
}

opencv_type_enum! { core::KmeansFlags }

// LogLevel /usr/include/opencv2/core/utils/logger.defines.hpp:25
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogLevel {
	LOG_LEVEL_SILENT = 0,
	LOG_LEVEL_FATAL = 1,
	LOG_LEVEL_ERROR = 2,
	LOG_LEVEL_WARNING = 3,
	LOG_LEVEL_INFO = 4,
	LOG_LEVEL_DEBUG = 5,
	LOG_LEVEL_VERBOSE = 6,
	ENUM_LOG_LEVEL_FORCE_INT = 2147483647,
}

opencv_type_enum! { core::LogLevel }

// NormTypes /usr/include/opencv2/core/base.hpp:159
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NormTypes {
	NORM_INF = 1,
	NORM_L1 = 2,
	NORM_L2 = 4,
	NORM_L2SQR = 5,
	NORM_HAMMING = 6,
	NORM_HAMMING2 = 7,
	// NORM_TYPE_MASK = 7 as isize, // duplicate discriminant
	NORM_RELATIVE = 8,
	NORM_MINMAX = 32,
}

opencv_type_enum! { core::NormTypes }

// OclVectorStrategy /usr/include/opencv2/core/ocl.hpp:709
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OclVectorStrategy {
	OCL_VECTOR_OWN = 0,
	OCL_VECTOR_MAX = 1,
	// OCL_VECTOR_DEFAULT = 0 as isize, // duplicate discriminant
}

opencv_type_enum! { core::OclVectorStrategy }

// Flags /usr/include/opencv2/core.hpp:2452
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PCA_Flags {
	DATA_AS_ROW = 0,
	DATA_AS_COL = 1,
	USE_AVG = 2,
}

opencv_type_enum! { core::PCA_Flags }

// Param /usr/include/opencv2/core.hpp:3232
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Param {
	INT = 0,
	BOOLEAN = 1,
	REAL = 2,
	STRING = 3,
	MAT = 4,
	MAT_VECTOR = 5,
	ALGORITHM = 6,
	FLOAT = 7,
	UNSIGNED_INT = 8,
	UINT64 = 9,
	UCHAR = 11,
	SCALAR = 12,
}

opencv_type_enum! { core::Param }

// ReduceTypes /usr/include/opencv2/core.hpp:230
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReduceTypes {
	REDUCE_SUM = 0,
	REDUCE_AVG = 1,
	REDUCE_MAX = 2,
	REDUCE_MIN = 3,
}

opencv_type_enum! { core::ReduceTypes }

// RenderModes /usr/include/opencv2/core/opengl.hpp:478
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RenderModes {
	POINTS = 0,
	LINES = 1,
	LINE_LOOP = 2,
	LINE_STRIP = 3,
	TRIANGLES = 4,
	TRIANGLE_STRIP = 5,
	TRIANGLE_FAN = 6,
	QUADS = 7,
	QUAD_STRIP = 8,
	POLYGON = 9,
}

opencv_type_enum! { core::RenderModes }

// RotateFlags /usr/include/opencv2/core.hpp:1105
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RotateFlags {
	ROTATE_90_CLOCKWISE = 0,
	ROTATE_180 = 1,
	ROTATE_90_COUNTERCLOCKWISE = 2,
}

opencv_type_enum! { core::RotateFlags }

// Flags /usr/include/opencv2/core.hpp:2689
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SVD_Flags {
	MODIFY_A = 1,
	NO_UV = 2,
	FULL_UV = 4,
}

opencv_type_enum! { core::SVD_Flags }

// SolveLPResult /usr/include/opencv2/core/optim.hpp:257
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SolveLPResult {
	SOLVELP_UNBOUNDED = -2,
	SOLVELP_UNFEASIBLE = -1,
	SOLVELP_SINGLE = 0,
	SOLVELP_MULTI = 1,
}

opencv_type_enum! { core::SolveLPResult }

// SortFlags /usr/include/opencv2/core.hpp:157
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SortFlags {
	SORT_EVERY_ROW = 0,
	SORT_EVERY_COLUMN = 1,
	// SORT_ASCENDING = 0 as isize, // duplicate discriminant
	SORT_DESCENDING = 16,
}

opencv_type_enum! { core::SortFlags }

// TYPE /usr/include/opencv2/core/utils/instrumentation.hpp:45
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TYPE {
	TYPE_GENERAL = 0,
	TYPE_MARKER = 1,
	TYPE_WRAPPER = 2,
	TYPE_FUN = 3,
}

opencv_type_enum! { core::TYPE }

// Type /usr/include/opencv2/core/types.hpp:880
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TermCriteria_Type {
	COUNT = 1,
	// MAX_ITER = 1 as isize, // duplicate discriminant
	EPS = 2,
}

opencv_type_enum! { core::TermCriteria_Type }

// Format /usr/include/opencv2/core/opengl.hpp:288
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Texture2D_Format {
	NONE = 0,
	DEPTH_COMPONENT = 6402,
	RGB = 6407,
	RGBA = 6408,
}

opencv_type_enum! { core::Texture2D_Format }

// MemoryFlag /usr/include/opencv2/core/mat.hpp:545
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UMatData_MemoryFlag {
	COPY_ON_MAP = 1,
	HOST_COPY_OBSOLETE = 2,
	DEVICE_COPY_OBSOLETE = 4,
	TEMP_UMAT = 8,
	TEMP_COPIED_UMAT = 24,
	USER_ALLOCATED = 32,
	DEVICE_MEM_MAPPED = 64,
	ASYNC_CLEANUP = 128,
}

opencv_type_enum! { core::UMatData_MemoryFlag }

// UMatUsageFlags /usr/include/opencv2/core/mat.hpp:462
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UMatUsageFlags {
	USAGE_DEFAULT = 0,
	USAGE_ALLOCATE_HOST_MEMORY = 1,
	USAGE_ALLOCATE_DEVICE_MEMORY = 2,
	USAGE_ALLOCATE_SHARED_MEMORY = 4,
	__UMAT_USAGE_FLAGS_32BIT = 2147483647,
}

opencv_type_enum! { core::UMatUsageFlags }

// KindFlag /usr/include/opencv2/core/mat.hpp:161
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum _InputArray_KindFlag {
	KIND_SHIFT = 16,
	FIXED_TYPE = -2147483648,
	FIXED_SIZE = 1073741824,
	KIND_MASK = 2031616,
	NONE = 0,
	MAT = 65536,
	MATX = 131072,
	STD_VECTOR = 196608,
	STD_VECTOR_VECTOR = 262144,
	STD_VECTOR_MAT = 327680,
	EXPR = 393216,
	OPENGL_BUFFER = 458752,
	CUDA_HOST_MEM = 524288,
	CUDA_GPU_MAT = 589824,
	UMAT = 655360,
	STD_VECTOR_UMAT = 720896,
	STD_BOOL_VECTOR = 786432,
	STD_VECTOR_CUDA_GPU_MAT = 851968,
	STD_ARRAY = 917504,
	STD_ARRAY_MAT = 983040,
}

opencv_type_enum! { core::_InputArray_KindFlag }

// DepthMask /usr/include/opencv2/core/mat.hpp:297
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum _OutputArray_DepthMask {
	DEPTH_MASK_8U = 1,
	DEPTH_MASK_8S = 2,
	DEPTH_MASK_16U = 4,
	DEPTH_MASK_16S = 8,
	DEPTH_MASK_32S = 16,
	DEPTH_MASK_32F = 32,
	DEPTH_MASK_64F = 64,
	DEPTH_MASK_16F = 128,
	DEPTH_MASK_ALL = 127,
	DEPTH_MASK_ALL_BUT_8S = 125,
	DEPTH_MASK_ALL_16F = 255,
	DEPTH_MASK_FLT = 96,
}

opencv_type_enum! { core::_OutputArray_DepthMask }

// VADisplay /usr/include/opencv2/core/va_intel.hpp:22
pub type va_display = *mut c_void;
// VASurfaceID /usr/include/opencv2/core/va_intel.hpp:23
pub type va_surface_id = u32;
// Affine3d /usr/include/opencv2/core/affine.hpp:291
pub type Affine3d = core::Affine3<f64>;
// Affine3f /usr/include/opencv2/core/affine.hpp:290
pub type Affine3f = core::Affine3<f32>;
// ResultType /usr/include/opencv2/core/base.hpp:386
pub type Hamming_result_type = i32;
// ValueType /usr/include/opencv2/core/base.hpp:385
pub type Hamming_value_type = u8;
// HammingLUT /usr/include/opencv2/core/base.hpp:393
pub type HammingLUT = core::Hamming;
// InputArray /usr/include/opencv2/core/mat.hpp:441
pub type InputArray<'a> = &'a core::_InputArray;
// InputArrayOfArrays /usr/include/opencv2/core/mat.hpp:442
pub type InputArrayOfArrays<'a> = core::InputArray<'a>;
// InputOutputArray /usr/include/opencv2/core/mat.hpp:445
pub type InputOutputArray<'a> = &'a core::_InputOutputArray;
// InputOutputArrayOfArrays /usr/include/opencv2/core/mat.hpp:446
pub type InputOutputArrayOfArrays<'a> = core::InputOutputArray<'a>;
// Mat1b /usr/include/opencv2/core/mat.hpp:2379
pub type Mat1b = core::Mat_<u8>;
// Mat1d /usr/include/opencv2/core/mat.hpp:2404
pub type Mat1d = core::Mat_<f64>;
// Mat1f /usr/include/opencv2/core/mat.hpp:2399
pub type Mat1f = core::Mat_<f32>;
// Mat1i /usr/include/opencv2/core/mat.hpp:2394
pub type Mat1i = core::Mat_<i32>;
// Mat1s /usr/include/opencv2/core/mat.hpp:2384
pub type Mat1s = core::Mat_<i16>;
// Mat1w /usr/include/opencv2/core/mat.hpp:2389
pub type Mat1w = core::Mat_<u16>;
// Mat2b /usr/include/opencv2/core/mat.hpp:2380
pub type Mat2b = core::Mat_<core::Vec2b>;
// Mat2d /usr/include/opencv2/core/mat.hpp:2405
pub type Mat2d = core::Mat_<core::Vec2d>;
// Mat2f /usr/include/opencv2/core/mat.hpp:2400
pub type Mat2f = core::Mat_<core::Vec2f>;
// Mat2i /usr/include/opencv2/core/mat.hpp:2395
pub type Mat2i = core::Mat_<core::Vec2i>;
// Mat2s /usr/include/opencv2/core/mat.hpp:2385
pub type Mat2s = core::Mat_<core::Vec2s>;
// Mat2w /usr/include/opencv2/core/mat.hpp:2390
pub type Mat2w = core::Mat_<core::Vec2w>;
// Mat3b /usr/include/opencv2/core/mat.hpp:2381
pub type Mat3b = core::Mat_<core::Vec3b>;
// Mat3d /usr/include/opencv2/core/mat.hpp:2406
pub type Mat3d = core::Mat_<core::Vec3d>;
// Mat3f /usr/include/opencv2/core/mat.hpp:2401
pub type Mat3f = core::Mat_<core::Vec3f>;
// Mat3i /usr/include/opencv2/core/mat.hpp:2396
pub type Mat3i = core::Mat_<core::Vec3i>;
// Mat3s /usr/include/opencv2/core/mat.hpp:2386
pub type Mat3s = core::Mat_<core::Vec3s>;
// Mat3w /usr/include/opencv2/core/mat.hpp:2391
pub type Mat3w = core::Mat_<core::Vec3w>;
// Mat4b /usr/include/opencv2/core/mat.hpp:2382
pub type Mat4b = core::Mat_<core::Vec4b>;
// Mat4d /usr/include/opencv2/core/mat.hpp:2407
pub type Mat4d = core::Mat_<core::Vec4d>;
// Mat4f /usr/include/opencv2/core/mat.hpp:2402
pub type Mat4f = core::Mat_<core::Vec4f>;
// Mat4i /usr/include/opencv2/core/mat.hpp:2397
pub type Mat4i = core::Mat_<core::Vec4i>;
// Mat4s /usr/include/opencv2/core/mat.hpp:2387
pub type Mat4s = core::Mat_<core::Vec4s>;
// Mat4w /usr/include/opencv2/core/mat.hpp:2392
pub type Mat4w = core::Mat_<core::Vec4w>;
// difference_type /usr/include/opencv2/core/mat.hpp:3065
pub type MatConstIterator_difference_type = ptrdiff_t;
// pointer /usr/include/opencv2/core/mat.hpp:3066
pub type MatConstIterator_pointer<'a, 'b> = &'a mut &'b u8;
// reference /usr/include/opencv2/core/mat.hpp:3067
pub type MatConstIterator_reference<'a> = &'a mut u8;
// value_type /usr/include/opencv2/core/mat.hpp:3064
pub type MatConstIterator_value_type<'a> = &'a mut u8;
// MatND /usr/include/opencv2/core/base.hpp:596
pub type MatND = core::Mat;
// Matx12d /usr/include/opencv2/core/matx.hpp:222
pub type Matx12d = core::Matx12<f64>;
// Matx12f /usr/include/opencv2/core/matx.hpp:221
pub type Matx12f = core::Matx12<f32>;
// Matx13d /usr/include/opencv2/core/matx.hpp:224
pub type Matx13d = core::Matx13<f64>;
// Matx13f /usr/include/opencv2/core/matx.hpp:223
pub type Matx13f = core::Matx13<f32>;
// Matx14d /usr/include/opencv2/core/matx.hpp:226
pub type Matx14d = core::Matx14<f64>;
// Matx14f /usr/include/opencv2/core/matx.hpp:225
pub type Matx14f = core::Matx14<f32>;
// Matx16d /usr/include/opencv2/core/matx.hpp:228
pub type Matx16d = core::Matx16<f64>;
// Matx16f /usr/include/opencv2/core/matx.hpp:227
pub type Matx16f = core::Matx16<f32>;
// Matx21d /usr/include/opencv2/core/matx.hpp:231
pub type Matx21d = core::Matx21<f64>;
// Matx21f /usr/include/opencv2/core/matx.hpp:230
pub type Matx21f = core::Matx21<f32>;
// Matx22d /usr/include/opencv2/core/matx.hpp:240
pub type Matx22d = core::Matx22<f64>;
// Matx22f /usr/include/opencv2/core/matx.hpp:239
pub type Matx22f = core::Matx22<f32>;
// Matx23d /usr/include/opencv2/core/matx.hpp:242
pub type Matx23d = core::Matx23<f64>;
// Matx23f /usr/include/opencv2/core/matx.hpp:241
pub type Matx23f = core::Matx23<f32>;
// Matx31d /usr/include/opencv2/core/matx.hpp:233
pub type Matx31d = core::Matx31<f64>;
// Matx31f /usr/include/opencv2/core/matx.hpp:232
pub type Matx31f = core::Matx31<f32>;
// Matx32d /usr/include/opencv2/core/matx.hpp:244
pub type Matx32d = core::Matx32<f64>;
// Matx32f /usr/include/opencv2/core/matx.hpp:243
pub type Matx32f = core::Matx32<f32>;
// Matx33d /usr/include/opencv2/core/matx.hpp:247
pub type Matx33d = core::Matx33<f64>;
// Matx33f /usr/include/opencv2/core/matx.hpp:246
pub type Matx33f = core::Matx33<f32>;
// Matx34d /usr/include/opencv2/core/matx.hpp:250
pub type Matx34d = core::Matx34<f64>;
// Matx34f /usr/include/opencv2/core/matx.hpp:249
pub type Matx34f = core::Matx34<f32>;
// Matx41d /usr/include/opencv2/core/matx.hpp:235
pub type Matx41d = core::Matx41<f64>;
// Matx41f /usr/include/opencv2/core/matx.hpp:234
pub type Matx41f = core::Matx41<f32>;
// Matx43d /usr/include/opencv2/core/matx.hpp:252
pub type Matx43d = core::Matx43<f64>;
// Matx43f /usr/include/opencv2/core/matx.hpp:251
pub type Matx43f = core::Matx43<f32>;
// Matx44d /usr/include/opencv2/core/matx.hpp:255
pub type Matx44d = core::Matx44<f64>;
// Matx44f /usr/include/opencv2/core/matx.hpp:254
pub type Matx44f = core::Matx44<f32>;
// Matx61d /usr/include/opencv2/core/matx.hpp:237
pub type Matx61d = core::Matx61<f64>;
// Matx61f /usr/include/opencv2/core/matx.hpp:236
pub type Matx61f = core::Matx61<f32>;
// Matx66d /usr/include/opencv2/core/matx.hpp:257
pub type Matx66d = core::Matx66<f64>;
// Matx66f /usr/include/opencv2/core/matx.hpp:256
pub type Matx66f = core::Matx66<f32>;
// OutputArray /usr/include/opencv2/core/mat.hpp:443
pub type OutputArray<'a> = &'a core::_OutputArray;
// OutputArrayOfArrays /usr/include/opencv2/core/mat.hpp:444
pub type OutputArrayOfArrays<'a> = core::OutputArray<'a>;
// Point /usr/include/opencv2/core/types.hpp:204
pub type Point = core::Point2i;
// Point2d /usr/include/opencv2/core/types.hpp:203
pub type Point2d = core::Point_<f64>;
// Point2f /usr/include/opencv2/core/types.hpp:202
pub type Point2f = core::Point_<f32>;
// Point2i /usr/include/opencv2/core/types.hpp:200
pub type Point2i = core::Point_<i32>;
// Point2l /usr/include/opencv2/core/types.hpp:201
pub type Point2l = core::Point_<i64>;
// Point3d /usr/include/opencv2/core/types.hpp:286
pub type Point3d = core::Point3_<f64>;
// Point3f /usr/include/opencv2/core/types.hpp:285
pub type Point3f = core::Point3_<f32>;
// Point3i /usr/include/opencv2/core/types.hpp:284
pub type Point3i = core::Point3_<i32>;
// Rect /usr/include/opencv2/core/types.hpp:484
pub type Rect = core::Rect2i;
// Rect2d /usr/include/opencv2/core/types.hpp:483
pub type Rect2d = core::Rect_<f64>;
// Rect2f /usr/include/opencv2/core/types.hpp:482
pub type Rect2f = core::Rect_<f32>;
// Rect2i /usr/include/opencv2/core/types.hpp:481
pub type Rect2i = core::Rect_<i32>;
// Scalar /usr/include/opencv2/core/types.hpp:691
pub type Scalar = core::Scalar_<f64>;
// Size /usr/include/opencv2/core/types.hpp:365
pub type Size = core::Size2i;
// Size2d /usr/include/opencv2/core/types.hpp:364
pub type Size2d = core::Size_<f64>;
// Size2f /usr/include/opencv2/core/types.hpp:363
pub type Size2f = core::Size_<f32>;
// Size2i /usr/include/opencv2/core/types.hpp:361
pub type Size2i = core::Size_<i32>;
// Size2l /usr/include/opencv2/core/types.hpp:362
pub type Size2l = core::Size_<i64>;
// const_iterator /usr/include/opencv2/core/mat.hpp:2716
pub type SparseMat_const_iterator = core::SparseMatConstIterator;
// iterator /usr/include/opencv2/core/mat.hpp:2715
pub type SparseMat_iterator = core::SparseMatIterator;
// Vec2b /usr/include/opencv2/core/matx.hpp:417
pub type Vec2b = core::VecN<u8, 2>;
// Vec2d /usr/include/opencv2/core/matx.hpp:440
pub type Vec2d = core::VecN<f64, 2>;
// Vec2f /usr/include/opencv2/core/matx.hpp:435
pub type Vec2f = core::VecN<f32, 2>;
// Vec2i /usr/include/opencv2/core/matx.hpp:429
pub type Vec2i = core::VecN<i32, 2>;
// Vec2s /usr/include/opencv2/core/matx.hpp:421
pub type Vec2s = core::VecN<i16, 2>;
// Vec2w /usr/include/opencv2/core/matx.hpp:425
pub type Vec2w = core::VecN<u16, 2>;
// Vec3b /usr/include/opencv2/core/matx.hpp:418
pub type Vec3b = core::VecN<u8, 3>;
// Vec3d /usr/include/opencv2/core/matx.hpp:441
pub type Vec3d = core::VecN<f64, 3>;
// Vec3f /usr/include/opencv2/core/matx.hpp:436
pub type Vec3f = core::VecN<f32, 3>;
// Vec3i /usr/include/opencv2/core/matx.hpp:430
pub type Vec3i = core::VecN<i32, 3>;
// Vec3s /usr/include/opencv2/core/matx.hpp:422
pub type Vec3s = core::VecN<i16, 3>;
// Vec3w /usr/include/opencv2/core/matx.hpp:426
pub type Vec3w = core::VecN<u16, 3>;
// Vec4b /usr/include/opencv2/core/matx.hpp:419
pub type Vec4b = core::VecN<u8, 4>;
// Vec4d /usr/include/opencv2/core/matx.hpp:442
pub type Vec4d = core::VecN<f64, 4>;
// Vec4f /usr/include/opencv2/core/matx.hpp:437
pub type Vec4f = core::VecN<f32, 4>;
// Vec4i /usr/include/opencv2/core/matx.hpp:431
pub type Vec4i = core::VecN<i32, 4>;
// Vec4s /usr/include/opencv2/core/matx.hpp:423
pub type Vec4s = core::VecN<i16, 4>;
// Vec4w /usr/include/opencv2/core/matx.hpp:427
pub type Vec4w = core::VecN<u16, 4>;
// Vec6d /usr/include/opencv2/core/matx.hpp:443
pub type Vec6d = core::VecN<f64, 6>;
// Vec6f /usr/include/opencv2/core/matx.hpp:438
pub type Vec6f = core::VecN<f32, 6>;
// Vec6i /usr/include/opencv2/core/matx.hpp:432
pub type Vec6i = core::VecN<i32, 6>;
// Vec8i /usr/include/opencv2/core/matx.hpp:433
pub type Vec8i = core::VecN<i32, 8>;
// IndexArray /usr/include/opencv2/core/cuda.hpp:363
pub type GpuMatND_IndexArray = core::Vector<i32>;
// SizeArray /usr/include/opencv2/core/cuda.hpp:361
pub type GpuMatND_SizeArray = core::Vector<i32>;
// StepArray /usr/include/opencv2/core/cuda.hpp:362
pub type GpuMatND_StepArray = core::Vector<size_t>;
// StreamCallback /usr/include/opencv2/core/cuda.hpp:854
pub type Stream_StreamCallback = Option<Box<dyn FnMut(i32) -> () + Send + Sync + 'static>>;
// hash_t /usr/include/opencv2/core/ocl.hpp:597
pub type ProgramSource_hash_t = u64;
// Cholesky(double *, size_t, int, double *, size_t, int) /usr/include/opencv2/core/base.hpp:567
#[inline]
pub fn cholesky(a: &mut f64, astep: size_t, m: i32, b: &mut f64, bstep: size_t, n: i32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_Cholesky_doubleX_size_t_int_doubleX_size_t_int(a, astep, m, b, bstep, n, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// Cholesky(float *, size_t, int, float *, size_t, int) /usr/include/opencv2/core/base.hpp:565
#[inline]
pub fn cholesky_f32(a: &mut f32, astep: size_t, m: i32, b: &mut f32, bstep: size_t, n: i32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_Cholesky_floatX_size_t_int_floatX_size_t_int(a, astep, m, b, bstep, n, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// LUT(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:563
#[inline]
pub fn lut(src: &dyn core::ToInputArray, lut: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(lut);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_LUT_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), lut.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// LU(double *, size_t, int, double *, size_t, int) /usr/include/opencv2/core/base.hpp:563
#[inline]
pub fn lu(a: &mut f64, astep: size_t, m: i32, b: &mut f64, bstep: size_t, n: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_LU_doubleX_size_t_int_doubleX_size_t_int(a, astep, m, b, bstep, n, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// LU(float *, size_t, int, float *, size_t, int) /usr/include/opencv2/core/base.hpp:561
#[inline]
pub fn lu_f32(a: &mut f32, astep: size_t, m: i32, b: &mut f32, bstep: size_t, n: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_LU_floatX_size_t_int_floatX_size_t_int(a, astep, m, b, bstep, n, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// Mahalanobis(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2083
#[inline]
pub fn mahalanobis(v1: &dyn core::ToInputArray, v2: &dyn core::ToInputArray, icovar: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(v1);
	input_array_arg!(v2);
	input_array_arg!(icovar);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_Mahalanobis_const__InputArrayR_const__InputArrayR_const__InputArrayR(v1.as_raw__InputArray(), v2.as_raw__InputArray(), icovar.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// PCABackProject(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2063
#[inline]
pub fn pca_back_project(data: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, eigenvectors: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(data);
	input_array_arg!(mean);
	input_array_arg!(eigenvectors);
	output_array_arg!(result);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_PCABackProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(data.as_raw__InputArray(), mean.as_raw__InputArray(), eigenvectors.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, double) /usr/include/opencv2/core.hpp:2054
#[inline]
pub fn pca_compute2_variance(data: &dyn core::ToInputArray, mean: &mut dyn core::ToInputOutputArray, eigenvectors: &mut dyn core::ToOutputArray, eigenvalues: &mut dyn core::ToOutputArray, retained_variance: f64) -> Result<()> {
	input_array_arg!(data);
	input_output_array_arg!(mean);
	output_array_arg!(eigenvectors);
	output_array_arg!(eigenvalues);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_double(data.as_raw__InputArray(), mean.as_raw__InputOutputArray(), eigenvectors.as_raw__OutputArray(), eigenvalues.as_raw__OutputArray(), retained_variance, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * max_components: 0
// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2045
#[inline]
pub fn pca_compute2(data: &dyn core::ToInputArray, mean: &mut dyn core::ToInputOutputArray, eigenvectors: &mut dyn core::ToOutputArray, eigenvalues: &mut dyn core::ToOutputArray, max_components: i32) -> Result<()> {
	input_array_arg!(data);
	input_output_array_arg!(mean);
	output_array_arg!(eigenvectors);
	output_array_arg!(eigenvalues);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int(data.as_raw__InputArray(), mean.as_raw__InputOutputArray(), eigenvectors.as_raw__OutputArray(), eigenvalues.as_raw__OutputArray(), max_components, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, double) /usr/include/opencv2/core.hpp:2050
#[inline]
pub fn pca_compute_variance(data: &dyn core::ToInputArray, mean: &mut dyn core::ToInputOutputArray, eigenvectors: &mut dyn core::ToOutputArray, retained_variance: f64) -> Result<()> {
	input_array_arg!(data);
	input_output_array_arg!(mean);
	output_array_arg!(eigenvectors);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_double(data.as_raw__InputArray(), mean.as_raw__InputOutputArray(), eigenvectors.as_raw__OutputArray(), retained_variance, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * max_components: 0
// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2041
#[inline]
pub fn pca_compute(data: &dyn core::ToInputArray, mean: &mut dyn core::ToInputOutputArray, eigenvectors: &mut dyn core::ToOutputArray, max_components: i32) -> Result<()> {
	input_array_arg!(data);
	input_output_array_arg!(mean);
	output_array_arg!(eigenvectors);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_int(data.as_raw__InputArray(), mean.as_raw__InputOutputArray(), eigenvectors.as_raw__OutputArray(), max_components, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// PCAProject(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2059
#[inline]
pub fn pca_project(data: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, eigenvectors: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(data);
	input_array_arg!(mean);
	input_array_arg!(eigenvectors);
	output_array_arg!(result);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_PCAProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(data.as_raw__InputArray(), mean.as_raw__InputArray(), eigenvectors.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * r: 255.
// PSNR(cv::InputArray, cv::InputArray, double) /usr/include/opencv2/core.hpp:723
#[inline]
pub fn psnr(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, r: f64) -> Result<f64> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_PSNR_const__InputArrayR_const__InputArrayR_double(src1.as_raw__InputArray(), src2.as_raw__InputArray(), r, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// SVBackSubst(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2070
#[inline]
pub fn sv_back_subst(w: &dyn core::ToInputArray, u: &dyn core::ToInputArray, vt: &dyn core::ToInputArray, rhs: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(w);
	input_array_arg!(u);
	input_array_arg!(vt);
	input_array_arg!(rhs);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_SVBackSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(w.as_raw__InputArray(), u.as_raw__InputArray(), vt.as_raw__InputArray(), rhs.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: 0
// SVDecomp(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2067
#[inline]
pub fn sv_decomp(src: &dyn core::ToInputArray, w: &mut dyn core::ToOutputArray, u: &mut dyn core::ToOutputArray, vt: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(w);
	output_array_arg!(u);
	output_array_arg!(vt);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_SVDecomp_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), w.as_raw__OutputArray(), u.as_raw__OutputArray(), vt.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// abs(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3768
#[inline]
pub fn abs_matexpr(e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_abs_const_MatExprR(e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// abs(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3764
#[inline]
pub fn abs(m: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_abs_const_MatR(m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// absdiff(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1397
#[inline]
pub fn absdiff(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * dtype: -1
// addWeighted(cv::InputArray, double, cv::InputArray, double, double, cv::OutputArray, int) /usr/include/opencv2/core.hpp:506
#[inline]
pub fn add_weighted(src1: &dyn core::ToInputArray, alpha: f64, src2: &dyn core::ToInputArray, beta: f64, gamma: f64, dst: &mut dyn core::ToOutputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int(src1.as_raw__InputArray(), alpha, src2.as_raw__InputArray(), beta, gamma, dst.as_raw__OutputArray(), dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
/// * dtype: -1
// add(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, int) /usr/include/opencv2/core.hpp:360
#[inline]
pub fn add(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * norm_type: NORM_L2
/// * k: 0
/// * mask: noArray()
/// * update: 0
/// * crosscheck: false
// batchDistance(cv::InputArray, cv::InputArray, cv::OutputArray, int, cv::OutputArray, int, int, cv::InputArray, int, bool) /usr/include/opencv2/core.hpp:730
#[inline]
pub fn batch_distance(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dist: &mut dyn core::ToOutputArray, dtype: i32, nidx: &mut dyn core::ToOutputArray, norm_type: i32, k: i32, mask: &dyn core::ToInputArray, update: i32, crosscheck: bool) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dist);
	output_array_arg!(nidx);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_batchDistance_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__OutputArrayR_int_int_const__InputArrayR_int_bool(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dist.as_raw__OutputArray(), dtype, nidx.as_raw__OutputArray(), norm_type, k, mask.as_raw__InputArray(), update, crosscheck, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
// bitwise_and(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1299
#[inline]
pub fn bitwise_and(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
// bitwise_not(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1371
#[inline]
pub fn bitwise_not(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
// bitwise_or(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1326
#[inline]
pub fn bitwise_or(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
// bitwise_xor(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1354
#[inline]
pub fn bitwise_xor(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// borderInterpolate(int, int, int) /usr/include/opencv2/core.hpp:270
#[inline]
pub fn border_interpolate(p: i32, len: i32, border_type: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_borderInterpolate_int_int_int(p, len, border_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * ctype: CV_64F
// calcCovarMatrix(cv::InputArray, cv::OutputArray, cv::InputOutputArray, int, int) /usr/include/opencv2/core.hpp:2037
#[inline]
pub fn calc_covar_matrix(samples: &dyn core::ToInputArray, covar: &mut dyn core::ToOutputArray, mean: &mut dyn core::ToInputOutputArray, flags: i32, ctype: i32) -> Result<()> {
	input_array_arg!(samples);
	output_array_arg!(covar);
	input_output_array_arg!(mean);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int_int(samples.as_raw__InputArray(), covar.as_raw__OutputArray(), mean.as_raw__InputOutputArray(), flags, ctype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * angle_in_degrees: false
// cartToPolar(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, bool) /usr/include/opencv2/core.hpp:1608
#[inline]
pub fn cart_to_polar(x: &dyn core::ToInputArray, y: &dyn core::ToInputArray, magnitude: &mut dyn core::ToOutputArray, angle: &mut dyn core::ToOutputArray, angle_in_degrees: bool) -> Result<()> {
	input_array_arg!(x);
	input_array_arg!(y);
	output_array_arg!(magnitude);
	output_array_arg!(angle);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray(), angle.as_raw__OutputArray(), angle_in_degrees, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// checkHardwareSupport(int) /usr/include/opencv2/core/utility.hpp:425
#[inline]
pub fn check_hardware_support(feature: i32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_checkHardwareSupport_int(feature, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * quiet: true
/// * pos: 0
/// * min_val: -DBL_MAX
/// * max_val: DBL_MAX
// checkRange(cv::InputArray, bool, cv::Point *, double, double) /usr/include/opencv2/core.hpp:1659
#[inline]
pub fn check_range(a: &dyn core::ToInputArray, quiet: bool, pos: &mut core::Point, min_val: f64, max_val: f64) -> Result<bool> {
	input_array_arg!(a);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_checkRange_const__InputArrayR_bool_PointX_double_double(a.as_raw__InputArray(), quiet, pos, min_val, max_val, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// compare(cv::InputArray, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1458
#[inline]
pub fn compare(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, cmpop: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), cmpop, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * lower_to_upper: false
// completeSymm(cv::InputOutputArray, bool) /usr/include/opencv2/core.hpp:1819
#[inline]
pub fn complete_symm(m: &mut dyn core::ToInputOutputArray, lower_to_upper: bool) -> Result<()> {
	input_output_array_arg!(m);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_completeSymm_const__InputOutputArrayR_bool(m.as_raw__InputOutputArray(), lower_to_upper, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// convertFp16(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:547
#[inline]
pub fn convert_fp16(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_convertFp16_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * alpha: 1
/// * beta: 0
// convertScaleAbs(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/core.hpp:534
#[inline]
pub fn convert_scale_abs(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, alpha: f64, beta: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_convertScaleAbs_const__InputArrayR_const__OutputArrayR_double_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha, beta, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * value: Scalar()
// copyMakeBorder(cv::InputArray, cv::OutputArray, int, int, int, int, int, const cv::Scalar &) /usr/include/opencv2/core.hpp:320
#[inline]
pub fn copy_make_border(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, value: core::Scalar) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_const_ScalarR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), top, bottom, left, right, border_type, &value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// copyTo(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1409
#[inline]
pub fn copy_to(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_copyTo_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// countNonZero(cv::InputArray) /usr/include/opencv2/core.hpp:581
#[inline]
pub fn count_non_zero(src: &dyn core::ToInputArray) -> Result<i32> {
	input_array_arg!(src);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_countNonZero_const__InputArrayR(src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// cubeRoot(float) /usr/include/opencv2/core/base.hpp:539
#[inline]
pub fn cube_root(val: f32) -> Result<f32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cubeRoot_float(val, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// createContinuous(int, int, int, cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:557
#[inline]
pub fn create_continuous(rows: i32, cols: i32, typ: i32, arr: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(arr);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_createContinuous_int_int_int_const__OutputArrayR(rows, cols, typ, arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// deviceSupports(cv::cuda::FeatureSet) /usr/include/opencv2/core/cuda.hpp:1010
#[inline]
pub fn device_supports(feature_set: core::FeatureSet) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_deviceSupports_FeatureSet(feature_set, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// ensureSizeIsEnough(int, int, int, cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:568
#[inline]
pub fn ensure_size_is_enough(rows: i32, cols: i32, typ: i32, arr: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(arr);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_ensureSizeIsEnough_int_int_int_const__OutputArrayR(rows, cols, typ, arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getCudaEnabledDeviceCount() /usr/include/opencv2/core/cuda.hpp:966
#[inline]
pub fn get_cuda_enabled_device_count() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_getCudaEnabledDeviceCount(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getDevice() /usr/include/opencv2/core/cuda.hpp:978
#[inline]
pub fn get_device() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_getDevice(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// printCudaDeviceInfo(int) /usr/include/opencv2/core/cuda.hpp:1252
#[inline]
pub fn print_cuda_device_info(device: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_printCudaDeviceInfo_int(device, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// printShortCudaDeviceInfo(int) /usr/include/opencv2/core/cuda.hpp:1253
#[inline]
pub fn print_short_cuda_device_info(device: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_printShortCudaDeviceInfo_int(device, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// registerPageLocked(cv::Mat &) /usr/include/opencv2/core/cuda.hpp:809
#[inline]
pub fn register_page_locked(m: &mut core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_registerPageLocked_MatR(m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// resetDevice() /usr/include/opencv2/core/cuda.hpp:985
#[inline]
pub fn reset_device() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_resetDevice(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setBufferPoolConfig(int, size_t, int) /usr/include/opencv2/core/cuda.hpp:709
#[inline]
pub fn set_buffer_pool_config(device_id: i32, stack_size: size_t, stack_count: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_setBufferPoolConfig_int_size_t_int(device_id, stack_size, stack_count, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setBufferPoolUsage(bool) /usr/include/opencv2/core/cuda.hpp:708
#[inline]
pub fn set_buffer_pool_usage(on: bool) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_setBufferPoolUsage_bool(on, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setDevice(int) /usr/include/opencv2/core/cuda.hpp:974
#[inline]
pub fn set_device(device: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_setDevice_int(device, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * device: 0
// setGlDevice(int) /usr/include/opencv2/core/opengl.hpp:572
#[inline]
pub fn set_gl_device(device: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_setGlDevice_int(device, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// unregisterPageLocked(cv::Mat &) /usr/include/opencv2/core/cuda.hpp:815
#[inline]
pub fn unregister_page_locked(m: &mut core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_unregisterPageLocked_MatR(m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: 0
// dct(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2272
#[inline]
pub fn dct(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dct_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// depthToString(int) /usr/include/opencv2/core/check.hpp:13
#[inline]
pub fn depth_to_string(depth: i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_depthToString_int(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// check_failed_MatChannels(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:85
#[inline]
pub fn check_failed_mat_channels_1(v: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_MatChannels_const_int_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_MatChannels(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:75
#[inline]
pub fn check_failed_mat_channels(v1: i32, v2: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_MatChannels_const_int_const_int_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_MatDepth(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:83
#[inline]
pub fn check_failed_mat_depth_1(v: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_MatDepth_const_int_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_MatDepth(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:73
#[inline]
pub fn check_failed_mat_depth(v1: i32, v2: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_MatDepth_const_int_const_int_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_MatType(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:84
#[inline]
pub fn check_failed_mat_type_1(v: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_MatType_const_int_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_MatType(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:74
#[inline]
pub fn check_failed_mat_type(v1: i32, v2: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_MatType_const_int_const_int_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const Size_<int>, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:81
#[inline]
pub fn check_failed_auto_9(v: core::Size_<i32>, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_Size__int__const_CheckContextR(v.opencv_as_extern(), ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const Size_<int>, const Size_<int>, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:72
#[inline]
pub fn check_failed_auto_4(v1: core::Size_<i32>, v2: core::Size_<i32>, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_Size__int__const_Size__int__const_CheckContextR(v1.opencv_as_extern(), v2.opencv_as_extern(), ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const double, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:80
#[inline]
pub fn check_failed_auto_8(v: f64, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_double_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const double, const double, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:71
#[inline]
pub fn check_failed_auto_3(v1: f64, v2: f64, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_double_const_double_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const float, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:79
#[inline]
pub fn check_failed_auto_7(v: f32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_float_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const float, const float, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:70
#[inline]
pub fn check_failed_auto_2(v1: f32, v2: f32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_float_const_float_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:77
#[inline]
pub fn check_failed_auto_5(v: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_int_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:68
#[inline]
pub fn check_failed_auto(v1: i32, v2: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_int_const_int_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const size_t, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:78
#[inline]
pub fn check_failed_auto_6(v: size_t, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_size_t_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const size_t, const size_t, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:69
#[inline]
pub fn check_failed_auto_1(v1: size_t, v2: size_t, ctx: &core::Detail_CheckContext) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_size_t_const_size_t_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// check_failed_auto(const std::string &, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:82
#[inline]
pub fn check_failed_auto_10(v1: &str, ctx: &core::Detail_CheckContext) -> Result<()> {
	extern_container_arg!(v1);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_check_failed_auto_const_stringR_const_CheckContextR(v1.opencv_as_extern(), ctx.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// determinant(cv::InputArray) /usr/include/opencv2/core.hpp:1851
#[inline]
pub fn determinant(mtx: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(mtx);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_determinant_const__InputArrayR(mtx.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: 0
/// * nonzero_rows: 0
// dft(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/core.hpp:2217
#[inline]
pub fn dft(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32, nonzero_rows: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dft_const__InputArrayR_const__OutputArrayR_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, nonzero_rows, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getTypeFromD3DFORMAT(const int) /usr/include/opencv2/core/directx.hpp:178
#[inline]
pub fn get_type_from_d3d_format(i_d3_dformat: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_directx_getTypeFromD3DFORMAT_const_int(i_d3_dformat, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getTypeFromDXGI_FORMAT(const int) /usr/include/opencv2/core/directx.hpp:173
#[inline]
pub fn get_type_from_dxgi_format(i_dxgi_format: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_directx_getTypeFromDXGI_FORMAT_const_int(i_dxgi_format, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * scale: 1
/// * dtype: -1
// divide(cv::InputArray, cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/core.hpp:453
#[inline]
pub fn divide2(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, scale: f64, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), scale, dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * dtype: -1
// divide(double, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:457
#[inline]
pub fn divide(scale: f64, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_divide_double_const__InputArrayR_const__OutputArrayR_int(scale, src2.as_raw__InputArray(), dst.as_raw__OutputArray(), dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// eigenNonSymmetric(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2010
#[inline]
pub fn eigen_non_symmetric(src: &dyn core::ToInputArray, eigenvalues: &mut dyn core::ToOutputArray, eigenvectors: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(eigenvalues);
	output_array_arg!(eigenvectors);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_eigenNonSymmetric_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), eigenvalues.as_raw__OutputArray(), eigenvectors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * eigenvectors: noArray()
// eigen(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1993
#[inline]
pub fn eigen(src: &dyn core::ToInputArray, eigenvalues: &mut dyn core::ToOutputArray, eigenvectors: &mut dyn core::ToOutputArray) -> Result<bool> {
	input_array_arg!(src);
	output_array_arg!(eigenvalues);
	output_array_arg!(eigenvectors);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_eigen_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), eigenvalues.as_raw__OutputArray(), eigenvectors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// error(const cv::Exception &) /usr/include/opencv2/core.hpp:155
#[inline]
pub fn error_1(exc: &core::Exception) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_error_const_ExceptionR(exc.as_raw_Exception(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// error(int, const cv::String &, const char *, const char *, int) /usr/include/opencv2/core/base.hpp:298
#[inline]
pub fn error(_code: i32, _err: &str, _func: &str, _file: &str, _line: i32) -> Result<()> {
	extern_container_arg!(_err);
	extern_container_arg!(_func);
	extern_container_arg!(_file);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_error_int_const_StringR_const_charX_const_charX_int(_code, _err.opencv_as_extern(), _func.opencv_as_extern(), _file.opencv_as_extern(), _line, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// exp(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1552
#[inline]
pub fn exp(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_exp_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// extractChannel(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1061
#[inline]
pub fn extract_channel(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, coi: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_extractChannel_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), coi, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// fastAtan2(float, float) /usr/include/opencv2/core/base.hpp:558
#[inline]
pub fn fast_atan2(y: f32, x: f32) -> Result<f32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fastAtan2_float_float(y, x, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// findNonZero(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:609
#[inline]
pub fn find_non_zero(src: &dyn core::ToInputArray, idx: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(idx);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findNonZero_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), idx.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// flip(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1103
#[inline]
pub fn flip(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flip_code: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_flip_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flip_code, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: 0
// gemm(cv::InputArray, cv::InputArray, double, cv::InputArray, double, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1696
#[inline]
pub fn gemm(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, alpha: f64, src3: &dyn core::ToInputArray, beta: f64, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(src3);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), alpha, src3.as_raw__InputArray(), beta, dst.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getBuildInformation() /usr/include/opencv2/core/utility.hpp:242
#[inline]
pub fn get_build_information() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getBuildInformation(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getCPUFeaturesLine() /usr/include/opencv2/core/utility.hpp:443
#[inline]
pub fn get_cpu_features_line() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getCPUFeaturesLine(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getCPUTickCount() /usr/include/opencv2/core/utility.hpp:415
#[inline]
pub fn get_cpu_tick_count() -> Result<i64> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getCPUTickCount(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getElemSize(int) /usr/include/opencv2/core/utility.hpp:568
#[inline]
pub fn get_elem_size(typ: i32) -> Result<size_t> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getElemSize_int(typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getHardwareFeatureName(int) /usr/include/opencv2/core/utility.hpp:431
#[inline]
pub fn get_hardware_feature_name(feature: i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getHardwareFeatureName_int(feature, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getLogLevel() /usr/include/opencv2/core/bindings_utils.hpp:292
#[inline]
pub fn get_log_level_1() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getLogLevel(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getNumThreads() /usr/include/opencv2/core/utility.hpp:218
#[inline]
pub fn get_num_threads() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getNumThreads(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getNumberOfCPUs() /usr/include/opencv2/core/utility.hpp:447
#[inline]
pub fn get_number_of_cpus() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getNumberOfCPUs(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getOptimalDFTSize(int) /usr/include/opencv2/core.hpp:2325
#[inline]
pub fn get_optimal_dft_size(vecsize: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getOptimalDFTSize_int(vecsize, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getThreadNum() /usr/include/opencv2/core/utility.hpp:234
#[inline]
pub fn get_thread_num() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getThreadNum(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getTickCount() /usr/include/opencv2/core/utility.hpp:268
#[inline]
pub fn get_tick_count() -> Result<i64> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getTickCount(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getTickFrequency() /usr/include/opencv2/core/utility.hpp:281
#[inline]
pub fn get_tick_frequency() -> Result<f64> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getTickFrequency(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getVersionMajor() /usr/include/opencv2/core/utility.hpp:253
#[inline]
pub fn get_version_major() -> i32 {
	let ret = unsafe { sys::cv_getVersionMajor() };
	ret
}

// getVersionMinor() /usr/include/opencv2/core/utility.hpp:256
#[inline]
pub fn get_version_minor() -> i32 {
	let ret = unsafe { sys::cv_getVersionMinor() };
	ret
}

// getVersionRevision() /usr/include/opencv2/core/utility.hpp:259
#[inline]
pub fn get_version_revision() -> i32 {
	let ret = unsafe { sys::cv_getVersionRevision() };
	ret
}

// getVersionString() /usr/include/opencv2/core/utility.hpp:250
#[inline]
pub fn get_version_string() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getVersionString(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * recursive: false
// glob(cv::String, std::vector<String> &, bool) /usr/include/opencv2/core/utility.hpp:180
#[inline]
pub fn glob(pattern: &str, result: &mut core::Vector<String>, recursive: bool) -> Result<()> {
	extern_container_arg!(mut pattern);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_glob_String_vector_String_R_bool(pattern.opencv_as_extern_mut(), result.as_raw_mut_VectorOfString(), recursive, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// haveOpenVX() /usr/include/opencv2/core/ovx.hpp:19
#[inline]
pub fn have_openvx() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_haveOpenVX(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// hconcat(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1189
#[inline]
pub fn hconcat2(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_hconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// hconcat(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/core.hpp:1208
#[inline]
pub fn hconcat(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_hconcat_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: 0
// idct(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2282
#[inline]
pub fn idct(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_idct_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: 0
/// * nonzero_rows: 0
// idft(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/core.hpp:2231
#[inline]
pub fn idft(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32, nonzero_rows: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_idft_const__InputArrayR_const__OutputArrayR_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, nonzero_rows, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// inRange(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1429
#[inline]
pub fn in_range(src: &dyn core::ToInputArray, lowerb: &dyn core::ToInputArray, upperb: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(lowerb);
	input_array_arg!(upperb);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_inRange_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), lowerb.as_raw__InputArray(), upperb.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// insertChannel(cv::InputArray, cv::InputOutputArray, int) /usr/include/opencv2/core.hpp:1069
#[inline]
pub fn insert_channel(src: &dyn core::ToInputArray, dst: &mut dyn core::ToInputOutputArray, coi: i32) -> Result<()> {
	input_array_arg!(src);
	input_output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_insertChannel_const__InputArrayR_const__InputOutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__InputOutputArray(), coi, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getFlags() /usr/include/opencv2/core/utils/instrumentation.hpp:117
#[inline]
pub fn get_flags() -> Result<core::FLAGS> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_instr_getFlags(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// resetTrace() /usr/include/opencv2/core/utils/instrumentation.hpp:106
#[inline]
pub fn reset_trace() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_instr_resetTrace(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setFlags(cv::instr::FLAGS) /usr/include/opencv2/core/utils/instrumentation.hpp:115
#[inline]
pub fn set_flags(mode_flags: core::FLAGS) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_instr_setFlags_FLAGS(mode_flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setUseInstrumentation(bool) /usr/include/opencv2/core/utils/instrumentation.hpp:105
#[inline]
pub fn set_use_instrumentation(flag: bool) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_instr_setUseInstrumentation_bool(flag, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// useInstrumentation() /usr/include/opencv2/core/utils/instrumentation.hpp:104
#[inline]
pub fn use_instrumentation() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_instr_useInstrumentation(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: DECOMP_LU
// invert(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1887
#[inline]
pub fn invert(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<f64> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_invert_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getIppErrorLocation() /usr/include/opencv2/core/base.hpp:635
#[inline]
pub fn get_ipp_error_location() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ipp_getIppErrorLocation(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getIppFeatures() /usr/include/opencv2/core/base.hpp:631
#[inline]
pub fn get_ipp_features() -> Result<u64> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ipp_getIppFeatures(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getIppStatus() /usr/include/opencv2/core/base.hpp:634
#[inline]
pub fn get_ipp_status() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ipp_getIppStatus(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getIppVersion() /usr/include/opencv2/core/base.hpp:638
#[inline]
pub fn get_ipp_version() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ipp_getIppVersion(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * funcname: NULL
/// * filename: NULL
/// * line: 0
// setIppStatus(int, const char *const, const char *const, int) /usr/include/opencv2/core/base.hpp:632
#[inline]
pub fn set_ipp_status(status: i32, funcname: &str, filename: &str, line: i32) -> Result<()> {
	extern_container_arg!(funcname);
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ipp_setIppStatus_int_const_charX_const_charX_int(status, funcname.opencv_as_extern(), filename.opencv_as_extern(), line, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setUseIPP_NotExact(bool) /usr/include/opencv2/core/base.hpp:643
#[inline]
pub fn set_use_ipp_not_exact(flag: bool) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ipp_setUseIPP_NotExact_bool(flag, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setUseIPP(bool) /usr/include/opencv2/core/base.hpp:637
#[inline]
pub fn set_use_ipp(flag: bool) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ipp_setUseIPP_bool(flag, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// useIPP() /usr/include/opencv2/core/base.hpp:636
#[inline]
pub fn use_ipp() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ipp_useIPP(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// useIPP_NotExact() /usr/include/opencv2/core/base.hpp:642
#[inline]
pub fn use_ipp_not_exact() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ipp_useIPP_NotExact(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * centers: noArray()
// kmeans(cv::InputArray, int, cv::InputOutputArray, cv::TermCriteria, int, int, cv::OutputArray) /usr/include/opencv2/core.hpp:3054
#[inline]
pub fn kmeans(data: &dyn core::ToInputArray, k: i32, best_labels: &mut dyn core::ToInputOutputArray, criteria: core::TermCriteria, attempts: i32, flags: i32, centers: &mut dyn core::ToOutputArray) -> Result<f64> {
	input_array_arg!(data);
	input_output_array_arg!(best_labels);
	output_array_arg!(centers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_kmeans_const__InputArrayR_int_const__InputOutputArrayR_TermCriteria_int_int_const__OutputArrayR(data.as_raw__InputArray(), k, best_labels.as_raw__InputOutputArray(), criteria.opencv_as_extern(), attempts, flags, centers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// log(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1565
#[inline]
pub fn log(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_log_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// magnitude(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1642
#[inline]
pub fn magnitude(x: &dyn core::ToInputArray, y: &dyn core::ToInputArray, magnitude: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(x);
	input_array_arg!(y);
	output_array_arg!(magnitude);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// max(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3743
#[inline]
pub fn max_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_max_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// max(const cv::Mat &, const cv::Mat &, cv::Mat &) /usr/include/opencv2/core.hpp:1496
#[inline]
pub fn max_mat_to(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_max_const_MatR_const_MatR_MatR(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// max(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3744
#[inline]
pub fn max_mat_f64(a: &core::Mat, s: f64) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_max_const_MatR_double(a.as_raw_Mat(), s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// max(const cv::UMat &, const cv::UMat &, cv::UMat &) /usr/include/opencv2/core.hpp:1500
#[inline]
pub fn max_umat_to(src1: &core::UMat, src2: &core::UMat, dst: &mut core::UMat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_max_const_UMatR_const_UMatR_UMatR(src1.as_raw_UMat(), src2.as_raw_UMat(), dst.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// max(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1492
#[inline]
pub fn max(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// max(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3745
#[inline]
pub fn max_f64_mat(s: f64, a: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_max_double_const_MatR(s, a.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
// meanStdDev(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:644
#[inline]
pub fn mean_std_dev(src: &dyn core::ToInputArray, mean: &mut dyn core::ToOutputArray, stddev: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(mean);
	output_array_arg!(stddev);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_meanStdDev_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), mean.as_raw__OutputArray(), stddev.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
// mean(cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:622
#[inline]
pub fn mean(src: &dyn core::ToInputArray, mask: &dyn core::ToInputArray) -> Result<core::Scalar> {
	input_array_arg!(src);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_mean_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// merge(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/core.hpp:953
#[inline]
pub fn merge(mv: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(mv);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_merge_const__InputArrayR_const__OutputArrayR(mv.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * max_val: 0
/// * min_idx: 0
/// * max_idx: 0
/// * mask: noArray()
// minMaxIdx(cv::InputArray, double *, double *, int *, int *, cv::InputArray) /usr/include/opencv2/core.hpp:885
#[inline]
pub fn min_max_idx(src: &dyn core::ToInputArray, min_val: Option<&mut f64>, max_val: Option<&mut f64>, min_idx: Option<&mut i32>, max_idx: Option<&mut i32>, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_minMaxIdx_const__InputArrayR_doubleX_doubleX_intX_intX_const__InputArrayR(src.as_raw__InputArray(), min_val.map_or(::core::ptr::null_mut(), |min_val| min_val as *mut _), max_val.map_or(::core::ptr::null_mut(), |max_val| max_val as *mut _), min_idx.map_or(::core::ptr::null_mut(), |min_idx| min_idx as *mut _), max_idx.map_or(::core::ptr::null_mut(), |max_idx| max_idx as *mut _), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * min_idx: 0
/// * max_idx: 0
// minMaxLoc(const cv::SparseMat &, double *, double *, int *, int *) /usr/include/opencv2/core.hpp:897
#[inline]
pub fn min_max_loc_sparse(a: &core::SparseMat, min_val: Option<&mut f64>, max_val: Option<&mut f64>, min_idx: Option<&mut i32>, max_idx: Option<&mut i32>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_minMaxLoc_const_SparseMatR_doubleX_doubleX_intX_intX(a.as_raw_SparseMat(), min_val.map_or(::core::ptr::null_mut(), |min_val| min_val as *mut _), max_val.map_or(::core::ptr::null_mut(), |max_val| max_val as *mut _), min_idx.map_or(::core::ptr::null_mut(), |min_idx| min_idx as *mut _), max_idx.map_or(::core::ptr::null_mut(), |max_idx| max_idx as *mut _), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * max_val: 0
/// * min_loc: 0
/// * max_loc: 0
/// * mask: noArray()
// minMaxLoc(cv::InputArray, double *, double *, cv::Point *, cv::Point *, cv::InputArray) /usr/include/opencv2/core.hpp:824
#[inline]
pub fn min_max_loc(src: &dyn core::ToInputArray, min_val: Option<&mut f64>, max_val: Option<&mut f64>, min_loc: Option<&mut core::Point>, max_loc: Option<&mut core::Point>, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(src.as_raw__InputArray(), min_val.map_or(::core::ptr::null_mut(), |min_val| min_val as *mut _), max_val.map_or(::core::ptr::null_mut(), |max_val| max_val as *mut _), min_loc.map_or(::core::ptr::null_mut(), |min_loc| min_loc as *mut _), max_loc.map_or(::core::ptr::null_mut(), |max_loc| max_loc as *mut _), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// min(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3735
#[inline]
pub fn min_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_min_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// min(const cv::Mat &, const cv::Mat &, cv::Mat &) /usr/include/opencv2/core.hpp:1475
#[inline]
pub fn min_mat_to(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_min_const_MatR_const_MatR_MatR(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// min(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3736
#[inline]
pub fn min_mat_f64(a: &core::Mat, s: f64) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_min_const_MatR_double(a.as_raw_Mat(), s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// min(const cv::UMat &, const cv::UMat &, cv::UMat &) /usr/include/opencv2/core.hpp:1479
#[inline]
pub fn min_umat_to(src1: &core::UMat, src2: &core::UMat, dst: &mut core::UMat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_min_const_UMatR_const_UMatR_UMatR(src1.as_raw_UMat(), src2.as_raw_UMat(), dst.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// min(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1471
#[inline]
pub fn min(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// min(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3737
#[inline]
pub fn min_f64_mat(s: f64, a: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_min_double_const_MatR(s, a.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// mixChannels(cv::InputArrayOfArrays, cv::InputOutputArrayOfArrays, const int *, size_t) /usr/include/opencv2/core.hpp:1036
#[inline]
pub fn mix_channels(src: &dyn core::ToInputArray, dst: &mut dyn core::ToInputOutputArray, from_to: &[i32]) -> Result<()> {
	input_array_arg!(src);
	input_output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_intX_size_t(src.as_raw__InputArray(), dst.as_raw__InputOutputArray(), from_to.as_ptr(), (from_to.len() / 2) as _, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// mixChannels(cv::InputArrayOfArrays, cv::InputOutputArrayOfArrays, const std::vector<int> &) /usr/include/opencv2/core.hpp:1052
#[inline]
pub fn mix_channels_vec(src: &dyn core::ToInputArray, dst: &mut dyn core::ToInputOutputArray, from_to: &core::Vector<i32>) -> Result<()> {
	input_array_arg!(src);
	input_output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_vector_int_R(src.as_raw__InputArray(), dst.as_raw__InputOutputArray(), from_to.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * conj_b: false
// mulSpectrums(cv::InputArray, cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/core.hpp:2301
#[inline]
pub fn mul_spectrums(a: &dyn core::ToInputArray, b: &dyn core::ToInputArray, c: &mut dyn core::ToOutputArray, flags: i32, conj_b: bool) -> Result<()> {
	input_array_arg!(a);
	input_array_arg!(b);
	output_array_arg!(c);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(a.as_raw__InputArray(), b.as_raw__InputArray(), c.as_raw__OutputArray(), flags, conj_b, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * delta: noArray()
/// * scale: 1
/// * dtype: -1
// mulTransposed(cv::InputArray, cv::OutputArray, bool, cv::InputArray, double, int) /usr/include/opencv2/core.hpp:1727
#[inline]
pub fn mul_transposed(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, a_ta: bool, delta: &dyn core::ToInputArray, scale: f64, dtype: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(delta);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_mulTransposed_const__InputArrayR_const__OutputArrayR_bool_const__InputArrayR_double_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), a_ta, delta.as_raw__InputArray(), scale, dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * scale: 1
/// * dtype: -1
// multiply(cv::InputArray, cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/core.hpp:425
#[inline]
pub fn multiply(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, scale: f64, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), scale, dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// noArray() /usr/include/opencv2/core/mat.hpp:448
#[inline]
pub fn no_array() -> core::_InputOutputArray {
	let ret = unsafe { sys::cv_noArray() };
	let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
	ret
}

// norm(const cv::SparseMat &, int) /usr/include/opencv2/core.hpp:702
#[inline]
pub fn norm_sparse(src: &core::SparseMat, norm_type: i32) -> Result<f64> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_norm_const_SparseMatR_int(src.as_raw_SparseMat(), norm_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * norm_type: NORM_L2
/// * mask: noArray()
// norm(cv::InputArray, cv::InputArray, int, cv::InputArray) /usr/include/opencv2/core.hpp:696
#[inline]
pub fn norm2(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, norm_type: i32, mask: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_norm_const__InputArrayR_const__InputArrayR_int_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), norm_type, mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * norm_type: NORM_L2
/// * mask: noArray()
// norm(cv::InputArray, int, cv::InputArray) /usr/include/opencv2/core.hpp:683
#[inline]
pub fn norm(src1: &dyn core::ToInputArray, norm_type: i32, mask: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(src1);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_norm_const__InputArrayR_int_const__InputArrayR(src1.as_raw__InputArray(), norm_type, mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// normalize(const cv::SparseMat &, cv::SparseMat &, double, int) /usr/include/opencv2/core.hpp:804
#[inline]
pub fn normalize_sparse(src: &core::SparseMat, dst: &mut core::SparseMat, alpha: f64, norm_type: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_normalize_const_SparseMatR_SparseMatR_double_int(src.as_raw_SparseMat(), dst.as_raw_mut_SparseMat(), alpha, norm_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * alpha: 1
/// * beta: 0
/// * norm_type: NORM_L2
/// * dtype: -1
/// * mask: noArray()
// normalize(cv::InputArray, cv::InputOutputArray, double, double, int, int, cv::InputArray) /usr/include/opencv2/core.hpp:794
#[inline]
pub fn normalize(src: &dyn core::ToInputArray, dst: &mut dyn core::ToInputOutputArray, alpha: f64, beta: f64, norm_type: i32, dtype: i32, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	input_output_array_arg!(dst);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_normalize_const__InputArrayR_const__InputOutputArrayR_double_double_int_int_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__InputOutputArray(), alpha, beta, norm_type, dtype, mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// attachContext(const cv::String &, void *, void *, void *) /usr/include/opencv2/core/ocl.hpp:361
#[inline]
pub unsafe fn attach_context(platform_name: &str, platform_id: *mut c_void, context: *mut c_void, device_id: *mut c_void) -> Result<()> {
	extern_container_arg!(platform_name);
	return_send!(via ocvrs_return);
	{ sys::cv_ocl_attachContext_const_StringR_voidX_voidX_voidX(platform_name.opencv_as_extern(), platform_id, context, device_id, ocvrs_return.as_mut_ptr()) };
	return_receive!(ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// buildOptionsAddMatrixDescription(cv::String &, const cv::String &, cv::InputArray) /usr/include/opencv2/core/ocl.hpp:737
#[inline]
pub fn build_options_add_matrix_description(build_options: &mut String, name: &str, _m: &dyn core::ToInputArray) -> Result<()> {
	string_arg_output_send!(via build_options_via);
	extern_container_arg!(name);
	input_array_arg!(_m);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_buildOptionsAddMatrixDescription_StringR_const_StringR_const__InputArrayR(&mut build_options_via, name.opencv_as_extern(), _m.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	string_arg_output_receive!(build_options_via => build_options);
	Ok(ret)
}

/// ## C++ default parameters
/// * src2: noArray()
/// * src3: noArray()
/// * src4: noArray()
/// * src5: noArray()
/// * src6: noArray()
/// * src7: noArray()
/// * src8: noArray()
/// * src9: noArray()
/// * strat: OCL_VECTOR_DEFAULT
// checkOptimalVectorWidth(const int *, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::ocl::OclVectorStrategy) /usr/include/opencv2/core/ocl.hpp:726
#[inline]
pub fn check_optimal_vector_width(vector_widths: &i32, src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, src3: &dyn core::ToInputArray, src4: &dyn core::ToInputArray, src5: &dyn core::ToInputArray, src6: &dyn core::ToInputArray, src7: &dyn core::ToInputArray, src8: &dyn core::ToInputArray, src9: &dyn core::ToInputArray, strat: core::OclVectorStrategy) -> Result<i32> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(src3);
	input_array_arg!(src4);
	input_array_arg!(src5);
	input_array_arg!(src6);
	input_array_arg!(src7);
	input_array_arg!(src8);
	input_array_arg!(src9);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(vector_widths, src1.as_raw__InputArray(), src2.as_raw__InputArray(), src3.as_raw__InputArray(), src4.as_raw__InputArray(), src5.as_raw__InputArray(), src6.as_raw__InputArray(), src7.as_raw__InputArray(), src8.as_raw__InputArray(), src9.as_raw__InputArray(), strat, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// convertFromBuffer(void *, size_t, int, int, int, cv::UMat &) /usr/include/opencv2/core/ocl.hpp:375
#[inline]
pub unsafe fn convert_from_buffer(cl_mem_buffer: *mut c_void, step: size_t, rows: i32, cols: i32, typ: i32, dst: &mut core::UMat) -> Result<()> {
	return_send!(via ocvrs_return);
	{ sys::cv_ocl_convertFromBuffer_voidX_size_t_int_int_int_UMatR(cl_mem_buffer, step, rows, cols, typ, dst.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// convertFromImage(void *, cv::UMat &) /usr/include/opencv2/core/ocl.hpp:384
#[inline]
pub unsafe fn convert_from_image(cl_mem_image: *mut c_void, dst: &mut core::UMat) -> Result<()> {
	return_send!(via ocvrs_return);
	{ sys::cv_ocl_convertFromImage_voidX_UMatR(cl_mem_image, dst.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// convertTypeStr(int, int, int, char *) /usr/include/opencv2/core/ocl.hpp:700
#[inline]
pub fn convert_type_str(sdepth: i32, ddepth: i32, cn: i32, buf: &mut String) -> Result<String> {
	string_arg_output_send!(via buf_via);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_convertTypeStr_int_int_int_charX(sdepth, ddepth, cn, &mut buf_via, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	string_arg_output_receive!(buf_via => buf);
	Ok(ret)
}

// finish() /usr/include/opencv2/core/ocl.hpp:59
#[inline]
pub fn finish() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_finish(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getOpenCLErrorString(int) /usr/include/opencv2/core/ocl.hpp:704
#[inline]
pub fn get_opencl_error_string(error_code: i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_getOpenCLErrorString_int(error_code, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getPlatfomsInfo(std::vector<PlatformInfo> &) /usr/include/opencv2/core/ocl.hpp:706
#[inline]
pub fn get_platfoms_info(platform_info: &mut core::Vector<core::PlatformInfo>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_getPlatfomsInfo_vector_PlatformInfo_R(platform_info.as_raw_mut_VectorOfPlatformInfo(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// haveAmdBlas() /usr/include/opencv2/core/ocl.hpp:56
#[inline]
pub fn have_amd_blas() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_haveAmdBlas(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// haveAmdFft() /usr/include/opencv2/core/ocl.hpp:57
#[inline]
pub fn have_amd_fft() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_haveAmdFft(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// haveOpenCL() /usr/include/opencv2/core/ocl.hpp:54
#[inline]
pub fn have_opencl() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_haveOpenCL(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// haveSVM() /usr/include/opencv2/core/ocl.hpp:61
#[inline]
pub fn have_svm() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_haveSVM(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
/// * name: NULL
// kernelToStr(cv::InputArray, int, const char *) /usr/include/opencv2/core/ocl.hpp:705
#[inline]
pub fn kernel_to_str(_kernel: &dyn core::ToInputArray, ddepth: i32, name: &str) -> Result<String> {
	input_array_arg!(_kernel);
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_kernelToStr_const__InputArrayR_int_const_charX(_kernel.as_raw__InputArray(), ddepth, name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// memopTypeToStr(int) /usr/include/opencv2/core/ocl.hpp:702
#[inline]
pub fn memop_type_to_str(t: i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_memopTypeToStr_int(t, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * src2: noArray()
/// * src3: noArray()
/// * src4: noArray()
/// * src5: noArray()
/// * src6: noArray()
/// * src7: noArray()
/// * src8: noArray()
/// * src9: noArray()
// predictOptimalVectorWidthMax(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core/ocl.hpp:733
#[inline]
pub fn predict_optimal_vector_width_max(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, src3: &dyn core::ToInputArray, src4: &dyn core::ToInputArray, src5: &dyn core::ToInputArray, src6: &dyn core::ToInputArray, src7: &dyn core::ToInputArray, src8: &dyn core::ToInputArray, src9: &dyn core::ToInputArray) -> Result<i32> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(src3);
	input_array_arg!(src4);
	input_array_arg!(src5);
	input_array_arg!(src6);
	input_array_arg!(src7);
	input_array_arg!(src8);
	input_array_arg!(src9);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_predictOptimalVectorWidthMax_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), src3.as_raw__InputArray(), src4.as_raw__InputArray(), src5.as_raw__InputArray(), src6.as_raw__InputArray(), src7.as_raw__InputArray(), src8.as_raw__InputArray(), src9.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * src2: noArray()
/// * src3: noArray()
/// * src4: noArray()
/// * src5: noArray()
/// * src6: noArray()
/// * src7: noArray()
/// * src8: noArray()
/// * src9: noArray()
/// * strat: OCL_VECTOR_DEFAULT
// predictOptimalVectorWidth(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::ocl::OclVectorStrategy) /usr/include/opencv2/core/ocl.hpp:721
#[inline]
pub fn predict_optimal_vector_width(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, src3: &dyn core::ToInputArray, src4: &dyn core::ToInputArray, src5: &dyn core::ToInputArray, src6: &dyn core::ToInputArray, src7: &dyn core::ToInputArray, src8: &dyn core::ToInputArray, src9: &dyn core::ToInputArray, strat: core::OclVectorStrategy) -> Result<i32> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(src3);
	input_array_arg!(src4);
	input_array_arg!(src5);
	input_array_arg!(src6);
	input_array_arg!(src7);
	input_array_arg!(src8);
	input_array_arg!(src9);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_predictOptimalVectorWidth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(src1.as_raw__InputArray(), src2.as_raw__InputArray(), src3.as_raw__InputArray(), src4.as_raw__InputArray(), src5.as_raw__InputArray(), src6.as_raw__InputArray(), src7.as_raw__InputArray(), src8.as_raw__InputArray(), src9.as_raw__InputArray(), strat, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setUseOpenCL(bool) /usr/include/opencv2/core/ocl.hpp:58
#[inline]
pub fn set_use_opencl(flag: bool) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_setUseOpenCL_bool(flag, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// typeToStr(int) /usr/include/opencv2/core/ocl.hpp:701
#[inline]
pub fn type_to_str(t: i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_typeToStr_int(t, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// useOpenCL() /usr/include/opencv2/core/ocl.hpp:55
#[inline]
pub fn use_opencl() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_useOpenCL(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// vecopTypeToStr(int) /usr/include/opencv2/core/ocl.hpp:703
#[inline]
pub fn vecop_type_to_str(t: i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ocl_vecopTypeToStr_int(t, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// convertFromGLTexture2D(const cv::ogl::Texture2D &, cv::OutputArray) /usr/include/opencv2/core/opengl.hpp:538
#[inline]
pub fn convert_from_gl_texture_2d(texture: &core::Texture2D, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ogl_convertFromGLTexture2D_const_Texture2DR_const__OutputArrayR(texture.as_raw_Texture2D(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// convertToGLTexture2D(cv::InputArray, cv::ogl::Texture2D &) /usr/include/opencv2/core/opengl.hpp:532
#[inline]
pub fn convert_to_gl_texture_2d(src: &dyn core::ToInputArray, texture: &mut core::Texture2D) -> Result<()> {
	input_array_arg!(src);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ogl_convertToGLTexture2D_const__InputArrayR_Texture2DR(src.as_raw__InputArray(), texture.as_raw_mut_Texture2D(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * access_flags: ACCESS_READ|ACCESS_WRITE
// mapGLBuffer(const cv::ogl::Buffer &, cv::AccessFlag) /usr/include/opencv2/core/opengl.hpp:551
#[inline]
pub fn map_gl_buffer(buffer: &core::Buffer, access_flags: core::AccessFlag) -> Result<core::UMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ogl_mapGLBuffer_const_BufferR_AccessFlag(buffer.as_raw_Buffer(), access_flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::UMat::opencv_from_extern(ret) };
	Ok(ret)
}

// initializeContextFromGL() /usr/include/opencv2/core/opengl.hpp:524
#[inline]
pub fn initialize_context_from_gl() -> Result<core::Context> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ogl_ocl_initializeContextFromGL(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Context::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * mode: POINTS
/// * color: Scalar::all(255)
// render(const cv::ogl::Arrays &, cv::InputArray, int, cv::Scalar) /usr/include/opencv2/core/opengl.hpp:513
#[inline]
pub fn render_2(arr: &core::Arrays, indices: &dyn core::ToInputArray, mode: i32, color: core::Scalar) -> Result<()> {
	input_array_arg!(indices);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ogl_render_const_ArraysR_const__InputArrayR_int_Scalar(arr.as_raw_Arrays(), indices.as_raw__InputArray(), mode, color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mode: POINTS
/// * color: Scalar::all(255)
// render(const cv::ogl::Arrays &, int, cv::Scalar) /usr/include/opencv2/core/opengl.hpp:505
#[inline]
pub fn render_1(arr: &core::Arrays, mode: i32, color: core::Scalar) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ogl_render_const_ArraysR_int_Scalar(arr.as_raw_Arrays(), mode, color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * wnd_rect: Rect_<double>(0.0,0.0,1.0,1.0)
/// * tex_rect: Rect_<double>(0.0,0.0,1.0,1.0)
// render(const cv::ogl::Texture2D &, Rect_<double>, Rect_<double>) /usr/include/opencv2/core/opengl.hpp:496
#[inline]
pub fn render(tex: &core::Texture2D, wnd_rect: core::Rect_<f64>, tex_rect: core::Rect_<f64>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ogl_render_const_Texture2DR_Rect__double__Rect__double_(tex.as_raw_Texture2D(), wnd_rect.opencv_as_extern(), tex_rect.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// unmapGLBuffer(cv::UMat &) /usr/include/opencv2/core/opengl.hpp:559
#[inline]
pub fn unmap_gl_buffer(u: &mut core::UMat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ogl_unmapGLBuffer_UMatR(u.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// operator+(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3613
#[inline]
pub fn add_matexpr_matexpr(e1: &core::MatExpr, e2: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_MatExprR_const_MatExprR(e1.as_raw_MatExpr(), e2.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3609
#[inline]
pub fn add_matexpr_mat(e: &core::MatExpr, m: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_MatExprR_const_MatR(e.as_raw_MatExpr(), m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::MatExpr &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3611
#[inline]
pub fn add_matexpr_scalar(e: &core::MatExpr, s: core::Scalar) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_MatExprR_const_ScalarR(e.as_raw_MatExpr(), &s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3610
#[inline]
pub fn add_mat_matexpr(m: &core::Mat, e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_MatR_const_MatExprR(m.as_raw_Mat(), e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3606
#[inline]
pub fn add_mat_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::Mat &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3607
#[inline]
pub fn add_mat_scalar(a: &core::Mat, s: core::Scalar) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_MatR_const_ScalarR(a.as_raw_Mat(), &s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::Scalar &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3612
#[inline]
pub fn add_scalar_matexpr(s: core::Scalar, e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_ScalarR_const_MatExprR(&s, e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::Scalar &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3608
#[inline]
pub fn add_scalar_mat(s: core::Scalar, a: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_ScalarR_const_MatR(&s, a.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3655
#[inline]
pub fn div_matexpr_matexpr(e1: &core::MatExpr, e2: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_const_MatExprR_const_MatExprR(e1.as_raw_MatExpr(), e2.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3651
#[inline]
pub fn div_matexpr_mat(e: &core::MatExpr, m: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_const_MatExprR_const_MatR(e.as_raw_MatExpr(), m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(const cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3653
#[inline]
pub fn div_matexpr_f64(e: &core::MatExpr, s: f64) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_const_MatExprR_double(e.as_raw_MatExpr(), s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3652
#[inline]
pub fn div_mat_matexpr(m: &core::Mat, e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_const_MatR_const_MatExprR(m.as_raw_Mat(), e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3648
#[inline]
pub fn div_mat_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3649
#[inline]
pub fn div_mat_f64(a: &core::Mat, s: f64) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_const_MatR_double(a.as_raw_Mat(), s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(double, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3654
#[inline]
pub fn div_f64_matexpr(s: f64, e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_double_const_MatExprR(s, e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3650
#[inline]
pub fn div_f64_mat(s: f64, a: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_double_const_MatR(s, a.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator==(const cv::FileNodeIterator &, const cv::FileNodeIterator &) /usr/include/opencv2/core/persistence.hpp:1331
#[inline]
pub fn equals_filenode_iter(it1: &core::FileNodeIterator, it2: &core::FileNodeIterator) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorEQ_const_FileNodeIteratorR_const_FileNodeIteratorR(it1.as_raw_FileNodeIterator(), it2.as_raw_FileNodeIterator(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// operator==(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3677
#[inline]
pub fn equals(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorEQ_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator==(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3678
#[inline]
pub fn equals_mat_f64(a: &core::Mat, s: f64) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorEQ_const_MatR_double(a.as_raw_Mat(), s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator==(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3679
#[inline]
pub fn equals_f64_mat(s: f64, a: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorEQ_double_const_MatR(s, a.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3633
#[inline]
pub fn sub_matexpr(e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_MatExprR(e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3626
#[inline]
pub fn sub_matexpr_matexpr(e1: &core::MatExpr, e2: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_MatExprR_const_MatExprR(e1.as_raw_MatExpr(), e2.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3622
#[inline]
pub fn sub_matexpr_mat(e: &core::MatExpr, m: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_MatExprR_const_MatR(e.as_raw_MatExpr(), m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::MatExpr &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3624
#[inline]
pub fn sub_matexpr_scalar(e: &core::MatExpr, s: core::Scalar) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_MatExprR_const_ScalarR(e.as_raw_MatExpr(), &s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3632
#[inline]
pub fn sub_mat(m: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_MatR(m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3623
#[inline]
pub fn sub_mat_matexpr(m: &core::Mat, e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_MatR_const_MatExprR(m.as_raw_Mat(), e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3619
#[inline]
pub fn sub_mat_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::Mat &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3620
#[inline]
pub fn sub_mat_scalar(a: &core::Mat, s: core::Scalar) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_MatR_const_ScalarR(a.as_raw_Mat(), &s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::Scalar &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3625
#[inline]
pub fn sub_scalar_matexpr(s: core::Scalar, e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_ScalarR_const_MatExprR(&s, e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::Scalar &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3621
#[inline]
pub fn sub_scalar_mat(s: core::Scalar, a: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_ScalarR_const_MatR(&s, a.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3642
#[inline]
pub fn mul_matexpr_matexpr(e1: &core::MatExpr, e2: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_const_MatExprR_const_MatExprR(e1.as_raw_MatExpr(), e2.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3638
#[inline]
pub fn mul_matexpr_mat(e: &core::MatExpr, m: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_const_MatExprR_const_MatR(e.as_raw_MatExpr(), m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(const cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3640
#[inline]
pub fn mul_matexpr_f64(e: &core::MatExpr, s: f64) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_const_MatExprR_double(e.as_raw_MatExpr(), s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3639
#[inline]
pub fn mul_mat_matexpr(m: &core::Mat, e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_const_MatR_const_MatExprR(m.as_raw_Mat(), e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3635
#[inline]
pub fn mul_mat_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3636
#[inline]
pub fn mul_mat_f64(a: &core::Mat, s: f64) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_const_MatR_double(a.as_raw_Mat(), s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(double, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3641
#[inline]
pub fn mul_f64_matexpr(s: f64, e: &core::MatExpr) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_double_const_MatExprR(s, e.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3637
#[inline]
pub fn mul_f64_mat(s: f64, a: &core::Mat) -> Result<core::MatExpr> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_double_const_MatR(s, a.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * nstripes: -1.
// parallel_for_(const cv::Range &, const cv::ParallelLoopBody &, double) /usr/include/opencv2/core/utility.hpp:587
#[inline]
pub fn parallel_for_(range: &core::Range, body: &dyn core::ParallelLoopBody, nstripes: f64) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_parallel_for__const_RangeR_const_ParallelLoopBodyR_double(range.as_raw_Range(), body.as_raw_ParallelLoopBody(), nstripes, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * val: 0
// patchNaNs(cv::InputOutputArray, double) /usr/include/opencv2/core.hpp:1666
#[inline]
pub fn patch_na_ns(a: &mut dyn core::ToInputOutputArray, val: f64) -> Result<()> {
	input_output_array_arg!(a);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_patchNaNs_const__InputOutputArrayR_double(a.as_raw__InputOutputArray(), val, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// perspectiveTransform(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1803
#[inline]
pub fn perspective_transform(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, m: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(m);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_perspectiveTransform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * angle_in_degrees: false
// phase(cv::InputArray, cv::InputArray, cv::OutputArray, bool) /usr/include/opencv2/core.hpp:1628
#[inline]
pub fn phase(x: &dyn core::ToInputArray, y: &dyn core::ToInputArray, angle: &mut dyn core::ToOutputArray, angle_in_degrees: bool) -> Result<()> {
	input_array_arg!(x);
	input_array_arg!(y);
	output_array_arg!(angle);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(x.as_raw__InputArray(), y.as_raw__InputArray(), angle.as_raw__OutputArray(), angle_in_degrees, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * angle_in_degrees: false
// polarToCart(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, bool) /usr/include/opencv2/core.hpp:1587
#[inline]
pub fn polar_to_cart(magnitude: &dyn core::ToInputArray, angle: &dyn core::ToInputArray, x: &mut dyn core::ToOutputArray, y: &mut dyn core::ToOutputArray, angle_in_degrees: bool) -> Result<()> {
	input_array_arg!(magnitude);
	input_array_arg!(angle);
	output_array_arg!(x);
	output_array_arg!(y);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(magnitude.as_raw__InputArray(), angle.as_raw__InputArray(), x.as_raw__OutputArray(), y.as_raw__OutputArray(), angle_in_degrees, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// pow(cv::InputArray, double, cv::OutputArray) /usr/include/opencv2/core.hpp:1536
#[inline]
pub fn pow(src: &dyn core::ToInputArray, power: f64, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_pow_const__InputArrayR_double_const__OutputArrayR(src.as_raw__InputArray(), power, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * iter_factor: 1.
/// * rng: 0
// randShuffle(cv::InputOutputArray, double, cv::RNG *) /usr/include/opencv2/core.hpp:2382
#[inline]
pub fn rand_shuffle(dst: &mut dyn core::ToInputOutputArray, iter_factor: f64, rng: &mut core::RNG) -> Result<()> {
	input_output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_randShuffle_const__InputOutputArrayR_double_RNGX(dst.as_raw__InputOutputArray(), iter_factor, rng.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// randn(cv::InputOutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2369
#[inline]
pub fn randn(dst: &mut dyn core::ToInputOutputArray, mean: &dyn core::ToInputArray, stddev: &dyn core::ToInputArray) -> Result<()> {
	input_output_array_arg!(dst);
	input_array_arg!(mean);
	input_array_arg!(stddev);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_randn_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(dst.as_raw__InputOutputArray(), mean.as_raw__InputArray(), stddev.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// randu(cv::InputOutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2356
#[inline]
pub fn randu(dst: &mut dyn core::ToInputOutputArray, low: &dyn core::ToInputArray, high: &dyn core::ToInputArray) -> Result<()> {
	input_output_array_arg!(dst);
	input_array_arg!(low);
	input_array_arg!(high);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_randu_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(dst.as_raw__InputOutputArray(), low.as_raw__InputArray(), high.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// read(const cv::FileNode &, cv::DMatch &, const cv::DMatch &) /usr/include/opencv2/core/persistence.hpp:734
#[inline]
pub fn read_dmatch(node: &core::FileNode, value: &mut core::DMatch, default_value: core::DMatch) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_DMatchR_const_DMatchR(node.as_raw_FileNode(), value, &default_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// read(const cv::FileNode &, cv::KeyPoint &, const cv::KeyPoint &) /usr/include/opencv2/core/persistence.hpp:733
#[inline]
pub fn read_keypoint(node: &core::FileNode, value: &mut core::KeyPoint, default_value: core::KeyPoint) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_KeyPointR_const_KeyPointR(node.as_raw_FileNode(), value, &default_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * default_mat: Mat()
// read(const cv::FileNode &, cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/persistence.hpp:727
#[inline]
pub fn read_mat(node: &core::FileNode, mat: &mut core::Mat, default_mat: &core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_MatR_const_MatR(node.as_raw_FileNode(), mat.as_raw_mut_Mat(), default_mat.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * default_mat: SparseMat()
// read(const cv::FileNode &, cv::SparseMat &, const cv::SparseMat &) /usr/include/opencv2/core/persistence.hpp:728
#[inline]
pub fn read_sparsemat(node: &core::FileNode, mat: &mut core::SparseMat, default_mat: &core::SparseMat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_SparseMatR_const_SparseMatR(node.as_raw_FileNode(), mat.as_raw_mut_SparseMat(), default_mat.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// read(const cv::FileNode &, double &, double) /usr/include/opencv2/core/persistence.hpp:725
#[inline]
pub fn read_f64(node: &core::FileNode, value: &mut f64, default_value: f64) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_doubleR_double(node.as_raw_FileNode(), value, default_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// read(const cv::FileNode &, float &, float) /usr/include/opencv2/core/persistence.hpp:724
#[inline]
pub fn read_f32(node: &core::FileNode, value: &mut f32, default_value: f32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_floatR_float(node.as_raw_FileNode(), value, default_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// read(const cv::FileNode &, int &, int) /usr/include/opencv2/core/persistence.hpp:723
#[inline]
pub fn read_i32(node: &core::FileNode, value: &mut i32, default_value: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_intR_int(node.as_raw_FileNode(), value, default_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// read(const cv::FileNode &, std::string &, const std::string &) /usr/include/opencv2/core/persistence.hpp:726
#[inline]
pub fn read_str(node: &core::FileNode, value: &mut String, default_value: &str) -> Result<()> {
	string_arg_output_send!(via value_via);
	extern_container_arg!(default_value);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_stringR_const_stringR(node.as_raw_FileNode(), &mut value_via, default_value.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	string_arg_output_receive!(value_via => value);
	Ok(ret)
}

// read(const cv::FileNode &, std::vector<DMatch> &) /usr/include/opencv2/core/persistence.hpp:731
#[inline]
pub fn read_dmatch_vec_legacy(node: &core::FileNode, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_vector_DMatch_R(node.as_raw_FileNode(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// read(const cv::FileNode &, std::vector<KeyPoint> &) /usr/include/opencv2/core/persistence.hpp:730
#[inline]
pub fn read_keypoint_vec_legacy(node: &core::FileNode, keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_vector_KeyPoint_R(node.as_raw_FileNode(), keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * last_index: false
// reduceArgMax(cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/core.hpp:860
#[inline]
pub fn reduce_arg_max(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, axis: i32, last_index: bool) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_reduceArgMax_const__InputArrayR_const__OutputArrayR_int_bool(src.as_raw__InputArray(), dst.as_raw__OutputArray(), axis, last_index, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * last_index: false
// reduceArgMin(cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/core.hpp:843
#[inline]
pub fn reduce_arg_min(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, axis: i32, last_index: bool) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_reduceArgMin_const__InputArrayR_const__OutputArrayR_int_bool(src.as_raw__InputArray(), dst.as_raw__OutputArray(), axis, last_index, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * dtype: -1
// reduce(cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/core.hpp:924
#[inline]
pub fn reduce(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, dim: i32, rtype: i32, dtype: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_reduce_const__InputArrayR_const__OutputArrayR_int_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dim, rtype, dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// repeat(const cv::Mat &, int, int) /usr/include/opencv2/core.hpp:1145
#[inline]
pub fn repeat(src: &core::Mat, ny: i32, nx: i32) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_repeat_const_MatR_int_int(src.as_raw_Mat(), ny, nx, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// repeat(cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/core.hpp:1136
#[inline]
pub fn repeat_to(src: &dyn core::ToInputArray, ny: i32, nx: i32, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_repeat_const__InputArrayR_int_int_const__OutputArrayR(src.as_raw__InputArray(), ny, nx, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// rotate(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1121
#[inline]
pub fn rotate(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, rotate_code: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rotate_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), rotate_code, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// addSamplesDataSearchPath(const cv::String &) /usr/include/opencv2/core/utility.hpp:1200
#[inline]
pub fn add_samples_data_search_path(path: &str) -> Result<()> {
	extern_container_arg!(path);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_samples_addSamplesDataSearchPath_const_StringR(path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// addSamplesDataSearchSubDirectory(const cv::String &) /usr/include/opencv2/core/utility.hpp:1209
#[inline]
pub fn add_samples_data_search_sub_directory(subdir: &str) -> Result<()> {
	extern_container_arg!(subdir);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_samples_addSamplesDataSearchSubDirectory_const_StringR(subdir.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * silent_mode: false
// findFileOrKeep(const cv::String &, bool) /usr/include/opencv2/core/utility.hpp:1183
#[inline]
pub fn find_file_or_keep(relative_path: &str, silent_mode: bool) -> Result<String> {
	extern_container_arg!(relative_path);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_samples_findFileOrKeep_const_StringR_bool(relative_path.opencv_as_extern(), silent_mode, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * required: true
/// * silent_mode: false
// findFile(const cv::String &, bool, bool) /usr/include/opencv2/core/utility.hpp:1181
#[inline]
pub fn find_file(relative_path: &str, required: bool, silent_mode: bool) -> Result<String> {
	extern_container_arg!(relative_path);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_samples_findFile_const_StringR_bool_bool(relative_path.opencv_as_extern(), required, silent_mode, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// scaleAdd(cv::InputArray, double, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:478
#[inline]
pub fn scale_add(src1: &dyn core::ToInputArray, alpha: f64, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_scaleAdd_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), alpha, src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setBreakOnError(bool) /usr/include/opencv2/core/utility.hpp:160
#[inline]
pub fn set_break_on_error(flag: bool) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_setBreakOnError_bool(flag, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * s: Scalar(1)
// setIdentity(cv::InputOutputArray, const cv::Scalar &) /usr/include/opencv2/core.hpp:1836
#[inline]
pub fn set_identity(mtx: &mut dyn core::ToInputOutputArray, s: core::Scalar) -> Result<()> {
	input_output_array_arg!(mtx);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_setIdentity_const__InputOutputArrayR_const_ScalarR(mtx.as_raw__InputOutputArray(), &s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setLogLevel(int) /usr/include/opencv2/core/bindings_utils.hpp:285
#[inline]
pub fn set_log_level_1(level: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_setLogLevel_int(level, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setNumThreads(int) /usr/include/opencv2/core/utility.hpp:200
#[inline]
pub fn set_num_threads(nthreads: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_setNumThreads_int(nthreads, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setRNGSeed(int) /usr/include/opencv2/core.hpp:2344
#[inline]
pub fn set_rng_seed(seed: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_setRNGSeed_int(seed, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setUseOpenVX(bool) /usr/include/opencv2/core/ovx.hpp:25
#[inline]
pub fn set_use_openvx(flag: bool) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_setUseOpenVX_bool(flag, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setUseOptimized(bool) /usr/include/opencv2/core/utility.hpp:560
#[inline]
pub fn set_use_optimized(onoff: bool) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_setUseOptimized_bool(onoff, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// solveCubic(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1962
#[inline]
pub fn solve_cubic(coeffs: &dyn core::ToInputArray, roots: &mut dyn core::ToOutputArray) -> Result<i32> {
	input_array_arg!(coeffs);
	output_array_arg!(roots);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solveCubic_const__InputArrayR_const__OutputArrayR(coeffs.as_raw__InputArray(), roots.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// solveLP(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core/optim.hpp:296
#[inline]
pub fn solve_lp(func: &dyn core::ToInputArray, constr: &dyn core::ToInputArray, z: &mut dyn core::ToOutputArray) -> Result<i32> {
	input_array_arg!(func);
	input_array_arg!(constr);
	output_array_arg!(z);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solveLP_const__InputArrayR_const__InputArrayR_const__OutputArrayR(func.as_raw__InputArray(), constr.as_raw__InputArray(), z.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * max_iters: 300
// solvePoly(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1972
#[inline]
pub fn solve_poly(coeffs: &dyn core::ToInputArray, roots: &mut dyn core::ToOutputArray, max_iters: i32) -> Result<f64> {
	input_array_arg!(coeffs);
	output_array_arg!(roots);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePoly_const__InputArrayR_const__OutputArrayR_int(coeffs.as_raw__InputArray(), roots.as_raw__OutputArray(), max_iters, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: DECOMP_LU
// solve(cv::InputArray, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1911
#[inline]
pub fn solve(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<bool> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// sortIdx(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1947
#[inline]
pub fn sort_idx(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sortIdx_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// sort(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1927
#[inline]
pub fn sort(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sort_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// split(const cv::Mat &, cv::Mat *) /usr/include/opencv2/core.hpp:970
#[inline]
pub fn split_slice(src: &core::Mat, mvbegin: &mut core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_split_const_MatR_MatX(src.as_raw_Mat(), mvbegin.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// split(cv::InputArray, cv::OutputArrayOfArrays) /usr/include/opencv2/core.hpp:976
#[inline]
pub fn split(m: &dyn core::ToInputArray, mv: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(m);
	output_array_arg!(mv);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_split_const__InputArrayR_const__OutputArrayR(m.as_raw__InputArray(), mv.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// sqrt(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1511
#[inline]
pub fn sqrt(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sqrt_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
/// * dtype: -1
// subtract(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, int) /usr/include/opencv2/core.hpp:400
#[inline]
pub fn subtract(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// sum(cv::InputArray) /usr/include/opencv2/core.hpp:572
#[inline]
pub fn sum_elems(src: &dyn core::ToInputArray) -> Result<core::Scalar> {
	input_array_arg!(src);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sum_const__InputArrayR(src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// swap(cv::Mat &, cv::Mat &) /usr/include/opencv2/core.hpp:240
#[inline]
pub fn swap(a: &mut core::Mat, b: &mut core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_swap_MatR_MatR(a.as_raw_mut_Mat(), b.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// swap(cv::UMat &, cv::UMat &) /usr/include/opencv2/core.hpp:242
#[inline]
pub fn swap_umat(a: &mut core::UMat, b: &mut core::UMat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_swap_UMatR_UMatR(a.as_raw_mut_UMat(), b.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * suffix: 0
// tempfile(const char *) /usr/include/opencv2/core/utility.hpp:179
#[inline]
pub fn tempfile(suffix: &str) -> Result<String> {
	extern_container_arg!(suffix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_tempfile_const_charX(suffix.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// theRNG() /usr/include/opencv2/core.hpp:2336
#[inline]
pub fn the_rng() -> Result<core::RNG> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_theRNG(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::RNG::opencv_from_extern(ret) };
	Ok(ret)
}

// trace(cv::InputArray) /usr/include/opencv2/core.hpp:1860
#[inline]
pub fn trace(mtx: &dyn core::ToInputArray) -> Result<core::Scalar> {
	input_array_arg!(mtx);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_trace_const__InputArrayR(mtx.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// transform(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1776
#[inline]
pub fn transform(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, m: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(m);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_transform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// transposeND(cv::InputArray, const std::vector<int> &, cv::OutputArray) /usr/include/opencv2/core.hpp:1750
#[inline]
pub fn transpose_nd(src: &dyn core::ToInputArray, order: &core::Vector<i32>, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_transposeND_const__InputArrayR_const_vector_int_R_const__OutputArrayR(src.as_raw__InputArray(), order.as_raw_VectorOfi32(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// transpose(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1740
#[inline]
pub fn transpose(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_transpose_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// typeToString(int) /usr/include/opencv2/core/check.hpp:16
#[inline]
pub fn type_to_string(typ: i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_typeToString_int(typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// useOpenVX() /usr/include/opencv2/core/ovx.hpp:22
#[inline]
pub fn use_openvx() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_useOpenVX(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// useOptimized() /usr/include/opencv2/core/utility.hpp:566
#[inline]
pub fn use_optimized() -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_useOptimized(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// dumpBool(bool) /usr/include/opencv2/core/bindings_utils.hpp:27
#[inline]
pub fn dump_bool(argument: bool) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpBool_bool(argument, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpCString(const char *) /usr/include/opencv2/core/bindings_utils.hpp:59
#[inline]
pub fn dump_c_string(argument: &str) -> Result<String> {
	extern_container_arg!(argument);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpCString_const_charX(argument.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpDouble(double) /usr/include/opencv2/core/bindings_utils.hpp:53
#[inline]
pub fn dump_double(argument: f64) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpDouble_double(argument, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpFloat(float) /usr/include/opencv2/core/bindings_utils.hpp:47
#[inline]
pub fn dump_float(argument: f32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpFloat_float(argument, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpInputArrayOfArrays(cv::InputArrayOfArrays) /usr/include/opencv2/core/bindings_utils.hpp:20
#[inline]
pub fn dump_input_array_of_arrays(argument: &dyn core::ToInputArray) -> Result<String> {
	input_array_arg!(argument);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpInputArrayOfArrays_const__InputArrayR(argument.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpInputArray(cv::InputArray) /usr/include/opencv2/core/bindings_utils.hpp:18
#[inline]
pub fn dump_input_array(argument: &dyn core::ToInputArray) -> Result<String> {
	input_array_arg!(argument);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpInputArray_const__InputArrayR(argument.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpInputOutputArrayOfArrays(cv::InputOutputArrayOfArrays) /usr/include/opencv2/core/bindings_utils.hpp:24
#[inline]
pub fn dump_input_output_array_of_arrays(argument: &mut dyn core::ToInputOutputArray) -> Result<String> {
	input_output_array_arg!(argument);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpInputOutputArrayOfArrays_const__InputOutputArrayR(argument.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpInputOutputArray(cv::InputOutputArray) /usr/include/opencv2/core/bindings_utils.hpp:22
#[inline]
pub fn dump_input_output_array(argument: &mut dyn core::ToInputOutputArray) -> Result<String> {
	input_output_array_arg!(argument);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpInputOutputArray_const__InputOutputArrayR(argument.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpInt(int) /usr/include/opencv2/core/bindings_utils.hpp:33
#[inline]
pub fn dump_int(argument: i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpInt_int(argument, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpRange(const cv::Range &) /usr/include/opencv2/core/bindings_utils.hpp:122
#[inline]
pub fn dump_range(argument: &core::Range) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpRange_const_RangeR(argument.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpRect(const cv::Rect &) /usr/include/opencv2/core/bindings_utils.hpp:85
#[inline]
pub fn dump_rect(argument: core::Rect) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpRect_const_RectR(&argument, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpRotatedRect(const cv::RotatedRect &) /usr/include/opencv2/core/bindings_utils.hpp:99
#[inline]
pub fn dump_rotated_rect(argument: &core::RotatedRect) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpRotatedRect_const_RotatedRectR(argument.as_raw_RotatedRect(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpSizeT(size_t) /usr/include/opencv2/core/bindings_utils.hpp:39
#[inline]
pub fn dump_size_t(argument: size_t) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpSizeT_size_t(argument, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpString(const cv::String &) /usr/include/opencv2/core/bindings_utils.hpp:65
#[inline]
pub fn dump_string(argument: &str) -> Result<String> {
	extern_container_arg!(argument);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpString_const_StringR(argument.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/core/bindings_utils.hpp:92
#[inline]
pub fn dump_term_criteria(argument: core::TermCriteria) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpTermCriteria_const_TermCriteriaR(&argument, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpVectorOfDouble(const std::vector<double> &) /usr/include/opencv2/core/bindings_utils.hpp:148
#[inline]
pub fn dump_vector_of_double(vec: &core::Vector<f64>) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpVectorOfDouble_const_vector_double_R(vec.as_raw_VectorOff64(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpVectorOfInt(const std::vector<int> &) /usr/include/opencv2/core/bindings_utils.hpp:146
#[inline]
pub fn dump_vector_of_int(vec: &core::Vector<i32>) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpVectorOfInt_const_vector_int_R(vec.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// dumpVectorOfRect(const std::vector<Rect> &) /usr/include/opencv2/core/bindings_utils.hpp:150
#[inline]
pub fn dump_vector_of_rect(vec: &core::Vector<core::Rect>) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_dumpVectorOfRect_const_vector_Rect_R(vec.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getCacheDirectoryForDownloads() /usr/include/opencv2/core/bindings_utils.hpp:276
#[inline]
pub fn get_cache_directory_for_downloads() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_fs_getCacheDirectoryForDownloads(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// generateVectorOfInt(size_t, std::vector<int> &) /usr/include/opencv2/core/bindings_utils.hpp:166
#[inline]
pub fn generate_vector_of_int(len: size_t, vec: &mut core::Vector<i32>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_generateVectorOfInt_size_t_vector_int_R(len, vec.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// generateVectorOfMat(size_t, int, int, int, std::vector<Mat> &) /usr/include/opencv2/core/bindings_utils.hpp:179
#[inline]
pub fn generate_vector_of_mat(len: size_t, rows: i32, cols: i32, dtype: i32, vec: &mut core::Vector<core::Mat>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_generateVectorOfMat_size_t_int_int_int_vector_Mat_R(len, rows, cols, dtype, vec.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// generateVectorOfRect(size_t, std::vector<Rect> &) /usr/include/opencv2/core/bindings_utils.hpp:153
#[inline]
pub fn generate_vector_of_rect(len: size_t, vec: &mut core::Vector<core::Rect>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_generateVectorOfRect_size_t_vector_Rect_R(len, vec.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getThreadID() /usr/include/opencv2/core/utility.hpp:1216
#[inline]
pub fn get_thread_id() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_getThreadID(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getLogLevel() /usr/include/opencv2/core/utils/logger.hpp:27
#[inline]
pub fn get_log_level() -> Result<core::LogLevel> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_logging_getLogLevel(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getLogTagLevel(const char *) /usr/include/opencv2/core/utils/logger.hpp:33
#[inline]
pub fn get_log_tag_level(tag: &str) -> Result<core::LogLevel> {
	extern_container_arg!(tag);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_logging_getLogTagLevel_const_charX(tag.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getGlobalLogTag() /usr/include/opencv2/core/utils/logger.hpp:38
#[inline]
pub fn get_global_log_tag() -> Result<core::LogTag> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_logging_internal_getGlobalLogTag(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::LogTag::opencv_from_extern(ret) };
	Ok(ret)
}

// writeLogMessageEx(cv::utils::logging::LogLevel, const char *, const char *, int, const char *, const char *) /usr/include/opencv2/core/utils/logger.hpp:44
#[inline]
pub fn write_log_message_ex(log_level: core::LogLevel, tag: &str, file: &str, line: i32, func: &str, message: &str) -> Result<()> {
	extern_container_arg!(tag);
	extern_container_arg!(file);
	extern_container_arg!(func);
	extern_container_arg!(message);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_logging_internal_writeLogMessageEx_LogLevel_const_charX_const_charX_int_const_charX_const_charX(log_level, tag.opencv_as_extern(), file.opencv_as_extern(), line, func.opencv_as_extern(), message.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// writeLogMessage(cv::utils::logging::LogLevel, const char *) /usr/include/opencv2/core/utils/logger.hpp:41
#[inline]
pub fn write_log_message(log_level: core::LogLevel, message: &str) -> Result<()> {
	extern_container_arg!(message);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_logging_internal_writeLogMessage_LogLevel_const_charX(log_level, message.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// registerLogTag(cv::utils::logging::LogTag *) /usr/include/opencv2/core/utils/logger.hpp:29
#[inline]
pub fn register_log_tag(plogtag: &mut core::LogTag) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_logging_registerLogTag_LogTagX(plogtag.as_raw_mut_LogTag(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setLogLevel(cv::utils::logging::LogLevel) /usr/include/opencv2/core/utils/logger.hpp:25
#[inline]
pub fn set_log_level(log_level: core::LogLevel) -> Result<core::LogLevel> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_logging_setLogLevel_LogLevel(log_level, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// setLogTagLevel(const char *, cv::utils::logging::LogLevel) /usr/include/opencv2/core/utils/logger.hpp:31
#[inline]
pub fn set_log_tag_level(tag: &str, level: core::LogLevel) -> Result<()> {
	extern_container_arg!(tag);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_logging_setLogTagLevel_const_charX_LogLevel(tag.opencv_as_extern(), level, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// testEchoBooleanFunction(bool) /usr/include/opencv2/core/bindings_utils.hpp:223
#[inline]
pub fn test_echo_boolean_function(flag: bool) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_nested_testEchoBooleanFunction_bool(flag, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// testAsyncArray(cv::InputArray) /usr/include/opencv2/core/bindings_utils.hpp:200
#[inline]
pub fn test_async_array(argument: &dyn core::ToInputArray) -> Result<core::AsyncArray> {
	input_array_arg!(argument);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_testAsyncArray_const__InputArrayR(argument.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::AsyncArray::opencv_from_extern(ret) };
	Ok(ret)
}

// testAsyncException() /usr/include/opencv2/core/bindings_utils.hpp:208
#[inline]
pub fn test_async_exception() -> Result<core::AsyncArray> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_testAsyncException(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::AsyncArray::opencv_from_extern(ret) };
	Ok(ret)
}

// testOverloadResolution(const cv::Rect &) /usr/include/opencv2/core/bindings_utils.hpp:78
#[inline]
pub fn test_overload_resolution_1(rect: core::Rect) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_testOverloadResolution_const_RectR(&rect, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * point: Point(42,24)
// testOverloadResolution(int, const cv::Point &) /usr/include/opencv2/core/bindings_utils.hpp:71
#[inline]
pub fn test_overload_resolution(value: i32, point: core::Point) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_testOverloadResolution_int_const_PointR(value, &point, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// testOverwriteNativeMethod(int) /usr/include/opencv2/core/bindings_utils.hpp:135
#[inline]
pub fn test_overwrite_native_method(argument: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_testOverwriteNativeMethod_int(argument, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// testRaiseGeneralException() /usr/include/opencv2/core/bindings_utils.hpp:194
#[inline]
pub fn test_raise_general_exception() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_testRaiseGeneralException(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * lambda: 2
/// * from: 3
// testReservedKeywordConversion(int, int, int) /usr/include/opencv2/core/bindings_utils.hpp:141
#[inline]
pub fn test_reserved_keyword_conversion(positional_argument: i32, lambda: i32, from: i32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_testReservedKeywordConversion_int_int_int(positional_argument, lambda, from, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// testRotatedRectVector(float, float, float, float, float) /usr/include/opencv2/core/bindings_utils.hpp:113
#[inline]
pub fn test_rotated_rect_vector(x: f32, y: f32, w: f32, h: f32, angle: f32) -> Result<core::Vector<core::RotatedRect>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_testRotatedRectVector_float_float_float_float_float(x, y, w, h, angle, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<core::RotatedRect>::opencv_from_extern(ret) };
	Ok(ret)
}

// testRotatedRect(float, float, float, float, float) /usr/include/opencv2/core/bindings_utils.hpp:107
#[inline]
pub fn test_rotated_rect(x: f32, y: f32, w: f32, h: f32, angle: f32) -> Result<core::RotatedRect> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_utils_testRotatedRect_float_float_float_float_float(x, y, w, h, angle, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::RotatedRect::opencv_from_extern(ret) };
	Ok(ret)
}

// convertFromVASurface(VADisplay, VASurfaceID, cv::Size, cv::OutputArray) /usr/include/opencv2/core/va_intel.hpp:69
#[inline]
pub unsafe fn convert_from_va_surface(display: core::va_display, surface: core::va_surface_id, size: core::Size, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	{ sys::cv_va_intel_convertFromVASurface_VADisplay_VASurfaceID_Size_const__OutputArrayR(display, surface, size.opencv_as_extern(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// convertToVASurface(VADisplay, cv::InputArray, VASurfaceID, cv::Size) /usr/include/opencv2/core/va_intel.hpp:61
#[inline]
pub unsafe fn convert_to_va_surface(display: core::va_display, src: &dyn core::ToInputArray, surface: core::va_surface_id, size: core::Size) -> Result<()> {
	input_array_arg!(src);
	return_send!(via ocvrs_return);
	{ sys::cv_va_intel_convertToVASurface_VADisplay_const__InputArrayR_VASurfaceID_Size(display, src.as_raw__InputArray(), surface, size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * try_interop: true
// initializeContextFromVA(VADisplay, bool) /usr/include/opencv2/core/va_intel.hpp:51
#[inline]
pub unsafe fn initialize_context_from_va(display: core::va_display, try_interop: bool) -> Result<core::Context> {
	return_send!(via ocvrs_return);
	{ sys::cv_va_intel_ocl_initializeContextFromVA_VADisplay_bool(display, try_interop, ocvrs_return.as_mut_ptr()) };
	return_receive!(ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = { core::Context::opencv_from_extern(ret) };
	Ok(ret)
}

// vconcat(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1254
#[inline]
pub fn vconcat2(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_vconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// vconcat(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/core.hpp:1272
#[inline]
pub fn vconcat(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_vconcat_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// writeScalar(cv::FileStorage &, const cv::String &) /usr/include/opencv2/core/persistence.hpp:716
#[inline]
pub fn write_scalar_str(fs: &mut core::FileStorage, value: &str) -> Result<()> {
	extern_container_arg!(value);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_writeScalar_FileStorageR_const_StringR(fs.as_raw_mut_FileStorage(), value.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// writeScalar(cv::FileStorage &, double) /usr/include/opencv2/core/persistence.hpp:715
#[inline]
pub fn write_scalar_f64(fs: &mut core::FileStorage, value: f64) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_writeScalar_FileStorageR_double(fs.as_raw_mut_FileStorage(), value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// writeScalar(cv::FileStorage &, float) /usr/include/opencv2/core/persistence.hpp:714
#[inline]
pub fn write_scalar_f32(fs: &mut core::FileStorage, value: f32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_writeScalar_FileStorageR_float(fs.as_raw_mut_FileStorage(), value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// writeScalar(cv::FileStorage &, int) /usr/include/opencv2/core/persistence.hpp:713
#[inline]
pub fn write_scalar_i32(fs: &mut core::FileStorage, value: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_writeScalar_FileStorageR_int(fs.as_raw_mut_FileStorage(), value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(cv::FileStorage &, const cv::String &, const cv::Mat &) /usr/include/opencv2/core/persistence.hpp:706
#[inline]
pub fn write_mat(fs: &mut core::FileStorage, name: &str, value: &core::Mat) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_MatR(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), value.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(cv::FileStorage &, const cv::String &, const cv::SparseMat &) /usr/include/opencv2/core/persistence.hpp:707
#[inline]
pub fn write_sparsemat(fs: &mut core::FileStorage, name: &str, value: &core::SparseMat) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_SparseMatR(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), value.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(cv::FileStorage &, const cv::String &, const cv::String &) /usr/include/opencv2/core/persistence.hpp:705
#[inline]
pub fn write_str(fs: &mut core::FileStorage, name: &str, value: &str) -> Result<()> {
	extern_container_arg!(name);
	extern_container_arg!(value);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_StringR(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), value.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(cv::FileStorage &, const cv::String &, const std::vector<DMatch> &) /usr/include/opencv2/core/persistence.hpp:710
#[inline]
pub fn write_dmatch_vec(fs: &mut core::FileStorage, name: &str, value: &core::Vector<core::DMatch>) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_vector_DMatch_R(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), value.as_raw_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(cv::FileStorage &, const cv::String &, const std::vector<KeyPoint> &) /usr/include/opencv2/core/persistence.hpp:709
#[inline]
pub fn write_keypoint_vec(fs: &mut core::FileStorage, name: &str, value: &core::Vector<core::KeyPoint>) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_vector_KeyPoint_R(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), value.as_raw_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(cv::FileStorage &, const cv::String &, double) /usr/include/opencv2/core/persistence.hpp:704
#[inline]
pub fn write_f64(fs: &mut core::FileStorage, name: &str, value: f64) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_double(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(cv::FileStorage &, const cv::String &, float) /usr/include/opencv2/core/persistence.hpp:703
#[inline]
pub fn write_f32(fs: &mut core::FileStorage, name: &str, value: f32) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_float(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(cv::FileStorage &, const cv::String &, int) /usr/include/opencv2/core/persistence.hpp:702
#[inline]
pub fn write_i32(fs: &mut core::FileStorage, name: &str, value: i32) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_int(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// Algorithm /usr/include/opencv2/core.hpp:3133
pub trait AlgorithmTraitConst {
	fn as_raw_Algorithm(&self) -> *const c_void;

	// write(cv::FileStorage &) /usr/include/opencv2/core.hpp:3145
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Algorithm_write_const_FileStorageR(self.as_raw_Algorithm(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * name: String()
	// write(const Ptr<cv::FileStorage> &, const cv::String &) /usr/include/opencv2/core.hpp:3150
	#[inline]
	fn write_with_name(&self, fs: &core::Ptr<core::FileStorage>, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Algorithm_write_const_const_Ptr_FileStorage_R_const_StringR(self.as_raw_Algorithm(), fs.as_raw_PtrOfFileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core.hpp:3158
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Algorithm_empty_const(self.as_raw_Algorithm(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// save(const cv::String &) /usr/include/opencv2/core.hpp:3222
	#[inline]
	fn save(&self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Algorithm_save_const_const_StringR(self.as_raw_Algorithm(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDefaultName() /usr/include/opencv2/core.hpp:3226
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Algorithm_getDefaultName_const(self.as_raw_Algorithm(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait AlgorithmTrait: core::AlgorithmTraitConst {
	fn as_raw_mut_Algorithm(&mut self) -> *mut c_void;

	// clear() /usr/include/opencv2/core.hpp:3141
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Algorithm_clear(self.as_raw_mut_Algorithm(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/core.hpp:3154
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Algorithm_read_const_FileNodeR(self.as_raw_mut_Algorithm(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Algorithm /usr/include/opencv2/core.hpp:3133
pub struct Algorithm {
	ptr: *mut c_void
}

opencv_type_boxed! { Algorithm }

impl Drop for Algorithm {
	fn drop(&mut self) {
		extern "C" { fn cv_Algorithm_delete(instance: *mut c_void); }
		unsafe { cv_Algorithm_delete(self.as_raw_mut_Algorithm()) };
	}
}

unsafe impl Send for Algorithm {}

impl core::AlgorithmTraitConst for Algorithm {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Algorithm {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Algorithm {
	// Algorithm() /usr/include/opencv2/core.hpp:3136
	#[inline]
	pub fn default() -> Result<core::Algorithm> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Algorithm_Algorithm(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Algorithm::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// AsyncArray /usr/include/opencv2/core/async.hpp:31
pub trait AsyncArrayTraitConst {
	fn as_raw_AsyncArray(&self) -> *const c_void;

	// get(cv::OutputArray) /usr/include/opencv2/core/async.hpp:50
	#[inline]
	fn get(&self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncArray_get_const_const__OutputArrayR(self.as_raw_AsyncArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// get(cv::OutputArray, int64) /usr/include/opencv2/core/async.hpp:60
	#[inline]
	fn get_with_timeout(&self, dst: &mut dyn core::ToOutputArray, timeout_ns: i64) -> Result<bool> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncArray_get_const_const__OutputArrayR_int64_t(self.as_raw_AsyncArray(), dst.as_raw__OutputArray(), timeout_ns, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// get(cv::OutputArray, double) /usr/include/opencv2/core/async.hpp:63
	#[inline]
	fn get_with_timeout_f64(&self, dst: &mut dyn core::ToOutputArray, timeout_ns: f64) -> Result<bool> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncArray_get_const_const__OutputArrayR_double(self.as_raw_AsyncArray(), dst.as_raw__OutputArray(), timeout_ns, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// wait_for(int64) /usr/include/opencv2/core/async.hpp:65
	#[inline]
	fn wait_for(&self, timeout_ns: i64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncArray_wait_for_const_int64_t(self.as_raw_AsyncArray(), timeout_ns, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// wait_for(double) /usr/include/opencv2/core/async.hpp:68
	#[inline]
	fn wait_for_f64(&self, timeout_ns: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncArray_wait_for_const_double(self.as_raw_AsyncArray(), timeout_ns, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// valid() /usr/include/opencv2/core/async.hpp:70
	#[inline]
	fn valid(&self) -> bool {
		let ret = unsafe { sys::cv_AsyncArray_valid_const(self.as_raw_AsyncArray()) };
		ret
	}
	
}

pub trait AsyncArrayTrait: core::AsyncArrayTraitConst {
	fn as_raw_mut_AsyncArray(&mut self) -> *mut c_void;

	// release() /usr/include/opencv2/core/async.hpp:38
	#[inline]
	fn release(&mut self) {
		let ret = unsafe { sys::cv_AsyncArray_release(self.as_raw_mut_AsyncArray()) };
		ret
	}
	
}

// AsyncArray /usr/include/opencv2/core/async.hpp:31
pub struct AsyncArray {
	ptr: *mut c_void
}

opencv_type_boxed! { AsyncArray }

impl Drop for AsyncArray {
	fn drop(&mut self) {
		extern "C" { fn cv_AsyncArray_delete(instance: *mut c_void); }
		unsafe { cv_AsyncArray_delete(self.as_raw_mut_AsyncArray()) };
	}
}

unsafe impl Send for AsyncArray {}

impl core::AsyncArrayTraitConst for AsyncArray {
	#[inline] fn as_raw_AsyncArray(&self) -> *const c_void { self.as_raw() }
}

impl core::AsyncArrayTrait for AsyncArray {
	#[inline] fn as_raw_mut_AsyncArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AsyncArray {
	// AsyncArray() /usr/include/opencv2/core/async.hpp:35
	#[inline]
	pub fn default() -> core::AsyncArray {
		let ret = unsafe { sys::cv_AsyncArray_AsyncArray() };
		let ret = unsafe { core::AsyncArray::opencv_from_extern(ret) };
		ret
	}
	
	// AsyncArray(const cv::AsyncArray &) /usr/include/opencv2/core/async.hpp:36
	#[inline]
	pub fn copy(o: &core::AsyncArray) -> core::AsyncArray {
		let ret = unsafe { sys::cv_AsyncArray_AsyncArray_const_AsyncArrayR(o.as_raw_AsyncArray()) };
		let ret = unsafe { core::AsyncArray::opencv_from_extern(ret) };
		ret
	}
	
	// AsyncArray(cv::AsyncArray &&) /usr/include/opencv2/core/async.hpp:73
	#[inline]
	pub fn copy_mut(o: &mut core::AsyncArray) -> Result<core::AsyncArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncArray_AsyncArray_AsyncArrayR(o.as_raw_mut_AsyncArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::AsyncArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for AsyncArray {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// AsyncPromise /usr/include/opencv2/core/detail/async_promise.hpp:22
pub trait AsyncPromiseTraitConst {
	fn as_raw_AsyncPromise(&self) -> *const c_void;

	// _getImpl() /usr/include/opencv2/core/detail/async_promise.hpp:63
	#[inline]
	fn _get_impl(&self) -> *mut c_void {
		let ret = unsafe { sys::cv_AsyncPromise__getImpl_const(self.as_raw_AsyncPromise()) };
		ret
	}
	
}

pub trait AsyncPromiseTrait: core::AsyncPromiseTraitConst {
	fn as_raw_mut_AsyncPromise(&mut self) -> *mut c_void;

	// release() /usr/include/opencv2/core/detail/async_promise.hpp:29
	#[inline]
	fn release(&mut self) {
		let ret = unsafe { sys::cv_AsyncPromise_release(self.as_raw_mut_AsyncPromise()) };
		ret
	}
	
	// getArrayResult() /usr/include/opencv2/core/detail/async_promise.hpp:34
	#[inline]
	fn get_array_result(&mut self) -> Result<core::AsyncArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncPromise_getArrayResult(self.as_raw_mut_AsyncPromise(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::AsyncArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// setValue(cv::InputArray) /usr/include/opencv2/core/detail/async_promise.hpp:39
	#[inline]
	fn set_value(&mut self, value: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(value);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncPromise_setValue_const__InputArrayR(self.as_raw_mut_AsyncPromise(), value.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setException(const cv::Exception &) /usr/include/opencv2/core/detail/async_promise.hpp:53
	#[inline]
	fn set_exception(&mut self, exception: &core::Exception) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncPromise_setException_const_ExceptionR(self.as_raw_mut_AsyncPromise(), exception.as_raw_Exception(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// AsyncPromise /usr/include/opencv2/core/detail/async_promise.hpp:22
pub struct AsyncPromise {
	ptr: *mut c_void
}

opencv_type_boxed! { AsyncPromise }

impl Drop for AsyncPromise {
	fn drop(&mut self) {
		extern "C" { fn cv_AsyncPromise_delete(instance: *mut c_void); }
		unsafe { cv_AsyncPromise_delete(self.as_raw_mut_AsyncPromise()) };
	}
}

unsafe impl Send for AsyncPromise {}

impl core::AsyncPromiseTraitConst for AsyncPromise {
	#[inline] fn as_raw_AsyncPromise(&self) -> *const c_void { self.as_raw() }
}

impl core::AsyncPromiseTrait for AsyncPromise {
	#[inline] fn as_raw_mut_AsyncPromise(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AsyncPromise {
	// AsyncPromise() /usr/include/opencv2/core/detail/async_promise.hpp:26
	#[inline]
	pub fn default() -> core::AsyncPromise {
		let ret = unsafe { sys::cv_AsyncPromise_AsyncPromise() };
		let ret = unsafe { core::AsyncPromise::opencv_from_extern(ret) };
		ret
	}
	
	// AsyncPromise(const cv::AsyncPromise &) /usr/include/opencv2/core/detail/async_promise.hpp:27
	#[inline]
	pub fn copy(o: &core::AsyncPromise) -> core::AsyncPromise {
		let ret = unsafe { sys::cv_AsyncPromise_AsyncPromise_const_AsyncPromiseR(o.as_raw_AsyncPromise()) };
		let ret = unsafe { core::AsyncPromise::opencv_from_extern(ret) };
		ret
	}
	
	// AsyncPromise(cv::AsyncPromise &&) /usr/include/opencv2/core/detail/async_promise.hpp:56
	#[inline]
	pub fn copy_mut(o: &mut core::AsyncPromise) -> Result<core::AsyncPromise> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AsyncPromise_AsyncPromise_AsyncPromiseR(o.as_raw_mut_AsyncPromise(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::AsyncPromise::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for AsyncPromise {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// CommandLineParser /usr/include/opencv2/core/utility.hpp:817
pub trait CommandLineParserTraitConst {
	fn as_raw_CommandLineParser(&self) -> *const c_void;

	// getPathToApplication() /usr/include/opencv2/core/utility.hpp:850
	#[inline]
	fn get_path_to_application(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CommandLineParser_getPathToApplication_const(self.as_raw_CommandLineParser(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// has(const cv::String &) /usr/include/opencv2/core/utility.hpp:927
	#[inline]
	fn has(&self, name: &str) -> Result<bool> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CommandLineParser_has_const_const_StringR(self.as_raw_CommandLineParser(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// check() /usr/include/opencv2/core/utility.hpp:934
	#[inline]
	fn check(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CommandLineParser_check_const(self.as_raw_CommandLineParser(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// printMessage() /usr/include/opencv2/core/utility.hpp:948
	#[inline]
	fn print_message(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CommandLineParser_printMessage_const(self.as_raw_CommandLineParser(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// printErrors() /usr/include/opencv2/core/utility.hpp:954
	#[inline]
	fn print_errors(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CommandLineParser_printErrors_const(self.as_raw_CommandLineParser(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait CommandLineParserTrait: core::CommandLineParserTraitConst {
	fn as_raw_mut_CommandLineParser(&mut self) -> *mut c_void;

	// about(const cv::String &) /usr/include/opencv2/core/utility.hpp:940
	#[inline]
	fn about(&mut self, message: &str) -> Result<()> {
		extern_container_arg!(message);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CommandLineParser_about_const_StringR(self.as_raw_mut_CommandLineParser(), message.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// CommandLineParser /usr/include/opencv2/core/utility.hpp:817
pub struct CommandLineParser {
	ptr: *mut c_void
}

opencv_type_boxed! { CommandLineParser }

impl Drop for CommandLineParser {
	fn drop(&mut self) {
		extern "C" { fn cv_CommandLineParser_delete(instance: *mut c_void); }
		unsafe { cv_CommandLineParser_delete(self.as_raw_mut_CommandLineParser()) };
	}
}

unsafe impl Send for CommandLineParser {}

impl core::CommandLineParserTraitConst for CommandLineParser {
	#[inline] fn as_raw_CommandLineParser(&self) -> *const c_void { self.as_raw() }
}

impl core::CommandLineParserTrait for CommandLineParser {
	#[inline] fn as_raw_mut_CommandLineParser(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CommandLineParser {
	// CommandLineParser(int, const char *const *, const cv::String &) /usr/include/opencv2/core/utility.hpp:829
	#[inline]
	pub fn new(argc: i32, argv: &[&str], keys: &str) -> Result<core::CommandLineParser> {
		string_array_arg!(argv);
		extern_container_arg!(keys);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CommandLineParser_CommandLineParser_int_const_charXX_const_StringR(argc, argv.as_ptr(), keys.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::CommandLineParser::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// CommandLineParser(const cv::CommandLineParser &) /usr/include/opencv2/core/utility.hpp:832
	#[inline]
	pub fn copy(parser: &core::CommandLineParser) -> Result<core::CommandLineParser> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CommandLineParser_CommandLineParser_const_CommandLineParserR(parser.as_raw_CommandLineParser(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::CommandLineParser::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// ConjGradSolver /usr/include/opencv2/core/optim.hpp:236
pub trait ConjGradSolverConst: core::MinProblemSolverConst {
	fn as_raw_ConjGradSolver(&self) -> *const c_void;

}

pub trait ConjGradSolver: core::ConjGradSolverConst + core::MinProblemSolver {
	fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void;

}

impl dyn ConjGradSolver + '_ {
	/// ## C++ default parameters
	/// * f: Ptr<ConjGradSolver::Function>()
	/// * termcrit: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5000,0.000001)
	// create(const Ptr<MinProblemSolver::Function> &, cv::TermCriteria) /usr/include/opencv2/core/optim.hpp:252
	#[inline]
	pub fn create(f: &core::Ptr<dyn core::MinProblemSolver_Function>, termcrit: core::TermCriteria) -> Result<core::Ptr<dyn core::ConjGradSolver>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ConjGradSolver_create_const_Ptr_Function_R_TermCriteria(f.as_raw_PtrOfMinProblemSolver_Function(), termcrit.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn core::ConjGradSolver>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// DMatch /usr/include/opencv2/core/types.hpp:830
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DMatch {
	pub query_idx: i32,
	pub train_idx: i32,
	pub img_idx: i32,
	pub distance: f32,
}

opencv_type_simple! { core::DMatch }

impl DMatch {
	// DMatch() /usr/include/opencv2/core/types.hpp:833
	#[inline]
	pub fn default() -> Result<core::DMatch> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DMatch_DMatch(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// DMatch(int, int, float) /usr/include/opencv2/core/types.hpp:834
	#[inline]
	pub fn new(_query_idx: i32, _train_idx: i32, _distance: f32) -> Result<core::DMatch> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DMatch_DMatch_int_int_float(_query_idx, _train_idx, _distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// DMatch(int, int, int, float) /usr/include/opencv2/core/types.hpp:835
	#[inline]
	pub fn new_index(_query_idx: i32, _train_idx: i32, _img_idx: i32, _distance: f32) -> Result<core::DMatch> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DMatch_DMatch_int_int_int_float(_query_idx, _train_idx, _img_idx, _distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// DownhillSolver /usr/include/opencv2/core/optim.hpp:155
pub trait DownhillSolverConst: core::MinProblemSolverConst {
	fn as_raw_DownhillSolver(&self) -> *const c_void;

	// getInitStep(cv::OutputArray) /usr/include/opencv2/core/optim.hpp:164
	#[inline]
	fn get_init_step(&self, step: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(step);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DownhillSolver_getInitStep_const_const__OutputArrayR(self.as_raw_DownhillSolver(), step.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DownhillSolver: core::DownhillSolverConst + core::MinProblemSolver {
	fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void;

	// setInitStep(cv::InputArray) /usr/include/opencv2/core/optim.hpp:180
	#[inline]
	fn set_init_step(&mut self, step: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(step);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DownhillSolver_setInitStep_const__InputArrayR(self.as_raw_mut_DownhillSolver(), step.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn DownhillSolver + '_ {
	/// ## C++ default parameters
	/// * f: Ptr<MinProblemSolver::Function>()
	/// * init_step: Mat_<double>(1,1,0.0)
	/// * termcrit: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5000,0.000001)
	// create(const Ptr<MinProblemSolver::Function> &, cv::InputArray, cv::TermCriteria) /usr/include/opencv2/core/optim.hpp:198
	#[inline]
	pub fn create(f: &core::Ptr<dyn core::MinProblemSolver_Function>, init_step: &dyn core::ToInputArray, termcrit: core::TermCriteria) -> Result<core::Ptr<dyn core::DownhillSolver>> {
		input_array_arg!(init_step);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DownhillSolver_create_const_Ptr_Function_R_const__InputArrayR_TermCriteria(f.as_raw_PtrOfMinProblemSolver_Function(), init_step.as_raw__InputArray(), termcrit.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn core::DownhillSolver>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// Exception /usr/include/opencv2/core.hpp:118
pub trait ExceptionTraitConst {
	fn as_raw_Exception(&self) -> *const c_void;

	// msg /usr/include/opencv2/core.hpp:138
	#[inline]
	fn msg(&self) -> String {
		let ret = unsafe { sys::cv_Exception_getPropMsg_const(self.as_raw_Exception()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// code /usr/include/opencv2/core.hpp:140
	#[inline]
	fn code(&self) -> i32 {
		let ret = unsafe { sys::cv_Exception_getPropCode_const(self.as_raw_Exception()) };
		ret
	}
	
	// err /usr/include/opencv2/core.hpp:141
	#[inline]
	fn err(&self) -> String {
		let ret = unsafe { sys::cv_Exception_getPropErr_const(self.as_raw_Exception()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// func /usr/include/opencv2/core.hpp:142
	#[inline]
	fn func(&self) -> String {
		let ret = unsafe { sys::cv_Exception_getPropFunc_const(self.as_raw_Exception()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// file /usr/include/opencv2/core.hpp:143
	#[inline]
	fn file(&self) -> String {
		let ret = unsafe { sys::cv_Exception_getPropFile_const(self.as_raw_Exception()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// line /usr/include/opencv2/core.hpp:144
	#[inline]
	fn line(&self) -> i32 {
		let ret = unsafe { sys::cv_Exception_getPropLine_const(self.as_raw_Exception()) };
		ret
	}
	
	// what() /usr/include/opencv2/core.hpp:135
	#[inline]
	fn what(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Exception_what_const(self.as_raw_Exception(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait ExceptionTrait: core::ExceptionTraitConst {
	fn as_raw_mut_Exception(&mut self) -> *mut c_void;

	// msg /usr/include/opencv2/core.hpp:138
	#[inline]
	fn set_msg(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_Exception_setPropMsg_String(self.as_raw_mut_Exception(), val.opencv_as_extern_mut()) };
		ret
	}
	
	// code /usr/include/opencv2/core.hpp:140
	#[inline]
	fn set_code(&mut self, val: i32) {
		let ret = unsafe { sys::cv_Exception_setPropCode_int(self.as_raw_mut_Exception(), val) };
		ret
	}
	
	// err /usr/include/opencv2/core.hpp:141
	#[inline]
	fn set_err(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_Exception_setPropErr_String(self.as_raw_mut_Exception(), val.opencv_as_extern_mut()) };
		ret
	}
	
	// func /usr/include/opencv2/core.hpp:142
	#[inline]
	fn set_func(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_Exception_setPropFunc_String(self.as_raw_mut_Exception(), val.opencv_as_extern_mut()) };
		ret
	}
	
	// file /usr/include/opencv2/core.hpp:143
	#[inline]
	fn set_file(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_Exception_setPropFile_String(self.as_raw_mut_Exception(), val.opencv_as_extern_mut()) };
		ret
	}
	
	// line /usr/include/opencv2/core.hpp:144
	#[inline]
	fn set_line(&mut self, val: i32) {
		let ret = unsafe { sys::cv_Exception_setPropLine_int(self.as_raw_mut_Exception(), val) };
		ret
	}
	
	// formatMessage() /usr/include/opencv2/core.hpp:136
	#[inline]
	fn format_message(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Exception_formatMessage(self.as_raw_mut_Exception(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Exception /usr/include/opencv2/core.hpp:118
pub struct Exception {
	ptr: *mut c_void
}

opencv_type_boxed! { Exception }

impl Drop for Exception {
	fn drop(&mut self) {
		extern "C" { fn cv_Exception_delete(instance: *mut c_void); }
		unsafe { cv_Exception_delete(self.as_raw_mut_Exception()) };
	}
}

unsafe impl Send for Exception {}

impl core::ExceptionTraitConst for Exception {
	#[inline] fn as_raw_Exception(&self) -> *const c_void { self.as_raw() }
}

impl core::ExceptionTrait for Exception {
	#[inline] fn as_raw_mut_Exception(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Exception {
	// Exception() /usr/include/opencv2/core.hpp:124
	#[inline]
	pub fn default() -> Result<core::Exception> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Exception_Exception(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Exception::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Exception(int, const cv::String &, const cv::String &, const cv::String &, int) /usr/include/opencv2/core.hpp:129
	#[inline]
	pub fn new(_code: i32, _err: &str, _func: &str, _file: &str, _line: i32) -> Result<core::Exception> {
		extern_container_arg!(_err);
		extern_container_arg!(_func);
		extern_container_arg!(_file);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Exception_Exception_int_const_StringR_const_StringR_const_StringR_int(_code, _err.opencv_as_extern(), _func.opencv_as_extern(), _file.opencv_as_extern(), _line, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Exception::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// FileNode /usr/include/opencv2/core/persistence.hpp:481
pub trait FileNodeTraitConst {
	fn as_raw_FileNode(&self) -> *const c_void;

	// blockIdx /usr/include/opencv2/core/persistence.hpp:623
	#[inline]
	fn block_idx(&self) -> size_t {
		let ret = unsafe { sys::cv_FileNode_getPropBlockIdx_const(self.as_raw_FileNode()) };
		ret
	}
	
	// ofs /usr/include/opencv2/core/persistence.hpp:624
	#[inline]
	fn ofs(&self) -> size_t {
		let ret = unsafe { sys::cv_FileNode_getPropOfs_const(self.as_raw_FileNode()) };
		ret
	}
	
	// operator[](const cv::String &) /usr/include/opencv2/core/persistence.hpp:530
	#[inline]
	fn get(&self, nodename: &str) -> Result<core::FileNode> {
		extern_container_arg!(nodename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_operator___const_const_StringR(self.as_raw_FileNode(), nodename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// operator[](const char *) /usr/include/opencv2/core/persistence.hpp:535
	#[inline]
	fn get_node(&self, nodename: &str) -> Result<core::FileNode> {
		extern_container_arg!(nodename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_operator___const_const_charX(self.as_raw_FileNode(), nodename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// operator[](int) /usr/include/opencv2/core/persistence.hpp:540
	#[inline]
	fn at(&self, i: i32) -> Result<core::FileNode> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_operator___const_int(self.as_raw_FileNode(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// keys() /usr/include/opencv2/core/persistence.hpp:545
	#[inline]
	fn keys(&self) -> Result<core::Vector<String>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_keys_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// type() /usr/include/opencv2/core/persistence.hpp:550
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_type_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/persistence.hpp:553
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_empty_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isNone() /usr/include/opencv2/core/persistence.hpp:555
	#[inline]
	fn is_none(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isNone_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isSeq() /usr/include/opencv2/core/persistence.hpp:557
	#[inline]
	fn is_seq(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isSeq_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isMap() /usr/include/opencv2/core/persistence.hpp:559
	#[inline]
	fn is_map(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isMap_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isInt() /usr/include/opencv2/core/persistence.hpp:561
	#[inline]
	fn is_int(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isInt_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isReal() /usr/include/opencv2/core/persistence.hpp:563
	#[inline]
	fn is_real(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isReal_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isString() /usr/include/opencv2/core/persistence.hpp:565
	#[inline]
	fn is_string(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isString_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isNamed() /usr/include/opencv2/core/persistence.hpp:567
	#[inline]
	fn is_named(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isNamed_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// name() /usr/include/opencv2/core/persistence.hpp:569
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_name_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// size() /usr/include/opencv2/core/persistence.hpp:571
	#[inline]
	fn size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_size_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// rawSize() /usr/include/opencv2/core/persistence.hpp:573
	#[inline]
	fn raw_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_rawSize_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator int() /usr/include/opencv2/core/persistence.hpp:575
	#[inline]
	fn to_i32(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_operator_int_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator float() /usr/include/opencv2/core/persistence.hpp:577
	#[inline]
	fn to_f32(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_operator_float_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator double() /usr/include/opencv2/core/persistence.hpp:579
	#[inline]
	fn to_f64(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_operator_double_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator basic_string() /usr/include/opencv2/core/persistence.hpp:581
	#[inline]
	fn to_string(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_operator_std_string_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ptr() /usr/include/opencv2/core/persistence.hpp:590
	#[inline]
	fn ptr(&self) -> Result<*const u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_ptr_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// begin() /usr/include/opencv2/core/persistence.hpp:593
	#[inline]
	fn begin(&self) -> Result<core::FileNodeIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_begin_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNodeIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// end() /usr/include/opencv2/core/persistence.hpp:595
	#[inline]
	fn end(&self) -> Result<core::FileNodeIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_end_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNodeIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// readRaw(const cv::String &, void *, size_t) /usr/include/opencv2/core/persistence.hpp:605
	#[inline]
	unsafe fn read_raw(&self, fmt: &str, vec: *mut c_void, len: size_t) -> Result<()> {
		extern_container_arg!(fmt);
		return_send!(via ocvrs_return);
		{ sys::cv_FileNode_readRaw_const_const_StringR_voidX_size_t(self.as_raw_FileNode(), fmt.opencv_as_extern(), vec, len, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// real() /usr/include/opencv2/core/persistence.hpp:613
	#[inline]
	fn real(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_real_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// string() /usr/include/opencv2/core/persistence.hpp:615
	#[inline]
	fn string(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_string_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// mat() /usr/include/opencv2/core/persistence.hpp:617
	#[inline]
	fn mat(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_mat_const(self.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait FileNodeTrait: core::FileNodeTraitConst {
	fn as_raw_mut_FileNode(&mut self) -> *mut c_void;

	// blockIdx /usr/include/opencv2/core/persistence.hpp:623
	#[inline]
	fn set_block_idx(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_FileNode_setPropBlockIdx_size_t(self.as_raw_mut_FileNode(), val) };
		ret
	}
	
	// ofs /usr/include/opencv2/core/persistence.hpp:624
	#[inline]
	fn set_ofs(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_FileNode_setPropOfs_size_t(self.as_raw_mut_FileNode(), val) };
		ret
	}
	
	// ptr() /usr/include/opencv2/core/persistence.hpp:589
	#[inline]
	fn ptr_1(&mut self) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_ptr(self.as_raw_mut_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * len: -1
	// setValue(int, const void *, int) /usr/include/opencv2/core/persistence.hpp:610
	#[inline]
	unsafe fn set_value(&mut self, typ: i32, value: *const c_void, len: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_FileNode_setValue_int_const_voidX_int(self.as_raw_mut_FileNode(), typ, value, len, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// FileNode /usr/include/opencv2/core/persistence.hpp:481
pub struct FileNode {
	ptr: *mut c_void
}

opencv_type_boxed! { FileNode }

impl Drop for FileNode {
	fn drop(&mut self) {
		extern "C" { fn cv_FileNode_delete(instance: *mut c_void); }
		unsafe { cv_FileNode_delete(self.as_raw_mut_FileNode()) };
	}
}

unsafe impl Send for FileNode {}

impl core::FileNodeTraitConst for FileNode {
	#[inline] fn as_raw_FileNode(&self) -> *const c_void { self.as_raw() }
}

impl core::FileNodeTrait for FileNode {
	#[inline] fn as_raw_mut_FileNode(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FileNode {
	// FileNode() /usr/include/opencv2/core/persistence.hpp:508
	#[inline]
	pub fn default() -> Result<core::FileNode> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_FileNode(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// FileNode(const cv::FileStorage *, size_t, size_t) /usr/include/opencv2/core/persistence.hpp:517
	#[inline]
	pub fn new(fs: &core::FileStorage, block_idx: size_t, ofs: size_t) -> Result<core::FileNode> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_FileNode_const_FileStorageX_size_t_size_t(fs.as_raw_FileStorage(), block_idx, ofs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// FileNode(const cv::FileNode &) /usr/include/opencv2/core/persistence.hpp:522
	#[inline]
	pub fn copy(node: &core::FileNode) -> Result<core::FileNode> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_FileNode_const_FileNodeR(node.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// isMap(int) /usr/include/opencv2/core/persistence.hpp:583
	#[inline]
	pub fn is_map(flags: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isMap_int(flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isSeq(int) /usr/include/opencv2/core/persistence.hpp:584
	#[inline]
	pub fn is_seq(flags: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isSeq_int(flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isCollection(int) /usr/include/opencv2/core/persistence.hpp:585
	#[inline]
	pub fn is_collection(flags: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isCollection_int(flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isEmptyCollection(int) /usr/include/opencv2/core/persistence.hpp:586
	#[inline]
	pub fn is_empty_collection(flags: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isEmptyCollection_int(flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isFlow(int) /usr/include/opencv2/core/persistence.hpp:587
	#[inline]
	pub fn is_flow(flags: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNode_isFlow_int(flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// FileNodeIterator /usr/include/opencv2/core/persistence.hpp:633
pub trait FileNodeIteratorTraitConst {
	fn as_raw_FileNodeIterator(&self) -> *const c_void;

	// operator*() /usr/include/opencv2/core/persistence.hpp:661
	#[inline]
	fn try_deref(&self) -> Result<core::FileNode> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNodeIterator_operatorX_const(self.as_raw_FileNodeIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// remaining() /usr/include/opencv2/core/persistence.hpp:682
	#[inline]
	fn remaining(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNodeIterator_remaining_const(self.as_raw_FileNodeIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// equalTo(const cv::FileNodeIterator &) /usr/include/opencv2/core/persistence.hpp:684
	#[inline]
	fn equal_to(&self, it: &core::FileNodeIterator) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNodeIterator_equalTo_const_const_FileNodeIteratorR(self.as_raw_FileNodeIterator(), it.as_raw_FileNodeIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FileNodeIteratorTrait: core::FileNodeIteratorTraitConst {
	fn as_raw_mut_FileNodeIterator(&mut self) -> *mut c_void;

	// operator++() /usr/include/opencv2/core/persistence.hpp:664
	#[inline]
	fn incr(&mut self) -> Result<core::FileNodeIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNodeIterator_operatorAA(self.as_raw_mut_FileNodeIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNodeIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * len: (size_t)INT_MAX
	// readRaw(const cv::String &, void *, size_t) /usr/include/opencv2/core/persistence.hpp:678
	#[inline]
	unsafe fn read_raw(&mut self, fmt: &str, vec: *mut c_void, len: size_t) -> Result<core::FileNodeIterator> {
		extern_container_arg!(fmt);
		return_send!(via ocvrs_return);
		{ sys::cv_FileNodeIterator_readRaw_const_StringR_voidX_size_t(self.as_raw_mut_FileNodeIterator(), fmt.opencv_as_extern(), vec, len, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::FileNodeIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// FileNodeIterator /usr/include/opencv2/core/persistence.hpp:633
pub struct FileNodeIterator {
	ptr: *mut c_void
}

opencv_type_boxed! { FileNodeIterator }

impl Drop for FileNodeIterator {
	fn drop(&mut self) {
		extern "C" { fn cv_FileNodeIterator_delete(instance: *mut c_void); }
		unsafe { cv_FileNodeIterator_delete(self.as_raw_mut_FileNodeIterator()) };
	}
}

unsafe impl Send for FileNodeIterator {}

impl core::FileNodeIteratorTraitConst for FileNodeIterator {
	#[inline] fn as_raw_FileNodeIterator(&self) -> *const c_void { self.as_raw() }
}

impl core::FileNodeIteratorTrait for FileNodeIterator {
	#[inline] fn as_raw_mut_FileNodeIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FileNodeIterator {
	// FileNodeIterator() /usr/include/opencv2/core/persistence.hpp:641
	#[inline]
	pub fn default() -> Result<core::FileNodeIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNodeIterator_FileNodeIterator(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNodeIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// FileNodeIterator(const cv::FileNode &, bool) /usr/include/opencv2/core/persistence.hpp:651
	#[inline]
	pub fn new(node: &core::FileNode, seek_end: bool) -> Result<core::FileNodeIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNodeIterator_FileNodeIterator_const_FileNodeR_bool(node.as_raw_FileNode(), seek_end, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNodeIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// FileNodeIterator(const cv::FileNodeIterator &) /usr/include/opencv2/core/persistence.hpp:656
	#[inline]
	pub fn copy(it: &core::FileNodeIterator) -> Result<core::FileNodeIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileNodeIterator_FileNodeIterator_const_FileNodeIteratorR(it.as_raw_FileNodeIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNodeIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// FileStorage /usr/include/opencv2/core/persistence.hpp:303
pub trait FileStorageTraitConst {
	fn as_raw_FileStorage(&self) -> *const c_void;

	// state /usr/include/opencv2/core/persistence.hpp:462
	#[inline]
	fn state(&self) -> i32 {
		let ret = unsafe { sys::cv_FileStorage_getPropState_const(self.as_raw_FileStorage()) };
		ret
	}
	
	// elname /usr/include/opencv2/core/persistence.hpp:463
	#[inline]
	fn elname(&self) -> String {
		let ret = unsafe { sys::cv_FileStorage_getPropElname_const(self.as_raw_FileStorage()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// isOpened() /usr/include/opencv2/core/persistence.hpp:368
	#[inline]
	fn is_opened(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_isOpened_const(self.as_raw_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFirstTopLevelNode() /usr/include/opencv2/core/persistence.hpp:386
	#[inline]
	fn get_first_top_level_node(&self) -> Result<core::FileNode> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_getFirstTopLevelNode_const(self.as_raw_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * streamidx: 0
	// root(int) /usr/include/opencv2/core/persistence.hpp:393
	#[inline]
	fn root(&self, streamidx: i32) -> Result<core::FileNode> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_root_const_int(self.as_raw_FileStorage(), streamidx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// operator[](const cv::String &) /usr/include/opencv2/core/persistence.hpp:399
	#[inline]
	fn get(&self, nodename: &str) -> Result<core::FileNode> {
		extern_container_arg!(nodename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_operator___const_const_StringR(self.as_raw_FileStorage(), nodename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// operator[](const char *) /usr/include/opencv2/core/persistence.hpp:402
	#[inline]
	fn get_node(&self, nodename: &str) -> Result<core::FileNode> {
		extern_container_arg!(nodename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_operator___const_const_charX(self.as_raw_FileStorage(), nodename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileNode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getFormat() /usr/include/opencv2/core/persistence.hpp:460
	#[inline]
	fn get_format(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_getFormat_const(self.as_raw_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FileStorageTrait: core::FileStorageTraitConst {
	fn as_raw_mut_FileStorage(&mut self) -> *mut c_void;

	// state /usr/include/opencv2/core/persistence.hpp:462
	#[inline]
	fn set_state(&mut self, val: i32) {
		let ret = unsafe { sys::cv_FileStorage_setPropState_int(self.as_raw_mut_FileStorage(), val) };
		ret
	}
	
	// elname /usr/include/opencv2/core/persistence.hpp:463
	#[inline]
	fn set_elname(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_FileStorage_setPropElname_string(self.as_raw_mut_FileStorage(), val.opencv_as_extern_mut()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * encoding: String()
	// open(const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:361
	#[inline]
	fn open(&mut self, filename: &str, flags: i32, encoding: &str) -> Result<bool> {
		extern_container_arg!(filename);
		extern_container_arg!(encoding);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_open_const_StringR_int_const_StringR(self.as_raw_mut_FileStorage(), filename.opencv_as_extern(), flags, encoding.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/persistence.hpp:374
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_release(self.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// releaseAndGetString() /usr/include/opencv2/core/persistence.hpp:381
	#[inline]
	fn release_and_get_string(&mut self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_releaseAndGetString(self.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// write(const cv::String &, int) /usr/include/opencv2/core/persistence.hpp:409
	#[inline]
	fn write_i32(&mut self, name: &str, val: i32) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_write_const_StringR_int(self.as_raw_mut_FileStorage(), name.opencv_as_extern(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// write(const cv::String &, double) /usr/include/opencv2/core/persistence.hpp:411
	#[inline]
	fn write_f64(&mut self, name: &str, val: f64) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_write_const_StringR_double(self.as_raw_mut_FileStorage(), name.opencv_as_extern(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// write(const cv::String &, const cv::String &) /usr/include/opencv2/core/persistence.hpp:413
	#[inline]
	fn write_str(&mut self, name: &str, val: &str) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_write_const_StringR_const_StringR(self.as_raw_mut_FileStorage(), name.opencv_as_extern(), val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// write(const cv::String &, const cv::Mat &) /usr/include/opencv2/core/persistence.hpp:415
	#[inline]
	fn write_mat(&mut self, name: &str, val: &core::Mat) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_write_const_StringR_const_MatR(self.as_raw_mut_FileStorage(), name.opencv_as_extern(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// write(const cv::String &, const std::vector<String> &) /usr/include/opencv2/core/persistence.hpp:417
	#[inline]
	fn write_str_vec(&mut self, name: &str, val: &core::Vector<String>) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_write_const_StringR_const_vector_String_R(self.as_raw_mut_FileStorage(), name.opencv_as_extern(), val.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// writeRaw(const cv::String &, const void *, size_t) /usr/include/opencv2/core/persistence.hpp:427
	#[inline]
	unsafe fn write_raw(&mut self, fmt: &str, vec: *const c_void, len: size_t) -> Result<()> {
		extern_container_arg!(fmt);
		return_send!(via ocvrs_return);
		{ sys::cv_FileStorage_writeRaw_const_StringR_const_voidX_size_t(self.as_raw_mut_FileStorage(), fmt.opencv_as_extern(), vec, len, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * append: false
	// writeComment(const cv::String &, bool) /usr/include/opencv2/core/persistence.hpp:437
	#[inline]
	fn write_comment(&mut self, comment: &str, append: bool) -> Result<()> {
		extern_container_arg!(comment);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_writeComment_const_StringR_bool(self.as_raw_mut_FileStorage(), comment.opencv_as_extern(), append, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * type_name: String()
	// startWriteStruct(const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:445
	#[inline]
	fn start_write_struct(&mut self, name: &str, flags: i32, type_name: &str) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(type_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_startWriteStruct_const_StringR_int_const_StringR(self.as_raw_mut_FileStorage(), name.opencv_as_extern(), flags, type_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// endWriteStruct() /usr/include/opencv2/core/persistence.hpp:449
	#[inline]
	fn end_write_struct(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_endWriteStruct(self.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// FileStorage /usr/include/opencv2/core/persistence.hpp:303
pub struct FileStorage {
	ptr: *mut c_void
}

opencv_type_boxed! { FileStorage }

impl Drop for FileStorage {
	fn drop(&mut self) {
		extern "C" { fn cv_FileStorage_delete(instance: *mut c_void); }
		unsafe { cv_FileStorage_delete(self.as_raw_mut_FileStorage()) };
	}
}

unsafe impl Send for FileStorage {}

impl core::FileStorageTraitConst for FileStorage {
	#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.as_raw() }
}

impl core::FileStorageTrait for FileStorage {
	#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FileStorage {
	// FileStorage() /usr/include/opencv2/core/persistence.hpp:336
	#[inline]
	pub fn default() -> Result<core::FileStorage> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_FileStorage(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileStorage::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * encoding: String()
	// FileStorage(const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:341
	#[inline]
	pub fn new(filename: &str, flags: i32, encoding: &str) -> Result<core::FileStorage> {
		extern_container_arg!(filename);
		extern_container_arg!(encoding);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_FileStorage_const_StringR_int_const_StringR(filename.opencv_as_extern(), flags, encoding.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::FileStorage::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getDefaultObjectName(const cv::String &) /usr/include/opencv2/core/persistence.hpp:455
	#[inline]
	pub fn get_default_object_name(filename: &str) -> Result<String> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FileStorage_getDefaultObjectName_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Formatted /usr/include/opencv2/core.hpp:3066
pub trait FormattedConst {
	fn as_raw_Formatted(&self) -> *const c_void;

}

pub trait Formatted: core::FormattedConst {
	fn as_raw_mut_Formatted(&mut self) -> *mut c_void;

	// next() /usr/include/opencv2/core.hpp:3069
	#[inline]
	fn next(&mut self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Formatted_next(self.as_raw_mut_Formatted(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// reset() /usr/include/opencv2/core.hpp:3070
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Formatted_reset(self.as_raw_mut_Formatted(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Formatter /usr/include/opencv2/core.hpp:3075
pub trait FormatterConst {
	fn as_raw_Formatter(&self) -> *const c_void;

	// format(const cv::Mat &) /usr/include/opencv2/core.hpp:3089
	#[inline]
	fn format(&self, mtx: &core::Mat) -> Result<core::Ptr<dyn core::Formatted>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Formatter_format_const_const_MatR(self.as_raw_Formatter(), mtx.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn core::Formatted>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Formatter: core::FormatterConst {
	fn as_raw_mut_Formatter(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * p: 4
	// set16fPrecision(int) /usr/include/opencv2/core.hpp:3091
	#[inline]
	fn set16f_precision(&mut self, p: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Formatter_set16fPrecision_int(self.as_raw_mut_Formatter(), p, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * p: 8
	// set32fPrecision(int) /usr/include/opencv2/core.hpp:3092
	#[inline]
	fn set32f_precision(&mut self, p: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Formatter_set32fPrecision_int(self.as_raw_mut_Formatter(), p, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * p: 16
	// set64fPrecision(int) /usr/include/opencv2/core.hpp:3093
	#[inline]
	fn set64f_precision(&mut self, p: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Formatter_set64fPrecision_int(self.as_raw_mut_Formatter(), p, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * ml: true
	// setMultiline(bool) /usr/include/opencv2/core.hpp:3094
	#[inline]
	fn set_multiline(&mut self, ml: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Formatter_setMultiline_bool(self.as_raw_mut_Formatter(), ml, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn Formatter + '_ {
	/// ## C++ default parameters
	/// * fmt: FMT_DEFAULT
	// get(Formatter::FormatType) /usr/include/opencv2/core.hpp:3096
	#[inline]
	pub fn get(fmt: core::Formatter_FormatType) -> Result<core::Ptr<dyn core::Formatter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Formatter_get_FormatType(fmt, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn core::Formatter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// Hamming /usr/include/opencv2/core/base.hpp:382
pub trait HammingTraitConst {
	fn as_raw_Hamming(&self) -> *const c_void;

}

pub trait HammingTrait: core::HammingTraitConst {
	fn as_raw_mut_Hamming(&mut self) -> *mut c_void;

}

// Hamming /usr/include/opencv2/core/base.hpp:382
pub struct Hamming {
	ptr: *mut c_void
}

opencv_type_boxed! { Hamming }

impl Drop for Hamming {
	fn drop(&mut self) {
		extern "C" { fn cv_Hamming_delete(instance: *mut c_void); }
		unsafe { cv_Hamming_delete(self.as_raw_mut_Hamming()) };
	}
}

unsafe impl Send for Hamming {}

impl core::HammingTraitConst for Hamming {
	#[inline] fn as_raw_Hamming(&self) -> *const c_void { self.as_raw() }
}

impl core::HammingTrait for Hamming {
	#[inline] fn as_raw_mut_Hamming(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Hamming {
	// normType /usr/include/opencv2/core/base.hpp:384
	pub const normType: u32 = 6;
}

// KeyPoint /usr/include/opencv2/core/types.hpp:733
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyPoint {
	pub pt: core::Point2f,
	pub size: f32,
	pub angle: f32,
	pub response: f32,
	pub octave: i32,
	pub class_id: i32,
}

opencv_type_simple! { core::KeyPoint }

impl KeyPoint {
	// hash() /usr/include/opencv2/core/types.hpp:758
	#[inline]
	pub fn hash(self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPoint_hash_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// KeyPoint() /usr/include/opencv2/core/types.hpp:737
	#[inline]
	pub fn default() -> Result<core::KeyPoint> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPoint_KeyPoint(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * angle: -1
	/// * response: 0
	/// * octave: 0
	/// * class_id: -1
	// KeyPoint(cv::Point2f, float, float, float, int, int) /usr/include/opencv2/core/types.hpp:746
	#[inline]
	pub fn new_point(pt: core::Point2f, size: f32, angle: f32, response: f32, octave: i32, class_id: i32) -> Result<core::KeyPoint> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int(pt.opencv_as_extern(), size, angle, response, octave, class_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * angle: -1
	/// * response: 0
	/// * octave: 0
	/// * class_id: -1
	// KeyPoint(float, float, float, float, float, int, int) /usr/include/opencv2/core/types.hpp:756
	#[inline]
	pub fn new_coords(x: f32, y: f32, size: f32, angle: f32, response: f32, octave: i32, class_id: i32) -> Result<core::KeyPoint> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int(x, y, size, angle, response, octave, class_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * keypoint_indexes: std::vector<int>()
	// convert(const std::vector<KeyPoint> &, std::vector<Point2f> &, const std::vector<int> &) /usr/include/opencv2/core/types.hpp:769
	#[inline]
	pub fn convert(keypoints: &core::Vector<core::KeyPoint>, points2f: &mut core::Vector<core::Point2f>, keypoint_indexes: &core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPoint_convert_const_vector_KeyPoint_R_vector_Point2f_R_const_vector_int_R(keypoints.as_raw_VectorOfKeyPoint(), points2f.as_raw_mut_VectorOfPoint2f(), keypoint_indexes.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * size: 1
	/// * response: 1
	/// * octave: 0
	/// * class_id: -1
	// convert(const std::vector<Point2f> &, std::vector<KeyPoint> &, float, float, int, int) /usr/include/opencv2/core/types.hpp:780
	#[inline]
	pub fn convert_to(points2f: &core::Vector<core::Point2f>, keypoints: &mut core::Vector<core::KeyPoint>, size: f32, response: f32, octave: i32, class_id: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPoint_convert_const_vector_Point2f_R_vector_KeyPoint_R_float_float_int_int(points2f.as_raw_VectorOfPoint2f(), keypoints.as_raw_mut_VectorOfKeyPoint(), size, response, octave, class_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// overlap(const cv::KeyPoint &, const cv::KeyPoint &) /usr/include/opencv2/core/types.hpp:791
	#[inline]
	pub fn overlap(kp1: core::KeyPoint, kp2: core::KeyPoint) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPoint_overlap_const_KeyPointR_const_KeyPointR(&kp1, &kp2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// LDA /usr/include/opencv2/core.hpp:2605
pub trait LDATraitConst {
	fn as_raw_LDA(&self) -> *const c_void;

	// save(const cv::String &) /usr/include/opencv2/core.hpp:2622
	#[inline]
	fn save(&self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_save_const_const_StringR(self.as_raw_LDA(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// save(cv::FileStorage &) /usr/include/opencv2/core.hpp:2630
	#[inline]
	fn save_1(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_save_const_FileStorageR(self.as_raw_LDA(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// eigenvectors() /usr/include/opencv2/core.hpp:2656
	#[inline]
	fn eigenvectors(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_eigenvectors_const(self.as_raw_LDA(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// eigenvalues() /usr/include/opencv2/core.hpp:2660
	#[inline]
	fn eigenvalues(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_eigenvalues_const(self.as_raw_LDA(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait LDATrait: core::LDATraitConst {
	fn as_raw_mut_LDA(&mut self) -> *mut c_void;

	// load(const cv::String &) /usr/include/opencv2/core.hpp:2626
	#[inline]
	fn load(&mut self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_load_const_StringR(self.as_raw_mut_LDA(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// load(const cv::FileStorage &) /usr/include/opencv2/core.hpp:2634
	#[inline]
	fn load_1(&mut self, node: &core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_load_const_FileStorageR(self.as_raw_mut_LDA(), node.as_raw_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// compute(cv::InputArrayOfArrays, cv::InputArray) /usr/include/opencv2/core.hpp:2642
	#[inline]
	fn compute(&mut self, src: &dyn core::ToInputArray, labels: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(labels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_compute_const__InputArrayR_const__InputArrayR(self.as_raw_mut_LDA(), src.as_raw__InputArray(), labels.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// project(cv::InputArray) /usr/include/opencv2/core.hpp:2647
	#[inline]
	fn project(&mut self, src: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_project_const__InputArrayR(self.as_raw_mut_LDA(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// reconstruct(cv::InputArray) /usr/include/opencv2/core.hpp:2652
	#[inline]
	fn reconstruct(&mut self, src: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_reconstruct_const__InputArrayR(self.as_raw_mut_LDA(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// LDA /usr/include/opencv2/core.hpp:2605
pub struct LDA {
	ptr: *mut c_void
}

opencv_type_boxed! { LDA }

impl Drop for LDA {
	fn drop(&mut self) {
		extern "C" { fn cv_LDA_delete(instance: *mut c_void); }
		unsafe { cv_LDA_delete(self.as_raw_mut_LDA()) };
	}
}

unsafe impl Send for LDA {}

impl core::LDATraitConst for LDA {
	#[inline] fn as_raw_LDA(&self) -> *const c_void { self.as_raw() }
}

impl core::LDATrait for LDA {
	#[inline] fn as_raw_mut_LDA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LDA {
	/// ## C++ default parameters
	/// * num_components: 0
	// LDA(int) /usr/include/opencv2/core.hpp:2611
	#[inline]
	pub fn new(num_components: i32) -> Result<core::LDA> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_LDA_int(num_components, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::LDA::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * num_components: 0
	// LDA(cv::InputArrayOfArrays, cv::InputArray, int) /usr/include/opencv2/core.hpp:2618
	#[inline]
	pub fn new_with_data(src: &dyn core::ToInputArray, labels: &dyn core::ToInputArray, num_components: i32) -> Result<core::LDA> {
		input_array_arg!(src);
		input_array_arg!(labels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_LDA_const__InputArrayR_const__InputArrayR_int(src.as_raw__InputArray(), labels.as_raw__InputArray(), num_components, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::LDA::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// subspaceProject(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2662
	#[inline]
	pub fn subspace_project(w: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, src: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(w);
		input_array_arg!(mean);
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_subspaceProject_const__InputArrayR_const__InputArrayR_const__InputArrayR(w.as_raw__InputArray(), mean.as_raw__InputArray(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// subspaceReconstruct(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2663
	#[inline]
	pub fn subspace_reconstruct(w: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, src: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(w);
		input_array_arg!(mean);
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LDA_subspaceReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR(w.as_raw__InputArray(), mean.as_raw__InputArray(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Mat /usr/include/opencv2/core/mat.hpp:810
pub trait MatTraitConst {
	fn as_raw_Mat(&self) -> *const c_void;

	// flags /usr/include/opencv2/core/mat.hpp:2112
	#[inline]
	fn flags(&self) -> i32 {
		let ret = unsafe { sys::cv_Mat_getPropFlags_const(self.as_raw_Mat()) };
		ret
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2114
	#[inline]
	fn dims(&self) -> i32 {
		let ret = unsafe { sys::cv_Mat_getPropDims_const(self.as_raw_Mat()) };
		ret
	}
	
	// rows /usr/include/opencv2/core/mat.hpp:2116
	#[inline]
	fn rows(&self) -> i32 {
		let ret = unsafe { sys::cv_Mat_getPropRows_const(self.as_raw_Mat()) };
		ret
	}
	
	// cols /usr/include/opencv2/core/mat.hpp:2116
	#[inline]
	fn cols(&self) -> i32 {
		let ret = unsafe { sys::cv_Mat_getPropCols_const(self.as_raw_Mat()) };
		ret
	}
	
	// datastart /usr/include/opencv2/core/mat.hpp:2121
	#[inline]
	fn datastart(&self) -> *const u8 {
		let ret = unsafe { sys::cv_Mat_getPropDatastart_const(self.as_raw_Mat()) };
		ret
	}
	
	// dataend /usr/include/opencv2/core/mat.hpp:2122
	#[inline]
	fn dataend(&self) -> *const u8 {
		let ret = unsafe { sys::cv_Mat_getPropDataend_const(self.as_raw_Mat()) };
		ret
	}
	
	// datalimit /usr/include/opencv2/core/mat.hpp:2123
	#[inline]
	fn datalimit(&self) -> *const u8 {
		let ret = unsafe { sys::cv_Mat_getPropDatalimit_const(self.as_raw_Mat()) };
		ret
	}
	
	// size /usr/include/opencv2/core/mat.hpp:2138
	#[inline]
	fn mat_size(&self) -> core::MatSize {
		let ret = unsafe { sys::cv_Mat_getPropSize_const(self.as_raw_Mat()) };
		let ret = unsafe { core::MatSize::opencv_from_extern(ret) };
		ret
	}
	
	// step /usr/include/opencv2/core/mat.hpp:2139
	#[inline]
	fn mat_step(&self) -> core::MatStep {
		let ret = unsafe { sys::cv_Mat_getPropStep_const(self.as_raw_Mat()) };
		let ret = unsafe { core::MatStep::opencv_from_extern(ret) };
		ret
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// getUMat(cv::AccessFlag, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:1085
	#[inline]
	fn get_umat(&self, access_flags: core::AccessFlag, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_getUMat_const_AccessFlag_UMatUsageFlags(self.as_raw_Mat(), access_flags, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// row(int) /usr/include/opencv2/core/mat.hpp:1120
	#[inline]
	fn row(&self, y: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_row_const_int(self.as_raw_Mat(), y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// col(int) /usr/include/opencv2/core/mat.hpp:1129
	#[inline]
	fn col(&self, x: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_col_const_int(self.as_raw_Mat(), x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// rowRange(int, int) /usr/include/opencv2/core/mat.hpp:1138
	#[inline]
	fn row_bounds(&self, startrow: i32, endrow: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_rowRange_const_int_int(self.as_raw_Mat(), startrow, endrow, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// rowRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:1143
	#[inline]
	fn row_range(&self, r: &core::Range) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_rowRange_const_const_RangeR(self.as_raw_Mat(), r.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// colRange(int, int) /usr/include/opencv2/core/mat.hpp:1152
	#[inline]
	fn col_bounds(&self, startcol: i32, endcol: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_colRange_const_int_int(self.as_raw_Mat(), startcol, endcol, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// colRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:1157
	#[inline]
	fn col_range(&self, r: &core::Range) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_colRange_const_const_RangeR(self.as_raw_Mat(), r.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * d: 0
	// diag(int) /usr/include/opencv2/core/mat.hpp:1193
	#[inline]
	fn diag(&self, d: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_diag_const_int(self.as_raw_Mat(), d, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// clone() /usr/include/opencv2/core/mat.hpp:1207
	#[inline]
	#[must_use]
	fn try_clone(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_clone_const(self.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/mat.hpp:1224
	#[inline]
	fn copy_to(&self, m: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_copyTo_const_const__OutputArrayR(self.as_raw_Mat(), m.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyTo(cv::OutputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:1232
	#[inline]
	fn copy_to_masked(&self, m: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		output_array_arg!(m);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_copyTo_const_const__OutputArrayR_const__InputArrayR(self.as_raw_Mat(), m.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * alpha: 1
	/// * beta: 0
	// convertTo(cv::OutputArray, int, double, double) /usr/include/opencv2/core/mat.hpp:1247
	#[inline]
	fn convert_to(&self, m: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
		output_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_convertTo_const_const__OutputArrayR_int_double_double(self.as_raw_Mat(), m.as_raw__OutputArray(), rtype, alpha, beta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * typ: -1
	// assignTo(cv::Mat &, int) /usr/include/opencv2/core/mat.hpp:1255
	#[inline]
	fn assign_to(&self, m: &mut core::Mat, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_assignTo_const_MatR_int(self.as_raw_Mat(), m.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * rows: 0
	// reshape(int, int) /usr/include/opencv2/core/mat.hpp:1295
	#[inline]
	fn reshape(&self, cn: i32, rows: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_reshape_const_int_int(self.as_raw_Mat(), cn, rows, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// reshape(int, int, const int *) /usr/include/opencv2/core/mat.hpp:1298
	#[inline]
	fn reshape_nd(&self, cn: i32, newsz: &[i32]) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_reshape_const_int_int_const_intX(self.as_raw_Mat(), cn, newsz.len() as _, newsz.as_ptr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// reshape(int, const std::vector<int> &) /usr/include/opencv2/core/mat.hpp:1301
	#[inline]
	fn reshape_nd_vec(&self, cn: i32, newshape: &core::Vector<i32>) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_reshape_const_int_const_vector_int_R(self.as_raw_Mat(), cn, newshape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// t() /usr/include/opencv2/core/mat.hpp:1313
	#[inline]
	fn t(&self) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_t_const(self.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * method: DECOMP_LU
	// inv(int) /usr/include/opencv2/core/mat.hpp:1322
	#[inline]
	fn inv(&self, method: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_inv_const_int(self.as_raw_Mat(), method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	// mul(cv::InputArray, double) /usr/include/opencv2/core/mat.hpp:1336
	#[inline]
	fn mul(&self, m: &dyn core::ToInputArray, scale: f64) -> Result<core::MatExpr> {
		input_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_mul_const_const__InputArrayR_double(self.as_raw_Mat(), m.as_raw__InputArray(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// cross(cv::InputArray) /usr/include/opencv2/core/mat.hpp:1345
	#[inline]
	fn cross(&self, m: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_cross_const_const__InputArrayR(self.as_raw_Mat(), m.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// dot(cv::InputArray) /usr/include/opencv2/core/mat.hpp:1355
	#[inline]
	fn dot(&self, m: &dyn core::ToInputArray) -> Result<f64> {
		input_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_dot_const_const__InputArrayR(self.as_raw_Mat(), m.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// locateROI(cv::Size &, cv::Point &) /usr/include/opencv2/core/mat.hpp:1602
	#[inline]
	fn locate_roi(&self, whole_size: &mut core::Size, ofs: &mut core::Point) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_locateROI_const_SizeR_PointR(self.as_raw_Mat(), whole_size, ofs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isContinuous() /usr/include/opencv2/core/mat.hpp:1741
	#[inline]
	fn is_continuous(&self) -> bool {
		let ret = unsafe { sys::cv_Mat_isContinuous_const(self.as_raw_Mat()) };
		ret
	}
	
	// isSubmatrix() /usr/include/opencv2/core/mat.hpp:1744
	#[inline]
	fn is_submatrix(&self) -> bool {
		let ret = unsafe { sys::cv_Mat_isSubmatrix_const(self.as_raw_Mat()) };
		ret
	}
	
	// elemSize() /usr/include/opencv2/core/mat.hpp:1751
	#[inline]
	fn elem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_elemSize_const(self.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize1() /usr/include/opencv2/core/mat.hpp:1758
	#[inline]
	fn elem_size1(&self) -> size_t {
		let ret = unsafe { sys::cv_Mat_elemSize1_const(self.as_raw_Mat()) };
		ret
	}
	
	// type() /usr/include/opencv2/core/mat.hpp:1765
	#[inline]
	fn typ(&self) -> i32 {
		let ret = unsafe { sys::cv_Mat_type_const(self.as_raw_Mat()) };
		ret
	}
	
	// depth() /usr/include/opencv2/core/mat.hpp:1780
	#[inline]
	fn depth(&self) -> i32 {
		let ret = unsafe { sys::cv_Mat_depth_const(self.as_raw_Mat()) };
		ret
	}
	
	// channels() /usr/include/opencv2/core/mat.hpp:1786
	#[inline]
	fn channels(&self) -> i32 {
		let ret = unsafe { sys::cv_Mat_channels_const(self.as_raw_Mat()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * i: 0
	// step1(int) /usr/include/opencv2/core/mat.hpp:1793
	#[inline]
	fn step1(&self, i: i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_step1_const_int(self.as_raw_Mat(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/mat.hpp:1800
	#[inline]
	fn empty(&self) -> bool {
		let ret = unsafe { sys::cv_Mat_empty_const(self.as_raw_Mat()) };
		ret
	}
	
	// total() /usr/include/opencv2/core/mat.hpp:1807
	#[inline]
	fn total(&self) -> size_t {
		let ret = unsafe { sys::cv_Mat_total_const(self.as_raw_Mat()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * end_dim: INT_MAX
	// total(int, int) /usr/include/opencv2/core/mat.hpp:1813
	#[inline]
	fn total_slice(&self, start_dim: i32, end_dim: i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_total_const_int_int(self.as_raw_Mat(), start_dim, end_dim, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * depth: -1
	/// * require_continuous: true
	// checkVector(int, int, bool) /usr/include/opencv2/core/mat.hpp:1836
	#[inline]
	fn check_vector(&self, elem_channels: i32, depth: i32, require_continuous: bool) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_checkVector_const_int_int_bool(self.as_raw_Mat(), elem_channels, depth, require_continuous, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i0: 0
	// ptr(int) /usr/include/opencv2/core/mat.hpp:1846
	#[inline]
	fn ptr(&self, i0: i32) -> Result<*const u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ptr_const_int(self.as_raw_Mat(), i0, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// ptr(int, int) /usr/include/opencv2/core/mat.hpp:1857
	#[inline]
	fn ptr_2d(&self, row: i32, col: i32) -> Result<*const u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ptr_const_int_int(self.as_raw_Mat(), row, col, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// ptr(int, int, int) /usr/include/opencv2/core/mat.hpp:1862
	#[inline]
	fn ptr_3d(&self, i0: i32, i1: i32, i2: i32) -> Result<*const u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ptr_const_int_int_int(self.as_raw_Mat(), i0, i1, i2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// ptr(const int *) /usr/include/opencv2/core/mat.hpp:1867
	#[inline]
	fn ptr_nd(&self, idx: &[i32]) -> Result<*const u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ptr_const_const_intX(self.as_raw_Mat(), idx.as_ptr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i0: 0
	// at(int) /usr/include/opencv2/core/mat.hpp:1934
	fn at<T: core::DataType>(&self, i0: i32) -> Result<&T> { core::mat_forward::at(self, i0) }
	
	// at(int, int) /usr/include/opencv2/core/mat.hpp:1944
	fn at_2d<T: core::DataType>(&self, row: i32, col: i32) -> Result<&T> { core::mat_forward::at_2d(self, row, col) }
	
	// at(int, int, int) /usr/include/opencv2/core/mat.hpp:1957
	fn at_3d<T: core::DataType>(&self, i0: i32, i1: i32, i2: i32) -> Result<&T> { core::mat_forward::at_3d(self, i0, i1, i2) }
	
	// at(const int *) /usr/include/opencv2/core/mat.hpp:1966
	fn at_nd<T: core::DataType>(&self, idx: &[i32]) -> Result<&T> { core::mat_forward::at_nd(self, idx) }
	
	// at(cv::Point) /usr/include/opencv2/core/mat.hpp:1982
	fn at_pt<T: core::DataType>(&self, pt: core::Point) -> Result<&T> { core::mat_forward::at_pt(self, pt) }
	
}

pub trait MatTrait: core::MatTraitConst {
	fn as_raw_mut_Mat(&mut self) -> *mut c_void;

	// flags /usr/include/opencv2/core/mat.hpp:2112
	#[inline]
	fn set_flags(&mut self, val: i32) {
		let ret = unsafe { sys::cv_Mat_setPropFlags_int(self.as_raw_mut_Mat(), val) };
		ret
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2114
	#[inline]
	fn set_dims(&mut self, val: i32) {
		let ret = unsafe { sys::cv_Mat_setPropDims_int(self.as_raw_mut_Mat(), val) };
		ret
	}
	
	// rows /usr/include/opencv2/core/mat.hpp:2116
	#[inline]
	fn set_rows(&mut self, val: i32) {
		let ret = unsafe { sys::cv_Mat_setPropRows_int(self.as_raw_mut_Mat(), val) };
		ret
	}
	
	// cols /usr/include/opencv2/core/mat.hpp:2116
	#[inline]
	fn set_cols(&mut self, val: i32) {
		let ret = unsafe { sys::cv_Mat_setPropCols_int(self.as_raw_mut_Mat(), val) };
		ret
	}
	
	// data /usr/include/opencv2/core/mat.hpp:2118
	#[inline]
	fn data_mut(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_Mat_getPropData(self.as_raw_mut_Mat()) };
		ret
	}
	
	// data /usr/include/opencv2/core/mat.hpp:2118
	#[inline]
	unsafe fn set_data(&mut self, val: *mut u8) {
		let ret = { sys::cv_Mat_setPropData_unsigned_charX(self.as_raw_mut_Mat(), val) };
		ret
	}
	
	// u /usr/include/opencv2/core/mat.hpp:2136
	#[inline]
	fn u(&mut self) -> core::UMatData {
		let ret = unsafe { sys::cv_Mat_getPropU(self.as_raw_mut_Mat()) };
		let ret = unsafe { core::UMatData::opencv_from_extern(ret) };
		ret
	}
	
	// u /usr/include/opencv2/core/mat.hpp:2136
	#[inline]
	fn set_u(&mut self, val: &mut core::UMatData) {
		let ret = unsafe { sys::cv_Mat_setPropU_UMatDataX(self.as_raw_mut_Mat(), val.as_raw_mut_UMatData()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * mask: noArray()
	// setTo(cv::InputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:1269
	#[inline]
	fn set_to(&mut self, value: &dyn core::ToInputArray, mask: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(value);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_setTo_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Mat(), value.as_raw__InputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create(int, int, int) /usr/include/opencv2/core/mat.hpp:1472
	#[inline]
	unsafe fn create_rows_cols(&mut self, rows: i32, cols: i32, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_create_int_int_int(self.as_raw_mut_Mat(), rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// create(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1478
	#[inline]
	unsafe fn create_size(&mut self, size: core::Size, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_create_Size_int(self.as_raw_mut_Mat(), size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// create(int, const int *, int) /usr/include/opencv2/core/mat.hpp:1485
	#[inline]
	unsafe fn create_nd(&mut self, sizes: &[i32], typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_create_int_const_intX_int(self.as_raw_mut_Mat(), sizes.len() as _, sizes.as_ptr(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// create(const std::vector<int> &, int) /usr/include/opencv2/core/mat.hpp:1491
	#[inline]
	unsafe fn create_nd_vec(&mut self, sizes: &core::Vector<i32>, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_create_const_vector_int_R_int(self.as_raw_mut_Mat(), sizes.as_raw_VectorOfi32(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// addref() /usr/include/opencv2/core/mat.hpp:1502
	#[inline]
	fn addref(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_addref(self.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/mat.hpp:1517
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_release(self.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// deallocate() /usr/include/opencv2/core/mat.hpp:1520
	#[inline]
	fn deallocate(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_deallocate(self.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// reserve(size_t) /usr/include/opencv2/core/mat.hpp:1531
	#[inline]
	fn reserve(&mut self, sz: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_reserve_size_t(self.as_raw_mut_Mat(), sz, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// reserveBuffer(size_t) /usr/include/opencv2/core/mat.hpp:1539
	#[inline]
	fn reserve_buffer(&mut self, sz: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_reserveBuffer_size_t(self.as_raw_mut_Mat(), sz, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// resize(size_t) /usr/include/opencv2/core/mat.hpp:1548
	#[inline]
	fn resize(&mut self, sz: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_resize_size_t(self.as_raw_mut_Mat(), sz, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// resize(size_t, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:1554
	#[inline]
	fn resize_with_default(&mut self, sz: size_t, s: core::Scalar) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_resize_size_t_const_ScalarR(self.as_raw_mut_Mat(), sz, &s, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// push_back(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:1581
	#[inline]
	fn push_back(&mut self, m: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_push_back_const_MatR(self.as_raw_mut_Mat(), m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * nelems: 1
	// pop_back(size_t) /usr/include/opencv2/core/mat.hpp:1589
	#[inline]
	fn pop_back(&mut self, nelems: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_pop_back_size_t(self.as_raw_mut_Mat(), nelems, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// adjustROI(int, int, int, int) /usr/include/opencv2/core/mat.hpp:1631
	#[inline]
	fn adjust_roi(&mut self, dtop: i32, dbottom: i32, dleft: i32, dright: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_adjustROI_int_int_int_int(self.as_raw_mut_Mat(), dtop, dbottom, dleft, dright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i0: 0
	// ptr(int) /usr/include/opencv2/core/mat.hpp:1844
	#[inline]
	fn ptr_mut(&mut self, i0: i32) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ptr_int(self.as_raw_mut_Mat(), i0, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// ptr(int, int) /usr/include/opencv2/core/mat.hpp:1852
	#[inline]
	fn ptr_2d_mut(&mut self, row: i32, col: i32) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ptr_int_int(self.as_raw_mut_Mat(), row, col, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// ptr(int, int, int) /usr/include/opencv2/core/mat.hpp:1860
	#[inline]
	fn ptr_3d_mut(&mut self, i0: i32, i1: i32, i2: i32) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ptr_int_int_int(self.as_raw_mut_Mat(), i0, i1, i2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// ptr(const int *) /usr/include/opencv2/core/mat.hpp:1865
	#[inline]
	fn ptr_nd_mut(&mut self, idx: &[i32]) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ptr_const_intX(self.as_raw_mut_Mat(), idx.as_ptr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i0: 0
	// at(int) /usr/include/opencv2/core/mat.hpp:1930
	fn at_mut<T: core::DataType>(&mut self, i0: i32) -> Result<&mut T> { core::mat_forward::at_mut(self, i0) }
	
	// at(int, int) /usr/include/opencv2/core/mat.hpp:1939
	fn at_2d_mut<T: core::DataType>(&mut self, row: i32, col: i32) -> Result<&mut T> { core::mat_forward::at_2d_mut(self, row, col) }
	
	// at(int, int, int) /usr/include/opencv2/core/mat.hpp:1951
	fn at_3d_mut<T: core::DataType>(&mut self, i0: i32, i1: i32, i2: i32) -> Result<&mut T> { core::mat_forward::at_3d_mut(self, i0, i1, i2) }
	
	// at(const int *) /usr/include/opencv2/core/mat.hpp:1962
	fn at_nd_mut<T: core::DataType>(&mut self, idx: &[i32]) -> Result<&mut T> { core::mat_forward::at_nd_mut(self, idx) }
	
	// at(cv::Point) /usr/include/opencv2/core/mat.hpp:1977
	fn at_pt_mut<T: core::DataType>(&mut self, pt: core::Point) -> Result<&mut T> { core::mat_forward::at_pt_mut(self, pt) }
	
	// updateContinuityFlag() /usr/include/opencv2/core/mat.hpp:2133
	#[inline]
	fn update_continuity_flag(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_updateContinuityFlag(self.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Mat /usr/include/opencv2/core/mat.hpp:810
pub struct Mat {
	ptr: *mut c_void
}

opencv_type_boxed! { Mat }

impl Drop for Mat {
	fn drop(&mut self) {
		extern "C" { fn cv_Mat_delete(instance: *mut c_void); }
		unsafe { cv_Mat_delete(self.as_raw_mut_Mat()) };
	}
}

unsafe impl Send for Mat {}

impl core::MatTraitConst for Mat {
	#[inline] fn as_raw_Mat(&self) -> *const c_void { self.as_raw() }
}

impl core::MatTrait for Mat {
	#[inline] fn as_raw_mut_Mat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Mat {
	// Mat() /usr/include/opencv2/core/mat.hpp:819
	#[inline]
	pub fn default() -> core::Mat {
		let ret = unsafe { sys::cv_Mat_Mat() };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// Mat(int, int, int) /usr/include/opencv2/core/mat.hpp:827
	#[inline]
	pub unsafe fn new_rows_cols(rows: i32, cols: i32, typ: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_Mat_int_int_int(rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(cv::Size, int) /usr/include/opencv2/core/mat.hpp:835
	#[inline]
	pub unsafe fn new_size(size: core::Size, typ: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_Mat_Size_int(size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(int, int, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:846
	#[inline]
	pub fn new_rows_cols_with_default(rows: i32, cols: i32, typ: i32, s: core::Scalar) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_int_int_int_const_ScalarR(rows, cols, typ, &s, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(cv::Size, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:857
	#[inline]
	pub fn new_size_with_default(size: core::Size, typ: i32, s: core::Scalar) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_Size_int_const_ScalarR(size.opencv_as_extern(), typ, &s, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(int, const int *, int) /usr/include/opencv2/core/mat.hpp:865
	#[inline]
	pub unsafe fn new_nd(ndims: i32, sizes: &i32, typ: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_Mat_int_const_intX_int(ndims, sizes, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(const std::vector<int> &, int) /usr/include/opencv2/core/mat.hpp:872
	#[inline]
	pub unsafe fn new_nd_vec(sizes: &core::Vector<i32>, typ: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_Mat_const_vector_int_R_int(sizes.as_raw_VectorOfi32(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(int, const int *, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:883
	#[inline]
	pub fn new_nd_with_default(sizes: &[i32], typ: i32, s: core::Scalar) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_int_const_intX_int_const_ScalarR(sizes.len() as _, sizes.as_ptr(), typ, &s, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(const std::vector<int> &, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:893
	#[inline]
	pub fn new_nd_vec_with_default(sizes: &core::Vector<i32>, typ: i32, s: core::Scalar) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_const_vector_int_R_int_const_ScalarR(sizes.as_raw_VectorOfi32(), typ, &s, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:903
	#[inline]
	pub fn copy(m: &core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_const_MatR(m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * step: AUTO_STEP
	// Mat(int, int, int, void *, size_t) /usr/include/opencv2/core/mat.hpp:919
	#[inline]
	pub unsafe fn new_rows_cols_with_data(rows: i32, cols: i32, typ: i32, data: *mut c_void, step: size_t) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_Mat_int_int_int_voidX_size_t(rows, cols, typ, data, step, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * step: AUTO_STEP
	// Mat(cv::Size, int, void *, size_t) /usr/include/opencv2/core/mat.hpp:935
	#[inline]
	pub unsafe fn new_size_with_data(size: core::Size, typ: i32, data: *mut c_void, step: size_t) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_Mat_Size_int_voidX_size_t(size.opencv_as_extern(), typ, data, step, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * steps: 0
	// Mat(int, const int *, int, void *, const size_t *) /usr/include/opencv2/core/mat.hpp:950
	#[inline]
	pub unsafe fn new_nd_with_data(sizes: &[i32], typ: i32, data: *mut c_void, steps: Option<&[size_t]>) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX(sizes.len() as _, sizes.as_ptr(), typ, data, steps.map_or(::core::ptr::null(), |steps| steps.as_ptr()), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * steps: 0
	// Mat(const std::vector<int> &, int, void *, const size_t *) /usr/include/opencv2/core/mat.hpp:964
	#[inline]
	pub unsafe fn new_nd_vec_with_data(sizes: &core::Vector<i32>, typ: i32, data: *mut c_void, steps: Option<&[size_t]>) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		{ sys::cv_Mat_Mat_const_vector_int_R_int_voidX_const_size_tX(sizes.as_raw_VectorOfi32(), typ, data, steps.map_or(::core::ptr::null(), |steps| steps.as_ptr()), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * col_range: Range::all()
	// Mat(const cv::Mat &, const cv::Range &, const cv::Range &) /usr/include/opencv2/core/mat.hpp:976
	#[inline]
	pub fn rowscols(m: &core::Mat, row_range: &core::Range, col_range: &core::Range) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_const_MatR_const_RangeR_const_RangeR(m.as_raw_Mat(), row_range.as_raw_Range(), col_range.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(const cv::Mat &, const cv::Rect &) /usr/include/opencv2/core/mat.hpp:986
	#[inline]
	pub fn roi(m: &core::Mat, roi: core::Rect) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_const_MatR_const_RectR(m.as_raw_Mat(), &roi, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(const cv::Mat &, const std::vector<Range> &) /usr/include/opencv2/core/mat.hpp:1006
	#[inline]
	pub fn ranges(m: &core::Mat, ranges: &core::Vector<core::Range>) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_const_MatR_const_vector_Range_R(m.as_raw_Mat(), ranges.as_raw_VectorOfRange(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:1060
	#[inline]
	pub fn from_gpumat(m: &core::GpuMat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_const_GpuMatR(m.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// diag(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:1200
	#[inline]
	#[must_use]
	pub fn diag_mat(d: &core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_diag_const_MatR(d.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// zeros(int, int, int) /usr/include/opencv2/core/mat.hpp:1371
	#[inline]
	#[must_use]
	pub fn zeros(rows: i32, cols: i32, typ: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_zeros_int_int_int(rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// zeros(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1377
	#[inline]
	#[must_use]
	pub fn zeros_size(size: core::Size, typ: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_zeros_Size_int(size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// zeros(int, const int *, int) /usr/include/opencv2/core/mat.hpp:1384
	#[inline]
	#[must_use]
	pub fn zeros_nd(sz: &[i32], typ: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_zeros_int_const_intX_int(sz.len() as _, sz.as_ptr(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ones(int, int, int) /usr/include/opencv2/core/mat.hpp:1402
	#[inline]
	#[must_use]
	pub fn ones(rows: i32, cols: i32, typ: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ones_int_int_int(rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ones(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1408
	#[inline]
	#[must_use]
	pub fn ones_size(size: core::Size, typ: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ones_Size_int(size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ones(int, const int *, int) /usr/include/opencv2/core/mat.hpp:1415
	#[inline]
	#[must_use]
	pub fn ones_nd(sz: &[i32], typ: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_ones_int_const_intX_int(sz.len() as _, sz.as_ptr(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// eye(int, int, int) /usr/include/opencv2/core/mat.hpp:1431
	#[inline]
	#[must_use]
	pub fn eye(rows: i32, cols: i32, typ: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_eye_int_int_int(rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// eye(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1437
	#[inline]
	#[must_use]
	pub fn eye_size(size: core::Size, typ: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_eye_Size_int(size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Mat(cv::Mat &&) /usr/include/opencv2/core/mat.hpp:2100
	#[inline]
	pub fn copy_mut(m: &mut core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Mat_Mat_MatR(m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Clone for Mat {
	#[inline]
	/// Calls try_clone() and panics if that fails
	fn clone(&self) -> Self {
		self.try_clone().expect("Cannot clone Mat")
	}
}

impl Default for Mat {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// MatConstIterator /usr/include/opencv2/core/mat.hpp:3061
pub trait MatConstIteratorTraitConst {
	fn as_raw_MatConstIterator(&self) -> *const c_void;

	// m /usr/include/opencv2/core/mat.hpp:3112
	#[inline]
	fn m(&self) -> core::Mat {
		let ret = unsafe { sys::cv_MatConstIterator_getPropM_const(self.as_raw_MatConstIterator()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// elemSize /usr/include/opencv2/core/mat.hpp:3113
	#[inline]
	fn elem_size(&self) -> size_t {
		let ret = unsafe { sys::cv_MatConstIterator_getPropElemSize_const(self.as_raw_MatConstIterator()) };
		ret
	}
	
	// ptr /usr/include/opencv2/core/mat.hpp:3114
	#[inline]
	fn ptr(&self) -> *const u8 {
		let ret = unsafe { sys::cv_MatConstIterator_getPropPtr_const(self.as_raw_MatConstIterator()) };
		ret
	}
	
	// sliceStart /usr/include/opencv2/core/mat.hpp:3115
	#[inline]
	fn slice_start(&self) -> *const u8 {
		let ret = unsafe { sys::cv_MatConstIterator_getPropSliceStart_const(self.as_raw_MatConstIterator()) };
		ret
	}
	
	// sliceEnd /usr/include/opencv2/core/mat.hpp:3116
	#[inline]
	fn slice_end(&self) -> *const u8 {
		let ret = unsafe { sys::cv_MatConstIterator_getPropSliceEnd_const(self.as_raw_MatConstIterator()) };
		ret
	}
	
	// operator*() /usr/include/opencv2/core/mat.hpp:3087
	#[inline]
	fn try_deref(&self) -> Result<*const u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_operatorX_const(self.as_raw_MatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator[](ptrdiff_t) /usr/include/opencv2/core/mat.hpp:3089
	#[inline]
	fn get(&self, i: ptrdiff_t) -> Result<*const u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_operator___const_ptrdiff_t(self.as_raw_MatConstIterator(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// pos() /usr/include/opencv2/core/mat.hpp:3104
	#[inline]
	fn pos(&self) -> Result<core::Point> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_pos_const(self.as_raw_MatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// pos(int *) /usr/include/opencv2/core/mat.hpp:3106
	#[inline]
	fn pos_to(&self, _idx: &mut i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_pos_const_intX(self.as_raw_MatConstIterator(), _idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// lpos() /usr/include/opencv2/core/mat.hpp:3108
	#[inline]
	fn lpos(&self) -> Result<ptrdiff_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_lpos_const(self.as_raw_MatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MatConstIteratorTrait: core::MatConstIteratorTraitConst {
	fn as_raw_mut_MatConstIterator(&mut self) -> *mut c_void;

	// elemSize /usr/include/opencv2/core/mat.hpp:3113
	#[inline]
	fn set_elem_size(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_MatConstIterator_setPropElemSize_size_t(self.as_raw_mut_MatConstIterator(), val) };
		ret
	}
	
	// operator--() /usr/include/opencv2/core/mat.hpp:3096
	#[inline]
	fn decr(&mut self) -> Result<core::MatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_operatorSS(self.as_raw_mut_MatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// operator++() /usr/include/opencv2/core/mat.hpp:3100
	#[inline]
	fn incr(&mut self) -> Result<core::MatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_operatorAA(self.as_raw_mut_MatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * relative: false
	// seek(ptrdiff_t, bool) /usr/include/opencv2/core/mat.hpp:3109
	#[inline]
	fn seek(&mut self, ofs: ptrdiff_t, relative: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_seek_ptrdiff_t_bool(self.as_raw_mut_MatConstIterator(), ofs, relative, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * relative: false
	// seek(const int *, bool) /usr/include/opencv2/core/mat.hpp:3110
	#[inline]
	fn seek_idx(&mut self, _idx: &i32, relative: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_seek_const_intX_bool(self.as_raw_mut_MatConstIterator(), _idx, relative, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// MatConstIterator /usr/include/opencv2/core/mat.hpp:3061
pub struct MatConstIterator {
	ptr: *mut c_void
}

opencv_type_boxed! { MatConstIterator }

impl Drop for MatConstIterator {
	fn drop(&mut self) {
		extern "C" { fn cv_MatConstIterator_delete(instance: *mut c_void); }
		unsafe { cv_MatConstIterator_delete(self.as_raw_mut_MatConstIterator()) };
	}
}

unsafe impl Send for MatConstIterator {}

impl core::MatConstIteratorTraitConst for MatConstIterator {
	#[inline] fn as_raw_MatConstIterator(&self) -> *const c_void { self.as_raw() }
}

impl core::MatConstIteratorTrait for MatConstIterator {
	#[inline] fn as_raw_mut_MatConstIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MatConstIterator {
	// MatConstIterator() /usr/include/opencv2/core/mat.hpp:3072
	#[inline]
	pub fn default() -> Result<core::MatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_MatConstIterator(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// MatConstIterator(const cv::Mat *) /usr/include/opencv2/core/mat.hpp:3074
	#[inline]
	pub fn over(_m: &core::Mat) -> Result<core::MatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_MatConstIterator_const_MatX(_m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * _col: 0
	// MatConstIterator(const cv::Mat *, int, int) /usr/include/opencv2/core/mat.hpp:3076
	#[inline]
	pub fn with_rows_cols(_m: &core::Mat, _row: i32, _col: i32) -> Result<core::MatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_MatConstIterator_const_MatX_int_int(_m.as_raw_Mat(), _row, _col, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// MatConstIterator(const cv::Mat *, cv::Point) /usr/include/opencv2/core/mat.hpp:3078
	#[inline]
	pub fn with_start(_m: &core::Mat, _pt: core::Point) -> Result<core::MatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_MatConstIterator_const_MatX_Point(_m.as_raw_Mat(), _pt.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// MatConstIterator(const cv::MatConstIterator &) /usr/include/opencv2/core/mat.hpp:3082
	#[inline]
	pub fn copy(it: &core::MatConstIterator) -> Result<core::MatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatConstIterator_MatConstIterator_const_MatConstIteratorR(it.as_raw_MatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// MatExpr /usr/include/opencv2/core/mat.hpp:3563
pub trait MatExprTraitConst {
	fn as_raw_MatExpr(&self) -> *const c_void;

	// flags /usr/include/opencv2/core/mat.hpp:3595
	#[inline]
	fn flags(&self) -> i32 {
		let ret = unsafe { sys::cv_MatExpr_getPropFlags_const(self.as_raw_MatExpr()) };
		ret
	}
	
	// a /usr/include/opencv2/core/mat.hpp:3597
	#[inline]
	fn a(&self) -> core::Mat {
		let ret = unsafe { sys::cv_MatExpr_getPropA_const(self.as_raw_MatExpr()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// b /usr/include/opencv2/core/mat.hpp:3597
	#[inline]
	fn b(&self) -> core::Mat {
		let ret = unsafe { sys::cv_MatExpr_getPropB_const(self.as_raw_MatExpr()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// c /usr/include/opencv2/core/mat.hpp:3597
	#[inline]
	fn c(&self) -> core::Mat {
		let ret = unsafe { sys::cv_MatExpr_getPropC_const(self.as_raw_MatExpr()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// alpha /usr/include/opencv2/core/mat.hpp:3598
	#[inline]
	fn alpha(&self) -> f64 {
		let ret = unsafe { sys::cv_MatExpr_getPropAlpha_const(self.as_raw_MatExpr()) };
		ret
	}
	
	// beta /usr/include/opencv2/core/mat.hpp:3598
	#[inline]
	fn beta(&self) -> f64 {
		let ret = unsafe { sys::cv_MatExpr_getPropBeta_const(self.as_raw_MatExpr()) };
		ret
	}
	
	// s /usr/include/opencv2/core/mat.hpp:3599
	#[inline]
	fn s(&self) -> core::Scalar {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_getPropS_const(self.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// operator Mat() /usr/include/opencv2/core/mat.hpp:3572
	#[inline]
	fn to_mat(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_operator_cv_Mat_const(self.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// size() /usr/include/opencv2/core/mat.hpp:3575
	#[inline]
	fn size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_size_const(self.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// type() /usr/include/opencv2/core/mat.hpp:3576
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_type_const(self.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// row(int) /usr/include/opencv2/core/mat.hpp:3578
	#[inline]
	fn row(&self, y: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_row_const_int(self.as_raw_MatExpr(), y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// col(int) /usr/include/opencv2/core/mat.hpp:3579
	#[inline]
	fn col(&self, x: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_col_const_int(self.as_raw_MatExpr(), x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * d: 0
	// diag(int) /usr/include/opencv2/core/mat.hpp:3580
	#[inline]
	fn diag(&self, d: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_diag_const_int(self.as_raw_MatExpr(), d, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// t() /usr/include/opencv2/core/mat.hpp:3584
	#[inline]
	fn t(&self) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_t_const(self.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * method: DECOMP_LU
	// inv(int) /usr/include/opencv2/core/mat.hpp:3585
	#[inline]
	fn inv(&self, method: i32) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_inv_const_int(self.as_raw_MatExpr(), method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	// mul(const cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3586
	#[inline]
	fn mul_matexpr(&self, e: &core::MatExpr, scale: f64) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_mul_const_const_MatExprR_double(self.as_raw_MatExpr(), e.as_raw_MatExpr(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	// mul(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3587
	#[inline]
	fn mul(&self, m: &core::Mat, scale: f64) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_mul_const_const_MatR_double(self.as_raw_MatExpr(), m.as_raw_Mat(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// cross(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3589
	#[inline]
	fn cross(&self, m: &core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_cross_const_const_MatR(self.as_raw_MatExpr(), m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// dot(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3590
	#[inline]
	fn dot(&self, m: &core::Mat) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_dot_const_const_MatR(self.as_raw_MatExpr(), m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MatExprTrait: core::MatExprTraitConst {
	fn as_raw_mut_MatExpr(&mut self) -> *mut c_void;

	// flags /usr/include/opencv2/core/mat.hpp:3595
	#[inline]
	fn set_flags(&mut self, val: i32) {
		let ret = unsafe { sys::cv_MatExpr_setPropFlags_int(self.as_raw_mut_MatExpr(), val) };
		ret
	}
	
	// a /usr/include/opencv2/core/mat.hpp:3597
	#[inline]
	fn set_a(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_MatExpr_setPropA_Mat(self.as_raw_mut_MatExpr(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// b /usr/include/opencv2/core/mat.hpp:3597
	#[inline]
	fn set_b(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_MatExpr_setPropB_Mat(self.as_raw_mut_MatExpr(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// c /usr/include/opencv2/core/mat.hpp:3597
	#[inline]
	fn set_c(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_MatExpr_setPropC_Mat(self.as_raw_mut_MatExpr(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// alpha /usr/include/opencv2/core/mat.hpp:3598
	#[inline]
	fn set_alpha(&mut self, val: f64) {
		let ret = unsafe { sys::cv_MatExpr_setPropAlpha_double(self.as_raw_mut_MatExpr(), val) };
		ret
	}
	
	// beta /usr/include/opencv2/core/mat.hpp:3598
	#[inline]
	fn set_beta(&mut self, val: f64) {
		let ret = unsafe { sys::cv_MatExpr_setPropBeta_double(self.as_raw_mut_MatExpr(), val) };
		ret
	}
	
	// s /usr/include/opencv2/core/mat.hpp:3599
	#[inline]
	fn set_s(&mut self, val: core::Scalar) {
		let ret = unsafe { sys::cv_MatExpr_setPropS_Scalar(self.as_raw_mut_MatExpr(), val.opencv_as_extern()) };
		ret
	}
	
	// swap(cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3592
	#[inline]
	fn swap(&mut self, b: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_swap_MatExprR(self.as_raw_mut_MatExpr(), b.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// MatExpr /usr/include/opencv2/core/mat.hpp:3563
pub struct MatExpr {
	ptr: *mut c_void
}

opencv_type_boxed! { MatExpr }

impl Drop for MatExpr {
	fn drop(&mut self) {
		extern "C" { fn cv_MatExpr_delete(instance: *mut c_void); }
		unsafe { cv_MatExpr_delete(self.as_raw_mut_MatExpr()) };
	}
}

unsafe impl Send for MatExpr {}

impl core::MatExprTraitConst for MatExpr {
	#[inline] fn as_raw_MatExpr(&self) -> *const c_void { self.as_raw() }
}

impl core::MatExprTrait for MatExpr {
	#[inline] fn as_raw_mut_MatExpr(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MatExpr {
	// MatExpr() /usr/include/opencv2/core/mat.hpp:3566
	#[inline]
	pub fn default() -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_MatExpr(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// MatExpr(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3567
	#[inline]
	pub fn from_mat(m: &core::Mat) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_MatExpr_const_MatR(m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * _a: Mat()
	/// * _b: Mat()
	/// * _c: Mat()
	/// * _alpha: 1
	/// * _beta: 1
	/// * _s: Scalar()
	// MatExpr(const cv::MatOp *, int, const cv::Mat &, const cv::Mat &, const cv::Mat &, double, double, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3569
	#[inline]
	pub fn new(_op: &dyn core::MatOp, _flags: i32, _a: &core::Mat, _b: &core::Mat, _c: &core::Mat, _alpha: f64, _beta: f64, _s: core::Scalar) -> Result<core::MatExpr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatExpr_MatExpr_const_MatOpX_int_const_MatR_const_MatR_const_MatR_double_double_const_ScalarR(_op.as_raw_MatOp(), _flags, _a.as_raw_Mat(), _b.as_raw_Mat(), _c.as_raw_Mat(), _alpha, _beta, &_s, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::MatExpr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// MatOp /usr/include/opencv2/core/mat.hpp:3476
pub trait MatOpConst {
	fn as_raw_MatOp(&self) -> *const c_void;

	// elementWise(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3482
	#[inline]
	fn element_wise(&self, expr: &core::MatExpr) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_elementWise_const_const_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * typ: -1
	// assign(const cv::MatExpr &, cv::Mat &, int) /usr/include/opencv2/core/mat.hpp:3483
	#[inline]
	fn assign(&self, expr: &core::MatExpr, m: &mut core::Mat, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_assign_const_const_MatExprR_MatR_int(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// roi(const cv::MatExpr &, const cv::Range &, const cv::Range &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3484
	#[inline]
	fn roi(&self, expr: &core::MatExpr, row_range: &core::Range, col_range: &core::Range, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_roi_const_const_MatExprR_const_RangeR_const_RangeR_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), row_range.as_raw_Range(), col_range.as_raw_Range(), res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// diag(const cv::MatExpr &, int, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3486
	#[inline]
	fn diag(&self, expr: &core::MatExpr, d: i32, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_diag_const_const_MatExprR_int_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), d, res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// augAssignAdd(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3487
	#[inline]
	fn aug_assign_add(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_augAssignAdd_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// augAssignSubtract(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3488
	#[inline]
	fn aug_assign_subtract(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_augAssignSubtract_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// augAssignMultiply(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3489
	#[inline]
	fn aug_assign_multiply(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_augAssignMultiply_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// augAssignDivide(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3490
	#[inline]
	fn aug_assign_divide(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_augAssignDivide_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// augAssignAnd(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3491
	#[inline]
	fn aug_assign_and(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_augAssignAnd_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// augAssignOr(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3492
	#[inline]
	fn aug_assign_or(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_augAssignOr_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// augAssignXor(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3493
	#[inline]
	fn aug_assign_xor(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_augAssignXor_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// add(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3495
	#[inline]
	fn add(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_add_const_const_MatExprR_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// add(const cv::MatExpr &, const cv::Scalar &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3496
	#[inline]
	fn add_scalar(&self, expr1: &core::MatExpr, s: core::Scalar, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_add_const_const_MatExprR_const_ScalarR_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), &s, res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// subtract(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3498
	#[inline]
	fn subtract(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_subtract_const_const_MatExprR_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// subtract(const cv::Scalar &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3499
	#[inline]
	fn subtract_scalar(&self, s: core::Scalar, expr: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_subtract_const_const_ScalarR_const_MatExprR_MatExprR(self.as_raw_MatOp(), &s, expr.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	// multiply(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3501
	#[inline]
	fn multiply(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr, scale: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_multiply_const_const_MatExprR_const_MatExprR_MatExprR_double(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// multiply(const cv::MatExpr &, double, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3502
	#[inline]
	fn multiply_f64(&self, expr1: &core::MatExpr, s: f64, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_multiply_const_const_MatExprR_double_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), s, res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	// divide(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3504
	#[inline]
	fn divide(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr, scale: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_divide_const_const_MatExprR_const_MatExprR_MatExprR_double(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// divide(double, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3505
	#[inline]
	fn divide_f64(&self, s: f64, expr: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_divide_const_double_const_MatExprR_MatExprR(self.as_raw_MatOp(), s, expr.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// abs(const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3507
	#[inline]
	fn abs(&self, expr: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_abs_const_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// transpose(const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3509
	#[inline]
	fn transpose(&self, expr: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_transpose_const_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// matmul(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3510
	#[inline]
	fn matmul(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_matmul_const_const_MatExprR_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// invert(const cv::MatExpr &, int, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3511
	#[inline]
	fn invert(&self, expr: &core::MatExpr, method: i32, res: &mut core::MatExpr) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_invert_const_const_MatExprR_int_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), method, res.as_raw_mut_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// size(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3513
	#[inline]
	fn size(&self, expr: &core::MatExpr) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_size_const_const_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// type(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3514
	#[inline]
	fn typ(&self, expr: &core::MatExpr) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatOp_type_const_const_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MatOp: core::MatOpConst {
	fn as_raw_mut_MatOp(&mut self) -> *mut c_void;

}

// MatSize /usr/include/opencv2/core/mat.hpp:586
pub trait MatSizeTraitConst {
	fn as_raw_MatSize(&self) -> *const c_void;

	// dims() /usr/include/opencv2/core/mat.hpp:589
	#[inline]
	fn dims(&self) -> i32 {
		let ret = unsafe { sys::cv_MatSize_dims_const(self.as_raw_MatSize()) };
		ret
	}
	
	// operator[](int) /usr/include/opencv2/core/mat.hpp:591
	#[inline]
	fn get(&self, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatSize_operator___const_int(self.as_raw_MatSize(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator const int *() /usr/include/opencv2/core/mat.hpp:593
	#[inline]
	fn to_xconst_i32(&self) -> *const i32 {
		let ret = unsafe { sys::cv_MatSize_operator_const_intX_const(self.as_raw_MatSize()) };
		ret
	}
	
	// operator==(const cv::MatSize &) /usr/include/opencv2/core/mat.hpp:594
	#[inline]
	fn equals(&self, sz: &core::MatSize) -> bool {
		let ret = unsafe { sys::cv_MatSize_operatorEQ_const_const_MatSizeR(self.as_raw_MatSize(), sz.as_raw_MatSize()) };
		ret
	}
	
}

pub trait MatSizeTrait: core::MatSizeTraitConst {
	fn as_raw_mut_MatSize(&mut self) -> *mut c_void;

	// p /usr/include/opencv2/core/mat.hpp:597
	#[inline]
	fn p(&mut self) -> *mut i32 {
		let ret = unsafe { sys::cv_MatSize_getPropP(self.as_raw_mut_MatSize()) };
		ret
	}
	
	// p /usr/include/opencv2/core/mat.hpp:597
	#[inline]
	unsafe fn set_p(&mut self, val: *mut i32) {
		let ret = { sys::cv_MatSize_setPropP_intX(self.as_raw_mut_MatSize(), val) };
		ret
	}
	
	// operator[](int) /usr/include/opencv2/core/mat.hpp:592
	#[inline]
	fn get_mut(&mut self, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatSize_operator___int(self.as_raw_mut_MatSize(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// MatSize /usr/include/opencv2/core/mat.hpp:586
pub struct MatSize {
	ptr: *mut c_void
}

opencv_type_boxed! { MatSize }

impl Drop for MatSize {
	fn drop(&mut self) {
		extern "C" { fn cv_MatSize_delete(instance: *mut c_void); }
		unsafe { cv_MatSize_delete(self.as_raw_mut_MatSize()) };
	}
}

unsafe impl Send for MatSize {}

impl core::MatSizeTraitConst for MatSize {
	#[inline] fn as_raw_MatSize(&self) -> *const c_void { self.as_raw() }
}

impl core::MatSizeTrait for MatSize {
	#[inline] fn as_raw_mut_MatSize(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MatSize {
	// MatSize(int *) /usr/include/opencv2/core/mat.hpp:588
	#[inline]
	pub fn new(_p: &mut i32) -> core::MatSize {
		let ret = unsafe { sys::cv_MatSize_MatSize_intX(_p) };
		let ret = unsafe { core::MatSize::opencv_from_extern(ret) };
		ret
	}
	
}

// MatStep /usr/include/opencv2/core/mat.hpp:600
pub trait MatStepTraitConst {
	fn as_raw_MatStep(&self) -> *const c_void;

	// operator[](int) /usr/include/opencv2/core/mat.hpp:604
	#[inline]
	fn get(&self, i: i32) -> size_t {
		let ret = unsafe { sys::cv_MatStep_operator___const_int(self.as_raw_MatStep(), i) };
		ret
	}
	
	// operator unsigned long() /usr/include/opencv2/core/mat.hpp:606
	#[inline]
	fn to_size_t(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MatStep_operator_size_t_const(self.as_raw_MatStep(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MatStepTrait: core::MatStepTraitConst {
	fn as_raw_mut_MatStep(&mut self) -> *mut c_void;

	// p /usr/include/opencv2/core/mat.hpp:609
	#[inline]
	fn p(&mut self) -> *mut size_t {
		let ret = unsafe { sys::cv_MatStep_getPropP(self.as_raw_mut_MatStep()) };
		ret
	}
	
	// p /usr/include/opencv2/core/mat.hpp:609
	#[inline]
	unsafe fn set_p(&mut self, val: *mut size_t) {
		let ret = { sys::cv_MatStep_setPropP_size_tX(self.as_raw_mut_MatStep(), val) };
		ret
	}
	
	// buf /usr/include/opencv2/core/mat.hpp:610
	#[inline]
	fn buf(&mut self) -> &mut [size_t; 2] {
		let ret = unsafe { sys::cv_MatStep_getPropBuf(self.as_raw_mut_MatStep()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}
	
	// operator[](int) /usr/include/opencv2/core/mat.hpp:605
	#[inline]
	fn get_mut(&mut self, i: i32) -> size_t {
		let ret = unsafe { sys::cv_MatStep_operator___int(self.as_raw_mut_MatStep(), i) };
		ret
	}
	
}

// MatStep /usr/include/opencv2/core/mat.hpp:600
pub struct MatStep {
	ptr: *mut c_void
}

opencv_type_boxed! { MatStep }

impl Drop for MatStep {
	fn drop(&mut self) {
		extern "C" { fn cv_MatStep_delete(instance: *mut c_void); }
		unsafe { cv_MatStep_delete(self.as_raw_mut_MatStep()) };
	}
}

unsafe impl Send for MatStep {}

impl core::MatStepTraitConst for MatStep {
	#[inline] fn as_raw_MatStep(&self) -> *const c_void { self.as_raw() }
}

impl core::MatStepTrait for MatStep {
	#[inline] fn as_raw_mut_MatStep(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MatStep {
	// MatStep() /usr/include/opencv2/core/mat.hpp:602
	#[inline]
	pub fn default() -> core::MatStep {
		let ret = unsafe { sys::cv_MatStep_MatStep() };
		let ret = unsafe { core::MatStep::opencv_from_extern(ret) };
		ret
	}
	
	// MatStep(size_t) /usr/include/opencv2/core/mat.hpp:603
	#[inline]
	pub fn new(s: size_t) -> core::MatStep {
		let ret = unsafe { sys::cv_MatStep_MatStep_size_t(s) };
		let ret = unsafe { core::MatStep::opencv_from_extern(ret) };
		ret
	}
	
}

impl Default for MatStep {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// Matx_AddOp /usr/include/opencv2/core/matx.hpp:68
pub trait Matx_AddOpTraitConst {
	fn as_raw_Matx_AddOp(&self) -> *const c_void;

}

pub trait Matx_AddOpTrait: core::Matx_AddOpTraitConst {
	fn as_raw_mut_Matx_AddOp(&mut self) -> *mut c_void;

}

// Matx_AddOp /usr/include/opencv2/core/matx.hpp:68
pub struct Matx_AddOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_AddOp }

impl Drop for Matx_AddOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_AddOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_AddOp_delete(self.as_raw_mut_Matx_AddOp()) };
	}
}

unsafe impl Send for Matx_AddOp {}

impl core::Matx_AddOpTraitConst for Matx_AddOp {
	#[inline] fn as_raw_Matx_AddOp(&self) -> *const c_void { self.as_raw() }
}

impl core::Matx_AddOpTrait for Matx_AddOp {
	#[inline] fn as_raw_mut_Matx_AddOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_AddOp {
	// Matx_AddOp() /usr/include/opencv2/core/matx.hpp:68
	#[inline]
	pub fn default() -> Result<core::Matx_AddOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_AddOp_Matx_AddOp(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_AddOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Matx_AddOp(const cv::Matx_AddOp &) /usr/include/opencv2/core/matx.hpp:68
	#[inline]
	pub fn copy(unnamed: &core::Matx_AddOp) -> Result<core::Matx_AddOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_AddOp_Matx_AddOp_const_Matx_AddOpR(unnamed.as_raw_Matx_AddOp(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_AddOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Matx_DivOp /usr/include/opencv2/core/matx.hpp:72
pub trait Matx_DivOpTraitConst {
	fn as_raw_Matx_DivOp(&self) -> *const c_void;

}

pub trait Matx_DivOpTrait: core::Matx_DivOpTraitConst {
	fn as_raw_mut_Matx_DivOp(&mut self) -> *mut c_void;

}

// Matx_DivOp /usr/include/opencv2/core/matx.hpp:72
pub struct Matx_DivOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_DivOp }

impl Drop for Matx_DivOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_DivOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_DivOp_delete(self.as_raw_mut_Matx_DivOp()) };
	}
}

unsafe impl Send for Matx_DivOp {}

impl core::Matx_DivOpTraitConst for Matx_DivOp {
	#[inline] fn as_raw_Matx_DivOp(&self) -> *const c_void { self.as_raw() }
}

impl core::Matx_DivOpTrait for Matx_DivOp {
	#[inline] fn as_raw_mut_Matx_DivOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_DivOp {
	// Matx_DivOp() /usr/include/opencv2/core/matx.hpp:72
	#[inline]
	pub fn default() -> Result<core::Matx_DivOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_DivOp_Matx_DivOp(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_DivOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Matx_DivOp(const cv::Matx_DivOp &) /usr/include/opencv2/core/matx.hpp:72
	#[inline]
	pub fn copy(unnamed: &core::Matx_DivOp) -> Result<core::Matx_DivOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_DivOp_Matx_DivOp_const_Matx_DivOpR(unnamed.as_raw_Matx_DivOp(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_DivOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Matx_MatMulOp /usr/include/opencv2/core/matx.hpp:73
pub trait Matx_MatMulOpTraitConst {
	fn as_raw_Matx_MatMulOp(&self) -> *const c_void;

}

pub trait Matx_MatMulOpTrait: core::Matx_MatMulOpTraitConst {
	fn as_raw_mut_Matx_MatMulOp(&mut self) -> *mut c_void;

}

// Matx_MatMulOp /usr/include/opencv2/core/matx.hpp:73
pub struct Matx_MatMulOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_MatMulOp }

impl Drop for Matx_MatMulOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_MatMulOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_MatMulOp_delete(self.as_raw_mut_Matx_MatMulOp()) };
	}
}

unsafe impl Send for Matx_MatMulOp {}

impl core::Matx_MatMulOpTraitConst for Matx_MatMulOp {
	#[inline] fn as_raw_Matx_MatMulOp(&self) -> *const c_void { self.as_raw() }
}

impl core::Matx_MatMulOpTrait for Matx_MatMulOp {
	#[inline] fn as_raw_mut_Matx_MatMulOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_MatMulOp {
	// Matx_MatMulOp() /usr/include/opencv2/core/matx.hpp:73
	#[inline]
	pub fn default() -> Result<core::Matx_MatMulOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_MatMulOp_Matx_MatMulOp(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_MatMulOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Matx_MatMulOp(const cv::Matx_MatMulOp &) /usr/include/opencv2/core/matx.hpp:73
	#[inline]
	pub fn copy(unnamed: &core::Matx_MatMulOp) -> Result<core::Matx_MatMulOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_MatMulOp_Matx_MatMulOp_const_Matx_MatMulOpR(unnamed.as_raw_Matx_MatMulOp(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_MatMulOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Matx_MulOp /usr/include/opencv2/core/matx.hpp:71
pub trait Matx_MulOpTraitConst {
	fn as_raw_Matx_MulOp(&self) -> *const c_void;

}

pub trait Matx_MulOpTrait: core::Matx_MulOpTraitConst {
	fn as_raw_mut_Matx_MulOp(&mut self) -> *mut c_void;

}

// Matx_MulOp /usr/include/opencv2/core/matx.hpp:71
pub struct Matx_MulOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_MulOp }

impl Drop for Matx_MulOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_MulOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_MulOp_delete(self.as_raw_mut_Matx_MulOp()) };
	}
}

unsafe impl Send for Matx_MulOp {}

impl core::Matx_MulOpTraitConst for Matx_MulOp {
	#[inline] fn as_raw_Matx_MulOp(&self) -> *const c_void { self.as_raw() }
}

impl core::Matx_MulOpTrait for Matx_MulOp {
	#[inline] fn as_raw_mut_Matx_MulOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_MulOp {
	// Matx_MulOp() /usr/include/opencv2/core/matx.hpp:71
	#[inline]
	pub fn default() -> Result<core::Matx_MulOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_MulOp_Matx_MulOp(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_MulOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Matx_MulOp(const cv::Matx_MulOp &) /usr/include/opencv2/core/matx.hpp:71
	#[inline]
	pub fn copy(unnamed: &core::Matx_MulOp) -> Result<core::Matx_MulOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_MulOp_Matx_MulOp_const_Matx_MulOpR(unnamed.as_raw_Matx_MulOp(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_MulOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Matx_ScaleOp /usr/include/opencv2/core/matx.hpp:70
pub trait Matx_ScaleOpTraitConst {
	fn as_raw_Matx_ScaleOp(&self) -> *const c_void;

}

pub trait Matx_ScaleOpTrait: core::Matx_ScaleOpTraitConst {
	fn as_raw_mut_Matx_ScaleOp(&mut self) -> *mut c_void;

}

// Matx_ScaleOp /usr/include/opencv2/core/matx.hpp:70
pub struct Matx_ScaleOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_ScaleOp }

impl Drop for Matx_ScaleOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_ScaleOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_ScaleOp_delete(self.as_raw_mut_Matx_ScaleOp()) };
	}
}

unsafe impl Send for Matx_ScaleOp {}

impl core::Matx_ScaleOpTraitConst for Matx_ScaleOp {
	#[inline] fn as_raw_Matx_ScaleOp(&self) -> *const c_void { self.as_raw() }
}

impl core::Matx_ScaleOpTrait for Matx_ScaleOp {
	#[inline] fn as_raw_mut_Matx_ScaleOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_ScaleOp {
	// Matx_ScaleOp() /usr/include/opencv2/core/matx.hpp:70
	#[inline]
	pub fn default() -> Result<core::Matx_ScaleOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_ScaleOp_Matx_ScaleOp(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_ScaleOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Matx_ScaleOp(const cv::Matx_ScaleOp &) /usr/include/opencv2/core/matx.hpp:70
	#[inline]
	pub fn copy(unnamed: &core::Matx_ScaleOp) -> Result<core::Matx_ScaleOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_ScaleOp_Matx_ScaleOp_const_Matx_ScaleOpR(unnamed.as_raw_Matx_ScaleOp(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_ScaleOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Matx_SubOp /usr/include/opencv2/core/matx.hpp:69
pub trait Matx_SubOpTraitConst {
	fn as_raw_Matx_SubOp(&self) -> *const c_void;

}

pub trait Matx_SubOpTrait: core::Matx_SubOpTraitConst {
	fn as_raw_mut_Matx_SubOp(&mut self) -> *mut c_void;

}

// Matx_SubOp /usr/include/opencv2/core/matx.hpp:69
pub struct Matx_SubOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_SubOp }

impl Drop for Matx_SubOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_SubOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_SubOp_delete(self.as_raw_mut_Matx_SubOp()) };
	}
}

unsafe impl Send for Matx_SubOp {}

impl core::Matx_SubOpTraitConst for Matx_SubOp {
	#[inline] fn as_raw_Matx_SubOp(&self) -> *const c_void { self.as_raw() }
}

impl core::Matx_SubOpTrait for Matx_SubOp {
	#[inline] fn as_raw_mut_Matx_SubOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_SubOp {
	// Matx_SubOp() /usr/include/opencv2/core/matx.hpp:69
	#[inline]
	pub fn default() -> Result<core::Matx_SubOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_SubOp_Matx_SubOp(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_SubOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Matx_SubOp(const cv::Matx_SubOp &) /usr/include/opencv2/core/matx.hpp:69
	#[inline]
	pub fn copy(unnamed: &core::Matx_SubOp) -> Result<core::Matx_SubOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_SubOp_Matx_SubOp_const_Matx_SubOpR(unnamed.as_raw_Matx_SubOp(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_SubOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Matx_TOp /usr/include/opencv2/core/matx.hpp:74
pub trait Matx_TOpTraitConst {
	fn as_raw_Matx_TOp(&self) -> *const c_void;

}

pub trait Matx_TOpTrait: core::Matx_TOpTraitConst {
	fn as_raw_mut_Matx_TOp(&mut self) -> *mut c_void;

}

// Matx_TOp /usr/include/opencv2/core/matx.hpp:74
pub struct Matx_TOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_TOp }

impl Drop for Matx_TOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_TOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_TOp_delete(self.as_raw_mut_Matx_TOp()) };
	}
}

unsafe impl Send for Matx_TOp {}

impl core::Matx_TOpTraitConst for Matx_TOp {
	#[inline] fn as_raw_Matx_TOp(&self) -> *const c_void { self.as_raw() }
}

impl core::Matx_TOpTrait for Matx_TOp {
	#[inline] fn as_raw_mut_Matx_TOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_TOp {
	// Matx_TOp() /usr/include/opencv2/core/matx.hpp:74
	#[inline]
	pub fn default() -> Result<core::Matx_TOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_TOp_Matx_TOp(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_TOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Matx_TOp(const cv::Matx_TOp &) /usr/include/opencv2/core/matx.hpp:74
	#[inline]
	pub fn copy(unnamed: &core::Matx_TOp) -> Result<core::Matx_TOp> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Matx_TOp_Matx_TOp_const_Matx_TOpR(unnamed.as_raw_Matx_TOp(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Matx_TOp::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// MinProblemSolver /usr/include/opencv2/core/optim.hpp:58
pub trait MinProblemSolverConst: core::AlgorithmTraitConst {
	fn as_raw_MinProblemSolver(&self) -> *const c_void;

	// getFunction() /usr/include/opencv2/core/optim.hpp:81
	#[inline]
	fn get_function(&self) -> Result<core::Ptr<dyn core::MinProblemSolver_Function>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MinProblemSolver_getFunction_const(self.as_raw_MinProblemSolver(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn core::MinProblemSolver_Function>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getTermCriteria() /usr/include/opencv2/core/optim.hpp:95
	#[inline]
	fn get_term_criteria(&self) -> Result<core::TermCriteria> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MinProblemSolver_getTermCriteria_const(self.as_raw_MinProblemSolver(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MinProblemSolver: core::AlgorithmTrait + core::MinProblemSolverConst {
	fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void;

	// setFunction(const Ptr<cv::MinProblemSolver::Function> &) /usr/include/opencv2/core/optim.hpp:89
	#[inline]
	fn set_function(&mut self, f: &core::Ptr<dyn core::MinProblemSolver_Function>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MinProblemSolver_setFunction_const_Ptr_Function_R(self.as_raw_mut_MinProblemSolver(), f.as_raw_PtrOfMinProblemSolver_Function(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/core/optim.hpp:108
	#[inline]
	fn set_term_criteria(&mut self, termcrit: core::TermCriteria) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MinProblemSolver_setTermCriteria_const_TermCriteriaR(self.as_raw_mut_MinProblemSolver(), &termcrit, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// minimize(cv::InputOutputArray) /usr/include/opencv2/core/optim.hpp:122
	#[inline]
	fn minimize(&mut self, x: &mut dyn core::ToInputOutputArray) -> Result<f64> {
		input_output_array_arg!(x);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MinProblemSolver_minimize_const__InputOutputArrayR(self.as_raw_mut_MinProblemSolver(), x.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Function /usr/include/opencv2/core/optim.hpp:63
pub trait MinProblemSolver_FunctionConst {
	fn as_raw_MinProblemSolver_Function(&self) -> *const c_void;

	// getDims() /usr/include/opencv2/core/optim.hpp:67
	#[inline]
	fn get_dims(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MinProblemSolver_Function_getDims_const(self.as_raw_MinProblemSolver_Function(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getGradientEps() /usr/include/opencv2/core/optim.hpp:68
	#[inline]
	fn get_gradient_eps(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MinProblemSolver_Function_getGradientEps_const(self.as_raw_MinProblemSolver_Function(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// calc(const double *) /usr/include/opencv2/core/optim.hpp:69
	#[inline]
	fn calc(&self, x: &f64) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MinProblemSolver_Function_calc_const_const_doubleX(self.as_raw_MinProblemSolver_Function(), x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MinProblemSolver_Function: core::MinProblemSolver_FunctionConst {
	fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void;

	// getGradient(const double *, double *) /usr/include/opencv2/core/optim.hpp:70
	#[inline]
	fn get_gradient(&mut self, x: &f64, grad: &mut f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MinProblemSolver_Function_getGradient_const_doubleX_doubleX(self.as_raw_mut_MinProblemSolver_Function(), x, grad, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Moments /usr/include/opencv2/core/types.hpp:948
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Moments {
	pub m00: f64,
	pub m10: f64,
	pub m01: f64,
	pub m20: f64,
	pub m11: f64,
	pub m02: f64,
	pub m30: f64,
	pub m21: f64,
	pub m12: f64,
	pub m03: f64,
	pub mu20: f64,
	pub mu11: f64,
	pub mu02: f64,
	pub mu30: f64,
	pub mu21: f64,
	pub mu12: f64,
	pub mu03: f64,
	pub nu20: f64,
	pub nu11: f64,
	pub nu02: f64,
	pub nu30: f64,
	pub nu21: f64,
	pub nu12: f64,
	pub nu03: f64,
}

opencv_type_simple! { core::Moments }

impl Moments {
	// Moments() /usr/include/opencv2/core/types.hpp:952
	#[inline]
	pub fn default() -> Result<core::Moments> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Moments_Moments(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// Moments(double, double, double, double, double, double, double, double, double, double) /usr/include/opencv2/core/types.hpp:954
	#[inline]
	pub fn new(m00: f64, m10: f64, m01: f64, m20: f64, m11: f64, m02: f64, m30: f64, m21: f64, m12: f64, m03: f64) -> Result<core::Moments> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Moments_Moments_double_double_double_double_double_double_double_double_double_double(m00, m10, m01, m20, m11, m02, m30, m21, m12, m03, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// PCA /usr/include/opencv2/core.hpp:2449
pub trait PCATraitConst {
	fn as_raw_PCA(&self) -> *const c_void;

	// eigenvectors /usr/include/opencv2/core.hpp:2588
	#[inline]
	fn eigenvectors(&self) -> core::Mat {
		let ret = unsafe { sys::cv_PCA_getPropEigenvectors_const(self.as_raw_PCA()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// eigenvalues /usr/include/opencv2/core.hpp:2589
	#[inline]
	fn eigenvalues(&self) -> core::Mat {
		let ret = unsafe { sys::cv_PCA_getPropEigenvalues_const(self.as_raw_PCA()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// mean /usr/include/opencv2/core.hpp:2590
	#[inline]
	fn mean(&self) -> core::Mat {
		let ret = unsafe { sys::cv_PCA_getPropMean_const(self.as_raw_PCA()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// project(cv::InputArray) /usr/include/opencv2/core.hpp:2536
	#[inline]
	fn project(&self, vec: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(vec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PCA_project_const_const__InputArrayR(self.as_raw_PCA(), vec.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// project(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2550
	#[inline]
	fn project_to(&self, vec: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(vec);
		output_array_arg!(result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PCA_project_const_const__InputArrayR_const__OutputArrayR(self.as_raw_PCA(), vec.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// backProject(cv::InputArray) /usr/include/opencv2/core.hpp:2565
	#[inline]
	fn back_project(&self, vec: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(vec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PCA_backProject_const_const__InputArrayR(self.as_raw_PCA(), vec.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// backProject(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2574
	#[inline]
	fn back_project_to(&self, vec: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(vec);
		output_array_arg!(result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PCA_backProject_const_const__InputArrayR_const__OutputArrayR(self.as_raw_PCA(), vec.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/core.hpp:2580
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PCA_write_const_FileStorageR(self.as_raw_PCA(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait PCATrait: core::PCATraitConst {
	fn as_raw_mut_PCA(&mut self) -> *mut c_void;

	// eigenvectors /usr/include/opencv2/core.hpp:2588
	#[inline]
	fn set_eigenvectors(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_PCA_setPropEigenvectors_Mat(self.as_raw_mut_PCA(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// eigenvalues /usr/include/opencv2/core.hpp:2589
	#[inline]
	fn set_eigenvalues(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_PCA_setPropEigenvalues_Mat(self.as_raw_mut_PCA(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// mean /usr/include/opencv2/core.hpp:2590
	#[inline]
	fn set_mean(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_PCA_setPropMean_Mat(self.as_raw_mut_PCA(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/core.hpp:2586
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PCA_read_const_FileNodeR(self.as_raw_mut_PCA(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// PCA /usr/include/opencv2/core.hpp:2449
pub struct PCA {
	ptr: *mut c_void
}

opencv_type_boxed! { PCA }

impl Drop for PCA {
	fn drop(&mut self) {
		extern "C" { fn cv_PCA_delete(instance: *mut c_void); }
		unsafe { cv_PCA_delete(self.as_raw_mut_PCA()) };
	}
}

unsafe impl Send for PCA {}

impl core::PCATraitConst for PCA {
	#[inline] fn as_raw_PCA(&self) -> *const c_void { self.as_raw() }
}

impl core::PCATrait for PCA {
	#[inline] fn as_raw_mut_PCA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PCA {
	// PCA() /usr/include/opencv2/core.hpp:2462
	#[inline]
	pub fn default() -> Result<core::PCA> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PCA_PCA(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::PCA::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * max_components: 0
	// PCA(cv::InputArray, cv::InputArray, int, int) /usr/include/opencv2/core.hpp:2473
	#[inline]
	pub fn new(data: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, flags: i32, max_components: i32) -> Result<core::PCA> {
		input_array_arg!(data);
		input_array_arg!(mean);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_int(data.as_raw__InputArray(), mean.as_raw__InputArray(), flags, max_components, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::PCA::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// PCA(cv::InputArray, cv::InputArray, int, double) /usr/include/opencv2/core.hpp:2485
	#[inline]
	pub fn new_with_variance(data: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, flags: i32, retained_variance: f64) -> Result<core::PCA> {
		input_array_arg!(data);
		input_array_arg!(mean);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_double(data.as_raw__InputArray(), mean.as_raw__InputArray(), flags, retained_variance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::PCA::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// ParallelLoopBody /usr/include/opencv2/core/utility.hpp:576
pub trait ParallelLoopBodyConst {
	fn as_raw_ParallelLoopBody(&self) -> *const c_void;

}

pub trait ParallelLoopBody: core::ParallelLoopBodyConst {
	fn as_raw_mut_ParallelLoopBody(&mut self) -> *mut c_void;

}

// RNG /usr/include/opencv2/core.hpp:2825
pub trait RNGTraitConst {
	fn as_raw_RNG(&self) -> *const c_void;

	// state /usr/include/opencv2/core.hpp:2972
	#[inline]
	fn state(&self) -> u64 {
		let ret = unsafe { sys::cv_RNG_getPropState_const(self.as_raw_RNG()) };
		ret
	}
	
	// operator==(const cv::RNG &) /usr/include/opencv2/core.hpp:2974
	#[inline]
	fn equals(&self, other: &core::RNG) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_operatorEQ_const_const_RNGR(self.as_raw_RNG(), other.as_raw_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RNGTrait: core::RNGTraitConst {
	fn as_raw_mut_RNG(&mut self) -> *mut c_void;

	// state /usr/include/opencv2/core.hpp:2972
	#[inline]
	fn set_state(&mut self, val: u64) {
		let ret = unsafe { sys::cv_RNG_setPropState_uint64_t(self.as_raw_mut_RNG(), val) };
		ret
	}
	
	// next() /usr/include/opencv2/core.hpp:2847
	#[inline]
	fn next(&mut self) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_next(self.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator unsigned char() /usr/include/opencv2/core.hpp:2855
	#[inline]
	fn to_u8(&mut self) -> Result<u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_operator_unsigned_char(self.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator signed char() /usr/include/opencv2/core.hpp:2857
	#[inline]
	fn to_i8(&mut self) -> Result<i8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_operator_signed_char(self.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator unsigned short() /usr/include/opencv2/core.hpp:2859
	#[inline]
	fn to_u16(&mut self) -> Result<u16> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_operator_unsigned_short(self.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator short() /usr/include/opencv2/core.hpp:2861
	#[inline]
	fn to_i16(&mut self) -> Result<i16> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_operator_short(self.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator unsigned int() /usr/include/opencv2/core.hpp:2863
	#[inline]
	fn to_u32(&mut self) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_operator_unsigned_int(self.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator int() /usr/include/opencv2/core.hpp:2865
	#[inline]
	fn to_i32(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_operator_int(self.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator float() /usr/include/opencv2/core.hpp:2867
	#[inline]
	fn to_f32(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_operator_float(self.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator double() /usr/include/opencv2/core.hpp:2869
	#[inline]
	fn to_f64(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_operator_double(self.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// uniform(int, int) /usr/include/opencv2/core.hpp:2920
	#[inline]
	fn uniform(&mut self, a: i32, b: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_uniform_int_int(self.as_raw_mut_RNG(), a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// uniform(float, float) /usr/include/opencv2/core.hpp:2922
	#[inline]
	fn uniform_1(&mut self, a: f32, b: f32) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_uniform_float_float(self.as_raw_mut_RNG(), a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// uniform(double, double) /usr/include/opencv2/core.hpp:2924
	#[inline]
	fn uniform_2(&mut self, a: f64, b: f64) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_uniform_double_double(self.as_raw_mut_RNG(), a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * saturate_range: false
	// fill(cv::InputOutputArray, int, cv::InputArray, cv::InputArray, bool) /usr/include/opencv2/core.hpp:2960
	#[inline]
	fn fill(&mut self, mat: &mut dyn core::ToInputOutputArray, dist_type: i32, a: &dyn core::ToInputArray, b: &dyn core::ToInputArray, saturate_range: bool) -> Result<()> {
		input_output_array_arg!(mat);
		input_array_arg!(a);
		input_array_arg!(b);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_fill_const__InputOutputArrayR_int_const__InputArrayR_const__InputArrayR_bool(self.as_raw_mut_RNG(), mat.as_raw__InputOutputArray(), dist_type, a.as_raw__InputArray(), b.as_raw__InputArray(), saturate_range, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// gaussian(double) /usr/include/opencv2/core.hpp:2970
	#[inline]
	fn gaussian(&mut self, sigma: f64) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_gaussian_double(self.as_raw_mut_RNG(), sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// RNG /usr/include/opencv2/core.hpp:2825
pub struct RNG {
	ptr: *mut c_void
}

opencv_type_boxed! { RNG }

impl Drop for RNG {
	fn drop(&mut self) {
		extern "C" { fn cv_RNG_delete(instance: *mut c_void); }
		unsafe { cv_RNG_delete(self.as_raw_mut_RNG()) };
	}
}

unsafe impl Send for RNG {}

impl core::RNGTraitConst for RNG {
	#[inline] fn as_raw_RNG(&self) -> *const c_void { self.as_raw() }
}

impl core::RNGTrait for RNG {
	#[inline] fn as_raw_mut_RNG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RNG {
	// RNG() /usr/include/opencv2/core.hpp:2840
	#[inline]
	pub fn default() -> Result<core::RNG> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_RNG(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::RNG::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// RNG(uint64) /usr/include/opencv2/core.hpp:2844
	#[inline]
	pub fn new(state: u64) -> Result<core::RNG> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_RNG_uint64_t(state, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::RNG::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// RNG_MT19937 /usr/include/opencv2/core.hpp:2982
pub trait RNG_MT19937TraitConst {
	fn as_raw_RNG_MT19937(&self) -> *const c_void;

}

pub trait RNG_MT19937Trait: core::RNG_MT19937TraitConst {
	fn as_raw_mut_RNG_MT19937(&mut self) -> *mut c_void;

	// seed(unsigned int) /usr/include/opencv2/core.hpp:2987
	#[inline]
	fn seed(&mut self, s: u32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_seed_unsigned_int(self.as_raw_mut_RNG_MT19937(), s, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// next() /usr/include/opencv2/core.hpp:2989
	#[inline]
	fn next(&mut self) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_next(self.as_raw_mut_RNG_MT19937(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator int() /usr/include/opencv2/core.hpp:2991
	#[inline]
	fn to_i32(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_operator_int(self.as_raw_mut_RNG_MT19937(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator unsigned int() /usr/include/opencv2/core.hpp:2992
	#[inline]
	fn to_u32(&mut self) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_operator_unsigned_int(self.as_raw_mut_RNG_MT19937(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator float() /usr/include/opencv2/core.hpp:2993
	#[inline]
	fn to_f32(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_operator_float(self.as_raw_mut_RNG_MT19937(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// operator double() /usr/include/opencv2/core.hpp:2994
	#[inline]
	fn to_f64(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_operator_double(self.as_raw_mut_RNG_MT19937(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// uniform(int, int) /usr/include/opencv2/core.hpp:3000
	#[inline]
	fn uniform(&mut self, a: i32, b: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_uniform_int_int(self.as_raw_mut_RNG_MT19937(), a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// uniform(float, float) /usr/include/opencv2/core.hpp:3002
	#[inline]
	fn uniform_1(&mut self, a: f32, b: f32) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_uniform_float_float(self.as_raw_mut_RNG_MT19937(), a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// uniform(double, double) /usr/include/opencv2/core.hpp:3004
	#[inline]
	fn uniform_2(&mut self, a: f64, b: f64) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_uniform_double_double(self.as_raw_mut_RNG_MT19937(), a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// RNG_MT19937 /usr/include/opencv2/core.hpp:2982
pub struct RNG_MT19937 {
	ptr: *mut c_void
}

opencv_type_boxed! { RNG_MT19937 }

impl Drop for RNG_MT19937 {
	fn drop(&mut self) {
		extern "C" { fn cv_RNG_MT19937_delete(instance: *mut c_void); }
		unsafe { cv_RNG_MT19937_delete(self.as_raw_mut_RNG_MT19937()) };
	}
}

unsafe impl Send for RNG_MT19937 {}

impl core::RNG_MT19937TraitConst for RNG_MT19937 {
	#[inline] fn as_raw_RNG_MT19937(&self) -> *const c_void { self.as_raw() }
}

impl core::RNG_MT19937Trait for RNG_MT19937 {
	#[inline] fn as_raw_mut_RNG_MT19937(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RNG_MT19937 {
	// RNG_MT19937() /usr/include/opencv2/core.hpp:2985
	#[inline]
	pub fn default() -> Result<core::RNG_MT19937> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_RNG_MT19937(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::RNG_MT19937::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// RNG_MT19937(unsigned int) /usr/include/opencv2/core.hpp:2986
	#[inline]
	pub fn new(s: u32) -> Result<core::RNG_MT19937> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RNG_MT19937_RNG_MT19937_unsigned_int(s, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::RNG_MT19937::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Range /usr/include/opencv2/core/types.hpp:611
pub trait RangeTraitConst {
	fn as_raw_Range(&self) -> *const c_void;

	// start /usr/include/opencv2/core/types.hpp:620
	#[inline]
	fn start(&self) -> i32 {
		let ret = unsafe { sys::cv_Range_getPropStart_const(self.as_raw_Range()) };
		ret
	}
	
	// end /usr/include/opencv2/core/types.hpp:620
	#[inline]
	fn end(&self) -> i32 {
		let ret = unsafe { sys::cv_Range_getPropEnd_const(self.as_raw_Range()) };
		ret
	}
	
	// size() /usr/include/opencv2/core/types.hpp:616
	#[inline]
	fn size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Range_size_const(self.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/types.hpp:617
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Range_empty_const(self.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RangeTrait: core::RangeTraitConst {
	fn as_raw_mut_Range(&mut self) -> *mut c_void;

	// start /usr/include/opencv2/core/types.hpp:620
	#[inline]
	fn set_start(&mut self, val: i32) {
		let ret = unsafe { sys::cv_Range_setPropStart_int(self.as_raw_mut_Range(), val) };
		ret
	}
	
	// end /usr/include/opencv2/core/types.hpp:620
	#[inline]
	fn set_end(&mut self, val: i32) {
		let ret = unsafe { sys::cv_Range_setPropEnd_int(self.as_raw_mut_Range(), val) };
		ret
	}
	
}

// Range /usr/include/opencv2/core/types.hpp:611
pub struct Range {
	ptr: *mut c_void
}

opencv_type_boxed! { Range }

impl Drop for Range {
	fn drop(&mut self) {
		extern "C" { fn cv_Range_delete(instance: *mut c_void); }
		unsafe { cv_Range_delete(self.as_raw_mut_Range()) };
	}
}

unsafe impl Send for Range {}

impl core::RangeTraitConst for Range {
	#[inline] fn as_raw_Range(&self) -> *const c_void { self.as_raw() }
}

impl core::RangeTrait for Range {
	#[inline] fn as_raw_mut_Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Range {
	// Range() /usr/include/opencv2/core/types.hpp:614
	#[inline]
	pub fn default() -> Result<core::Range> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Range_Range(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Range::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Range(int, int) /usr/include/opencv2/core/types.hpp:615
	#[inline]
	pub fn new(_start: i32, _end: i32) -> Result<core::Range> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Range_Range_int_int(_start, _end, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Range::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// all() /usr/include/opencv2/core/types.hpp:618
	#[inline]
	pub fn all() -> Result<core::Range> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Range_all(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Range::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// RotatedRect /usr/include/opencv2/core/types.hpp:525
pub trait RotatedRectTraitConst {
	fn as_raw_RotatedRect(&self) -> *const c_void;

	// center /usr/include/opencv2/core/types.hpp:552
	#[inline]
	fn center(&self) -> core::Point2f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RotatedRect_getPropCenter_const(self.as_raw_RotatedRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// size /usr/include/opencv2/core/types.hpp:554
	#[inline]
	fn size(&self) -> core::Size2f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RotatedRect_getPropSize_const(self.as_raw_RotatedRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// angle /usr/include/opencv2/core/types.hpp:556
	#[inline]
	fn angle(&self) -> f32 {
		let ret = unsafe { sys::cv_RotatedRect_getPropAngle_const(self.as_raw_RotatedRect()) };
		ret
	}
	
	// points(cv::Point2f *) /usr/include/opencv2/core/types.hpp:546
	#[inline]
	fn points(&self, pts: &mut [core::Point2f]) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RotatedRect_points_const_Point2fX(self.as_raw_RotatedRect(), pts.as_mut_ptr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// boundingRect() /usr/include/opencv2/core/types.hpp:548
	#[inline]
	fn bounding_rect(&self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RotatedRect_boundingRect_const(self.as_raw_RotatedRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// boundingRect2f() /usr/include/opencv2/core/types.hpp:550
	#[inline]
	fn bounding_rect2f(&self) -> Result<core::Rect_<f32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RotatedRect_boundingRect2f_const(self.as_raw_RotatedRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RotatedRectTrait: core::RotatedRectTraitConst {
	fn as_raw_mut_RotatedRect(&mut self) -> *mut c_void;

	// center /usr/include/opencv2/core/types.hpp:552
	#[inline]
	fn set_center(&mut self, val: core::Point2f) {
		let ret = unsafe { sys::cv_RotatedRect_setPropCenter_Point2f(self.as_raw_mut_RotatedRect(), val.opencv_as_extern()) };
		ret
	}
	
	// size /usr/include/opencv2/core/types.hpp:554
	#[inline]
	fn set_size(&mut self, val: core::Size2f) {
		let ret = unsafe { sys::cv_RotatedRect_setPropSize_Size2f(self.as_raw_mut_RotatedRect(), val.opencv_as_extern()) };
		ret
	}
	
	// angle /usr/include/opencv2/core/types.hpp:556
	#[inline]
	fn set_angle(&mut self, val: f32) {
		let ret = unsafe { sys::cv_RotatedRect_setPropAngle_float(self.as_raw_mut_RotatedRect(), val) };
		ret
	}
	
}

// RotatedRect /usr/include/opencv2/core/types.hpp:525
pub struct RotatedRect {
	ptr: *mut c_void
}

opencv_type_boxed! { RotatedRect }

impl Drop for RotatedRect {
	fn drop(&mut self) {
		extern "C" { fn cv_RotatedRect_delete(instance: *mut c_void); }
		unsafe { cv_RotatedRect_delete(self.as_raw_mut_RotatedRect()) };
	}
}

unsafe impl Send for RotatedRect {}

impl core::RotatedRectTraitConst for RotatedRect {
	#[inline] fn as_raw_RotatedRect(&self) -> *const c_void { self.as_raw() }
}

impl core::RotatedRectTrait for RotatedRect {
	#[inline] fn as_raw_mut_RotatedRect(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RotatedRect {
	// RotatedRect() /usr/include/opencv2/core/types.hpp:529
	#[inline]
	pub fn default() -> Result<core::RotatedRect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RotatedRect_RotatedRect(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::RotatedRect::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// RotatedRect(const cv::Point2f &, const cv::Size2f &, float) /usr/include/opencv2/core/types.hpp:536
	#[inline]
	pub fn new(center: core::Point2f, size: core::Size2f, angle: f32) -> Result<core::RotatedRect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RotatedRect_RotatedRect_const_Point2fR_const_Size2fR_float(&center, &size, angle, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::RotatedRect::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// RotatedRect(const cv::Point2f &, const cv::Point2f &, const cv::Point2f &) /usr/include/opencv2/core/types.hpp:541
	#[inline]
	pub fn for_points(point1: core::Point2f, point2: core::Point2f, point3: core::Point2f) -> Result<core::RotatedRect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RotatedRect_RotatedRect_const_Point2fR_const_Point2fR_const_Point2fR(&point1, &point2, &point3, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::RotatedRect::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// SVD /usr/include/opencv2/core.hpp:2686
pub trait SVDTraitConst {
	fn as_raw_SVD(&self) -> *const c_void;

	// u /usr/include/opencv2/core.hpp:2810
	#[inline]
	fn u(&self) -> core::Mat {
		let ret = unsafe { sys::cv_SVD_getPropU_const(self.as_raw_SVD()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// w /usr/include/opencv2/core.hpp:2810
	#[inline]
	fn w(&self) -> core::Mat {
		let ret = unsafe { sys::cv_SVD_getPropW_const(self.as_raw_SVD()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// vt /usr/include/opencv2/core.hpp:2810
	#[inline]
	fn vt(&self) -> core::Mat {
		let ret = unsafe { sys::cv_SVD_getPropVt_const(self.as_raw_SVD()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// backSubst(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2796
	#[inline]
	fn back_subst(&self, rhs: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(rhs);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SVD_backSubst_const_const__InputArrayR_const__OutputArrayR(self.as_raw_SVD(), rhs.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait SVDTrait: core::SVDTraitConst {
	fn as_raw_mut_SVD(&mut self) -> *mut c_void;

	// u /usr/include/opencv2/core.hpp:2810
	#[inline]
	fn set_u(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_SVD_setPropU_Mat(self.as_raw_mut_SVD(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// w /usr/include/opencv2/core.hpp:2810
	#[inline]
	fn set_w(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_SVD_setPropW_Mat(self.as_raw_mut_SVD(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// vt /usr/include/opencv2/core.hpp:2810
	#[inline]
	fn set_vt(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_SVD_setPropVt_Mat(self.as_raw_mut_SVD(), val.as_raw_mut_Mat()) };
		ret
	}
	
}

// SVD /usr/include/opencv2/core.hpp:2686
pub struct SVD {
	ptr: *mut c_void
}

opencv_type_boxed! { SVD }

impl Drop for SVD {
	fn drop(&mut self) {
		extern "C" { fn cv_SVD_delete(instance: *mut c_void); }
		unsafe { cv_SVD_delete(self.as_raw_mut_SVD()) };
	}
}

unsafe impl Send for SVD {}

impl core::SVDTraitConst for SVD {
	#[inline] fn as_raw_SVD(&self) -> *const c_void { self.as_raw() }
}

impl core::SVDTrait for SVD {
	#[inline] fn as_raw_mut_SVD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SVD {
	// SVD() /usr/include/opencv2/core.hpp:2706
	#[inline]
	pub fn default() -> Result<core::SVD> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SVD_SVD(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SVD::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flags: 0
	// SVD(cv::InputArray, int) /usr/include/opencv2/core.hpp:2713
	#[inline]
	pub fn new(src: &dyn core::ToInputArray, flags: i32) -> Result<core::SVD> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SVD_SVD_const__InputArrayR_int(src.as_raw__InputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SVD::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flags: 0
	// compute(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2745
	#[inline]
	pub fn compute_ext(src: &dyn core::ToInputArray, w: &mut dyn core::ToOutputArray, u: &mut dyn core::ToOutputArray, vt: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(w);
		output_array_arg!(u);
		output_array_arg!(vt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), w.as_raw__OutputArray(), u.as_raw__OutputArray(), vt.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flags: 0
	// compute(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2754
	#[inline]
	pub fn compute(src: &dyn core::ToInputArray, w: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(w);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SVD_compute_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), w.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// backSubst(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2758
	#[inline]
	pub fn back_subst_multi(w: &dyn core::ToInputArray, u: &dyn core::ToInputArray, vt: &dyn core::ToInputArray, rhs: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(w);
		input_array_arg!(u);
		input_array_arg!(vt);
		input_array_arg!(rhs);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SVD_backSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(w.as_raw__InputArray(), u.as_raw__InputArray(), vt.as_raw__InputArray(), rhs.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// solveZ(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2772
	#[inline]
	pub fn solve_z(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SVD_solveZ_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SparseMat /usr/include/opencv2/core/mat.hpp:2712
pub trait SparseMatTraitConst {
	fn as_raw_SparseMat(&self) -> *const c_void;

	// flags /usr/include/opencv2/core/mat.hpp:2971
	#[inline]
	fn flags(&self) -> i32 {
		let ret = unsafe { sys::cv_SparseMat_getPropFlags_const(self.as_raw_SparseMat()) };
		ret
	}
	
	// clone() /usr/include/opencv2/core/mat.hpp:2779
	#[inline]
	#[must_use]
	fn try_clone(&self) -> Result<core::SparseMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_clone_const(self.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// copyTo(cv::SparseMat &) /usr/include/opencv2/core/mat.hpp:2782
	#[inline]
	fn copy_to(&self, m: &mut core::SparseMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_copyTo_const_SparseMatR(self.as_raw_SparseMat(), m.as_raw_mut_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyTo(cv::Mat &) /usr/include/opencv2/core/mat.hpp:2784
	#[inline]
	fn copy_to_mat(&self, m: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_copyTo_const_MatR(self.as_raw_SparseMat(), m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * alpha: 1
	// convertTo(cv::SparseMat &, int, double) /usr/include/opencv2/core/mat.hpp:2786
	#[inline]
	fn convert_to(&self, m: &mut core::SparseMat, rtype: i32, alpha: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_convertTo_const_SparseMatR_int_double(self.as_raw_SparseMat(), m.as_raw_mut_SparseMat(), rtype, alpha, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * alpha: 1
	/// * beta: 0
	// convertTo(cv::Mat &, int, double, double) /usr/include/opencv2/core/mat.hpp:2797
	#[inline]
	fn convert_to_1(&self, m: &mut core::Mat, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_convertTo_const_MatR_int_double_double(self.as_raw_SparseMat(), m.as_raw_mut_Mat(), rtype, alpha, beta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * typ: -1
	// assignTo(cv::SparseMat &, int) /usr/include/opencv2/core/mat.hpp:2800
	#[inline]
	fn assign_to(&self, m: &mut core::SparseMat, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_assignTo_const_SparseMatR_int(self.as_raw_SparseMat(), m.as_raw_mut_SparseMat(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize() /usr/include/opencv2/core/mat.hpp:2819
	#[inline]
	fn elem_size(&self) -> size_t {
		let ret = unsafe { sys::cv_SparseMat_elemSize_const(self.as_raw_SparseMat()) };
		ret
	}
	
	// elemSize1() /usr/include/opencv2/core/mat.hpp:2821
	#[inline]
	fn elem_size1(&self) -> size_t {
		let ret = unsafe { sys::cv_SparseMat_elemSize1_const(self.as_raw_SparseMat()) };
		ret
	}
	
	// type() /usr/include/opencv2/core/mat.hpp:2824
	#[inline]
	fn typ(&self) -> i32 {
		let ret = unsafe { sys::cv_SparseMat_type_const(self.as_raw_SparseMat()) };
		ret
	}
	
	// depth() /usr/include/opencv2/core/mat.hpp:2826
	#[inline]
	fn depth(&self) -> i32 {
		let ret = unsafe { sys::cv_SparseMat_depth_const(self.as_raw_SparseMat()) };
		ret
	}
	
	// channels() /usr/include/opencv2/core/mat.hpp:2828
	#[inline]
	fn channels(&self) -> i32 {
		let ret = unsafe { sys::cv_SparseMat_channels_const(self.as_raw_SparseMat()) };
		ret
	}
	
	// size() /usr/include/opencv2/core/mat.hpp:2831
	#[inline]
	fn size(&self) -> Result<*const i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_size_const(self.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// size(int) /usr/include/opencv2/core/mat.hpp:2833
	#[inline]
	fn size_1(&self, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_size_const_int(self.as_raw_SparseMat(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// dims() /usr/include/opencv2/core/mat.hpp:2835
	#[inline]
	fn dims(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_dims_const(self.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// nzcount() /usr/include/opencv2/core/mat.hpp:2837
	#[inline]
	fn nzcount(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_nzcount_const(self.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hash(int) /usr/include/opencv2/core/mat.hpp:2840
	#[inline]
	fn hash(&self, i0: i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_hash_const_int(self.as_raw_SparseMat(), i0, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hash(int, int) /usr/include/opencv2/core/mat.hpp:2842
	#[inline]
	fn hash_1(&self, i0: i32, i1: i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_hash_const_int_int(self.as_raw_SparseMat(), i0, i1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hash(int, int, int) /usr/include/opencv2/core/mat.hpp:2844
	#[inline]
	fn hash_2(&self, i0: i32, i1: i32, i2: i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_hash_const_int_int_int(self.as_raw_SparseMat(), i0, i1, i2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hash(const int *) /usr/include/opencv2/core/mat.hpp:2846
	#[inline]
	fn hash_3(&self, idx: &i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_hash_const_const_intX(self.as_raw_SparseMat(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// begin() /usr/include/opencv2/core/mat.hpp:2942
	#[inline]
	fn begin(&self) -> Result<core::SparseMatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_begin_const(self.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// end() /usr/include/opencv2/core/mat.hpp:2952
	#[inline]
	fn end(&self) -> Result<core::SparseMatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_end_const(self.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// node(size_t) /usr/include/opencv2/core/mat.hpp:2965
	#[inline]
	fn node(&self, nidx: size_t) -> Result<core::SparseMat_Node> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_node_const_size_t(self.as_raw_SparseMat(), nidx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat_Node::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait SparseMatTrait: core::SparseMatTraitConst {
	fn as_raw_mut_SparseMat(&mut self) -> *mut c_void;

	// flags /usr/include/opencv2/core/mat.hpp:2971
	#[inline]
	fn set_flags(&mut self, val: i32) {
		let ret = unsafe { sys::cv_SparseMat_setPropFlags_int(self.as_raw_mut_SparseMat(), val) };
		ret
	}
	
	// hdr /usr/include/opencv2/core/mat.hpp:2972
	#[inline]
	fn hdr(&mut self) -> core::SparseMat_Hdr {
		let ret = unsafe { sys::cv_SparseMat_getPropHdr(self.as_raw_mut_SparseMat()) };
		let ret = unsafe { core::SparseMat_Hdr::opencv_from_extern(ret) };
		ret
	}
	
	// hdr /usr/include/opencv2/core/mat.hpp:2972
	#[inline]
	fn set_hdr(&mut self, val: &mut core::SparseMat_Hdr) {
		let ret = unsafe { sys::cv_SparseMat_setPropHdr_HdrX(self.as_raw_mut_SparseMat(), val.as_raw_mut_SparseMat_Hdr()) };
		ret
	}
	
	// create(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2808
	#[inline]
	fn create(&mut self, dims: i32, _sizes: &i32, _type: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_create_int_const_intX_int(self.as_raw_mut_SparseMat(), dims, _sizes, _type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clear() /usr/include/opencv2/core/mat.hpp:2810
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_clear(self.as_raw_mut_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// addref() /usr/include/opencv2/core/mat.hpp:2812
	#[inline]
	fn addref(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_addref(self.as_raw_mut_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/mat.hpp:2814
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_release(self.as_raw_mut_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * hashval: 0
	// ptr(int, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2860
	#[inline]
	fn ptr(&mut self, i0: i32, create_missing: bool, hashval: &mut size_t) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_ptr_int_bool_size_tX(self.as_raw_mut_SparseMat(), i0, create_missing, hashval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * hashval: 0
	// ptr(int, int, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2862
	#[inline]
	fn ptr_1(&mut self, i0: i32, i1: i32, create_missing: bool, hashval: &mut size_t) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_ptr_int_int_bool_size_tX(self.as_raw_mut_SparseMat(), i0, i1, create_missing, hashval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * hashval: 0
	// ptr(int, int, int, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2864
	#[inline]
	fn ptr_2(&mut self, i0: i32, i1: i32, i2: i32, create_missing: bool, hashval: &mut size_t) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_ptr_int_int_int_bool_size_tX(self.as_raw_mut_SparseMat(), i0, i1, i2, create_missing, hashval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * hashval: 0
	// ptr(const int *, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2866
	#[inline]
	fn ptr_3(&mut self, idx: &i32, create_missing: bool, hashval: &mut size_t) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_ptr_const_intX_bool_size_tX(self.as_raw_mut_SparseMat(), idx, create_missing, hashval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * hashval: 0
	// erase(int, int, size_t *) /usr/include/opencv2/core/mat.hpp:2927
	#[inline]
	fn erase(&mut self, i0: i32, i1: i32, hashval: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_erase_int_int_size_tX(self.as_raw_mut_SparseMat(), i0, i1, hashval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * hashval: 0
	// erase(int, int, int, size_t *) /usr/include/opencv2/core/mat.hpp:2929
	#[inline]
	fn erase_1(&mut self, i0: i32, i1: i32, i2: i32, hashval: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_erase_int_int_int_size_tX(self.as_raw_mut_SparseMat(), i0, i1, i2, hashval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * hashval: 0
	// erase(const int *, size_t *) /usr/include/opencv2/core/mat.hpp:2931
	#[inline]
	fn erase_2(&mut self, idx: &i32, hashval: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_erase_const_intX_size_tX(self.as_raw_mut_SparseMat(), idx, hashval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// begin() /usr/include/opencv2/core/mat.hpp:2938
	#[inline]
	fn begin_mut(&mut self) -> Result<core::SparseMatIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_begin(self.as_raw_mut_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// end() /usr/include/opencv2/core/mat.hpp:2950
	#[inline]
	fn end_mut(&mut self) -> Result<core::SparseMatIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_end(self.as_raw_mut_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// node(size_t) /usr/include/opencv2/core/mat.hpp:2964
	#[inline]
	fn node_1(&mut self, nidx: size_t) -> Result<core::SparseMat_Node> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_node_size_t(self.as_raw_mut_SparseMat(), nidx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat_Node::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// newNode(const int *, size_t) /usr/include/opencv2/core/mat.hpp:2967
	#[inline]
	fn new_node(&mut self, idx: &i32, hashval: size_t) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_newNode_const_intX_size_t(self.as_raw_mut_SparseMat(), idx, hashval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// removeNode(size_t, size_t, size_t) /usr/include/opencv2/core/mat.hpp:2968
	#[inline]
	fn remove_node(&mut self, hidx: size_t, nidx: size_t, previdx: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_removeNode_size_t_size_t_size_t(self.as_raw_mut_SparseMat(), hidx, nidx, previdx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// resizeHashTab(size_t) /usr/include/opencv2/core/mat.hpp:2969
	#[inline]
	fn resize_hash_tab(&mut self, newsize: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_resizeHashTab_size_t(self.as_raw_mut_SparseMat(), newsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SparseMat /usr/include/opencv2/core/mat.hpp:2712
pub struct SparseMat {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMat }

impl Drop for SparseMat {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMat_delete(instance: *mut c_void); }
		unsafe { cv_SparseMat_delete(self.as_raw_mut_SparseMat()) };
	}
}

unsafe impl Send for SparseMat {}

impl core::SparseMatTraitConst for SparseMat {
	#[inline] fn as_raw_SparseMat(&self) -> *const c_void { self.as_raw() }
}

impl core::SparseMatTrait for SparseMat {
	#[inline] fn as_raw_mut_SparseMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMat {
	// SparseMat() /usr/include/opencv2/core/mat.hpp:2749
	#[inline]
	pub fn default() -> Result<core::SparseMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_SparseMat(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// SparseMat(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2756
	#[inline]
	pub fn new(dims: i32, _sizes: &i32, _type: i32) -> Result<core::SparseMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_SparseMat_int_const_intX_int(dims, _sizes, _type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// SparseMat(const cv::SparseMat &) /usr/include/opencv2/core/mat.hpp:2762
	#[inline]
	pub fn copy(m: &core::SparseMat) -> Result<core::SparseMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_SparseMat_const_SparseMatR(m.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// SparseMat(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:2768
	#[inline]
	pub fn from_mat(m: &core::Mat) -> Result<core::SparseMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_SparseMat_const_MatR(m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Clone for SparseMat {
	#[inline]
	/// Calls try_clone() and panics if that fails
	fn clone(&self) -> Self {
		self.try_clone().expect("Cannot clone SparseMat")
	}
}

// Hdr /usr/include/opencv2/core/mat.hpp:2721
pub trait SparseMat_HdrTraitConst {
	fn as_raw_SparseMat_Hdr(&self) -> *const c_void;

	// refcount /usr/include/opencv2/core/mat.hpp:2725
	#[inline]
	fn refcount(&self) -> i32 {
		let ret = unsafe { sys::cv_SparseMat_Hdr_getPropRefcount_const(self.as_raw_SparseMat_Hdr()) };
		ret
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2726
	#[inline]
	fn dims(&self) -> i32 {
		let ret = unsafe { sys::cv_SparseMat_Hdr_getPropDims_const(self.as_raw_SparseMat_Hdr()) };
		ret
	}
	
	// valueOffset /usr/include/opencv2/core/mat.hpp:2727
	#[inline]
	fn value_offset(&self) -> i32 {
		let ret = unsafe { sys::cv_SparseMat_Hdr_getPropValueOffset_const(self.as_raw_SparseMat_Hdr()) };
		ret
	}
	
	// nodeSize /usr/include/opencv2/core/mat.hpp:2728
	#[inline]
	fn node_size(&self) -> size_t {
		let ret = unsafe { sys::cv_SparseMat_Hdr_getPropNodeSize_const(self.as_raw_SparseMat_Hdr()) };
		ret
	}
	
	// nodeCount /usr/include/opencv2/core/mat.hpp:2729
	#[inline]
	fn node_count(&self) -> size_t {
		let ret = unsafe { sys::cv_SparseMat_Hdr_getPropNodeCount_const(self.as_raw_SparseMat_Hdr()) };
		ret
	}
	
	// freeList /usr/include/opencv2/core/mat.hpp:2730
	#[inline]
	fn free_list(&self) -> size_t {
		let ret = unsafe { sys::cv_SparseMat_Hdr_getPropFreeList_const(self.as_raw_SparseMat_Hdr()) };
		ret
	}
	
	// pool /usr/include/opencv2/core/mat.hpp:2731
	#[inline]
	fn pool(&self) -> core::Vector<u8> {
		let ret = unsafe { sys::cv_SparseMat_Hdr_getPropPool_const(self.as_raw_SparseMat_Hdr()) };
		let ret = unsafe { core::Vector::<u8>::opencv_from_extern(ret) };
		ret
	}
	
	// hashtab /usr/include/opencv2/core/mat.hpp:2732
	#[inline]
	fn hashtab(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_SparseMat_Hdr_getPropHashtab_const(self.as_raw_SparseMat_Hdr()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait SparseMat_HdrTrait: core::SparseMat_HdrTraitConst {
	fn as_raw_mut_SparseMat_Hdr(&mut self) -> *mut c_void;

	// refcount /usr/include/opencv2/core/mat.hpp:2725
	#[inline]
	fn set_refcount(&mut self, val: i32) {
		let ret = unsafe { sys::cv_SparseMat_Hdr_setPropRefcount_int(self.as_raw_mut_SparseMat_Hdr(), val) };
		ret
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2726
	#[inline]
	fn set_dims(&mut self, val: i32) {
		let ret = unsafe { sys::cv_SparseMat_Hdr_setPropDims_int(self.as_raw_mut_SparseMat_Hdr(), val) };
		ret
	}
	
	// valueOffset /usr/include/opencv2/core/mat.hpp:2727
	#[inline]
	fn set_value_offset(&mut self, val: i32) {
		let ret = unsafe { sys::cv_SparseMat_Hdr_setPropValueOffset_int(self.as_raw_mut_SparseMat_Hdr(), val) };
		ret
	}
	
	// nodeSize /usr/include/opencv2/core/mat.hpp:2728
	#[inline]
	fn set_node_size(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_SparseMat_Hdr_setPropNodeSize_size_t(self.as_raw_mut_SparseMat_Hdr(), val) };
		ret
	}
	
	// nodeCount /usr/include/opencv2/core/mat.hpp:2729
	#[inline]
	fn set_node_count(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_SparseMat_Hdr_setPropNodeCount_size_t(self.as_raw_mut_SparseMat_Hdr(), val) };
		ret
	}
	
	// freeList /usr/include/opencv2/core/mat.hpp:2730
	#[inline]
	fn set_free_list(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_SparseMat_Hdr_setPropFreeList_size_t(self.as_raw_mut_SparseMat_Hdr(), val) };
		ret
	}
	
	// pool /usr/include/opencv2/core/mat.hpp:2731
	#[inline]
	fn set_pool(&mut self, mut val: core::Vector<u8>) {
		let ret = unsafe { sys::cv_SparseMat_Hdr_setPropPool_vector_unsigned_char_(self.as_raw_mut_SparseMat_Hdr(), val.as_raw_mut_VectorOfu8()) };
		ret
	}
	
	// hashtab /usr/include/opencv2/core/mat.hpp:2732
	#[inline]
	fn set_hashtab(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_SparseMat_Hdr_setPropHashtab_vector_size_t_(self.as_raw_mut_SparseMat_Hdr(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	// size /usr/include/opencv2/core/mat.hpp:2733
	#[inline]
	fn size(&mut self) -> &mut [i32; 32] {
		let ret = unsafe { sys::cv_SparseMat_Hdr_getPropSize(self.as_raw_mut_SparseMat_Hdr()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}
	
	// clear() /usr/include/opencv2/core/mat.hpp:2724
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_Hdr_clear(self.as_raw_mut_SparseMat_Hdr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Hdr /usr/include/opencv2/core/mat.hpp:2721
pub struct SparseMat_Hdr {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMat_Hdr }

impl Drop for SparseMat_Hdr {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMat_Hdr_delete(instance: *mut c_void); }
		unsafe { cv_SparseMat_Hdr_delete(self.as_raw_mut_SparseMat_Hdr()) };
	}
}

unsafe impl Send for SparseMat_Hdr {}

impl core::SparseMat_HdrTraitConst for SparseMat_Hdr {
	#[inline] fn as_raw_SparseMat_Hdr(&self) -> *const c_void { self.as_raw() }
}

impl core::SparseMat_HdrTrait for SparseMat_Hdr {
	#[inline] fn as_raw_mut_SparseMat_Hdr(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMat_Hdr {
	// Hdr(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2723
	#[inline]
	pub fn new(_sizes: &[i32], _type: i32) -> Result<core::SparseMat_Hdr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMat_Hdr_Hdr_int_const_intX_int(_sizes.len() as _, _sizes.as_ptr(), _type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat_Hdr::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Node /usr/include/opencv2/core/mat.hpp:2737
pub trait SparseMat_NodeTraitConst {
	fn as_raw_SparseMat_Node(&self) -> *const c_void;

	// hashval /usr/include/opencv2/core/mat.hpp:2740
	#[inline]
	fn hashval(&self) -> size_t {
		let ret = unsafe { sys::cv_SparseMat_Node_getPropHashval_const(self.as_raw_SparseMat_Node()) };
		ret
	}
	
	// next /usr/include/opencv2/core/mat.hpp:2742
	#[inline]
	fn next(&self) -> size_t {
		let ret = unsafe { sys::cv_SparseMat_Node_getPropNext_const(self.as_raw_SparseMat_Node()) };
		ret
	}
	
}

pub trait SparseMat_NodeTrait: core::SparseMat_NodeTraitConst {
	fn as_raw_mut_SparseMat_Node(&mut self) -> *mut c_void;

	// hashval /usr/include/opencv2/core/mat.hpp:2740
	#[inline]
	fn set_hashval(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_SparseMat_Node_setPropHashval_size_t(self.as_raw_mut_SparseMat_Node(), val) };
		ret
	}
	
	// next /usr/include/opencv2/core/mat.hpp:2742
	#[inline]
	fn set_next(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_SparseMat_Node_setPropNext_size_t(self.as_raw_mut_SparseMat_Node(), val) };
		ret
	}
	
	// idx /usr/include/opencv2/core/mat.hpp:2744
	#[inline]
	fn idx(&mut self) -> &mut [i32; 32] {
		let ret = unsafe { sys::cv_SparseMat_Node_getPropIdx(self.as_raw_mut_SparseMat_Node()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}
	
}

// Node /usr/include/opencv2/core/mat.hpp:2737
pub struct SparseMat_Node {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMat_Node }

impl Drop for SparseMat_Node {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMat_Node_delete(instance: *mut c_void); }
		unsafe { cv_SparseMat_Node_delete(self.as_raw_mut_SparseMat_Node()) };
	}
}

unsafe impl Send for SparseMat_Node {}

impl core::SparseMat_NodeTraitConst for SparseMat_Node {
	#[inline] fn as_raw_SparseMat_Node(&self) -> *const c_void { self.as_raw() }
}

impl core::SparseMat_NodeTrait for SparseMat_Node {
	#[inline] fn as_raw_mut_SparseMat_Node(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMat_Node {
}

// SparseMatConstIterator /usr/include/opencv2/core/mat.hpp:3237
pub trait SparseMatConstIteratorTraitConst {
	fn as_raw_SparseMatConstIterator(&self) -> *const c_void;

	// m /usr/include/opencv2/core/mat.hpp:3267
	#[inline]
	fn m(&self) -> core::SparseMat {
		let ret = unsafe { sys::cv_SparseMatConstIterator_getPropM_const(self.as_raw_SparseMatConstIterator()) };
		let ret = unsafe { core::SparseMat::opencv_from_extern(ret) };
		ret
	}
	
	// hashidx /usr/include/opencv2/core/mat.hpp:3268
	#[inline]
	fn hashidx(&self) -> size_t {
		let ret = unsafe { sys::cv_SparseMatConstIterator_getPropHashidx_const(self.as_raw_SparseMatConstIterator()) };
		ret
	}
	
	// node() /usr/include/opencv2/core/mat.hpp:3253
	#[inline]
	fn node(&self) -> Result<core::SparseMat_Node> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatConstIterator_node_const(self.as_raw_SparseMatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat_Node::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait SparseMatConstIteratorTrait: core::SparseMatConstIteratorTraitConst {
	fn as_raw_mut_SparseMatConstIterator(&mut self) -> *mut c_void;

	// hashidx /usr/include/opencv2/core/mat.hpp:3268
	#[inline]
	fn set_hashidx(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_SparseMatConstIterator_setPropHashidx_size_t(self.as_raw_mut_SparseMatConstIterator(), val) };
		ret
	}
	
	// ptr /usr/include/opencv2/core/mat.hpp:3269
	#[inline]
	fn ptr(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_SparseMatConstIterator_getPropPtr(self.as_raw_mut_SparseMatConstIterator()) };
		ret
	}
	
	// ptr /usr/include/opencv2/core/mat.hpp:3269
	#[inline]
	unsafe fn set_ptr(&mut self, val: *mut u8) {
		let ret = { sys::cv_SparseMatConstIterator_setPropPtr_unsigned_charX(self.as_raw_mut_SparseMatConstIterator(), val) };
		ret
	}
	
	// operator--() /usr/include/opencv2/core/mat.hpp:3256
	#[inline]
	#[cfg(not(target_os = "windows"))]
	fn decr(&mut self) -> Result<core::SparseMatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatConstIterator_operatorSS(self.as_raw_mut_SparseMatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// operator++() /usr/include/opencv2/core/mat.hpp:3260
	#[inline]
	fn incr(&mut self) -> Result<core::SparseMatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatConstIterator_operatorAA(self.as_raw_mut_SparseMatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// seekEnd() /usr/include/opencv2/core/mat.hpp:3265
	#[inline]
	fn seek_end(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatConstIterator_seekEnd(self.as_raw_mut_SparseMatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SparseMatConstIterator /usr/include/opencv2/core/mat.hpp:3237
pub struct SparseMatConstIterator {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMatConstIterator }

impl Drop for SparseMatConstIterator {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMatConstIterator_delete(instance: *mut c_void); }
		unsafe { cv_SparseMatConstIterator_delete(self.as_raw_mut_SparseMatConstIterator()) };
	}
}

unsafe impl Send for SparseMatConstIterator {}

impl core::SparseMatConstIteratorTraitConst for SparseMatConstIterator {
	#[inline] fn as_raw_SparseMatConstIterator(&self) -> *const c_void { self.as_raw() }
}

impl core::SparseMatConstIteratorTrait for SparseMatConstIterator {
	#[inline] fn as_raw_mut_SparseMatConstIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMatConstIterator {
	// SparseMatConstIterator() /usr/include/opencv2/core/mat.hpp:3241
	#[inline]
	pub fn default() -> Result<core::SparseMatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatConstIterator_SparseMatConstIterator(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// SparseMatConstIterator(const cv::SparseMat *) /usr/include/opencv2/core/mat.hpp:3243
	#[inline]
	pub fn new(_m: &core::SparseMat) -> Result<core::SparseMatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatX(_m.as_raw_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// SparseMatConstIterator(const cv::SparseMatConstIterator &) /usr/include/opencv2/core/mat.hpp:3245
	#[inline]
	pub fn copy(it: &core::SparseMatConstIterator) -> Result<core::SparseMatConstIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatConstIteratorR(it.as_raw_SparseMatConstIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatConstIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// SparseMatIterator /usr/include/opencv2/core/mat.hpp:3281
pub trait SparseMatIteratorTraitConst: core::SparseMatConstIteratorTraitConst {
	fn as_raw_SparseMatIterator(&self) -> *const c_void;

	// node() /usr/include/opencv2/core/mat.hpp:3298
	#[inline]
	fn node(&self) -> Result<core::SparseMat_Node> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatIterator_node_const(self.as_raw_SparseMatIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMat_Node::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait SparseMatIteratorTrait: core::SparseMatConstIteratorTrait + core::SparseMatIteratorTraitConst {
	fn as_raw_mut_SparseMatIterator(&mut self) -> *mut c_void;

	// operator++() /usr/include/opencv2/core/mat.hpp:3301
	#[inline]
	fn incr(&mut self) -> Result<core::SparseMatIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatIterator_operatorAA(self.as_raw_mut_SparseMatIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// SparseMatIterator /usr/include/opencv2/core/mat.hpp:3281
pub struct SparseMatIterator {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMatIterator }

impl Drop for SparseMatIterator {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMatIterator_delete(instance: *mut c_void); }
		unsafe { cv_SparseMatIterator_delete(self.as_raw_mut_SparseMatIterator()) };
	}
}

unsafe impl Send for SparseMatIterator {}

impl core::SparseMatConstIteratorTraitConst for SparseMatIterator {
	#[inline] fn as_raw_SparseMatConstIterator(&self) -> *const c_void { self.as_raw() }
}

impl core::SparseMatConstIteratorTrait for SparseMatIterator {
	#[inline] fn as_raw_mut_SparseMatConstIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::SparseMatIteratorTraitConst for SparseMatIterator {
	#[inline] fn as_raw_SparseMatIterator(&self) -> *const c_void { self.as_raw() }
}

impl core::SparseMatIteratorTrait for SparseMatIterator {
	#[inline] fn as_raw_mut_SparseMatIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMatIterator {
	// SparseMatIterator() /usr/include/opencv2/core/mat.hpp:3285
	#[inline]
	pub fn default() -> Result<core::SparseMatIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatIterator_SparseMatIterator(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// SparseMatIterator(cv::SparseMat *) /usr/include/opencv2/core/mat.hpp:3287
	#[inline]
	pub fn new(_m: &mut core::SparseMat) -> Result<core::SparseMatIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatIterator_SparseMatIterator_SparseMatX(_m.as_raw_mut_SparseMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// SparseMatIterator(const cv::SparseMatIterator &) /usr/include/opencv2/core/mat.hpp:3291
	#[inline]
	pub fn copy(it: &core::SparseMatIterator) -> Result<core::SparseMatIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseMatIterator_SparseMatIterator_const_SparseMatIteratorR(it.as_raw_SparseMatIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::SparseMatIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SparseMatIterator, core::SparseMatConstIterator, cv_SparseMatIterator_to_SparseMatConstIterator }

// TLSDataContainer /usr/include/opencv2/core/utils/tls.hpp:25
pub trait TLSDataContainerConst {
	fn as_raw_TLSDataContainer(&self) -> *const c_void;

}

pub trait TLSDataContainer: core::TLSDataContainerConst {
	fn as_raw_mut_TLSDataContainer(&mut self) -> *mut c_void;

	// cleanup() /usr/include/opencv2/core/utils/tls.hpp:49
	#[inline]
	fn cleanup(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TLSDataContainer_cleanup(self.as_raw_mut_TLSDataContainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// TermCriteria /usr/include/opencv2/core/types.hpp:874
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TermCriteria {
	pub typ: i32,
	pub max_count: i32,
	pub epsilon: f64,
}

opencv_type_simple! { core::TermCriteria }

impl TermCriteria {
	// isValid() /usr/include/opencv2/core/types.hpp:896
	#[inline]
	pub fn is_valid(self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TermCriteria_isValid_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// TermCriteria() /usr/include/opencv2/core/types.hpp:888
	#[inline]
	pub fn default() -> Result<core::TermCriteria> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TermCriteria_TermCriteria(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// TermCriteria(int, int, double) /usr/include/opencv2/core/types.hpp:894
	#[inline]
	pub fn new(typ: i32, max_count: i32, epsilon: f64) -> Result<core::TermCriteria> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TermCriteria_TermCriteria_int_int_double(typ, max_count, epsilon, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// TickMeter /usr/include/opencv2/core/utility.hpp:294
pub trait TickMeterTraitConst {
	fn as_raw_TickMeter(&self) -> *const c_void;

	// getTimeTicks() /usr/include/opencv2/core/utility.hpp:321
	#[inline]
	fn get_time_ticks(&self) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_getTimeTicks_const(self.as_raw_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getTimeMicro() /usr/include/opencv2/core/utility.hpp:327
	#[inline]
	fn get_time_micro(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_getTimeMicro_const(self.as_raw_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getTimeMilli() /usr/include/opencv2/core/utility.hpp:333
	#[inline]
	fn get_time_milli(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_getTimeMilli_const(self.as_raw_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getTimeSec() /usr/include/opencv2/core/utility.hpp:339
	#[inline]
	fn get_time_sec(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_getTimeSec_const(self.as_raw_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getCounter() /usr/include/opencv2/core/utility.hpp:345
	#[inline]
	fn get_counter(&self) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_getCounter_const(self.as_raw_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFPS() /usr/include/opencv2/core/utility.hpp:351
	#[inline]
	fn get_fps(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_getFPS_const(self.as_raw_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getAvgTimeSec() /usr/include/opencv2/core/utility.hpp:360
	#[inline]
	fn get_avg_time_sec(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_getAvgTimeSec_const(self.as_raw_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getAvgTimeMilli() /usr/include/opencv2/core/utility.hpp:368
	#[inline]
	fn get_avg_time_milli(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_getAvgTimeMilli_const(self.as_raw_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TickMeterTrait: core::TickMeterTraitConst {
	fn as_raw_mut_TickMeter(&mut self) -> *mut c_void;

	// start() /usr/include/opencv2/core/utility.hpp:304
	#[inline]
	fn start(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_start(self.as_raw_mut_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// stop() /usr/include/opencv2/core/utility.hpp:310
	#[inline]
	fn stop(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_stop(self.as_raw_mut_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// reset() /usr/include/opencv2/core/utility.hpp:374
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_reset(self.as_raw_mut_TickMeter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// TickMeter /usr/include/opencv2/core/utility.hpp:294
pub struct TickMeter {
	ptr: *mut c_void
}

opencv_type_boxed! { TickMeter }

impl Drop for TickMeter {
	fn drop(&mut self) {
		extern "C" { fn cv_TickMeter_delete(instance: *mut c_void); }
		unsafe { cv_TickMeter_delete(self.as_raw_mut_TickMeter()) };
	}
}

unsafe impl Send for TickMeter {}

impl core::TickMeterTraitConst for TickMeter {
	#[inline] fn as_raw_TickMeter(&self) -> *const c_void { self.as_raw() }
}

impl core::TickMeterTrait for TickMeter {
	#[inline] fn as_raw_mut_TickMeter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TickMeter {
	// TickMeter() /usr/include/opencv2/core/utility.hpp:298
	#[inline]
	pub fn default() -> Result<core::TickMeter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TickMeter_TickMeter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::TickMeter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// UMat /usr/include/opencv2/core/mat.hpp:2410
pub trait UMatTraitConst {
	fn as_raw_UMat(&self) -> *const c_void;

	// flags /usr/include/opencv2/core/mat.hpp:2592
	#[inline]
	fn flags(&self) -> i32 {
		let ret = unsafe { sys::cv_UMat_getPropFlags_const(self.as_raw_UMat()) };
		ret
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2595
	#[inline]
	fn dims(&self) -> i32 {
		let ret = unsafe { sys::cv_UMat_getPropDims_const(self.as_raw_UMat()) };
		ret
	}
	
	// rows /usr/include/opencv2/core/mat.hpp:2598
	#[inline]
	fn rows(&self) -> i32 {
		let ret = unsafe { sys::cv_UMat_getPropRows_const(self.as_raw_UMat()) };
		ret
	}
	
	// cols /usr/include/opencv2/core/mat.hpp:2601
	#[inline]
	fn cols(&self) -> i32 {
		let ret = unsafe { sys::cv_UMat_getPropCols_const(self.as_raw_UMat()) };
		ret
	}
	
	// usageFlags /usr/include/opencv2/core/mat.hpp:2607
	#[inline]
	fn usage_flags(&self) -> core::UMatUsageFlags {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_getPropUsageFlags_const(self.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// offset /usr/include/opencv2/core/mat.hpp:2619
	#[inline]
	fn offset(&self) -> size_t {
		let ret = unsafe { sys::cv_UMat_getPropOffset_const(self.as_raw_UMat()) };
		ret
	}
	
	// size /usr/include/opencv2/core/mat.hpp:2622
	#[inline]
	fn mat_size(&self) -> core::MatSize {
		let ret = unsafe { sys::cv_UMat_getPropSize_const(self.as_raw_UMat()) };
		let ret = unsafe { core::MatSize::opencv_from_extern(ret) };
		ret
	}
	
	// step /usr/include/opencv2/core/mat.hpp:2625
	#[inline]
	fn mat_step(&self) -> core::MatStep {
		let ret = unsafe { sys::cv_UMat_getPropStep_const(self.as_raw_UMat()) };
		let ret = unsafe { core::MatStep::opencv_from_extern(ret) };
		ret
	}
	
	// getMat(cv::AccessFlag) /usr/include/opencv2/core/mat.hpp:2445
	#[inline]
	fn get_mat(&self, flags: core::AccessFlag) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_getMat_const_AccessFlag(self.as_raw_UMat(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// row(int) /usr/include/opencv2/core/mat.hpp:2448
	#[inline]
	fn row(&self, y: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_row_const_int(self.as_raw_UMat(), y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// col(int) /usr/include/opencv2/core/mat.hpp:2450
	#[inline]
	fn col(&self, x: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_col_const_int(self.as_raw_UMat(), x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// rowRange(int, int) /usr/include/opencv2/core/mat.hpp:2452
	#[inline]
	fn row_bounds(&self, startrow: i32, endrow: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_rowRange_const_int_int(self.as_raw_UMat(), startrow, endrow, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// rowRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:2453
	#[inline]
	fn row_range(&self, r: &core::Range) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_rowRange_const_const_RangeR(self.as_raw_UMat(), r.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// colRange(int, int) /usr/include/opencv2/core/mat.hpp:2455
	#[inline]
	fn col_bounds(&self, startcol: i32, endcol: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_colRange_const_int_int(self.as_raw_UMat(), startcol, endcol, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// colRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:2456
	#[inline]
	fn col_range(&self, r: &core::Range) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_colRange_const_const_RangeR(self.as_raw_UMat(), r.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * d: 0
	// diag(int) /usr/include/opencv2/core/mat.hpp:2461
	#[inline]
	fn diag(&self, d: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_diag_const_int(self.as_raw_UMat(), d, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// clone() /usr/include/opencv2/core/mat.hpp:2467
	#[inline]
	#[must_use]
	fn try_clone(&self) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_clone_const(self.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/mat.hpp:2470
	#[inline]
	fn copy_to(&self, m: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_copyTo_const_const__OutputArrayR(self.as_raw_UMat(), m.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyTo(cv::OutputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:2472
	#[inline]
	fn copy_to_masked(&self, m: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		output_array_arg!(m);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_copyTo_const_const__OutputArrayR_const__InputArrayR(self.as_raw_UMat(), m.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * alpha: 1
	/// * beta: 0
	// convertTo(cv::OutputArray, int, double, double) /usr/include/opencv2/core/mat.hpp:2474
	#[inline]
	fn convert_to(&self, m: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
		output_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_convertTo_const_const__OutputArrayR_int_double_double(self.as_raw_UMat(), m.as_raw__OutputArray(), rtype, alpha, beta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * typ: -1
	// assignTo(cv::UMat &, int) /usr/include/opencv2/core/mat.hpp:2476
	#[inline]
	fn assign_to(&self, m: &mut core::UMat, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_assignTo_const_UMatR_int(self.as_raw_UMat(), m.as_raw_mut_UMat(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * rows: 0
	// reshape(int, int) /usr/include/opencv2/core/mat.hpp:2484
	#[inline]
	fn reshape(&self, cn: i32, rows: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_reshape_const_int_int(self.as_raw_UMat(), cn, rows, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// reshape(int, int, const int *) /usr/include/opencv2/core/mat.hpp:2485
	#[inline]
	fn reshape_1(&self, cn: i32, newndims: i32, newsz: &i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_reshape_const_int_int_const_intX(self.as_raw_UMat(), cn, newndims, newsz, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// t() /usr/include/opencv2/core/mat.hpp:2488
	#[inline]
	fn t(&self) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_t_const(self.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * method: DECOMP_LU
	// inv(int) /usr/include/opencv2/core/mat.hpp:2490
	#[inline]
	fn inv(&self, method: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_inv_const_int(self.as_raw_UMat(), method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	// mul(cv::InputArray, double) /usr/include/opencv2/core/mat.hpp:2492
	#[inline]
	fn mul(&self, m: &dyn core::ToInputArray, scale: f64) -> Result<core::UMat> {
		input_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_mul_const_const__InputArrayR_double(self.as_raw_UMat(), m.as_raw__InputArray(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// dot(cv::InputArray) /usr/include/opencv2/core/mat.hpp:2495
	#[inline]
	fn dot(&self, m: &dyn core::ToInputArray) -> Result<f64> {
		input_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_dot_const_const__InputArrayR(self.as_raw_UMat(), m.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// locateROI(cv::Size &, cv::Point &) /usr/include/opencv2/core/mat.hpp:2534
	#[inline]
	fn locate_roi(&self, whole_size: &mut core::Size, ofs: &mut core::Point) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_locateROI_const_SizeR_PointR(self.as_raw_UMat(), whole_size, ofs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isContinuous() /usr/include/opencv2/core/mat.hpp:2547
	#[inline]
	fn is_continuous(&self) -> bool {
		let ret = unsafe { sys::cv_UMat_isContinuous_const(self.as_raw_UMat()) };
		ret
	}
	
	// isSubmatrix() /usr/include/opencv2/core/mat.hpp:2550
	#[inline]
	fn is_submatrix(&self) -> bool {
		let ret = unsafe { sys::cv_UMat_isSubmatrix_const(self.as_raw_UMat()) };
		ret
	}
	
	// elemSize() /usr/include/opencv2/core/mat.hpp:2554
	#[inline]
	fn elem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_elemSize_const(self.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize1() /usr/include/opencv2/core/mat.hpp:2556
	#[inline]
	fn elem_size1(&self) -> size_t {
		let ret = unsafe { sys::cv_UMat_elemSize1_const(self.as_raw_UMat()) };
		ret
	}
	
	// type() /usr/include/opencv2/core/mat.hpp:2558
	#[inline]
	fn typ(&self) -> i32 {
		let ret = unsafe { sys::cv_UMat_type_const(self.as_raw_UMat()) };
		ret
	}
	
	// depth() /usr/include/opencv2/core/mat.hpp:2560
	#[inline]
	fn depth(&self) -> i32 {
		let ret = unsafe { sys::cv_UMat_depth_const(self.as_raw_UMat()) };
		ret
	}
	
	// channels() /usr/include/opencv2/core/mat.hpp:2562
	#[inline]
	fn channels(&self) -> i32 {
		let ret = unsafe { sys::cv_UMat_channels_const(self.as_raw_UMat()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * i: 0
	// step1(int) /usr/include/opencv2/core/mat.hpp:2564
	#[inline]
	fn step1(&self, i: i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_step1_const_int(self.as_raw_UMat(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/mat.hpp:2566
	#[inline]
	fn empty(&self) -> bool {
		let ret = unsafe { sys::cv_UMat_empty_const(self.as_raw_UMat()) };
		ret
	}
	
	// total() /usr/include/opencv2/core/mat.hpp:2568
	#[inline]
	fn total(&self) -> size_t {
		let ret = unsafe { sys::cv_UMat_total_const(self.as_raw_UMat()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * depth: -1
	/// * require_continuous: true
	// checkVector(int, int, bool) /usr/include/opencv2/core/mat.hpp:2571
	#[inline]
	fn check_vector(&self, elem_channels: i32, depth: i32, require_continuous: bool) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_checkVector_const_int_int_bool(self.as_raw_UMat(), elem_channels, depth, require_continuous, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// handle(cv::AccessFlag) /usr/include/opencv2/core/mat.hpp:2580
	#[inline]
	fn handle(&self, access_flags: core::AccessFlag) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_handle_const_AccessFlag(self.as_raw_UMat(), access_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// ndoffset(size_t *) /usr/include/opencv2/core/mat.hpp:2581
	#[inline]
	fn ndoffset(&self, ofs: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_ndoffset_const_size_tX(self.as_raw_UMat(), ofs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait UMatTrait: core::UMatTraitConst {
	fn as_raw_mut_UMat(&mut self) -> *mut c_void;

	// flags /usr/include/opencv2/core/mat.hpp:2592
	#[inline]
	fn set_flags(&mut self, val: i32) {
		let ret = unsafe { sys::cv_UMat_setPropFlags_int(self.as_raw_mut_UMat(), val) };
		ret
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2595
	#[inline]
	fn set_dims(&mut self, val: i32) {
		let ret = unsafe { sys::cv_UMat_setPropDims_int(self.as_raw_mut_UMat(), val) };
		ret
	}
	
	// rows /usr/include/opencv2/core/mat.hpp:2598
	#[inline]
	fn set_rows(&mut self, val: i32) {
		let ret = unsafe { sys::cv_UMat_setPropRows_int(self.as_raw_mut_UMat(), val) };
		ret
	}
	
	// cols /usr/include/opencv2/core/mat.hpp:2601
	#[inline]
	fn set_cols(&mut self, val: i32) {
		let ret = unsafe { sys::cv_UMat_setPropCols_int(self.as_raw_mut_UMat(), val) };
		ret
	}
	
	// usageFlags /usr/include/opencv2/core/mat.hpp:2607
	#[inline]
	fn set_usage_flags(&mut self, val: core::UMatUsageFlags) {
		let ret = unsafe { sys::cv_UMat_setPropUsageFlags_UMatUsageFlags(self.as_raw_mut_UMat(), val) };
		ret
	}
	
	// u /usr/include/opencv2/core/mat.hpp:2616
	#[inline]
	fn u(&mut self) -> core::UMatData {
		let ret = unsafe { sys::cv_UMat_getPropU(self.as_raw_mut_UMat()) };
		let ret = unsafe { core::UMatData::opencv_from_extern(ret) };
		ret
	}
	
	// u /usr/include/opencv2/core/mat.hpp:2616
	#[inline]
	fn set_u(&mut self, val: &mut core::UMatData) {
		let ret = unsafe { sys::cv_UMat_setPropU_UMatDataX(self.as_raw_mut_UMat(), val.as_raw_mut_UMatData()) };
		ret
	}
	
	// offset /usr/include/opencv2/core/mat.hpp:2619
	#[inline]
	fn set_offset(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_UMat_setPropOffset_size_t(self.as_raw_mut_UMat(), val) };
		ret
	}
	
	/// ## C++ default parameters
	/// * mask: noArray()
	// setTo(cv::InputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:2481
	#[inline]
	fn set_to(&mut self, value: &dyn core::ToInputArray, mask: &dyn core::ToInputArray) -> Result<core::UMat> {
		input_array_arg!(value);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_setTo_const__InputArrayR_const__InputArrayR(self.as_raw_mut_UMat(), value.as_raw__InputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// create(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2517
	#[inline]
	unsafe fn create_rows_cols(&mut self, rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_UMat_create_int_int_int_UMatUsageFlags(self.as_raw_mut_UMat(), rows, cols, typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// create(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2518
	#[inline]
	unsafe fn create_size(&mut self, size: core::Size, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_UMat_create_Size_int_UMatUsageFlags(self.as_raw_mut_UMat(), size.opencv_as_extern(), typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// create(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2519
	#[inline]
	unsafe fn create_nd(&mut self, sizes: &[i32], typ: i32, usage_flags: core::UMatUsageFlags) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_UMat_create_int_const_intX_int_UMatUsageFlags(self.as_raw_mut_UMat(), sizes.len() as _, sizes.as_ptr(), typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// create(const std::vector<int> &, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2520
	#[inline]
	unsafe fn create_nd_vec(&mut self, sizes: &core::Vector<i32>, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_UMat_create_const_vector_int_R_int_UMatUsageFlags(self.as_raw_mut_UMat(), sizes.as_raw_VectorOfi32(), typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// addref() /usr/include/opencv2/core/mat.hpp:2523
	#[inline]
	fn addref(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_addref(self.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/mat.hpp:2526
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_release(self.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// deallocate() /usr/include/opencv2/core/mat.hpp:2529
	#[inline]
	fn deallocate(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_deallocate(self.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// adjustROI(int, int, int, int) /usr/include/opencv2/core/mat.hpp:2536
	#[inline]
	fn adjust_roi(&mut self, dtop: i32, dbottom: i32, dleft: i32, dright: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_adjustROI_int_int_int_int(self.as_raw_mut_UMat(), dtop, dbottom, dleft, dright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// updateContinuityFlag() /usr/include/opencv2/core/mat.hpp:2613
	#[inline]
	fn update_continuity_flag(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_updateContinuityFlag(self.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// UMat /usr/include/opencv2/core/mat.hpp:2410
pub struct UMat {
	ptr: *mut c_void
}

opencv_type_boxed! { UMat }

impl Drop for UMat {
	fn drop(&mut self) {
		extern "C" { fn cv_UMat_delete(instance: *mut c_void); }
		unsafe { cv_UMat_delete(self.as_raw_mut_UMat()) };
	}
}

unsafe impl Send for UMat {}

impl core::UMatTraitConst for UMat {
	#[inline] fn as_raw_UMat(&self) -> *const c_void { self.as_raw() }
}

impl core::UMatTrait for UMat {
	#[inline] fn as_raw_mut_UMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl UMat {
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// UMat(cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2414
	#[inline]
	pub fn new(usage_flags: core::UMatUsageFlags) -> core::UMat {
		let ret = unsafe { sys::cv_UMat_UMat_UMatUsageFlags(usage_flags) };
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		ret
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// UMat(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2417
	#[inline]
	pub unsafe fn new_rows_cols(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_UMat_UMat_int_int_int_UMatUsageFlags(rows, cols, typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// UMat(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2418
	#[inline]
	pub unsafe fn new_size(size: core::Size, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_UMat_UMat_Size_int_UMatUsageFlags(size.opencv_as_extern(), typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// UMat(int, int, int, const cv::Scalar &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2420
	#[inline]
	pub fn new_rows_cols_with_default(rows: i32, cols: i32, typ: i32, s: core::Scalar, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_UMat_int_int_int_const_ScalarR_UMatUsageFlags(rows, cols, typ, &s, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// UMat(cv::Size, int, const cv::Scalar &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2421
	#[inline]
	pub fn new_size_with_default(size: core::Size, typ: i32, s: core::Scalar, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_UMat_Size_int_const_ScalarR_UMatUsageFlags(size.opencv_as_extern(), typ, &s, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// UMat(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2424
	#[inline]
	pub unsafe fn new_nd(sizes: &[i32], typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_UMat_UMat_int_const_intX_int_UMatUsageFlags(sizes.len() as _, sizes.as_ptr(), typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	// UMat(int, const int *, int, const cv::Scalar &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2425
	#[inline]
	pub fn new_nd_with_default(sizes: &[i32], typ: i32, s: core::Scalar, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_UMat_int_const_intX_int_const_ScalarR_UMatUsageFlags(sizes.len() as _, sizes.as_ptr(), typ, &s, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// UMat(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:2428
	#[inline]
	pub fn copy(m: &core::UMat) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_UMat_const_UMatR(m.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * col_range: Range::all()
	// UMat(const cv::UMat &, const cv::Range &, const cv::Range &) /usr/include/opencv2/core/mat.hpp:2431
	#[inline]
	pub fn rowscols(m: &core::UMat, row_range: &core::Range, col_range: &core::Range) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_UMat_const_UMatR_const_RangeR_const_RangeR(m.as_raw_UMat(), row_range.as_raw_Range(), col_range.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// UMat(const cv::UMat &, const cv::Rect &) /usr/include/opencv2/core/mat.hpp:2432
	#[inline]
	pub fn roi(m: &core::UMat, roi: core::Rect) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_UMat_const_UMatR_const_RectR(m.as_raw_UMat(), &roi, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// UMat(const cv::UMat &, const std::vector<Range> &) /usr/include/opencv2/core/mat.hpp:2434
	#[inline]
	pub fn ranges(m: &core::UMat, ranges: &core::Vector<core::Range>) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_UMat_const_UMatR_const_vector_Range_R(m.as_raw_UMat(), ranges.as_raw_VectorOfRange(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// diag(const cv::UMat &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2463
	#[inline]
	#[must_use]
	pub fn diag(d: &core::UMat, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_diag_const_UMatR_UMatUsageFlags(d.as_raw_UMat(), usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// diag(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:2464
	#[inline]
	#[must_use]
	pub fn diag_1(d: &core::UMat) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_diag_const_UMatR(d.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// zeros(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2498
	#[inline]
	#[must_use]
	pub fn zeros(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_zeros_int_int_int_UMatUsageFlags(rows, cols, typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// zeros(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2499
	#[inline]
	#[must_use]
	pub fn zeros_1(size: core::Size, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_zeros_Size_int_UMatUsageFlags(size.opencv_as_extern(), typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// zeros(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2500
	#[inline]
	#[must_use]
	pub fn zeros_2(ndims: i32, sz: &i32, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_zeros_int_const_intX_int_UMatUsageFlags(ndims, sz, typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// zeros(int, int, int) /usr/include/opencv2/core/mat.hpp:2501
	#[inline]
	#[must_use]
	pub fn zeros_3(rows: i32, cols: i32, typ: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_zeros_int_int_int(rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// zeros(cv::Size, int) /usr/include/opencv2/core/mat.hpp:2502
	#[inline]
	#[must_use]
	pub fn zeros_4(size: core::Size, typ: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_zeros_Size_int(size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// zeros(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2503
	#[inline]
	#[must_use]
	pub fn zeros_5(ndims: i32, sz: &i32, typ: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_zeros_int_const_intX_int(ndims, sz, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ones(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2504
	#[inline]
	#[must_use]
	pub fn ones(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_ones_int_int_int_UMatUsageFlags(rows, cols, typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ones(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2505
	#[inline]
	#[must_use]
	pub fn ones_1(size: core::Size, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_ones_Size_int_UMatUsageFlags(size.opencv_as_extern(), typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ones(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2506
	#[inline]
	#[must_use]
	pub fn ones_2(ndims: i32, sz: &i32, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_ones_int_const_intX_int_UMatUsageFlags(ndims, sz, typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ones(int, int, int) /usr/include/opencv2/core/mat.hpp:2507
	#[inline]
	#[must_use]
	pub fn ones_3(rows: i32, cols: i32, typ: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_ones_int_int_int(rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ones(cv::Size, int) /usr/include/opencv2/core/mat.hpp:2508
	#[inline]
	#[must_use]
	pub fn ones_4(size: core::Size, typ: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_ones_Size_int(size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ones(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2509
	#[inline]
	#[must_use]
	pub fn ones_5(ndims: i32, sz: &i32, typ: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_ones_int_const_intX_int(ndims, sz, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// eye(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2510
	#[inline]
	#[must_use]
	pub fn eye(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_eye_int_int_int_UMatUsageFlags(rows, cols, typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// eye(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2511
	#[inline]
	#[must_use]
	pub fn eye_1(size: core::Size, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_eye_Size_int_UMatUsageFlags(size.opencv_as_extern(), typ, usage_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// eye(int, int, int) /usr/include/opencv2/core/mat.hpp:2512
	#[inline]
	#[must_use]
	pub fn eye_2(rows: i32, cols: i32, typ: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_eye_int_int_int(rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// eye(cv::Size, int) /usr/include/opencv2/core/mat.hpp:2513
	#[inline]
	#[must_use]
	pub fn eye_3(size: core::Size, typ: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_eye_Size_int(size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// UMat(cv::UMat &&) /usr/include/opencv2/core/mat.hpp:2573
	#[inline]
	pub fn copy_mut(m: &mut core::UMat) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMat_UMat_UMatR(m.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Clone for UMat {
	#[inline]
	/// Calls try_clone() and panics if that fails
	fn clone(&self) -> Self {
		self.try_clone().expect("Cannot clone UMat")
	}
}

// UMatData /usr/include/opencv2/core/mat.hpp:543
pub trait UMatDataTraitConst {
	fn as_raw_UMatData(&self) -> *const c_void;

	// urefcount /usr/include/opencv2/core/mat.hpp:569
	#[inline]
	fn urefcount(&self) -> i32 {
		let ret = unsafe { sys::cv_UMatData_getPropUrefcount_const(self.as_raw_UMatData()) };
		ret
	}
	
	// refcount /usr/include/opencv2/core/mat.hpp:570
	#[inline]
	fn refcount(&self) -> i32 {
		let ret = unsafe { sys::cv_UMatData_getPropRefcount_const(self.as_raw_UMatData()) };
		ret
	}
	
	// size /usr/include/opencv2/core/mat.hpp:573
	#[inline]
	fn size(&self) -> size_t {
		let ret = unsafe { sys::cv_UMatData_getPropSize_const(self.as_raw_UMatData()) };
		ret
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:575
	#[inline]
	fn flags(&self) -> core::UMatData_MemoryFlag {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_getPropFlags_const(self.as_raw_UMatData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// allocatorFlags_ /usr/include/opencv2/core/mat.hpp:578
	#[inline]
	fn allocator_flags_(&self) -> i32 {
		let ret = unsafe { sys::cv_UMatData_getPropAllocatorFlags__const(self.as_raw_UMatData()) };
		ret
	}
	
	// mapcount /usr/include/opencv2/core/mat.hpp:579
	#[inline]
	fn mapcount(&self) -> i32 {
		let ret = unsafe { sys::cv_UMatData_getPropMapcount_const(self.as_raw_UMatData()) };
		ret
	}
	
	// hostCopyObsolete() /usr/include/opencv2/core/mat.hpp:557
	#[inline]
	fn host_copy_obsolete(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_hostCopyObsolete_const(self.as_raw_UMatData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// deviceCopyObsolete() /usr/include/opencv2/core/mat.hpp:558
	#[inline]
	fn device_copy_obsolete(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_deviceCopyObsolete_const(self.as_raw_UMatData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// deviceMemMapped() /usr/include/opencv2/core/mat.hpp:559
	#[inline]
	fn device_mem_mapped(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_deviceMemMapped_const(self.as_raw_UMatData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyOnMap() /usr/include/opencv2/core/mat.hpp:560
	#[inline]
	fn copy_on_map(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_copyOnMap_const(self.as_raw_UMatData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// tempUMat() /usr/include/opencv2/core/mat.hpp:561
	#[inline]
	fn temp_umat(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_tempUMat_const(self.as_raw_UMatData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// tempCopiedUMat() /usr/include/opencv2/core/mat.hpp:562
	#[inline]
	fn temp_copied_umat(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_tempCopiedUMat_const(self.as_raw_UMatData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait UMatDataTrait: core::UMatDataTraitConst {
	fn as_raw_mut_UMatData(&mut self) -> *mut c_void;

	// urefcount /usr/include/opencv2/core/mat.hpp:569
	#[inline]
	fn set_urefcount(&mut self, val: i32) {
		let ret = unsafe { sys::cv_UMatData_setPropUrefcount_int(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// refcount /usr/include/opencv2/core/mat.hpp:570
	#[inline]
	fn set_refcount(&mut self, val: i32) {
		let ret = unsafe { sys::cv_UMatData_setPropRefcount_int(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// data /usr/include/opencv2/core/mat.hpp:571
	#[inline]
	fn data(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_UMatData_getPropData(self.as_raw_mut_UMatData()) };
		ret
	}
	
	// data /usr/include/opencv2/core/mat.hpp:571
	#[inline]
	unsafe fn set_data(&mut self, val: *mut u8) {
		let ret = { sys::cv_UMatData_setPropData_unsigned_charX(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// origdata /usr/include/opencv2/core/mat.hpp:572
	#[inline]
	fn origdata(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_UMatData_getPropOrigdata(self.as_raw_mut_UMatData()) };
		ret
	}
	
	// origdata /usr/include/opencv2/core/mat.hpp:572
	#[inline]
	unsafe fn set_origdata(&mut self, val: *mut u8) {
		let ret = { sys::cv_UMatData_setPropOrigdata_unsigned_charX(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// size /usr/include/opencv2/core/mat.hpp:573
	#[inline]
	fn set_size(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_UMatData_setPropSize_size_t(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:575
	#[inline]
	fn set_flags(&mut self, val: core::UMatData_MemoryFlag) {
		let ret = unsafe { sys::cv_UMatData_setPropFlags_MemoryFlag(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// handle /usr/include/opencv2/core/mat.hpp:576
	#[inline]
	fn handle(&mut self) -> *mut c_void {
		let ret = unsafe { sys::cv_UMatData_getPropHandle(self.as_raw_mut_UMatData()) };
		ret
	}
	
	// handle /usr/include/opencv2/core/mat.hpp:576
	#[inline]
	unsafe fn set_handle(&mut self, val: *mut c_void) {
		let ret = { sys::cv_UMatData_setPropHandle_voidX(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// userdata /usr/include/opencv2/core/mat.hpp:577
	#[inline]
	fn userdata(&mut self) -> *mut c_void {
		let ret = unsafe { sys::cv_UMatData_getPropUserdata(self.as_raw_mut_UMatData()) };
		ret
	}
	
	// userdata /usr/include/opencv2/core/mat.hpp:577
	#[inline]
	unsafe fn set_userdata(&mut self, val: *mut c_void) {
		let ret = { sys::cv_UMatData_setPropUserdata_voidX(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// allocatorFlags_ /usr/include/opencv2/core/mat.hpp:578
	#[inline]
	fn set_allocator_flags_(&mut self, val: i32) {
		let ret = unsafe { sys::cv_UMatData_setPropAllocatorFlags__int(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// mapcount /usr/include/opencv2/core/mat.hpp:579
	#[inline]
	fn set_mapcount(&mut self, val: i32) {
		let ret = unsafe { sys::cv_UMatData_setPropMapcount_int(self.as_raw_mut_UMatData(), val) };
		ret
	}
	
	// originalUMatData /usr/include/opencv2/core/mat.hpp:580
	#[inline]
	fn original_umat_data(&mut self) -> core::UMatData {
		let ret = unsafe { sys::cv_UMatData_getPropOriginalUMatData(self.as_raw_mut_UMatData()) };
		let ret = unsafe { core::UMatData::opencv_from_extern(ret) };
		ret
	}
	
	// originalUMatData /usr/include/opencv2/core/mat.hpp:580
	#[inline]
	fn set_original_umat_data(&mut self, val: &mut core::UMatData) {
		let ret = unsafe { sys::cv_UMatData_setPropOriginalUMatData_UMatDataX(self.as_raw_mut_UMatData(), val.as_raw_mut_UMatData()) };
		ret
	}
	
	// lock() /usr/include/opencv2/core/mat.hpp:554
	#[inline]
	fn lock(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_lock(self.as_raw_mut_UMatData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// unlock() /usr/include/opencv2/core/mat.hpp:555
	#[inline]
	fn unlock(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_unlock(self.as_raw_mut_UMatData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// markHostCopyObsolete(bool) /usr/include/opencv2/core/mat.hpp:563
	#[inline]
	fn mark_host_copy_obsolete(&mut self, flag: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_markHostCopyObsolete_bool(self.as_raw_mut_UMatData(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// markDeviceCopyObsolete(bool) /usr/include/opencv2/core/mat.hpp:564
	#[inline]
	fn mark_device_copy_obsolete(&mut self, flag: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_markDeviceCopyObsolete_bool(self.as_raw_mut_UMatData(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// markDeviceMemMapped(bool) /usr/include/opencv2/core/mat.hpp:565
	#[inline]
	fn mark_device_mem_mapped(&mut self, flag: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UMatData_markDeviceMemMapped_bool(self.as_raw_mut_UMatData(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// UMatData /usr/include/opencv2/core/mat.hpp:543
pub struct UMatData {
	ptr: *mut c_void
}

opencv_type_boxed! { UMatData }

impl Drop for UMatData {
	fn drop(&mut self) {
		extern "C" { fn cv_UMatData_delete(instance: *mut c_void); }
		unsafe { cv_UMatData_delete(self.as_raw_mut_UMatData()) };
	}
}

unsafe impl Send for UMatData {}

impl core::UMatDataTraitConst for UMatData {
	#[inline] fn as_raw_UMatData(&self) -> *const c_void { self.as_raw() }
}

impl core::UMatDataTrait for UMatData {
	#[inline] fn as_raw_mut_UMatData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl UMatData {
}

// _InputArray /usr/include/opencv2/core/mat.hpp:158
pub trait _InputArrayTraitConst {
	fn as_raw__InputArray(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * idx: -1
	// getMat(int) /usr/include/opencv2/core/mat.hpp:217
	#[inline]
	fn get_mat(&self, idx: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getMat_const_int(self.as_raw__InputArray(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	// getMat_(int) /usr/include/opencv2/core/mat.hpp:218
	#[inline]
	fn get_mat_(&self, idx: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getMat__const_int(self.as_raw__InputArray(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	// getUMat(int) /usr/include/opencv2/core/mat.hpp:219
	#[inline]
	fn get_umat(&self, idx: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getUMat_const_int(self.as_raw__InputArray(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getMatVector(std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:220
	#[inline]
	fn get_mat_vector(&self, mv: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getMatVector_const_vector_Mat_R(self.as_raw__InputArray(), mv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUMatVector(std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:221
	#[inline]
	fn get_umat_vector(&self, umv: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getUMatVector_const_vector_UMat_R(self.as_raw__InputArray(), umv.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getGpuMatVector(std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:222
	#[inline]
	fn get_gpu_mat_vector(&self, gpumv: &mut core::Vector<core::GpuMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getGpuMatVector_const_vector_GpuMat_R(self.as_raw__InputArray(), gpumv.as_raw_mut_VectorOfGpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getGpuMat() /usr/include/opencv2/core/mat.hpp:223
	#[inline]
	fn get_gpu_mat(&self) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getGpuMat_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getOGlBuffer() /usr/include/opencv2/core/mat.hpp:224
	#[inline]
	fn get_o_gl_buffer(&self) -> Result<core::Buffer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getOGlBuffer_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Buffer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getFlags() /usr/include/opencv2/core/mat.hpp:226
	#[inline]
	fn get_flags(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getFlags_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getObj() /usr/include/opencv2/core/mat.hpp:227
	#[inline]
	fn get_obj(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getObj_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSz() /usr/include/opencv2/core/mat.hpp:228
	#[inline]
	fn get_sz(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_getSz_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// kind() /usr/include/opencv2/core/mat.hpp:230
	#[inline]
	fn kind(&self) -> Result<core::_InputArray_KindFlag> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_kind_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// dims(int) /usr/include/opencv2/core/mat.hpp:231
	#[inline]
	fn dims(&self, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_dims_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// cols(int) /usr/include/opencv2/core/mat.hpp:232
	#[inline]
	fn cols(&self, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_cols_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// rows(int) /usr/include/opencv2/core/mat.hpp:233
	#[inline]
	fn rows(&self, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_rows_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// size(int) /usr/include/opencv2/core/mat.hpp:234
	#[inline]
	fn size(&self, i: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_size_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// sizend(int *, int) /usr/include/opencv2/core/mat.hpp:235
	#[inline]
	fn sizend(&self, sz: &mut i32, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_sizend_const_intX_int(self.as_raw__InputArray(), sz, i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// sameSize(const cv::_InputArray &) /usr/include/opencv2/core/mat.hpp:236
	#[inline]
	fn same_size(&self, arr: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_sameSize_const_const__InputArrayR(self.as_raw__InputArray(), arr.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// total(int) /usr/include/opencv2/core/mat.hpp:237
	#[inline]
	fn total(&self, i: i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_total_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// type(int) /usr/include/opencv2/core/mat.hpp:238
	#[inline]
	fn typ(&self, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_type_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// depth(int) /usr/include/opencv2/core/mat.hpp:239
	#[inline]
	fn depth(&self, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_depth_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// channels(int) /usr/include/opencv2/core/mat.hpp:240
	#[inline]
	fn channels(&self, i: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_channels_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// isContinuous(int) /usr/include/opencv2/core/mat.hpp:241
	#[inline]
	fn is_continuous(&self, i: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isContinuous_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// isSubmatrix(int) /usr/include/opencv2/core/mat.hpp:242
	#[inline]
	fn is_submatrix(&self, i: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isSubmatrix_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/mat.hpp:243
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_empty_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyTo(const cv::_OutputArray &) /usr/include/opencv2/core/mat.hpp:244
	#[inline]
	fn copy_to(&self, arr: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_copyTo_const_const__OutputArrayR(self.as_raw__InputArray(), arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyTo(const cv::_OutputArray &, const cv::_InputArray &) /usr/include/opencv2/core/mat.hpp:245
	#[inline]
	fn copy_to_masked(&self, arr: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		output_array_arg!(arr);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_copyTo_const_const__OutputArrayR_const__InputArrayR(self.as_raw__InputArray(), arr.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// offset(int) /usr/include/opencv2/core/mat.hpp:246
	#[inline]
	fn offset(&self, i: i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_offset_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// step(int) /usr/include/opencv2/core/mat.hpp:247
	#[inline]
	fn step(&self, i: i32) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_step_const_int(self.as_raw__InputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isMat() /usr/include/opencv2/core/mat.hpp:248
	#[inline]
	fn is_mat(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isMat_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isUMat() /usr/include/opencv2/core/mat.hpp:249
	#[inline]
	fn is_umat(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isUMat_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isMatVector() /usr/include/opencv2/core/mat.hpp:250
	#[inline]
	fn is_mat_vector(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isMatVector_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isUMatVector() /usr/include/opencv2/core/mat.hpp:251
	#[inline]
	fn is_umat_vector(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isUMatVector_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isMatx() /usr/include/opencv2/core/mat.hpp:252
	#[inline]
	fn is_matx(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isMatx_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isVector() /usr/include/opencv2/core/mat.hpp:253
	#[inline]
	fn is_vector(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isVector_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isGpuMat() /usr/include/opencv2/core/mat.hpp:254
	#[inline]
	fn is_gpu_mat(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isGpuMat_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isGpuMatVector() /usr/include/opencv2/core/mat.hpp:255
	#[inline]
	fn is_gpu_mat_vector(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray_isGpuMatVector_const(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait _InputArrayTrait: core::_InputArrayTraitConst {
	fn as_raw_mut__InputArray(&mut self) -> *mut c_void;

}

// _InputArray /usr/include/opencv2/core/mat.hpp:158
pub struct _InputArray {
	ptr: *mut c_void
}

opencv_type_boxed! { _InputArray }

impl Drop for _InputArray {
	fn drop(&mut self) {
		extern "C" { fn cv__InputArray_delete(instance: *mut c_void); }
		unsafe { cv__InputArray_delete(self.as_raw_mut__InputArray()) };
	}
}

unsafe impl Send for _InputArray {}

impl core::_InputArrayTraitConst for _InputArray {
	#[inline] fn as_raw__InputArray(&self) -> *const c_void { self.as_raw() }
}

impl core::_InputArrayTrait for _InputArray {
	#[inline] fn as_raw_mut__InputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl _InputArray {
	// _InputArray() /usr/include/opencv2/core/mat.hpp:189
	#[inline]
	pub fn default() -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(int, void *) /usr/include/opencv2/core/mat.hpp:190
	#[inline]
	pub unsafe fn new(_flags: i32, _obj: *mut c_void) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		{ sys::cv__InputArray__InputArray_int_voidX(_flags, _obj, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:191
	#[inline]
	pub fn from_mat(m: &core::Mat) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_MatR(m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:192
	#[inline]
	pub fn from_matexpr(expr: &core::MatExpr) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_MatExprR(expr.as_raw_MatExpr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:193
	#[inline]
	pub fn from_mat_vec(vec: &core::Vector<core::Mat>) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_vector_Mat_R(vec.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const std::vector<bool> &) /usr/include/opencv2/core/mat.hpp:196
	#[inline]
	pub fn from_bool_vec(vec: &core::Vector<bool>) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_vector_bool_R(vec.as_raw_VectorOfbool(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const double &) /usr/include/opencv2/core/mat.hpp:202
	#[inline]
	pub fn from_f64(val: &f64) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_doubleR(val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:203
	#[inline]
	pub fn from_gpumat(d_mat: &core::GpuMat) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_GpuMatR(d_mat.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:204
	#[inline]
	pub fn from_gpumat_vec(d_mat_array: &core::Vector<core::GpuMat>) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_vector_GpuMat_R(d_mat_array.as_raw_VectorOfGpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:205
	#[inline]
	pub fn new_1(buf: &core::Buffer) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_BufferR(buf.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:206
	#[inline]
	pub fn from_hostmem(cuda_mem: &core::HostMem) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_HostMemR(cuda_mem.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:208
	#[inline]
	pub fn from_umat(um: &core::UMat) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_UMatR(um.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputArray(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:209
	#[inline]
	pub fn from_umat_vec(umv: &core::Vector<core::UMat>) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputArray__InputArray_const_vector_UMat_R(umv.as_raw_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// _InputOutputArray /usr/include/opencv2/core/mat.hpp:385
pub trait _InputOutputArrayTraitConst: core::_OutputArrayTraitConst {
	fn as_raw__InputOutputArray(&self) -> *const c_void;

}

pub trait _InputOutputArrayTrait: core::_InputOutputArrayTraitConst + core::_OutputArrayTrait {
	fn as_raw_mut__InputOutputArray(&mut self) -> *mut c_void;

}

// _InputOutputArray /usr/include/opencv2/core/mat.hpp:385
pub struct _InputOutputArray {
	ptr: *mut c_void
}

opencv_type_boxed! { _InputOutputArray }

impl Drop for _InputOutputArray {
	fn drop(&mut self) {
		extern "C" { fn cv__InputOutputArray_delete(instance: *mut c_void); }
		unsafe { cv__InputOutputArray_delete(self.as_raw_mut__InputOutputArray()) };
	}
}

unsafe impl Send for _InputOutputArray {}

impl core::_InputArrayTraitConst for _InputOutputArray {
	#[inline] fn as_raw__InputArray(&self) -> *const c_void { self.as_raw() }
}

impl core::_InputArrayTrait for _InputOutputArray {
	#[inline] fn as_raw_mut__InputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::_OutputArrayTraitConst for _InputOutputArray {
	#[inline] fn as_raw__OutputArray(&self) -> *const c_void { self.as_raw() }
}

impl core::_OutputArrayTrait for _InputOutputArray {
	#[inline] fn as_raw_mut__OutputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::_InputOutputArrayTraitConst for _InputOutputArray {
	#[inline] fn as_raw__InputOutputArray(&self) -> *const c_void { self.as_raw() }
}

impl core::_InputOutputArrayTrait for _InputOutputArray {
	#[inline] fn as_raw_mut__InputOutputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl _InputOutputArray {
	// _InputOutputArray() /usr/include/opencv2/core/mat.hpp:388
	#[inline]
	pub fn default() -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(int, void *) /usr/include/opencv2/core/mat.hpp:389
	#[inline]
	pub unsafe fn new(_flags: i32, _obj: *mut c_void) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		{ sys::cv__InputOutputArray__InputOutputArray_int_voidX(_flags, _obj, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(cv::Mat &) /usr/include/opencv2/core/mat.hpp:390
	#[inline]
	pub fn from_mat_mut(m: &mut core::Mat) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_MatR(m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:391
	#[inline]
	pub fn from_mat_vec_mut(vec: &mut core::Vector<core::Mat>) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_vector_Mat_R(vec.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:392
	#[inline]
	pub fn from_gpumat_mut(d_mat: &mut core::GpuMat) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_GpuMatR(d_mat.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:393
	#[inline]
	pub fn new_1(buf: &mut core::Buffer) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_BufferR(buf.as_raw_mut_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:394
	#[inline]
	pub fn from_hostmem_mut(cuda_mem: &mut core::HostMem) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_HostMemR(cuda_mem.as_raw_mut_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(cv::UMat &) /usr/include/opencv2/core/mat.hpp:403
	#[inline]
	pub fn from_umat_mut(m: &mut core::UMat) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_UMatR(m.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:404
	#[inline]
	pub fn from_umat_vec_mut(vec: &mut core::Vector<core::UMat>) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_vector_UMat_R(vec.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:406
	#[inline]
	pub fn from_mat(m: &core::Mat) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_MatR(m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:407
	#[inline]
	pub fn from_mat_vec(vec: &core::Vector<core::Mat>) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_vector_Mat_R(vec.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:408
	#[inline]
	pub fn from_gpumat(d_mat: &core::GpuMat) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_GpuMatR(d_mat.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(const std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:409
	#[inline]
	pub fn from_gpumat_vec(d_mat: &core::Vector<core::GpuMat>) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_vector_GpuMat_R(d_mat.as_raw_VectorOfGpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(const ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:410
	#[inline]
	pub fn new_2(buf: &core::Buffer) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_BufferR(buf.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(const cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:411
	#[inline]
	pub fn from_hostmem(cuda_mem: &core::HostMem) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_HostMemR(cuda_mem.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:419
	#[inline]
	pub fn from_umat(m: &core::UMat) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_UMatR(m.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _InputOutputArray(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:420
	#[inline]
	pub fn from_umat_vec(vec: &core::Vector<core::UMat>) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_vector_UMat_R(vec.as_raw_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { _InputOutputArray, core::_InputArray, cv__InputOutputArray_to__InputArray }

boxed_cast_base! { _InputOutputArray, core::_OutputArray, cv__InputOutputArray_to__OutputArray }

// _OutputArray /usr/include/opencv2/core/mat.hpp:294
pub trait _OutputArrayTraitConst: core::_InputArrayTraitConst {
	fn as_raw__OutputArray(&self) -> *const c_void;

	// fixedSize() /usr/include/opencv2/core/mat.hpp:357
	#[inline]
	fn fixed_size(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_fixedSize_const(self.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// fixedType() /usr/include/opencv2/core/mat.hpp:358
	#[inline]
	fn fixed_type(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_fixedType_const(self.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// needed() /usr/include/opencv2/core/mat.hpp:359
	#[inline]
	fn needed(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_needed_const(self.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// getMatRef(int) /usr/include/opencv2/core/mat.hpp:360
	#[inline]
	fn get_mat_ref(&self, i: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_getMatRef_const_int(self.as_raw__OutputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	// getUMatRef(int) /usr/include/opencv2/core/mat.hpp:361
	#[inline]
	fn get_umat_ref(&self, i: i32) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_getUMatRef_const_int(self.as_raw__OutputArray(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getGpuMatRef() /usr/include/opencv2/core/mat.hpp:362
	#[inline]
	fn get_gpu_mat_ref(&self) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_getGpuMatRef_const(self.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getGpuMatVecRef() /usr/include/opencv2/core/mat.hpp:363
	#[inline]
	fn get_gpu_mat_vec_ref(&self) -> Result<core::Vector<core::GpuMat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_getGpuMatVecRef_const(self.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::GpuMat>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getOGlBufferRef() /usr/include/opencv2/core/mat.hpp:364
	#[inline]
	fn get_o_gl_buffer_ref(&self) -> Result<core::Buffer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_getOGlBufferRef_const(self.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Buffer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getHostMemRef() /usr/include/opencv2/core/mat.hpp:365
	#[inline]
	fn get_host_mem_ref(&self) -> Result<core::HostMem> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_getHostMemRef_const(self.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::HostMem::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	/// * allow_transposed: false
	/// * fixed_depth_mask: static_cast<_OutputArray::DepthMask>(0)
	// create(cv::Size, int, int, bool, _OutputArray::DepthMask) /usr/include/opencv2/core/mat.hpp:366
	#[inline]
	fn create_size(&self, sz: core::Size, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_create_const_Size_int_int_bool_DepthMask(self.as_raw__OutputArray(), sz.opencv_as_extern(), typ, i, allow_transposed, fixed_depth_mask, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	/// * allow_transposed: false
	/// * fixed_depth_mask: static_cast<_OutputArray::DepthMask>(0)
	// create(int, int, int, int, bool, _OutputArray::DepthMask) /usr/include/opencv2/core/mat.hpp:367
	#[inline]
	fn create(&self, rows: i32, cols: i32, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_create_const_int_int_int_int_bool_DepthMask(self.as_raw__OutputArray(), rows, cols, typ, i, allow_transposed, fixed_depth_mask, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: -1
	/// * allow_transposed: false
	/// * fixed_depth_mask: static_cast<_OutputArray::DepthMask>(0)
	// create(int, const int *, int, int, bool, _OutputArray::DepthMask) /usr/include/opencv2/core/mat.hpp:368
	#[inline]
	fn create_nd(&self, size: &[i32], typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_create_const_int_const_intX_int_int_bool_DepthMask(self.as_raw__OutputArray(), size.len() as _, size.as_ptr(), typ, i, allow_transposed, fixed_depth_mask, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// createSameSize(const cv::_InputArray &, int) /usr/include/opencv2/core/mat.hpp:369
	#[inline]
	unsafe fn create_same_size(&self, arr: &dyn core::ToInputArray, mtype: i32) -> Result<()> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		{ sys::cv__OutputArray_createSameSize_const_const__InputArrayR_int(self.as_raw__OutputArray(), arr.as_raw__InputArray(), mtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/mat.hpp:370
	#[inline]
	fn release(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_release_const(self.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clear() /usr/include/opencv2/core/mat.hpp:371
	#[inline]
	fn clear(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_clear_const(self.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mask: _InputArray()
	// setTo(const cv::_InputArray &, const cv::_InputArray &) /usr/include/opencv2/core/mat.hpp:372
	#[inline]
	fn set_to(&self, value: &dyn core::ToInputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(value);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_setTo_const_const__InputArrayR_const__InputArrayR(self.as_raw__OutputArray(), value.as_raw__InputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// assign(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:374
	#[inline]
	fn assign(&self, u: &core::UMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_assign_const_const_UMatR(self.as_raw__OutputArray(), u.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// assign(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:375
	#[inline]
	fn assign_1(&self, m: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_assign_const_const_MatR(self.as_raw__OutputArray(), m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// assign(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:377
	#[inline]
	fn assign_2(&self, v: &core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_assign_const_const_vector_UMat_R(self.as_raw__OutputArray(), v.as_raw_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// assign(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:378
	#[inline]
	fn assign_3(&self, v: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_assign_const_const_vector_Mat_R(self.as_raw__OutputArray(), v.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// move(cv::UMat &) /usr/include/opencv2/core/mat.hpp:380
	#[inline]
	fn move_(&self, u: &mut core::UMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_move_const_UMatR(self.as_raw__OutputArray(), u.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// move(cv::Mat &) /usr/include/opencv2/core/mat.hpp:381
	#[inline]
	fn move__1(&self, m: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray_move_const_MatR(self.as_raw__OutputArray(), m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait _OutputArrayTrait: core::_InputArrayTrait + core::_OutputArrayTraitConst {
	fn as_raw_mut__OutputArray(&mut self) -> *mut c_void;

}

// _OutputArray /usr/include/opencv2/core/mat.hpp:294
pub struct _OutputArray {
	ptr: *mut c_void
}

opencv_type_boxed! { _OutputArray }

impl Drop for _OutputArray {
	fn drop(&mut self) {
		extern "C" { fn cv__OutputArray_delete(instance: *mut c_void); }
		unsafe { cv__OutputArray_delete(self.as_raw_mut__OutputArray()) };
	}
}

unsafe impl Send for _OutputArray {}

impl core::_InputArrayTraitConst for _OutputArray {
	#[inline] fn as_raw__InputArray(&self) -> *const c_void { self.as_raw() }
}

impl core::_InputArrayTrait for _OutputArray {
	#[inline] fn as_raw_mut__InputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::_OutputArrayTraitConst for _OutputArray {
	#[inline] fn as_raw__OutputArray(&self) -> *const c_void { self.as_raw() }
}

impl core::_OutputArrayTrait for _OutputArray {
	#[inline] fn as_raw_mut__OutputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl _OutputArray {
	// _OutputArray() /usr/include/opencv2/core/mat.hpp:313
	#[inline]
	pub fn default() -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(int, void *) /usr/include/opencv2/core/mat.hpp:314
	#[inline]
	pub unsafe fn new(_flags: i32, _obj: *mut c_void) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		{ sys::cv__OutputArray__OutputArray_int_voidX(_flags, _obj, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(cv::Mat &) /usr/include/opencv2/core/mat.hpp:315
	#[inline]
	pub fn from_mat_mut(m: &mut core::Mat) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_MatR(m.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:316
	#[inline]
	pub fn from_mat_vec_mut(vec: &mut core::Vector<core::Mat>) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_vector_Mat_R(vec.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:317
	#[inline]
	pub fn from_gpumat_mut(d_mat: &mut core::GpuMat) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_GpuMatR(d_mat.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:318
	#[inline]
	pub fn from_gpumat_vec_mut(d_mat: &mut core::Vector<core::GpuMat>) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_vector_GpuMat_R(d_mat.as_raw_mut_VectorOfGpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:319
	#[inline]
	pub fn new_1(buf: &mut core::Buffer) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_BufferR(buf.as_raw_mut_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:320
	#[inline]
	pub fn from_hostmem_mut(cuda_mem: &mut core::HostMem) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_HostMemR(cuda_mem.as_raw_mut_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(cv::UMat &) /usr/include/opencv2/core/mat.hpp:330
	#[inline]
	pub fn from_umat_mut(m: &mut core::UMat) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_UMatR(m.as_raw_mut_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:331
	#[inline]
	pub fn from_umat_vec_mut(vec: &mut core::Vector<core::UMat>) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_vector_UMat_R(vec.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:333
	#[inline]
	pub fn from_mat(m: &core::Mat) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_const_MatR(m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:334
	#[inline]
	pub fn from_mat_vec(vec: &core::Vector<core::Mat>) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_const_vector_Mat_R(vec.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:335
	#[inline]
	pub fn from_gpumat(d_mat: &core::GpuMat) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_const_GpuMatR(d_mat.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(const ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:337
	#[inline]
	pub fn new_2(buf: &core::Buffer) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_const_BufferR(buf.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(const cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:338
	#[inline]
	pub fn from_hostmem(cuda_mem: &core::HostMem) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_const_HostMemR(cuda_mem.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:346
	#[inline]
	pub fn from_umat(m: &core::UMat) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_const_UMatR(m.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// _OutputArray(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:347
	#[inline]
	pub fn from_umat_vec(vec: &core::Vector<core::UMat>) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv__OutputArray__OutputArray_const_vector_UMat_R(vec.as_raw_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { _OutputArray, core::_InputArray, cv__OutputArray_to__InputArray }

// BufferPool /usr/include/opencv2/core/cuda.hpp:687
pub trait BufferPoolTraitConst {
	fn as_raw_BufferPool(&self) -> *const c_void;

	// getAllocator() /usr/include/opencv2/core/cuda.hpp:701
	#[inline]
	fn get_allocator(&self) -> Result<core::Ptr<dyn core::GpuMat_Allocator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BufferPool_getAllocator_const(self.as_raw_BufferPool(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn core::GpuMat_Allocator>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait BufferPoolTrait: core::BufferPoolTraitConst {
	fn as_raw_mut_BufferPool(&mut self) -> *mut c_void;

	// getBuffer(int, int, int) /usr/include/opencv2/core/cuda.hpp:695
	#[inline]
	fn get_buffer(&mut self, rows: i32, cols: i32, typ: i32) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BufferPool_getBuffer_int_int_int(self.as_raw_mut_BufferPool(), rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getBuffer(cv::Size, int) /usr/include/opencv2/core/cuda.hpp:698
	#[inline]
	fn get_buffer_1(&mut self, size: core::Size, typ: i32) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BufferPool_getBuffer_Size_int(self.as_raw_mut_BufferPool(), size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// BufferPool /usr/include/opencv2/core/cuda.hpp:687
pub struct BufferPool {
	ptr: *mut c_void
}

opencv_type_boxed! { BufferPool }

impl Drop for BufferPool {
	fn drop(&mut self) {
		extern "C" { fn cv_BufferPool_delete(instance: *mut c_void); }
		unsafe { cv_BufferPool_delete(self.as_raw_mut_BufferPool()) };
	}
}

unsafe impl Send for BufferPool {}

impl core::BufferPoolTraitConst for BufferPool {
	#[inline] fn as_raw_BufferPool(&self) -> *const c_void { self.as_raw() }
}

impl core::BufferPoolTrait for BufferPool {
	#[inline] fn as_raw_mut_BufferPool(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BufferPool {
	// BufferPool(cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:692
	#[inline]
	pub fn new(stream: &mut core::Stream) -> Result<core::BufferPool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BufferPool_BufferPool_StreamR(stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::BufferPool::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// DeviceInfo /usr/include/opencv2/core/cuda.hpp:1045
pub trait DeviceInfoTraitConst {
	fn as_raw_DeviceInfo(&self) -> *const c_void;

	// deviceID() /usr/include/opencv2/core/cuda.hpp:1062
	#[inline]
	fn device_id(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_deviceID_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// name() /usr/include/opencv2/core/cuda.hpp:1065
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_name_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// totalGlobalMem() /usr/include/opencv2/core/cuda.hpp:1068
	#[inline]
	fn total_global_mem(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_totalGlobalMem_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// sharedMemPerBlock() /usr/include/opencv2/core/cuda.hpp:1071
	#[inline]
	fn shared_mem_per_block(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_sharedMemPerBlock_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// regsPerBlock() /usr/include/opencv2/core/cuda.hpp:1074
	#[inline]
	fn regs_per_block(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_regsPerBlock_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// warpSize() /usr/include/opencv2/core/cuda.hpp:1077
	#[inline]
	fn warp_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_warpSize_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// memPitch() /usr/include/opencv2/core/cuda.hpp:1080
	#[inline]
	fn mem_pitch(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_memPitch_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxThreadsPerBlock() /usr/include/opencv2/core/cuda.hpp:1083
	#[inline]
	fn max_threads_per_block(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxThreadsPerBlock_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxThreadsDim() /usr/include/opencv2/core/cuda.hpp:1086
	#[inline]
	fn max_threads_dim(&self) -> Result<core::Vec3i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxThreadsDim_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxGridSize() /usr/include/opencv2/core/cuda.hpp:1089
	#[inline]
	fn max_grid_size(&self) -> Result<core::Vec3i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxGridSize_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clockRate() /usr/include/opencv2/core/cuda.hpp:1092
	#[inline]
	fn clock_rate(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_clockRate_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// totalConstMem() /usr/include/opencv2/core/cuda.hpp:1095
	#[inline]
	fn total_const_mem(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_totalConstMem_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// majorVersion() /usr/include/opencv2/core/cuda.hpp:1098
	#[inline]
	fn major_version(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_majorVersion_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// minorVersion() /usr/include/opencv2/core/cuda.hpp:1101
	#[inline]
	fn minor_version(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_minorVersion_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// textureAlignment() /usr/include/opencv2/core/cuda.hpp:1104
	#[inline]
	fn texture_alignment(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_textureAlignment_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// texturePitchAlignment() /usr/include/opencv2/core/cuda.hpp:1107
	#[inline]
	fn texture_pitch_alignment(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_texturePitchAlignment_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// multiProcessorCount() /usr/include/opencv2/core/cuda.hpp:1110
	#[inline]
	fn multi_processor_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_multiProcessorCount_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// kernelExecTimeoutEnabled() /usr/include/opencv2/core/cuda.hpp:1113
	#[inline]
	fn kernel_exec_timeout_enabled(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_kernelExecTimeoutEnabled_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// integrated() /usr/include/opencv2/core/cuda.hpp:1116
	#[inline]
	fn integrated(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_integrated_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// canMapHostMemory() /usr/include/opencv2/core/cuda.hpp:1119
	#[inline]
	fn can_map_host_memory(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_canMapHostMemory_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// computeMode() /usr/include/opencv2/core/cuda.hpp:1130
	#[inline]
	fn compute_mode(&self) -> Result<core::DeviceInfo_ComputeMode> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_computeMode_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture1D() /usr/include/opencv2/core/cuda.hpp:1133
	#[inline]
	fn max_texture1_d(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture1D_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture1DMipmap() /usr/include/opencv2/core/cuda.hpp:1136
	#[inline]
	fn max_texture1_d_mipmap(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture1DMipmap_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture1DLinear() /usr/include/opencv2/core/cuda.hpp:1139
	#[inline]
	fn max_texture1_d_linear(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture1DLinear_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture2D() /usr/include/opencv2/core/cuda.hpp:1142
	#[inline]
	fn max_texture_2d(&self) -> Result<core::Vec2i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2D_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture2DMipmap() /usr/include/opencv2/core/cuda.hpp:1145
	#[inline]
	fn max_texture2_d_mipmap(&self) -> Result<core::Vec2i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2DMipmap_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture2DLinear() /usr/include/opencv2/core/cuda.hpp:1148
	#[inline]
	fn max_texture2_d_linear(&self) -> Result<core::Vec3i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2DLinear_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture2DGather() /usr/include/opencv2/core/cuda.hpp:1151
	#[inline]
	fn max_texture2_d_gather(&self) -> Result<core::Vec2i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2DGather_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture3D() /usr/include/opencv2/core/cuda.hpp:1154
	#[inline]
	fn max_texture_3d(&self) -> Result<core::Vec3i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture3D_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTextureCubemap() /usr/include/opencv2/core/cuda.hpp:1157
	#[inline]
	fn max_texture_cubemap(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTextureCubemap_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture1DLayered() /usr/include/opencv2/core/cuda.hpp:1160
	#[inline]
	fn max_texture1_d_layered(&self) -> Result<core::Vec2i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture1DLayered_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTexture2DLayered() /usr/include/opencv2/core/cuda.hpp:1163
	#[inline]
	fn max_texture2_d_layered(&self) -> Result<core::Vec3i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2DLayered_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxTextureCubemapLayered() /usr/include/opencv2/core/cuda.hpp:1166
	#[inline]
	fn max_texture_cubemap_layered(&self) -> Result<core::Vec2i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxTextureCubemapLayered_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxSurface1D() /usr/include/opencv2/core/cuda.hpp:1169
	#[inline]
	fn max_surface1_d(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface1D_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxSurface2D() /usr/include/opencv2/core/cuda.hpp:1172
	#[inline]
	fn max_surface_2d(&self) -> Result<core::Vec2i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface2D_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxSurface3D() /usr/include/opencv2/core/cuda.hpp:1175
	#[inline]
	fn max_surface_3d(&self) -> Result<core::Vec3i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface3D_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxSurface1DLayered() /usr/include/opencv2/core/cuda.hpp:1178
	#[inline]
	fn max_surface1_d_layered(&self) -> Result<core::Vec2i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface1DLayered_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxSurface2DLayered() /usr/include/opencv2/core/cuda.hpp:1181
	#[inline]
	fn max_surface2_d_layered(&self) -> Result<core::Vec3i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface2DLayered_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxSurfaceCubemap() /usr/include/opencv2/core/cuda.hpp:1184
	#[inline]
	fn max_surface_cubemap(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxSurfaceCubemap_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxSurfaceCubemapLayered() /usr/include/opencv2/core/cuda.hpp:1187
	#[inline]
	fn max_surface_cubemap_layered(&self) -> Result<core::Vec2i> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxSurfaceCubemapLayered_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// surfaceAlignment() /usr/include/opencv2/core/cuda.hpp:1190
	#[inline]
	fn surface_alignment(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_surfaceAlignment_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// concurrentKernels() /usr/include/opencv2/core/cuda.hpp:1193
	#[inline]
	fn concurrent_kernels(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_concurrentKernels_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// ECCEnabled() /usr/include/opencv2/core/cuda.hpp:1196
	#[inline]
	fn ecc_enabled(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_ECCEnabled_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// pciBusID() /usr/include/opencv2/core/cuda.hpp:1199
	#[inline]
	fn pci_bus_id(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_pciBusID_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// pciDeviceID() /usr/include/opencv2/core/cuda.hpp:1202
	#[inline]
	fn pci_device_id(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_pciDeviceID_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// pciDomainID() /usr/include/opencv2/core/cuda.hpp:1205
	#[inline]
	fn pci_domain_id(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_pciDomainID_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// tccDriver() /usr/include/opencv2/core/cuda.hpp:1208
	#[inline]
	fn tcc_driver(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_tccDriver_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// asyncEngineCount() /usr/include/opencv2/core/cuda.hpp:1211
	#[inline]
	fn async_engine_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_asyncEngineCount_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// unifiedAddressing() /usr/include/opencv2/core/cuda.hpp:1214
	#[inline]
	fn unified_addressing(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_unifiedAddressing_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// memoryClockRate() /usr/include/opencv2/core/cuda.hpp:1217
	#[inline]
	fn memory_clock_rate(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_memoryClockRate_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// memoryBusWidth() /usr/include/opencv2/core/cuda.hpp:1220
	#[inline]
	fn memory_bus_width(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_memoryBusWidth_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// l2CacheSize() /usr/include/opencv2/core/cuda.hpp:1223
	#[inline]
	fn l2_cache_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_l2CacheSize_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxThreadsPerMultiProcessor() /usr/include/opencv2/core/cuda.hpp:1226
	#[inline]
	fn max_threads_per_multi_processor(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_maxThreadsPerMultiProcessor_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// queryMemory(size_t &, size_t &) /usr/include/opencv2/core/cuda.hpp:1229
	#[inline]
	fn query_memory(&self, total_memory: &mut size_t, free_memory: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_queryMemory_const_size_tR_size_tR(self.as_raw_DeviceInfo(), total_memory, free_memory, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// freeMemory() /usr/include/opencv2/core/cuda.hpp:1230
	#[inline]
	fn free_memory(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_freeMemory_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// totalMemory() /usr/include/opencv2/core/cuda.hpp:1231
	#[inline]
	fn total_memory(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_totalMemory_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// supports(cv::cuda::FeatureSet) /usr/include/opencv2/core/cuda.hpp:1239
	#[inline]
	fn supports(&self, feature_set: core::FeatureSet) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_supports_const_FeatureSet(self.as_raw_DeviceInfo(), feature_set, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isCompatible() /usr/include/opencv2/core/cuda.hpp:1246
	#[inline]
	fn is_compatible(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_isCompatible_const(self.as_raw_DeviceInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DeviceInfoTrait: core::DeviceInfoTraitConst {
	fn as_raw_mut_DeviceInfo(&mut self) -> *mut c_void;

}

// DeviceInfo /usr/include/opencv2/core/cuda.hpp:1045
pub struct DeviceInfo {
	ptr: *mut c_void
}

opencv_type_boxed! { DeviceInfo }

impl Drop for DeviceInfo {
	fn drop(&mut self) {
		extern "C" { fn cv_DeviceInfo_delete(instance: *mut c_void); }
		unsafe { cv_DeviceInfo_delete(self.as_raw_mut_DeviceInfo()) };
	}
}

unsafe impl Send for DeviceInfo {}

impl core::DeviceInfoTraitConst for DeviceInfo {
	#[inline] fn as_raw_DeviceInfo(&self) -> *const c_void { self.as_raw() }
}

impl core::DeviceInfoTrait for DeviceInfo {
	#[inline] fn as_raw_mut_DeviceInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DeviceInfo {
	// DeviceInfo() /usr/include/opencv2/core/cuda.hpp:1049
	#[inline]
	pub fn default() -> Result<core::DeviceInfo> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_DeviceInfo(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::DeviceInfo::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// DeviceInfo(int) /usr/include/opencv2/core/cuda.hpp:1058
	#[inline]
	pub fn new(device_id: i32) -> Result<core::DeviceInfo> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_DeviceInfo_DeviceInfo_int(device_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::DeviceInfo::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Event /usr/include/opencv2/core/cuda.hpp:916
pub trait EventTraitConst {
	fn as_raw_Event(&self) -> *const c_void;

	// queryIfComplete() /usr/include/opencv2/core/cuda.hpp:933
	#[inline]
	fn query_if_complete(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Event_queryIfComplete_const(self.as_raw_Event(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait EventTrait: core::EventTraitConst {
	fn as_raw_mut_Event(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * stream: Stream::Null()
	// record(cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:930
	#[inline]
	fn record(&mut self, stream: &mut core::Stream) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Event_record_StreamR(self.as_raw_mut_Event(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// waitForCompletion() /usr/include/opencv2/core/cuda.hpp:936
	#[inline]
	fn wait_for_completion(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Event_waitForCompletion(self.as_raw_mut_Event(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Event /usr/include/opencv2/core/cuda.hpp:916
pub struct Event {
	ptr: *mut c_void
}

opencv_type_boxed! { Event }

impl Drop for Event {
	fn drop(&mut self) {
		extern "C" { fn cv_Event_delete(instance: *mut c_void); }
		unsafe { cv_Event_delete(self.as_raw_mut_Event()) };
	}
}

unsafe impl Send for Event {}

impl core::EventTraitConst for Event {
	#[inline] fn as_raw_Event(&self) -> *const c_void { self.as_raw() }
}

impl core::EventTrait for Event {
	#[inline] fn as_raw_mut_Event(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Event {
	/// ## C++ default parameters
	/// * flags: Event::CreateFlags::DEFAULT
	// Event(const Event::CreateFlags) /usr/include/opencv2/core/cuda.hpp:927
	#[inline]
	pub fn new(flags: core::Event_CreateFlags) -> Result<core::Event> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Event_Event_const_CreateFlags(flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Event::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// elapsedTime(const cv::cuda::Event &, const cv::cuda::Event &) /usr/include/opencv2/core/cuda.hpp:939
	#[inline]
	pub fn elapsed_time(start: &core::Event, end: &core::Event) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Event_elapsedTime_const_EventR_const_EventR(start.as_raw_Event(), end.as_raw_Event(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// GpuData /usr/include/opencv2/core/cuda.hpp:343
pub trait GpuDataTraitConst {
	fn as_raw_GpuData(&self) -> *const c_void;

	// size /usr/include/opencv2/core/cuda.hpp:355
	#[inline]
	fn size(&self) -> size_t {
		let ret = unsafe { sys::cv_cuda_GpuData_getPropSize_const(self.as_raw_GpuData()) };
		ret
	}
	
}

pub trait GpuDataTrait: core::GpuDataTraitConst {
	fn as_raw_mut_GpuData(&mut self) -> *mut c_void;

	// data /usr/include/opencv2/core/cuda.hpp:354
	#[inline]
	fn data(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_cuda_GpuData_getPropData(self.as_raw_mut_GpuData()) };
		ret
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:354
	#[inline]
	unsafe fn set_data(&mut self, val: *mut u8) {
		let ret = { sys::cv_cuda_GpuData_setPropData_unsigned_charX(self.as_raw_mut_GpuData(), val) };
		ret
	}
	
	// size /usr/include/opencv2/core/cuda.hpp:355
	#[inline]
	fn set_size(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_cuda_GpuData_setPropSize_size_t(self.as_raw_mut_GpuData(), val) };
		ret
	}
	
}

// GpuData /usr/include/opencv2/core/cuda.hpp:343
pub struct GpuData {
	ptr: *mut c_void
}

opencv_type_boxed! { GpuData }

impl Drop for GpuData {
	fn drop(&mut self) {
		extern "C" { fn cv_GpuData_delete(instance: *mut c_void); }
		unsafe { cv_GpuData_delete(self.as_raw_mut_GpuData()) };
	}
}

unsafe impl Send for GpuData {}

impl core::GpuDataTraitConst for GpuData {
	#[inline] fn as_raw_GpuData(&self) -> *const c_void { self.as_raw() }
}

impl core::GpuDataTrait for GpuData {
	#[inline] fn as_raw_mut_GpuData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GpuData {
	// GpuData(size_t) /usr/include/opencv2/core/cuda.hpp:345
	#[inline]
	pub fn new(_size: size_t) -> Result<core::GpuData> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuData_GpuData_size_t(_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuData::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// GpuMat /usr/include/opencv2/core/cuda.hpp:105
pub trait GpuMatTraitConst {
	fn as_raw_GpuMat(&self) -> *const c_void;

	// flags /usr/include/opencv2/core/cuda.hpp:320
	#[inline]
	fn flags(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_GpuMat_getPropFlags_const(self.as_raw_GpuMat()) };
		ret
	}
	
	// rows /usr/include/opencv2/core/cuda.hpp:323
	#[inline]
	fn rows(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_GpuMat_getPropRows_const(self.as_raw_GpuMat()) };
		ret
	}
	
	// cols /usr/include/opencv2/core/cuda.hpp:323
	#[inline]
	fn cols(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_GpuMat_getPropCols_const(self.as_raw_GpuMat()) };
		ret
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:326
	#[inline]
	fn step(&self) -> size_t {
		let ret = unsafe { sys::cv_cuda_GpuMat_getPropStep_const(self.as_raw_GpuMat()) };
		ret
	}
	
	// dataend /usr/include/opencv2/core/cuda.hpp:337
	#[inline]
	fn dataend(&self) -> *const u8 {
		let ret = unsafe { sys::cv_cuda_GpuMat_getPropDataend_const(self.as_raw_GpuMat()) };
		ret
	}
	
	// download(cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:185
	#[inline]
	fn download(&self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_download_const_const__OutputArrayR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// download(cv::OutputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:195
	#[inline]
	fn download_async(&self, dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_download_const_const__OutputArrayR_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clone() /usr/include/opencv2/core/cuda.hpp:198
	#[inline]
	fn try_clone(&self) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_clone_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:201
	#[inline]
	fn copy_to(&self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_copyTo_const_const__OutputArrayR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyTo(cv::OutputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:204
	#[inline]
	fn copy_to_1(&self, dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyTo(cv::OutputArray, cv::InputArray) /usr/include/opencv2/core/cuda.hpp:207
	#[inline]
	fn copy_to_2(&self, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyTo(cv::OutputArray, cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:210
	#[inline]
	fn copy_to_3(&self, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// convertTo(cv::OutputArray, int) /usr/include/opencv2/core/cuda.hpp:225
	#[inline]
	fn convert_to(&self, dst: &mut dyn core::ToOutputArray, rtype: i32) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// convertTo(cv::OutputArray, int, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:228
	#[inline]
	fn convert_to_1(&self, dst: &mut dyn core::ToOutputArray, rtype: i32, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * beta: 0.0
	// convertTo(cv::OutputArray, int, double, double) /usr/include/opencv2/core/cuda.hpp:231
	#[inline]
	fn convert_to_2(&self, dst: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype, alpha, beta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// convertTo(cv::OutputArray, int, double, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:234
	#[inline]
	fn convert_to_3(&self, dst: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype, alpha, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// convertTo(cv::OutputArray, int, double, double, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:237
	#[inline]
	fn convert_to_4(&self, dst: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, beta: f64, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype, alpha, beta, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * typ: -1
	// assignTo(cv::cuda::GpuMat &, int) /usr/include/opencv2/core/cuda.hpp:239
	#[inline]
	fn assign_to(&self, m: &mut core::GpuMat, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_assignTo_const_GpuMatR_int(self.as_raw_GpuMat(), m.as_raw_mut_GpuMat(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * y: 0
	// ptr(int) /usr/include/opencv2/core/cuda.hpp:243
	#[inline]
	fn ptr(&self, y: i32) -> Result<*const u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_ptr_const_int(self.as_raw_GpuMat(), y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// row(int) /usr/include/opencv2/core/cuda.hpp:253
	#[inline]
	fn row(&self, y: i32) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_row_const_int(self.as_raw_GpuMat(), y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// col(int) /usr/include/opencv2/core/cuda.hpp:256
	#[inline]
	fn col(&self, x: i32) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_col_const_int(self.as_raw_GpuMat(), x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// rowRange(int, int) /usr/include/opencv2/core/cuda.hpp:259
	#[inline]
	fn row_range(&self, startrow: i32, endrow: i32) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_rowRange_const_int_int(self.as_raw_GpuMat(), startrow, endrow, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// rowRange(cv::Range) /usr/include/opencv2/core/cuda.hpp:260
	#[inline]
	fn row_range_1(&self, mut r: core::Range) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_rowRange_const_Range(self.as_raw_GpuMat(), r.as_raw_mut_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// colRange(int, int) /usr/include/opencv2/core/cuda.hpp:263
	#[inline]
	fn col_range(&self, startcol: i32, endcol: i32) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_colRange_const_int_int(self.as_raw_GpuMat(), startcol, endcol, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// colRange(cv::Range) /usr/include/opencv2/core/cuda.hpp:264
	#[inline]
	fn col_range_1(&self, mut r: core::Range) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_colRange_const_Range(self.as_raw_GpuMat(), r.as_raw_mut_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * rows: 0
	// reshape(int, int) /usr/include/opencv2/core/cuda.hpp:272
	#[inline]
	fn reshape(&self, cn: i32, rows: i32) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_reshape_const_int_int(self.as_raw_GpuMat(), cn, rows, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// locateROI(cv::Size &, cv::Point &) /usr/include/opencv2/core/cuda.hpp:275
	#[inline]
	fn locate_roi(&self, whole_size: &mut core::Size, ofs: &mut core::Point) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_locateROI_const_SizeR_PointR(self.as_raw_GpuMat(), whole_size, ofs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isContinuous() /usr/include/opencv2/core/cuda.hpp:282
	#[inline]
	fn is_continuous(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_isContinuous_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize() /usr/include/opencv2/core/cuda.hpp:285
	#[inline]
	fn elem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_elemSize_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize1() /usr/include/opencv2/core/cuda.hpp:288
	#[inline]
	fn elem_size1(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_elemSize1_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// type() /usr/include/opencv2/core/cuda.hpp:291
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_type_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// depth() /usr/include/opencv2/core/cuda.hpp:294
	#[inline]
	fn depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_depth_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// channels() /usr/include/opencv2/core/cuda.hpp:297
	#[inline]
	fn channels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_channels_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// step1() /usr/include/opencv2/core/cuda.hpp:300
	#[inline]
	fn step1(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_step1_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// size() /usr/include/opencv2/core/cuda.hpp:303
	#[inline]
	fn size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_size_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/cuda.hpp:306
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_empty_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// cudaPtr() /usr/include/opencv2/core/cuda.hpp:309
	#[inline]
	fn cuda_ptr(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_cudaPtr_const(self.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GpuMatTrait: core::GpuMatTraitConst {
	fn as_raw_mut_GpuMat(&mut self) -> *mut c_void;

	// flags /usr/include/opencv2/core/cuda.hpp:320
	#[inline]
	fn set_flags(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_GpuMat_setPropFlags_int(self.as_raw_mut_GpuMat(), val) };
		ret
	}
	
	// rows /usr/include/opencv2/core/cuda.hpp:323
	#[inline]
	fn set_rows(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_GpuMat_setPropRows_int(self.as_raw_mut_GpuMat(), val) };
		ret
	}
	
	// cols /usr/include/opencv2/core/cuda.hpp:323
	#[inline]
	fn set_cols(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_GpuMat_setPropCols_int(self.as_raw_mut_GpuMat(), val) };
		ret
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:326
	#[inline]
	fn set_step(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_cuda_GpuMat_setPropStep_size_t(self.as_raw_mut_GpuMat(), val) };
		ret
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:329
	#[inline]
	fn data(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_cuda_GpuMat_getPropData(self.as_raw_mut_GpuMat()) };
		ret
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:329
	#[inline]
	unsafe fn set_data(&mut self, val: *mut u8) {
		let ret = { sys::cv_cuda_GpuMat_setPropData_unsigned_charX(self.as_raw_mut_GpuMat(), val) };
		ret
	}
	
	// refcount /usr/include/opencv2/core/cuda.hpp:333
	#[inline]
	fn refcount(&mut self) -> *mut i32 {
		let ret = unsafe { sys::cv_cuda_GpuMat_getPropRefcount(self.as_raw_mut_GpuMat()) };
		ret
	}
	
	// refcount /usr/include/opencv2/core/cuda.hpp:333
	#[inline]
	unsafe fn set_refcount(&mut self, val: *mut i32) {
		let ret = { sys::cv_cuda_GpuMat_setPropRefcount_intX(self.as_raw_mut_GpuMat(), val) };
		ret
	}
	
	// datastart /usr/include/opencv2/core/cuda.hpp:336
	#[inline]
	fn datastart(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_cuda_GpuMat_getPropDatastart(self.as_raw_mut_GpuMat()) };
		ret
	}
	
	// datastart /usr/include/opencv2/core/cuda.hpp:336
	#[inline]
	unsafe fn set_datastart(&mut self, val: *mut u8) {
		let ret = { sys::cv_cuda_GpuMat_setPropDatastart_unsigned_charX(self.as_raw_mut_GpuMat(), val) };
		ret
	}
	
	// allocator /usr/include/opencv2/core/cuda.hpp:340
	#[inline]
	fn allocator(&mut self) -> types::AbstractRefMut<dyn core::GpuMat_Allocator> {
		let ret = unsafe { sys::cv_cuda_GpuMat_getPropAllocator(self.as_raw_mut_GpuMat()) };
		let ret = unsafe { types::AbstractRefMut::<dyn core::GpuMat_Allocator>::opencv_from_extern(ret) };
		ret
	}
	
	// allocator /usr/include/opencv2/core/cuda.hpp:340
	#[inline]
	unsafe fn set_allocator(&mut self, val: &mut dyn core::GpuMat_Allocator) {
		let ret = { sys::cv_cuda_GpuMat_setPropAllocator_AllocatorX(self.as_raw_mut_GpuMat(), val.as_raw_mut_GpuMat_Allocator()) };
		ret
	}
	
	// create(int, int, int) /usr/include/opencv2/core/cuda.hpp:154
	#[inline]
	fn create(&mut self, rows: i32, cols: i32, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_create_int_int_int(self.as_raw_mut_GpuMat(), rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// create(cv::Size, int) /usr/include/opencv2/core/cuda.hpp:155
	#[inline]
	fn create_1(&mut self, size: core::Size, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_create_Size_int(self.as_raw_mut_GpuMat(), size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/cuda.hpp:158
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_release(self.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// swap(cv::cuda::GpuMat &) /usr/include/opencv2/core/cuda.hpp:161
	#[inline]
	fn swap(&mut self, mat: &mut core::GpuMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_swap_GpuMatR(self.as_raw_mut_GpuMat(), mat.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// upload(cv::InputArray) /usr/include/opencv2/core/cuda.hpp:168
	#[inline]
	fn upload(&mut self, arr: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_upload_const__InputArrayR(self.as_raw_mut_GpuMat(), arr.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// upload(cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:178
	#[inline]
	fn upload_async(&mut self, arr: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_upload_const__InputArrayR_StreamR(self.as_raw_mut_GpuMat(), arr.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setTo(cv::Scalar) /usr/include/opencv2/core/cuda.hpp:213
	#[inline]
	fn set_to(&mut self, s: core::Scalar) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_setTo_Scalar(self.as_raw_mut_GpuMat(), s.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// setTo(cv::Scalar, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:216
	#[inline]
	fn set_to_1(&mut self, s: core::Scalar, stream: &mut core::Stream) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_setTo_Scalar_StreamR(self.as_raw_mut_GpuMat(), s.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// setTo(cv::Scalar, cv::InputArray) /usr/include/opencv2/core/cuda.hpp:219
	#[inline]
	fn set_to_2(&mut self, s: core::Scalar, mask: &dyn core::ToInputArray) -> Result<core::GpuMat> {
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR(self.as_raw_mut_GpuMat(), s.opencv_as_extern(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// setTo(cv::Scalar, cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:222
	#[inline]
	fn set_to_3(&mut self, s: core::Scalar, mask: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<core::GpuMat> {
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR_StreamR(self.as_raw_mut_GpuMat(), s.opencv_as_extern(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * y: 0
	// ptr(int) /usr/include/opencv2/core/cuda.hpp:242
	#[inline]
	fn ptr_1(&mut self, y: i32) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_ptr_int(self.as_raw_mut_GpuMat(), y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// adjustROI(int, int, int, int) /usr/include/opencv2/core/cuda.hpp:278
	#[inline]
	fn adjust_roi(&mut self, dtop: i32, dbottom: i32, dleft: i32, dright: i32) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_adjustROI_int_int_int_int(self.as_raw_mut_GpuMat(), dtop, dbottom, dleft, dright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// updateContinuityFlag() /usr/include/opencv2/core/cuda.hpp:312
	#[inline]
	fn update_continuity_flag(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_updateContinuityFlag(self.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// GpuMat /usr/include/opencv2/core/cuda.hpp:105
pub struct GpuMat {
	ptr: *mut c_void
}

opencv_type_boxed! { GpuMat }

impl Drop for GpuMat {
	fn drop(&mut self) {
		extern "C" { fn cv_GpuMat_delete(instance: *mut c_void); }
		unsafe { cv_GpuMat_delete(self.as_raw_mut_GpuMat()) };
	}
}

unsafe impl Send for GpuMat {}

impl core::GpuMatTraitConst for GpuMat {
	#[inline] fn as_raw_GpuMat(&self) -> *const c_void { self.as_raw() }
}

impl core::GpuMatTrait for GpuMat {
	#[inline] fn as_raw_mut_GpuMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GpuMat {
	// defaultAllocator() /usr/include/opencv2/core/cuda.hpp:119
	#[inline]
	pub fn default_allocator() -> Result<types::AbstractRefMut<'static, dyn core::GpuMat_Allocator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_defaultAllocator(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { types::AbstractRefMut::<'static, dyn core::GpuMat_Allocator>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// setDefaultAllocator(GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:120
	#[inline]
	pub unsafe fn set_default_allocator(allocator: &mut dyn core::GpuMat_Allocator) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMat_setDefaultAllocator_AllocatorX(allocator.as_raw_mut_GpuMat_Allocator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	// GpuMat(GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:123
	#[inline]
	pub unsafe fn new(allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMat_GpuMat_AllocatorX(allocator.as_raw_mut_GpuMat_Allocator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	// GpuMat(int, int, int, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:126
	#[inline]
	pub unsafe fn new_rows_cols(rows: i32, cols: i32, typ: i32, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMat_GpuMat_int_int_int_AllocatorX(rows, cols, typ, allocator.as_raw_mut_GpuMat_Allocator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	// GpuMat(cv::Size, int, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:127
	#[inline]
	pub unsafe fn new_size(size: core::Size, typ: i32, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMat_GpuMat_Size_int_AllocatorX(size.opencv_as_extern(), typ, allocator.as_raw_mut_GpuMat_Allocator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	// GpuMat(int, int, int, cv::Scalar, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:130
	#[inline]
	pub unsafe fn new_rows_cols_with_default(rows: i32, cols: i32, typ: i32, s: core::Scalar, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMat_GpuMat_int_int_int_Scalar_AllocatorX(rows, cols, typ, s.opencv_as_extern(), allocator.as_raw_mut_GpuMat_Allocator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	// GpuMat(cv::Size, int, cv::Scalar, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:131
	#[inline]
	pub unsafe fn new_size_with_default(size: core::Size, typ: i32, s: core::Scalar, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMat_GpuMat_Size_int_Scalar_AllocatorX(size.opencv_as_extern(), typ, s.opencv_as_extern(), allocator.as_raw_mut_GpuMat_Allocator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// GpuMat(const cv::cuda::GpuMat &) /usr/include/opencv2/core/cuda.hpp:134
	#[inline]
	pub fn copy(m: &core::GpuMat) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_GpuMat_const_GpuMatR(m.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * step: Mat::AUTO_STEP
	// GpuMat(int, int, int, void *, size_t) /usr/include/opencv2/core/cuda.hpp:137
	#[inline]
	pub unsafe fn new_rows_cols_with_data(rows: i32, cols: i32, typ: i32, data: *mut c_void, step: size_t) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMat_GpuMat_int_int_int_voidX_size_t(rows, cols, typ, data, step, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * step: Mat::AUTO_STEP
	// GpuMat(cv::Size, int, void *, size_t) /usr/include/opencv2/core/cuda.hpp:138
	#[inline]
	pub unsafe fn new_size_with_data(size: core::Size, typ: i32, data: *mut c_void, step: size_t) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMat_GpuMat_Size_int_voidX_size_t(size.opencv_as_extern(), typ, data, step, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// GpuMat(const cv::cuda::GpuMat &, cv::Range, cv::Range) /usr/include/opencv2/core/cuda.hpp:141
	#[inline]
	pub fn rowscols(m: &core::GpuMat, mut row_range: core::Range, mut col_range: core::Range) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_GpuMat_const_GpuMatR_Range_Range(m.as_raw_GpuMat(), row_range.as_raw_mut_Range(), col_range.as_raw_mut_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// GpuMat(const cv::cuda::GpuMat &, cv::Rect) /usr/include/opencv2/core/cuda.hpp:142
	#[inline]
	pub fn roi(m: &core::GpuMat, roi: core::Rect) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_GpuMat_const_GpuMatR_Rect(m.as_raw_GpuMat(), roi.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	// GpuMat(cv::InputArray, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:145
	#[inline]
	pub unsafe fn from_hostmem(arr: &dyn core::ToInputArray, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMat_GpuMat_const__InputArrayR_AllocatorX(arr.as_raw__InputArray(), allocator.as_raw_mut_GpuMat_Allocator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Clone for GpuMat {
	#[inline]
	/// Calls try_clone() and panics if that fails
	fn clone(&self) -> Self {
		self.try_clone().expect("Cannot clone GpuMat")
	}
}

// Allocator /usr/include/opencv2/core/cuda.hpp:108
pub trait GpuMat_AllocatorConst {
	fn as_raw_GpuMat_Allocator(&self) -> *const c_void;

}

pub trait GpuMat_Allocator: core::GpuMat_AllocatorConst {
	fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void;

	// allocate(cv::cuda::GpuMat *, int, int, size_t) /usr/include/opencv2/core/cuda.hpp:114
	#[inline]
	fn allocate(&mut self, mat: &mut core::GpuMat, rows: i32, cols: i32, elem_size: size_t) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_Allocator_allocate_GpuMatX_int_int_size_t(self.as_raw_mut_GpuMat_Allocator(), mat.as_raw_mut_GpuMat(), rows, cols, elem_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// free(cv::cuda::GpuMat *) /usr/include/opencv2/core/cuda.hpp:115
	#[inline]
	fn free(&mut self, mat: &mut core::GpuMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMat_Allocator_free_GpuMatX(self.as_raw_mut_GpuMat_Allocator(), mat.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// GpuMatND /usr/include/opencv2/core/cuda.hpp:358
pub trait GpuMatNDTraitConst {
	fn as_raw_GpuMatND(&self) -> *const c_void;

	// flags /usr/include/opencv2/core/cuda.hpp:513
	#[inline]
	fn flags(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_GpuMatND_getPropFlags_const(self.as_raw_GpuMatND()) };
		ret
	}
	
	// dims /usr/include/opencv2/core/cuda.hpp:516
	#[inline]
	fn dims(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_GpuMatND_getPropDims_const(self.as_raw_GpuMatND()) };
		ret
	}
	
	// size /usr/include/opencv2/core/cuda.hpp:519
	#[inline]
	fn size(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_cuda_GpuMatND_getPropSize_const(self.as_raw_GpuMatND()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:524
	#[inline]
	fn step(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_cuda_GpuMatND_getPropStep_const(self.as_raw_GpuMatND()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	// clone() /usr/include/opencv2/core/cuda.hpp:410
	#[inline]
	fn try_clone(&self) -> Result<core::GpuMatND> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_clone_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMatND::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// clone(cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:415
	#[inline]
	fn clone_1(&self, stream: &mut core::Stream) -> Result<core::GpuMatND> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_clone_const_StreamR(self.as_raw_GpuMatND(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMatND::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// createGpuMatHeader(cv::cuda::GpuMatND::IndexArray, cv::Range, cv::Range) /usr/include/opencv2/core/cuda.hpp:429
	#[inline]
	fn create_gpu_mat_header(&self, mut idx: core::GpuMatND_IndexArray, mut row_range: core::Range, mut col_range: core::Range) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_createGpuMatHeader_const_IndexArray_Range_Range(self.as_raw_GpuMatND(), idx.as_raw_mut_VectorOfi32(), row_range.as_raw_mut_Range(), col_range.as_raw_mut_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// createGpuMatHeader() /usr/include/opencv2/core/cuda.hpp:437
	#[inline]
	fn create_gpu_mat_header_1(&self) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_createGpuMatHeader_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// operator GpuMat() /usr/include/opencv2/core/cuda.hpp:450
	#[inline]
	fn to_gpu_mat(&self) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_operator_cv_cuda_GpuMat_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// download(cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:468
	#[inline]
	fn download(&self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_download_const_const__OutputArrayR(self.as_raw_GpuMatND(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// download(cv::OutputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:469
	#[inline]
	fn download_1(&self, dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_download_const_const__OutputArrayR_StreamR(self.as_raw_GpuMatND(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isContinuous() /usr/include/opencv2/core/cuda.hpp:473
	#[inline]
	fn is_continuous(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_isContinuous_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isSubmatrix() /usr/include/opencv2/core/cuda.hpp:476
	#[inline]
	fn is_submatrix(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_isSubmatrix_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize() /usr/include/opencv2/core/cuda.hpp:479
	#[inline]
	fn elem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_elemSize_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize1() /usr/include/opencv2/core/cuda.hpp:482
	#[inline]
	fn elem_size1(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_elemSize1_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/cuda.hpp:485
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_empty_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// external() /usr/include/opencv2/core/cuda.hpp:488
	#[inline]
	fn external(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_external_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDevicePtr() /usr/include/opencv2/core/cuda.hpp:491
	#[inline]
	fn get_device_ptr(&self) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_getDevicePtr_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// total() /usr/include/opencv2/core/cuda.hpp:494
	#[inline]
	fn total(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_total_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// totalMemSize() /usr/include/opencv2/core/cuda.hpp:497
	#[inline]
	fn total_mem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_totalMemSize_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// type() /usr/include/opencv2/core/cuda.hpp:500
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_type_const(self.as_raw_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GpuMatNDTrait: core::GpuMatNDTraitConst {
	fn as_raw_mut_GpuMatND(&mut self) -> *mut c_void;

	// flags /usr/include/opencv2/core/cuda.hpp:513
	#[inline]
	fn set_flags(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_GpuMatND_setPropFlags_int(self.as_raw_mut_GpuMatND(), val) };
		ret
	}
	
	// dims /usr/include/opencv2/core/cuda.hpp:516
	#[inline]
	fn set_dims(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_GpuMatND_setPropDims_int(self.as_raw_mut_GpuMatND(), val) };
		ret
	}
	
	// size /usr/include/opencv2/core/cuda.hpp:519
	#[inline]
	fn set_size(&mut self, mut val: core::GpuMatND_SizeArray) {
		let ret = unsafe { sys::cv_cuda_GpuMatND_setPropSize_SizeArray(self.as_raw_mut_GpuMatND(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:524
	#[inline]
	fn set_step(&mut self, mut val: core::GpuMatND_StepArray) {
		let ret = unsafe { sys::cv_cuda_GpuMatND_setPropStep_StepArray(self.as_raw_mut_GpuMatND(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	// create(cv::cuda::GpuMatND::SizeArray, int) /usr/include/opencv2/core/cuda.hpp:399
	#[inline]
	fn create(&mut self, mut size: core::GpuMatND_SizeArray, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_create_SizeArray_int(self.as_raw_mut_GpuMatND(), size.as_raw_mut_VectorOfi32(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/cuda.hpp:401
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_release(self.as_raw_mut_GpuMatND(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// swap(cv::cuda::GpuMatND &) /usr/include/opencv2/core/cuda.hpp:403
	#[inline]
	fn swap(&mut self, m: &mut core::GpuMatND) {
		let ret = unsafe { sys::cv_cuda_GpuMatND_swap_GpuMatNDR(self.as_raw_mut_GpuMatND(), m.as_raw_mut_GpuMatND()) };
		ret
	}
	
	// upload(cv::InputArray) /usr/include/opencv2/core/cuda.hpp:466
	#[inline]
	fn upload(&mut self, src: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_upload_const__InputArrayR(self.as_raw_mut_GpuMatND(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// upload(cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:467
	#[inline]
	fn upload_1(&mut self, src: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_upload_const__InputArrayR_StreamR(self.as_raw_mut_GpuMatND(), src.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// GpuMatND /usr/include/opencv2/core/cuda.hpp:358
pub struct GpuMatND {
	ptr: *mut c_void
}

opencv_type_boxed! { GpuMatND }

impl Drop for GpuMatND {
	fn drop(&mut self) {
		extern "C" { fn cv_GpuMatND_delete(instance: *mut c_void); }
		unsafe { cv_GpuMatND_delete(self.as_raw_mut_GpuMatND()) };
	}
}

unsafe impl Send for GpuMatND {}

impl core::GpuMatNDTraitConst for GpuMatND {
	#[inline] fn as_raw_GpuMatND(&self) -> *const c_void { self.as_raw() }
}

impl core::GpuMatNDTrait for GpuMatND {
	#[inline] fn as_raw_mut_GpuMatND(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GpuMatND {
	// GpuMatND() /usr/include/opencv2/core/cuda.hpp:369
	#[inline]
	pub fn default() -> Result<core::GpuMatND> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_GpuMatND(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMatND::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// GpuMatND(cv::cuda::GpuMatND::SizeArray, int) /usr/include/opencv2/core/cuda.hpp:376
	#[inline]
	pub fn new(mut size: core::GpuMatND_SizeArray, typ: i32) -> Result<core::GpuMatND> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_GpuMatND_GpuMatND_SizeArray_int(size.as_raw_mut_VectorOfi32(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMatND::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * step: StepArray()
	// GpuMatND(cv::cuda::GpuMatND::SizeArray, int, void *, cv::cuda::GpuMatND::StepArray) /usr/include/opencv2/core/cuda.hpp:390
	#[inline]
	pub unsafe fn new_1(mut size: core::GpuMatND_SizeArray, typ: i32, data: *mut c_void, mut step: core::GpuMatND_StepArray) -> Result<core::GpuMatND> {
		return_send!(via ocvrs_return);
		{ sys::cv_cuda_GpuMatND_GpuMatND_SizeArray_int_voidX_StepArray(size.as_raw_mut_VectorOfi32(), typ, data, step.as_raw_mut_VectorOfsize_t(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::GpuMatND::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// GpuMatND(const cv::cuda::GpuMatND &) /usr/include/opencv2/core/cuda.hpp:452
	#[inline]
	pub fn copy(unnamed: &core::GpuMatND) -> core::GpuMatND {
		let ret = unsafe { sys::cv_cuda_GpuMatND_GpuMatND_const_GpuMatNDR(unnamed.as_raw_GpuMatND()) };
		let ret = unsafe { core::GpuMatND::opencv_from_extern(ret) };
		ret
	}
	
	// GpuMatND(cv::cuda::GpuMatND &&) /usr/include/opencv2/core/cuda.hpp:459
	#[inline]
	pub fn copy_mut(unnamed: &mut core::GpuMatND) -> core::GpuMatND {
		let ret = unsafe { sys::cv_cuda_GpuMatND_GpuMatND_GpuMatNDR(unnamed.as_raw_mut_GpuMatND()) };
		let ret = unsafe { core::GpuMatND::opencv_from_extern(ret) };
		ret
	}
	
}

impl Clone for GpuMatND {
	#[inline]
	/// Calls try_clone() and panics if that fails
	fn clone(&self) -> Self {
		self.try_clone().expect("Cannot clone GpuMatND")
	}
}

// HostMem /usr/include/opencv2/core/cuda.hpp:730
pub trait HostMemTraitConst {
	fn as_raw_HostMem(&self) -> *const c_void;

	// flags /usr/include/opencv2/core/cuda.hpp:792
	#[inline]
	fn flags(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_HostMem_getPropFlags_const(self.as_raw_HostMem()) };
		ret
	}
	
	// rows /usr/include/opencv2/core/cuda.hpp:793
	#[inline]
	fn rows(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_HostMem_getPropRows_const(self.as_raw_HostMem()) };
		ret
	}
	
	// cols /usr/include/opencv2/core/cuda.hpp:793
	#[inline]
	fn cols(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_HostMem_getPropCols_const(self.as_raw_HostMem()) };
		ret
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:794
	#[inline]
	fn step(&self) -> size_t {
		let ret = unsafe { sys::cv_cuda_HostMem_getPropStep_const(self.as_raw_HostMem()) };
		ret
	}
	
	// dataend /usr/include/opencv2/core/cuda.hpp:800
	#[inline]
	fn dataend(&self) -> *const u8 {
		let ret = unsafe { sys::cv_cuda_HostMem_getPropDataend_const(self.as_raw_HostMem()) };
		ret
	}
	
	// alloc_type /usr/include/opencv2/core/cuda.hpp:802
	#[inline]
	fn alloc_type(&self) -> core::HostMem_AllocType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_getPropAlloc_type_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// clone() /usr/include/opencv2/core/cuda.hpp:755
	#[inline]
	fn try_clone(&self) -> Result<core::HostMem> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_clone_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::HostMem::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * rows: 0
	// reshape(int, int) /usr/include/opencv2/core/cuda.hpp:763
	#[inline]
	fn reshape(&self, cn: i32, rows: i32) -> Result<core::HostMem> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_reshape_const_int_int(self.as_raw_HostMem(), cn, rows, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::HostMem::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// createMatHeader() /usr/include/opencv2/core/cuda.hpp:769
	#[inline]
	fn create_mat_header(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_createMatHeader_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// createGpuMatHeader() /usr/include/opencv2/core/cuda.hpp:778
	#[inline]
	fn create_gpu_mat_header(&self) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_createGpuMatHeader_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// isContinuous() /usr/include/opencv2/core/cuda.hpp:781
	#[inline]
	fn is_continuous(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_isContinuous_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize() /usr/include/opencv2/core/cuda.hpp:782
	#[inline]
	fn elem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_elemSize_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize1() /usr/include/opencv2/core/cuda.hpp:783
	#[inline]
	fn elem_size1(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_elemSize1_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// type() /usr/include/opencv2/core/cuda.hpp:784
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_type_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// depth() /usr/include/opencv2/core/cuda.hpp:785
	#[inline]
	fn depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_depth_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// channels() /usr/include/opencv2/core/cuda.hpp:786
	#[inline]
	fn channels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_channels_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// step1() /usr/include/opencv2/core/cuda.hpp:787
	#[inline]
	fn step1(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_step1_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// size() /usr/include/opencv2/core/cuda.hpp:788
	#[inline]
	fn size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_size_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/cuda.hpp:789
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_empty_const(self.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait HostMemTrait: core::HostMemTraitConst {
	fn as_raw_mut_HostMem(&mut self) -> *mut c_void;

	// flags /usr/include/opencv2/core/cuda.hpp:792
	#[inline]
	fn set_flags(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_HostMem_setPropFlags_int(self.as_raw_mut_HostMem(), val) };
		ret
	}
	
	// rows /usr/include/opencv2/core/cuda.hpp:793
	#[inline]
	fn set_rows(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_HostMem_setPropRows_int(self.as_raw_mut_HostMem(), val) };
		ret
	}
	
	// cols /usr/include/opencv2/core/cuda.hpp:793
	#[inline]
	fn set_cols(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_HostMem_setPropCols_int(self.as_raw_mut_HostMem(), val) };
		ret
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:794
	#[inline]
	fn set_step(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_cuda_HostMem_setPropStep_size_t(self.as_raw_mut_HostMem(), val) };
		ret
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:796
	#[inline]
	fn data(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_cuda_HostMem_getPropData(self.as_raw_mut_HostMem()) };
		ret
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:796
	#[inline]
	unsafe fn set_data(&mut self, val: *mut u8) {
		let ret = { sys::cv_cuda_HostMem_setPropData_unsigned_charX(self.as_raw_mut_HostMem(), val) };
		ret
	}
	
	// refcount /usr/include/opencv2/core/cuda.hpp:797
	#[inline]
	fn refcount(&mut self) -> *mut i32 {
		let ret = unsafe { sys::cv_cuda_HostMem_getPropRefcount(self.as_raw_mut_HostMem()) };
		ret
	}
	
	// refcount /usr/include/opencv2/core/cuda.hpp:797
	#[inline]
	unsafe fn set_refcount(&mut self, val: *mut i32) {
		let ret = { sys::cv_cuda_HostMem_setPropRefcount_intX(self.as_raw_mut_HostMem(), val) };
		ret
	}
	
	// datastart /usr/include/opencv2/core/cuda.hpp:799
	#[inline]
	fn datastart(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_cuda_HostMem_getPropDatastart(self.as_raw_mut_HostMem()) };
		ret
	}
	
	// datastart /usr/include/opencv2/core/cuda.hpp:799
	#[inline]
	unsafe fn set_datastart(&mut self, val: *mut u8) {
		let ret = { sys::cv_cuda_HostMem_setPropDatastart_unsigned_charX(self.as_raw_mut_HostMem(), val) };
		ret
	}
	
	// alloc_type /usr/include/opencv2/core/cuda.hpp:802
	#[inline]
	fn set_alloc_type(&mut self, val: core::HostMem_AllocType) {
		let ret = unsafe { sys::cv_cuda_HostMem_setPropAlloc_type_AllocType(self.as_raw_mut_HostMem(), val) };
		ret
	}
	
	// swap(cv::cuda::HostMem &) /usr/include/opencv2/core/cuda.hpp:752
	#[inline]
	fn swap(&mut self, b: &mut core::HostMem) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_swap_HostMemR(self.as_raw_mut_HostMem(), b.as_raw_mut_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// create(int, int, int) /usr/include/opencv2/core/cuda.hpp:758
	#[inline]
	fn create(&mut self, rows: i32, cols: i32, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_create_int_int_int(self.as_raw_mut_HostMem(), rows, cols, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// create(cv::Size, int) /usr/include/opencv2/core/cuda.hpp:759
	#[inline]
	fn create_1(&mut self, size: core::Size, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_create_Size_int(self.as_raw_mut_HostMem(), size.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/cuda.hpp:766
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_release(self.as_raw_mut_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// HostMem /usr/include/opencv2/core/cuda.hpp:730
pub struct HostMem {
	ptr: *mut c_void
}

opencv_type_boxed! { HostMem }

impl Drop for HostMem {
	fn drop(&mut self) {
		extern "C" { fn cv_HostMem_delete(instance: *mut c_void); }
		unsafe { cv_HostMem_delete(self.as_raw_mut_HostMem()) };
	}
}

unsafe impl Send for HostMem {}

impl core::HostMemTraitConst for HostMem {
	#[inline] fn as_raw_HostMem(&self) -> *const c_void { self.as_raw() }
}

impl core::HostMemTrait for HostMem {
	#[inline] fn as_raw_mut_HostMem(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl HostMem {
	/// ## C++ default parameters
	/// * alloc_type: HostMem::AllocType::PAGE_LOCKED
	// HostMem(HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:737
	#[inline]
	pub fn new(alloc_type: core::HostMem_AllocType) -> Result<core::HostMem> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_HostMem_AllocType(alloc_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::HostMem::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// HostMem(const cv::cuda::HostMem &) /usr/include/opencv2/core/cuda.hpp:739
	#[inline]
	pub fn copy(m: &core::HostMem) -> Result<core::HostMem> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_HostMem_const_HostMemR(m.as_raw_HostMem(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::HostMem::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * alloc_type: HostMem::AllocType::PAGE_LOCKED
	// HostMem(int, int, int, HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:741
	#[inline]
	pub fn new_1(rows: i32, cols: i32, typ: i32, alloc_type: core::HostMem_AllocType) -> Result<core::HostMem> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_HostMem_int_int_int_AllocType(rows, cols, typ, alloc_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::HostMem::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * alloc_type: HostMem::AllocType::PAGE_LOCKED
	// HostMem(cv::Size, int, HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:742
	#[inline]
	pub fn new_2(size: core::Size, typ: i32, alloc_type: core::HostMem_AllocType) -> Result<core::HostMem> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_HostMem_Size_int_AllocType(size.opencv_as_extern(), typ, alloc_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::HostMem::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * alloc_type: HostMem::AllocType::PAGE_LOCKED
	// HostMem(cv::InputArray, HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:745
	#[inline]
	pub fn new_3(arr: &dyn core::ToInputArray, alloc_type: core::HostMem_AllocType) -> Result<core::HostMem> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_HostMem_HostMem_const__InputArrayR_AllocType(arr.as_raw__InputArray(), alloc_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::HostMem::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Clone for HostMem {
	#[inline]
	/// Calls try_clone() and panics if that fails
	fn clone(&self) -> Self {
		self.try_clone().expect("Cannot clone HostMem")
	}
}

// Stream /usr/include/opencv2/core/cuda.hpp:848
pub trait StreamTraitConst {
	fn as_raw_Stream(&self) -> *const c_void;

	// queryIfComplete() /usr/include/opencv2/core/cuda.hpp:876
	#[inline]
	fn query_if_complete(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Stream_queryIfComplete_const(self.as_raw_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// cudaPtr() /usr/include/opencv2/core/cuda.hpp:903
	#[inline]
	fn cuda_ptr(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Stream_cudaPtr_const(self.as_raw_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait StreamTrait: core::StreamTraitConst {
	fn as_raw_mut_Stream(&mut self) -> *mut c_void;

	// waitForCompletion() /usr/include/opencv2/core/cuda.hpp:880
	#[inline]
	fn wait_for_completion(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Stream_waitForCompletion(self.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// waitEvent(const cv::cuda::Event &) /usr/include/opencv2/core/cuda.hpp:884
	#[inline]
	fn wait_event(&mut self, event: &core::Event) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Stream_waitEvent_const_EventR(self.as_raw_mut_Stream(), event.as_raw_Event(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// enqueueHostCallback(cv::cuda::Stream::StreamCallback, void *) /usr/include/opencv2/core/cuda.hpp:894
	#[inline]
	fn enqueue_host_callback(&mut self, callback: core::Stream_StreamCallback) -> Result<()> {
		callback_arg!(callback_trampoline(status: i32, user_data: *mut c_void) -> () => user_data in callbacks => callback(status: i32) -> ());
		userdata_arg!(user_data in callbacks => callback);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Stream_enqueueHostCallback_StreamCallback_voidX(self.as_raw_mut_Stream(), callback_trampoline, user_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Stream /usr/include/opencv2/core/cuda.hpp:848
pub struct Stream {
	ptr: *mut c_void
}

opencv_type_boxed! { Stream }

impl Drop for Stream {
	fn drop(&mut self) {
		extern "C" { fn cv_Stream_delete(instance: *mut c_void); }
		unsafe { cv_Stream_delete(self.as_raw_mut_Stream()) };
	}
}

unsafe impl Send for Stream {}

impl core::StreamTraitConst for Stream {
	#[inline] fn as_raw_Stream(&self) -> *const c_void { self.as_raw() }
}

impl core::StreamTrait for Stream {
	#[inline] fn as_raw_mut_Stream(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Stream {
	// Stream() /usr/include/opencv2/core/cuda.hpp:857
	#[inline]
	pub fn default() -> Result<core::Stream> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Stream_Stream(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Stream::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Stream(const Ptr<GpuMat::Allocator> &) /usr/include/opencv2/core/cuda.hpp:860
	#[inline]
	pub fn new(allocator: &core::Ptr<dyn core::GpuMat_Allocator>) -> Result<core::Stream> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Stream_Stream_const_Ptr_Allocator_R(allocator.as_raw_PtrOfGpuMat_Allocator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Stream::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Stream(const size_t) /usr/include/opencv2/core/cuda.hpp:872
	#[inline]
	pub fn new_1(cuda_flags: size_t) -> Result<core::Stream> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Stream_Stream_const_size_t(cuda_flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Stream::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Null() /usr/include/opencv2/core/cuda.hpp:897
	#[inline]
	pub fn null() -> Result<core::Stream> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_Stream_Null(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Stream::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// TargetArchs /usr/include/opencv2/core/cuda.hpp:1018
pub trait TargetArchsTraitConst {
	fn as_raw_TargetArchs(&self) -> *const c_void;

}

pub trait TargetArchsTrait: core::TargetArchsTraitConst {
	fn as_raw_mut_TargetArchs(&mut self) -> *mut c_void;

}

// TargetArchs /usr/include/opencv2/core/cuda.hpp:1018
pub struct TargetArchs {
	ptr: *mut c_void
}

opencv_type_boxed! { TargetArchs }

impl Drop for TargetArchs {
	fn drop(&mut self) {
		extern "C" { fn cv_TargetArchs_delete(instance: *mut c_void); }
		unsafe { cv_TargetArchs_delete(self.as_raw_mut_TargetArchs()) };
	}
}

unsafe impl Send for TargetArchs {}

impl core::TargetArchsTraitConst for TargetArchs {
	#[inline] fn as_raw_TargetArchs(&self) -> *const c_void { self.as_raw() }
}

impl core::TargetArchsTrait for TargetArchs {
	#[inline] fn as_raw_mut_TargetArchs(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TargetArchs {
	// builtWith(cv::cuda::FeatureSet) /usr/include/opencv2/core/cuda.hpp:1025
	#[inline]
	pub fn built_with(feature_set: core::FeatureSet) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_TargetArchs_builtWith_FeatureSet(feature_set, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// has(int, int) /usr/include/opencv2/core/cuda.hpp:1033
	#[inline]
	pub fn has(major: i32, minor: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_TargetArchs_has_int_int(major, minor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hasPtx(int, int) /usr/include/opencv2/core/cuda.hpp:1034
	#[inline]
	pub fn has_ptx(major: i32, minor: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_TargetArchs_hasPtx_int_int(major, minor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hasBin(int, int) /usr/include/opencv2/core/cuda.hpp:1035
	#[inline]
	pub fn has_bin(major: i32, minor: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_TargetArchs_hasBin_int_int(major, minor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hasEqualOrLessPtx(int, int) /usr/include/opencv2/core/cuda.hpp:1037
	#[inline]
	pub fn has_equal_or_less_ptx(major: i32, minor: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_TargetArchs_hasEqualOrLessPtx_int_int(major, minor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hasEqualOrGreater(int, int) /usr/include/opencv2/core/cuda.hpp:1038
	#[inline]
	pub fn has_equal_or_greater(major: i32, minor: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_TargetArchs_hasEqualOrGreater_int_int(major, minor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hasEqualOrGreaterPtx(int, int) /usr/include/opencv2/core/cuda.hpp:1039
	#[inline]
	pub fn has_equal_or_greater_ptx(major: i32, minor: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_TargetArchs_hasEqualOrGreaterPtx_int_int(major, minor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hasEqualOrGreaterBin(int, int) /usr/include/opencv2/core/cuda.hpp:1040
	#[inline]
	pub fn has_equal_or_greater_bin(major: i32, minor: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_TargetArchs_hasEqualOrGreaterBin_int_int(major, minor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// CheckContext /usr/include/opencv2/core/check.hpp:39
pub trait Detail_CheckContextTraitConst {
	fn as_raw_Detail_CheckContext(&self) -> *const c_void;

	// func /usr/include/opencv2/core/check.hpp:40
	#[inline]
	fn func(&self) -> String {
		let ret = unsafe { sys::cv_detail_CheckContext_getPropFunc_const(self.as_raw_Detail_CheckContext()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// file /usr/include/opencv2/core/check.hpp:41
	#[inline]
	fn file(&self) -> String {
		let ret = unsafe { sys::cv_detail_CheckContext_getPropFile_const(self.as_raw_Detail_CheckContext()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// line /usr/include/opencv2/core/check.hpp:42
	#[inline]
	fn line(&self) -> i32 {
		let ret = unsafe { sys::cv_detail_CheckContext_getPropLine_const(self.as_raw_Detail_CheckContext()) };
		ret
	}
	
	// testOp /usr/include/opencv2/core/check.hpp:43
	#[inline]
	fn test_op(&self) -> core::Detail_TestOp {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CheckContext_getPropTestOp_const(self.as_raw_Detail_CheckContext(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// message /usr/include/opencv2/core/check.hpp:44
	#[inline]
	fn message(&self) -> String {
		let ret = unsafe { sys::cv_detail_CheckContext_getPropMessage_const(self.as_raw_Detail_CheckContext()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// p1_str /usr/include/opencv2/core/check.hpp:45
	#[inline]
	fn p1_str(&self) -> String {
		let ret = unsafe { sys::cv_detail_CheckContext_getPropP1_str_const(self.as_raw_Detail_CheckContext()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// p2_str /usr/include/opencv2/core/check.hpp:46
	#[inline]
	fn p2_str(&self) -> String {
		let ret = unsafe { sys::cv_detail_CheckContext_getPropP2_str_const(self.as_raw_Detail_CheckContext()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait Detail_CheckContextTrait: core::Detail_CheckContextTraitConst {
	fn as_raw_mut_Detail_CheckContext(&mut self) -> *mut c_void;

	// line /usr/include/opencv2/core/check.hpp:42
	#[inline]
	fn set_line(&mut self, val: i32) {
		let ret = unsafe { sys::cv_detail_CheckContext_setPropLine_int(self.as_raw_mut_Detail_CheckContext(), val) };
		ret
	}
	
	// testOp /usr/include/opencv2/core/check.hpp:43
	#[inline]
	fn set_test_op(&mut self, val: core::Detail_TestOp) {
		let ret = unsafe { sys::cv_detail_CheckContext_setPropTestOp_TestOp(self.as_raw_mut_Detail_CheckContext(), val) };
		ret
	}
	
}

// CheckContext /usr/include/opencv2/core/check.hpp:39
pub struct Detail_CheckContext {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CheckContext }

impl Drop for Detail_CheckContext {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CheckContext_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CheckContext_delete(self.as_raw_mut_Detail_CheckContext()) };
	}
}

unsafe impl Send for Detail_CheckContext {}

impl core::Detail_CheckContextTraitConst for Detail_CheckContext {
	#[inline] fn as_raw_Detail_CheckContext(&self) -> *const c_void { self.as_raw() }
}

impl core::Detail_CheckContextTrait for Detail_CheckContext {
	#[inline] fn as_raw_mut_Detail_CheckContext(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CheckContext {
}

// NodeData /usr/include/opencv2/core/utils/instrumentation.hpp:69
pub trait NodeDataTraitConst {
	fn as_raw_NodeData(&self) -> *const c_void;

	// m_funName /usr/include/opencv2/core/utils/instrumentation.hpp:77
	#[inline]
	fn m_fun_name(&self) -> String {
		let ret = unsafe { sys::cv_instr_NodeData_getPropM_funName_const(self.as_raw_NodeData()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// m_instrType /usr/include/opencv2/core/utils/instrumentation.hpp:78
	#[inline]
	fn m_instr_type(&self) -> core::TYPE {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_instr_NodeData_getPropM_instrType_const(self.as_raw_NodeData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// m_implType /usr/include/opencv2/core/utils/instrumentation.hpp:79
	#[inline]
	fn m_impl_type(&self) -> core::IMPL {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_instr_NodeData_getPropM_implType_const(self.as_raw_NodeData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// m_fileName /usr/include/opencv2/core/utils/instrumentation.hpp:80
	#[inline]
	fn m_file_name(&self) -> String {
		let ret = unsafe { sys::cv_instr_NodeData_getPropM_fileName_const(self.as_raw_NodeData()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// m_lineNum /usr/include/opencv2/core/utils/instrumentation.hpp:81
	#[inline]
	fn m_line_num(&self) -> i32 {
		let ret = unsafe { sys::cv_instr_NodeData_getPropM_lineNum_const(self.as_raw_NodeData()) };
		ret
	}
	
	// m_alwaysExpand /usr/include/opencv2/core/utils/instrumentation.hpp:83
	#[inline]
	fn m_always_expand(&self) -> bool {
		let ret = unsafe { sys::cv_instr_NodeData_getPropM_alwaysExpand_const(self.as_raw_NodeData()) };
		ret
	}
	
	// m_funError /usr/include/opencv2/core/utils/instrumentation.hpp:84
	#[inline]
	fn m_fun_error(&self) -> bool {
		let ret = unsafe { sys::cv_instr_NodeData_getPropM_funError_const(self.as_raw_NodeData()) };
		ret
	}
	
	// m_counter /usr/include/opencv2/core/utils/instrumentation.hpp:86
	#[inline]
	fn m_counter(&self) -> i32 {
		let ret = unsafe { sys::cv_instr_NodeData_getPropM_counter_const(self.as_raw_NodeData()) };
		ret
	}
	
	// m_ticksTotal /usr/include/opencv2/core/utils/instrumentation.hpp:87
	#[inline]
	fn m_ticks_total(&self) -> u64 {
		let ret = unsafe { sys::cv_instr_NodeData_getPropM_ticksTotal_const(self.as_raw_NodeData()) };
		ret
	}
	
	// m_threads /usr/include/opencv2/core/utils/instrumentation.hpp:89
	#[inline]
	fn m_threads(&self) -> i32 {
		let ret = unsafe { sys::cv_instr_NodeData_getPropM_threads_const(self.as_raw_NodeData()) };
		ret
	}
	
	// getTotalMs() /usr/include/opencv2/core/utils/instrumentation.hpp:92
	#[inline]
	fn get_total_ms(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_instr_NodeData_getTotalMs_const(self.as_raw_NodeData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMeanMs() /usr/include/opencv2/core/utils/instrumentation.hpp:93
	#[inline]
	fn get_mean_ms(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_instr_NodeData_getMeanMs_const(self.as_raw_NodeData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait NodeDataTrait: core::NodeDataTraitConst {
	fn as_raw_mut_NodeData(&mut self) -> *mut c_void;

	// m_funName /usr/include/opencv2/core/utils/instrumentation.hpp:77
	#[inline]
	fn set_m_fun_name(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_instr_NodeData_setPropM_funName_String(self.as_raw_mut_NodeData(), val.opencv_as_extern_mut()) };
		ret
	}
	
	// m_instrType /usr/include/opencv2/core/utils/instrumentation.hpp:78
	#[inline]
	fn set_m_instr_type(&mut self, val: core::TYPE) {
		let ret = unsafe { sys::cv_instr_NodeData_setPropM_instrType_TYPE(self.as_raw_mut_NodeData(), val) };
		ret
	}
	
	// m_implType /usr/include/opencv2/core/utils/instrumentation.hpp:79
	#[inline]
	fn set_m_impl_type(&mut self, val: core::IMPL) {
		let ret = unsafe { sys::cv_instr_NodeData_setPropM_implType_IMPL(self.as_raw_mut_NodeData(), val) };
		ret
	}
	
	// m_lineNum /usr/include/opencv2/core/utils/instrumentation.hpp:81
	#[inline]
	fn set_m_line_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_instr_NodeData_setPropM_lineNum_int(self.as_raw_mut_NodeData(), val) };
		ret
	}
	
	// m_retAddress /usr/include/opencv2/core/utils/instrumentation.hpp:82
	#[inline]
	fn m_ret_address(&mut self) -> *mut c_void {
		let ret = unsafe { sys::cv_instr_NodeData_getPropM_retAddress(self.as_raw_mut_NodeData()) };
		ret
	}
	
	// m_retAddress /usr/include/opencv2/core/utils/instrumentation.hpp:82
	#[inline]
	unsafe fn set_m_ret_address(&mut self, val: *mut c_void) {
		let ret = { sys::cv_instr_NodeData_setPropM_retAddress_voidX(self.as_raw_mut_NodeData(), val) };
		ret
	}
	
	// m_alwaysExpand /usr/include/opencv2/core/utils/instrumentation.hpp:83
	#[inline]
	fn set_m_always_expand(&mut self, val: bool) {
		let ret = unsafe { sys::cv_instr_NodeData_setPropM_alwaysExpand_bool(self.as_raw_mut_NodeData(), val) };
		ret
	}
	
	// m_funError /usr/include/opencv2/core/utils/instrumentation.hpp:84
	#[inline]
	fn set_m_fun_error(&mut self, val: bool) {
		let ret = unsafe { sys::cv_instr_NodeData_setPropM_funError_bool(self.as_raw_mut_NodeData(), val) };
		ret
	}
	
	// m_counter /usr/include/opencv2/core/utils/instrumentation.hpp:86
	#[inline]
	fn set_m_counter(&mut self, val: i32) {
		let ret = unsafe { sys::cv_instr_NodeData_setPropM_counter_int(self.as_raw_mut_NodeData(), val) };
		ret
	}
	
	// m_ticksTotal /usr/include/opencv2/core/utils/instrumentation.hpp:87
	#[inline]
	fn set_m_ticks_total(&mut self, val: u64) {
		let ret = unsafe { sys::cv_instr_NodeData_setPropM_ticksTotal_uint64_t(self.as_raw_mut_NodeData(), val) };
		ret
	}
	
	// m_threads /usr/include/opencv2/core/utils/instrumentation.hpp:89
	#[inline]
	fn set_m_threads(&mut self, val: i32) {
		let ret = unsafe { sys::cv_instr_NodeData_setPropM_threads_int(self.as_raw_mut_NodeData(), val) };
		ret
	}
	
}

// NodeData /usr/include/opencv2/core/utils/instrumentation.hpp:69
pub struct NodeData {
	ptr: *mut c_void
}

opencv_type_boxed! { NodeData }

impl Drop for NodeData {
	fn drop(&mut self) {
		extern "C" { fn cv_NodeData_delete(instance: *mut c_void); }
		unsafe { cv_NodeData_delete(self.as_raw_mut_NodeData()) };
	}
}

unsafe impl Send for NodeData {}

impl core::NodeDataTraitConst for NodeData {
	#[inline] fn as_raw_NodeData(&self) -> *const c_void { self.as_raw() }
}

impl core::NodeDataTrait for NodeData {
	#[inline] fn as_raw_mut_NodeData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NodeData {
	/// ## C++ default parameters
	/// * fun_name: 0
	/// * file_name: NULL
	/// * line_num: 0
	/// * ret_address: NULL
	/// * always_expand: false
	/// * instr_type: TYPE_GENERAL
	/// * impl_type: IMPL_PLAIN
	// NodeData(const char *, const char *, int, void *, bool, cv::instr::TYPE, cv::instr::IMPL) /usr/include/opencv2/core/utils/instrumentation.hpp:72
	#[inline]
	pub unsafe fn new(fun_name: &str, file_name: &str, line_num: i32, ret_address: *mut c_void, always_expand: bool, instr_type: core::TYPE, impl_type: core::IMPL) -> Result<core::NodeData> {
		extern_container_arg!(fun_name);
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		{ sys::cv_instr_NodeData_NodeData_const_charX_const_charX_int_voidX_bool_TYPE_IMPL(fun_name.opencv_as_extern(), file_name.opencv_as_extern(), line_num, ret_address, always_expand, instr_type, impl_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::NodeData::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// NodeData(cv::instr::NodeData &) /usr/include/opencv2/core/utils/instrumentation.hpp:73
	#[inline]
	pub fn copy_mut(ref_: &mut core::NodeData) -> Result<core::NodeData> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_instr_NodeData_NodeData_NodeDataR(ref_.as_raw_mut_NodeData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::NodeData::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// WriteStructContext /usr/include/opencv2/core/persistence.hpp:810
pub trait WriteStructContextTraitConst {
	fn as_raw_WriteStructContext(&self) -> *const c_void;

}

pub trait WriteStructContextTrait: core::WriteStructContextTraitConst {
	fn as_raw_mut_WriteStructContext(&mut self) -> *mut c_void;

}

// WriteStructContext /usr/include/opencv2/core/persistence.hpp:810
pub struct WriteStructContext {
	ptr: *mut c_void
}

opencv_type_boxed! { WriteStructContext }

impl Drop for WriteStructContext {
	fn drop(&mut self) {
		extern "C" { fn cv_WriteStructContext_delete(instance: *mut c_void); }
		unsafe { cv_WriteStructContext_delete(self.as_raw_mut_WriteStructContext()) };
	}
}

unsafe impl Send for WriteStructContext {}

impl core::WriteStructContextTraitConst for WriteStructContext {
	#[inline] fn as_raw_WriteStructContext(&self) -> *const c_void { self.as_raw() }
}

impl core::WriteStructContextTrait for WriteStructContext {
	#[inline] fn as_raw_mut_WriteStructContext(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WriteStructContext {
	/// ## C++ default parameters
	/// * type_name: String()
	// WriteStructContext(cv::FileStorage &, const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:813
	#[inline]
	pub fn new(_fs: &mut core::FileStorage, name: &str, flags: i32, type_name: &str) -> Result<core::WriteStructContext> {
		extern_container_arg!(name);
		extern_container_arg!(type_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_internal_WriteStructContext_WriteStructContext_FileStorageR_const_StringR_int_const_StringR(_fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), flags, type_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::WriteStructContext::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Context /usr/include/opencv2/core/ocl.hpp:253
pub trait ContextTraitConst {
	fn as_raw_Context(&self) -> *const c_void;

	// ndevices() /usr/include/opencv2/core/ocl.hpp:269
	#[inline]
	fn ndevices(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_ndevices_const(self.as_raw_Context(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// device(size_t) /usr/include/opencv2/core/ocl.hpp:270
	#[inline]
	fn device(&self, idx: size_t) -> Result<core::Device> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_device_const_size_t(self.as_raw_Context(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Device::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:284
	#[inline]
	fn ptr(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_ptr_const(self.as_raw_Context(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getOpenCLContextProperty(int) /usr/include/opencv2/core/ocl.hpp:291
	#[inline]
	fn get_opencl_context_property(&self, property_id: i32) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_getOpenCLContextProperty_const_int(self.as_raw_Context(), property_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// useSVM() /usr/include/opencv2/core/ocl.hpp:293
	#[inline]
	fn use_svm(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_useSVM_const(self.as_raw_Context(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:322
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_empty_const(self.as_raw_Context(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ContextTrait: core::ContextTraitConst {
	fn as_raw_mut_Context(&mut self) -> *mut c_void;

	// create() /usr/include/opencv2/core/ocl.hpp:265
	#[inline]
	fn create(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_create(self.as_raw_mut_Context(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// create(int) /usr/include/opencv2/core/ocl.hpp:267
	#[inline]
	fn create_with_type(&mut self, dtype: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_create_int(self.as_raw_mut_Context(), dtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getProg(const cv::ocl::ProgramSource &, const cv::String &, cv::String &) /usr/include/opencv2/core/ocl.hpp:271
	#[inline]
	fn get_prog(&mut self, prog: &core::ProgramSource, buildopt: &str, errmsg: &mut String) -> Result<core::Program> {
		extern_container_arg!(buildopt);
		string_arg_output_send!(via errmsg_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_getProg_const_ProgramSourceR_const_StringR_StringR(self.as_raw_mut_Context(), prog.as_raw_ProgramSource(), buildopt.opencv_as_extern(), &mut errmsg_via, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Program::opencv_from_extern(ret) };
		string_arg_output_receive!(errmsg_via => errmsg);
		Ok(ret)
	}
	
	// unloadProg(cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:273
	#[inline]
	fn unload_prog(&mut self, prog: &mut core::Program) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_unloadProg_ProgramR(self.as_raw_mut_Context(), prog.as_raw_mut_Program(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setUseSVM(bool) /usr/include/opencv2/core/ocl.hpp:294
	#[inline]
	fn set_use_svm(&mut self, enabled: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_setUseSVM_bool(self.as_raw_mut_Context(), enabled, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/ocl.hpp:303
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_release(self.as_raw_mut_Context(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Context /usr/include/opencv2/core/ocl.hpp:253
pub struct Context {
	ptr: *mut c_void
}

opencv_type_boxed! { Context }

impl Drop for Context {
	fn drop(&mut self) {
		extern "C" { fn cv_Context_delete(instance: *mut c_void); }
		unsafe { cv_Context_delete(self.as_raw_mut_Context()) };
	}
}

unsafe impl Send for Context {}

impl core::ContextTraitConst for Context {
	#[inline] fn as_raw_Context(&self) -> *const c_void { self.as_raw() }
}

impl core::ContextTrait for Context {
	#[inline] fn as_raw_mut_Context(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Context {
	// Context() /usr/include/opencv2/core/ocl.hpp:256
	#[inline]
	pub fn default() -> core::Context {
		let ret = unsafe { sys::cv_ocl_Context_Context() };
		let ret = unsafe { core::Context::opencv_from_extern(ret) };
		ret
	}
	
	// Context(int) /usr/include/opencv2/core/ocl.hpp:257
	#[inline]
	pub fn new_with_type(dtype: i32) -> Result<core::Context> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_Context_int(dtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Context::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Context(const cv::ocl::Context &) /usr/include/opencv2/core/ocl.hpp:259
	#[inline]
	pub fn copy(c: &core::Context) -> Result<core::Context> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_Context_const_ContextR(c.as_raw_Context(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Context::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Context(cv::ocl::Context &&) /usr/include/opencv2/core/ocl.hpp:261
	#[inline]
	pub fn copy_mut(c: &mut core::Context) -> core::Context {
		let ret = unsafe { sys::cv_ocl_Context_Context_ContextR(c.as_raw_mut_Context()) };
		let ret = unsafe { core::Context::opencv_from_extern(ret) };
		ret
	}
	
	/// ## C++ default parameters
	/// * initialize: true
	// getDefault(bool) /usr/include/opencv2/core/ocl.hpp:280
	#[inline]
	pub fn get_default(initialize: bool) -> Result<core::Context> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_getDefault_bool(initialize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Context::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// fromHandle(void *) /usr/include/opencv2/core/ocl.hpp:299
	#[inline]
	pub unsafe fn from_handle(context: *mut c_void) -> Result<core::Context> {
		return_send!(via ocvrs_return);
		{ sys::cv_ocl_Context_fromHandle_voidX(context, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Context::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// fromDevice(const ocl::Device &) /usr/include/opencv2/core/ocl.hpp:300
	#[inline]
	pub fn from_device(device: &core::Device) -> Result<core::Context> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_fromDevice_const_DeviceR(device.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Context::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create(const std::string &) /usr/include/opencv2/core/ocl.hpp:301
	#[inline]
	pub fn create(configuration: &str) -> Result<core::Context> {
		extern_container_arg!(configuration);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Context_create_const_stringR(configuration.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Context::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for Context {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// UserContext /usr/include/opencv2/core/ocl.hpp:305
pub trait Context_UserContextTraitConst {
	fn as_raw_Context_UserContext(&self) -> *const c_void;

}

pub trait Context_UserContextTrait: core::Context_UserContextTraitConst {
	fn as_raw_mut_Context_UserContext(&mut self) -> *mut c_void;

}

// UserContext /usr/include/opencv2/core/ocl.hpp:305
pub struct Context_UserContext {
	ptr: *mut c_void
}

opencv_type_boxed! { Context_UserContext }

impl Drop for Context_UserContext {
	fn drop(&mut self) {
		extern "C" { fn cv_Context_UserContext_delete(instance: *mut c_void); }
		unsafe { cv_Context_UserContext_delete(self.as_raw_mut_Context_UserContext()) };
	}
}

unsafe impl Send for Context_UserContext {}

impl core::Context_UserContextTraitConst for Context_UserContext {
	#[inline] fn as_raw_Context_UserContext(&self) -> *const c_void { self.as_raw() }
}

impl core::Context_UserContextTrait for Context_UserContext {
	#[inline] fn as_raw_mut_Context_UserContext(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Context_UserContext {
}

// Device /usr/include/opencv2/core/ocl.hpp:72
pub trait DeviceTraitConst {
	fn as_raw_Device(&self) -> *const c_void;

	// name() /usr/include/opencv2/core/ocl.hpp:96
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_name_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// extensions() /usr/include/opencv2/core/ocl.hpp:97
	#[inline]
	fn extensions(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_extensions_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// isExtensionSupported(const cv::String &) /usr/include/opencv2/core/ocl.hpp:98
	#[inline]
	fn is_extension_supported(&self, extension_name: &str) -> Result<bool> {
		extern_container_arg!(extension_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_isExtensionSupported_const_const_StringR(self.as_raw_Device(), extension_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// version() /usr/include/opencv2/core/ocl.hpp:99
	#[inline]
	fn version(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_version_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// vendorName() /usr/include/opencv2/core/ocl.hpp:100
	#[inline]
	fn vendor_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_vendorName_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// OpenCL_C_Version() /usr/include/opencv2/core/ocl.hpp:101
	#[inline]
	fn opencl_c_version(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_OpenCL_C_Version_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// OpenCLVersion() /usr/include/opencv2/core/ocl.hpp:102
	#[inline]
	fn opencl_version(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_OpenCLVersion_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// deviceVersionMajor() /usr/include/opencv2/core/ocl.hpp:103
	#[inline]
	fn device_version_major(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_deviceVersionMajor_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// deviceVersionMinor() /usr/include/opencv2/core/ocl.hpp:104
	#[inline]
	fn device_version_minor(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_deviceVersionMinor_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// driverVersion() /usr/include/opencv2/core/ocl.hpp:105
	#[inline]
	fn driver_version(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_driverVersion_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:106
	#[inline]
	fn ptr(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_ptr_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// type() /usr/include/opencv2/core/ocl.hpp:108
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_type_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// addressBits() /usr/include/opencv2/core/ocl.hpp:110
	#[inline]
	fn address_bits(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_addressBits_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// available() /usr/include/opencv2/core/ocl.hpp:111
	#[inline]
	fn available(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_available_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// compilerAvailable() /usr/include/opencv2/core/ocl.hpp:112
	#[inline]
	fn compiler_available(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_compilerAvailable_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// linkerAvailable() /usr/include/opencv2/core/ocl.hpp:113
	#[inline]
	fn linker_available(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_linkerAvailable_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// doubleFPConfig() /usr/include/opencv2/core/ocl.hpp:126
	#[inline]
	fn double_fp_config(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_doubleFPConfig_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// singleFPConfig() /usr/include/opencv2/core/ocl.hpp:127
	#[inline]
	fn single_fp_config(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_singleFPConfig_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// halfFPConfig() /usr/include/opencv2/core/ocl.hpp:128
	#[inline]
	fn half_fp_config(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_halfFPConfig_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// endianLittle() /usr/include/opencv2/core/ocl.hpp:130
	#[inline]
	fn endian_little(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_endianLittle_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// errorCorrectionSupport() /usr/include/opencv2/core/ocl.hpp:131
	#[inline]
	fn error_correction_support(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_errorCorrectionSupport_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// executionCapabilities() /usr/include/opencv2/core/ocl.hpp:138
	#[inline]
	fn execution_capabilities(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_executionCapabilities_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// globalMemCacheSize() /usr/include/opencv2/core/ocl.hpp:140
	#[inline]
	fn global_mem_cache_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_globalMemCacheSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// globalMemCacheType() /usr/include/opencv2/core/ocl.hpp:148
	#[inline]
	fn global_mem_cache_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_globalMemCacheType_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// globalMemCacheLineSize() /usr/include/opencv2/core/ocl.hpp:149
	#[inline]
	fn global_mem_cache_line_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_globalMemCacheLineSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// globalMemSize() /usr/include/opencv2/core/ocl.hpp:150
	#[inline]
	fn global_mem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_globalMemSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// localMemSize() /usr/include/opencv2/core/ocl.hpp:152
	#[inline]
	fn local_mem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_localMemSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// localMemType() /usr/include/opencv2/core/ocl.hpp:159
	#[inline]
	fn local_mem_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_localMemType_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// hostUnifiedMemory() /usr/include/opencv2/core/ocl.hpp:160
	#[inline]
	fn host_unified_memory(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_hostUnifiedMemory_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// imageSupport() /usr/include/opencv2/core/ocl.hpp:162
	#[inline]
	fn image_support(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_imageSupport_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// imageFromBufferSupport() /usr/include/opencv2/core/ocl.hpp:164
	#[inline]
	fn image_from_buffer_support(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_imageFromBufferSupport_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// imagePitchAlignment() /usr/include/opencv2/core/ocl.hpp:165
	#[inline]
	fn image_pitch_alignment(&self) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_imagePitchAlignment_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// imageBaseAddressAlignment() /usr/include/opencv2/core/ocl.hpp:166
	#[inline]
	fn image_base_address_alignment(&self) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_imageBaseAddressAlignment_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// intelSubgroupsSupport() /usr/include/opencv2/core/ocl.hpp:169
	#[inline]
	fn intel_subgroups_support(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_intelSubgroupsSupport_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// image2DMaxWidth() /usr/include/opencv2/core/ocl.hpp:171
	#[inline]
	fn image2_d_max_width(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_image2DMaxWidth_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// image2DMaxHeight() /usr/include/opencv2/core/ocl.hpp:172
	#[inline]
	fn image2_d_max_height(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_image2DMaxHeight_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// image3DMaxWidth() /usr/include/opencv2/core/ocl.hpp:174
	#[inline]
	fn image3_d_max_width(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_image3DMaxWidth_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// image3DMaxHeight() /usr/include/opencv2/core/ocl.hpp:175
	#[inline]
	fn image3_d_max_height(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_image3DMaxHeight_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// image3DMaxDepth() /usr/include/opencv2/core/ocl.hpp:176
	#[inline]
	fn image3_d_max_depth(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_image3DMaxDepth_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// imageMaxBufferSize() /usr/include/opencv2/core/ocl.hpp:178
	#[inline]
	fn image_max_buffer_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_imageMaxBufferSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// imageMaxArraySize() /usr/include/opencv2/core/ocl.hpp:179
	#[inline]
	fn image_max_array_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_imageMaxArraySize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// vendorID() /usr/include/opencv2/core/ocl.hpp:188
	#[inline]
	fn vendor_id(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_vendorID_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isAMD() /usr/include/opencv2/core/ocl.hpp:193
	#[inline]
	fn is_amd(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_isAMD_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isIntel() /usr/include/opencv2/core/ocl.hpp:194
	#[inline]
	fn is_intel(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_isIntel_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isNVidia() /usr/include/opencv2/core/ocl.hpp:195
	#[inline]
	fn is_n_vidia(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_isNVidia_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxClockFrequency() /usr/include/opencv2/core/ocl.hpp:197
	#[inline]
	fn max_clock_frequency(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxClockFrequency_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxComputeUnits() /usr/include/opencv2/core/ocl.hpp:198
	#[inline]
	fn max_compute_units(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxComputeUnits_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxConstantArgs() /usr/include/opencv2/core/ocl.hpp:199
	#[inline]
	fn max_constant_args(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxConstantArgs_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxConstantBufferSize() /usr/include/opencv2/core/ocl.hpp:200
	#[inline]
	fn max_constant_buffer_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxConstantBufferSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxMemAllocSize() /usr/include/opencv2/core/ocl.hpp:202
	#[inline]
	fn max_mem_alloc_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxMemAllocSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxParameterSize() /usr/include/opencv2/core/ocl.hpp:203
	#[inline]
	fn max_parameter_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxParameterSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxReadImageArgs() /usr/include/opencv2/core/ocl.hpp:205
	#[inline]
	fn max_read_image_args(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxReadImageArgs_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxWriteImageArgs() /usr/include/opencv2/core/ocl.hpp:206
	#[inline]
	fn max_write_image_args(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxWriteImageArgs_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxSamplers() /usr/include/opencv2/core/ocl.hpp:207
	#[inline]
	fn max_samplers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxSamplers_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxWorkGroupSize() /usr/include/opencv2/core/ocl.hpp:209
	#[inline]
	fn max_work_group_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxWorkGroupSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxWorkItemDims() /usr/include/opencv2/core/ocl.hpp:210
	#[inline]
	fn max_work_item_dims(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxWorkItemDims_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// maxWorkItemSizes(size_t *) /usr/include/opencv2/core/ocl.hpp:211
	#[inline]
	fn max_work_item_sizes(&self, unnamed: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_maxWorkItemSizes_const_size_tX(self.as_raw_Device(), unnamed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// memBaseAddrAlign() /usr/include/opencv2/core/ocl.hpp:213
	#[inline]
	fn mem_base_addr_align(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_memBaseAddrAlign_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// nativeVectorWidthChar() /usr/include/opencv2/core/ocl.hpp:215
	#[inline]
	fn native_vector_width_char(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_nativeVectorWidthChar_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// nativeVectorWidthShort() /usr/include/opencv2/core/ocl.hpp:216
	#[inline]
	fn native_vector_width_short(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_nativeVectorWidthShort_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// nativeVectorWidthInt() /usr/include/opencv2/core/ocl.hpp:217
	#[inline]
	fn native_vector_width_int(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_nativeVectorWidthInt_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// nativeVectorWidthLong() /usr/include/opencv2/core/ocl.hpp:218
	#[inline]
	fn native_vector_width_long(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_nativeVectorWidthLong_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// nativeVectorWidthFloat() /usr/include/opencv2/core/ocl.hpp:219
	#[inline]
	fn native_vector_width_float(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_nativeVectorWidthFloat_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// nativeVectorWidthDouble() /usr/include/opencv2/core/ocl.hpp:220
	#[inline]
	fn native_vector_width_double(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_nativeVectorWidthDouble_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// nativeVectorWidthHalf() /usr/include/opencv2/core/ocl.hpp:221
	#[inline]
	fn native_vector_width_half(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_nativeVectorWidthHalf_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// preferredVectorWidthChar() /usr/include/opencv2/core/ocl.hpp:223
	#[inline]
	fn preferred_vector_width_char(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_preferredVectorWidthChar_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// preferredVectorWidthShort() /usr/include/opencv2/core/ocl.hpp:224
	#[inline]
	fn preferred_vector_width_short(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_preferredVectorWidthShort_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// preferredVectorWidthInt() /usr/include/opencv2/core/ocl.hpp:225
	#[inline]
	fn preferred_vector_width_int(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_preferredVectorWidthInt_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// preferredVectorWidthLong() /usr/include/opencv2/core/ocl.hpp:226
	#[inline]
	fn preferred_vector_width_long(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_preferredVectorWidthLong_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// preferredVectorWidthFloat() /usr/include/opencv2/core/ocl.hpp:227
	#[inline]
	fn preferred_vector_width_float(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_preferredVectorWidthFloat_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// preferredVectorWidthDouble() /usr/include/opencv2/core/ocl.hpp:228
	#[inline]
	fn preferred_vector_width_double(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_preferredVectorWidthDouble_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// preferredVectorWidthHalf() /usr/include/opencv2/core/ocl.hpp:229
	#[inline]
	fn preferred_vector_width_half(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_preferredVectorWidthHalf_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// printfBufferSize() /usr/include/opencv2/core/ocl.hpp:231
	#[inline]
	fn printf_buffer_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_printfBufferSize_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// profilingTimerResolution() /usr/include/opencv2/core/ocl.hpp:232
	#[inline]
	fn profiling_timer_resolution(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_profilingTimerResolution_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:247
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_empty_const(self.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DeviceTrait: core::DeviceTraitConst {
	fn as_raw_mut_Device(&mut self) -> *mut c_void;

	// set(void *) /usr/include/opencv2/core/ocl.hpp:83
	#[inline]
	unsafe fn set(&mut self, d: *mut c_void) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_ocl_Device_set_voidX(self.as_raw_mut_Device(), d, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Device /usr/include/opencv2/core/ocl.hpp:72
pub struct Device {
	ptr: *mut c_void
}

opencv_type_boxed! { Device }

impl Drop for Device {
	fn drop(&mut self) {
		extern "C" { fn cv_Device_delete(instance: *mut c_void); }
		unsafe { cv_Device_delete(self.as_raw_mut_Device()) };
	}
}

unsafe impl Send for Device {}

impl core::DeviceTraitConst for Device {
	#[inline] fn as_raw_Device(&self) -> *const c_void { self.as_raw() }
}

impl core::DeviceTrait for Device {
	#[inline] fn as_raw_mut_Device(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Device {
	// Device() /usr/include/opencv2/core/ocl.hpp:75
	#[inline]
	pub fn default() -> core::Device {
		let ret = unsafe { sys::cv_ocl_Device_Device() };
		let ret = unsafe { core::Device::opencv_from_extern(ret) };
		ret
	}
	
	// Device(void *) /usr/include/opencv2/core/ocl.hpp:76
	#[inline]
	pub unsafe fn new(d: *mut c_void) -> Result<core::Device> {
		return_send!(via ocvrs_return);
		{ sys::cv_ocl_Device_Device_voidX(d, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Device::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Device(const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:77
	#[inline]
	pub fn copy(d: &core::Device) -> Result<core::Device> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_Device_const_DeviceR(d.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Device::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Device(cv::ocl::Device &&) /usr/include/opencv2/core/ocl.hpp:79
	#[inline]
	pub fn copy_mut(d: &mut core::Device) -> core::Device {
		let ret = unsafe { sys::cv_ocl_Device_Device_DeviceR(d.as_raw_mut_Device()) };
		let ret = unsafe { core::Device::opencv_from_extern(ret) };
		ret
	}
	
	// getDefault() /usr/include/opencv2/core/ocl.hpp:234
	#[inline]
	pub fn get_default() -> Result<core::Device> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Device_getDefault(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Device::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// fromHandle(void *) /usr/include/opencv2/core/ocl.hpp:243
	#[inline]
	pub unsafe fn from_handle(d: *mut c_void) -> Result<core::Device> {
		return_send!(via ocvrs_return);
		{ sys::cv_ocl_Device_fromHandle_voidX(d, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Device::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for Device {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// Image2D /usr/include/opencv2/core/ocl.hpp:739
pub trait Image2DTraitConst {
	fn as_raw_Image2D(&self) -> *const c_void;

	// ptr() /usr/include/opencv2/core/ocl.hpp:767
	#[inline]
	fn ptr(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Image2D_ptr_const(self.as_raw_Image2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Image2DTrait: core::Image2DTraitConst {
	fn as_raw_mut_Image2D(&mut self) -> *mut c_void;

}

// Image2D /usr/include/opencv2/core/ocl.hpp:739
pub struct Image2D {
	ptr: *mut c_void
}

opencv_type_boxed! { Image2D }

impl Drop for Image2D {
	fn drop(&mut self) {
		extern "C" { fn cv_Image2D_delete(instance: *mut c_void); }
		unsafe { cv_Image2D_delete(self.as_raw_mut_Image2D()) };
	}
}

unsafe impl Send for Image2D {}

impl core::Image2DTraitConst for Image2D {
	#[inline] fn as_raw_Image2D(&self) -> *const c_void { self.as_raw() }
}

impl core::Image2DTrait for Image2D {
	#[inline] fn as_raw_mut_Image2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Image2D {
	// Image2D() /usr/include/opencv2/core/ocl.hpp:742
	#[inline]
	pub fn default() -> core::Image2D {
		let ret = unsafe { sys::cv_ocl_Image2D_Image2D() };
		let ret = unsafe { core::Image2D::opencv_from_extern(ret) };
		ret
	}
	
	/// ## C++ default parameters
	/// * norm: false
	/// * alias: false
	// Image2D(const cv::UMat &, bool, bool) /usr/include/opencv2/core/ocl.hpp:750
	#[inline]
	pub fn new(src: &core::UMat, norm: bool, alias: bool) -> Result<core::Image2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Image2D_Image2D_const_UMatR_bool_bool(src.as_raw_UMat(), norm, alias, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Image2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Image2D(const cv::ocl::Image2D &) /usr/include/opencv2/core/ocl.hpp:751
	#[inline]
	pub fn copy(i: &core::Image2D) -> Result<core::Image2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Image2D_Image2D_const_Image2DR(i.as_raw_Image2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Image2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Image2D(cv::ocl::Image2D &&) /usr/include/opencv2/core/ocl.hpp:755
	#[inline]
	pub fn copy_mut(unnamed: &mut core::Image2D) -> core::Image2D {
		let ret = unsafe { sys::cv_ocl_Image2D_Image2D_Image2DR(unnamed.as_raw_mut_Image2D()) };
		let ret = unsafe { core::Image2D::opencv_from_extern(ret) };
		ret
	}
	
	// canCreateAlias(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:761
	#[inline]
	pub fn can_create_alias(u: &core::UMat) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Image2D_canCreateAlias_const_UMatR(u.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isFormatSupported(int, int, bool) /usr/include/opencv2/core/ocl.hpp:765
	#[inline]
	pub fn is_format_supported(depth: i32, cn: i32, norm: bool) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Image2D_isFormatSupported_int_int_bool(depth, cn, norm, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl Default for Image2D {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// Kernel /usr/include/opencv2/core/ocl.hpp:456
pub trait KernelTraitConst {
	fn as_raw_Kernel(&self) -> *const c_void;

	// empty() /usr/include/opencv2/core/ocl.hpp:469
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_empty_const(self.as_raw_Kernel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// workGroupSize() /usr/include/opencv2/core/ocl.hpp:537
	#[inline]
	fn work_group_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_workGroupSize_const(self.as_raw_Kernel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// preferedWorkGroupSizeMultiple() /usr/include/opencv2/core/ocl.hpp:538
	#[inline]
	fn prefered_work_group_size_multiple(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_preferedWorkGroupSizeMultiple_const(self.as_raw_Kernel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// compileWorkGroupSize(size_t *) /usr/include/opencv2/core/ocl.hpp:539
	#[inline]
	fn compile_work_group_size(&self, wsz: &mut [size_t]) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_compileWorkGroupSize_const_size_tX(self.as_raw_Kernel(), wsz.as_mut_ptr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// localMemSize() /usr/include/opencv2/core/ocl.hpp:540
	#[inline]
	fn local_mem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_localMemSize_const(self.as_raw_Kernel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:542
	#[inline]
	fn ptr(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_ptr_const(self.as_raw_Kernel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait KernelTrait: core::KernelTraitConst {
	fn as_raw_mut_Kernel(&mut self) -> *mut c_void;

	// create(const char *, const cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:470
	#[inline]
	fn create(&mut self, kname: &str, prog: &core::Program) -> Result<bool> {
		extern_container_arg!(kname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_create_const_charX_const_ProgramR(self.as_raw_mut_Kernel(), kname.opencv_as_extern(), prog.as_raw_Program(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * errmsg: 0
	// create(const char *, const cv::ocl::ProgramSource &, const cv::String &, cv::String *) /usr/include/opencv2/core/ocl.hpp:471
	#[inline]
	fn create_ext(&mut self, kname: &str, prog: &core::ProgramSource, buildopts: &str, errmsg: &mut String) -> Result<bool> {
		extern_container_arg!(kname);
		extern_container_arg!(buildopts);
		string_arg_output_send!(via errmsg_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR_StringX(self.as_raw_mut_Kernel(), kname.opencv_as_extern(), prog.as_raw_ProgramSource(), buildopts.opencv_as_extern(), &mut errmsg_via, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(errmsg_via => errmsg);
		Ok(ret)
	}
	
	// set(int, const void *, size_t) /usr/include/opencv2/core/ocl.hpp:474
	#[inline]
	unsafe fn set(&mut self, i: i32, value: *const c_void, sz: size_t) -> Result<i32> {
		return_send!(via ocvrs_return);
		{ sys::cv_ocl_Kernel_set_int_const_voidX_size_t(self.as_raw_mut_Kernel(), i, value, sz, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// set(int, const cv::ocl::Image2D &) /usr/include/opencv2/core/ocl.hpp:475
	#[inline]
	fn set_1(&mut self, i: i32, image_2d: &core::Image2D) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_set_int_const_Image2DR(self.as_raw_mut_Kernel(), i, image_2d.as_raw_Image2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// set(int, const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:476
	#[inline]
	fn set_umat(&mut self, i: i32, m: &core::UMat) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_set_int_const_UMatR(self.as_raw_mut_Kernel(), i, m.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// set(int, const cv::ocl::KernelArg &) /usr/include/opencv2/core/ocl.hpp:477
	#[inline]
	fn set_kernel_arg(&mut self, i: i32, arg: &core::KernelArg) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_set_int_const_KernelArgR(self.as_raw_mut_Kernel(), i, arg.as_raw_KernelArg(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * q: Queue()
	// run(int, size_t *, size_t *, bool, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:515
	#[inline]
	fn run(&mut self, dims: i32, globalsize: &mut [size_t], localsize: &mut [size_t], sync: bool, q: &core::Queue) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_run_int_size_tX_size_tX_bool_const_QueueR(self.as_raw_mut_Kernel(), dims, globalsize.as_mut_ptr(), localsize.as_mut_ptr(), sync, q.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * q: Queue()
	// run_(int, size_t *, size_t *, bool, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:526
	#[inline]
	fn run_(&mut self, dims: i32, globalsize: &mut [size_t], localsize: &mut [size_t], sync: bool, q: &core::Queue) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_run__int_size_tX_size_tX_bool_const_QueueR(self.as_raw_mut_Kernel(), dims, globalsize.as_mut_ptr(), localsize.as_mut_ptr(), sync, q.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * q: Queue()
	// runTask(bool, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:528
	#[inline]
	fn run_task(&mut self, sync: bool, q: &core::Queue) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_runTask_bool_const_QueueR(self.as_raw_mut_Kernel(), sync, q.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * q: Queue()
	// runProfiling(int, size_t *, size_t *, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:535
	#[inline]
	fn run_profiling(&mut self, dims: i32, globalsize: &mut [size_t], localsize: &mut [size_t], q: &core::Queue) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_runProfiling_int_size_tX_size_tX_const_QueueR(self.as_raw_mut_Kernel(), dims, globalsize.as_mut_ptr(), localsize.as_mut_ptr(), q.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Kernel /usr/include/opencv2/core/ocl.hpp:456
pub struct Kernel {
	ptr: *mut c_void
}

opencv_type_boxed! { Kernel }

impl Drop for Kernel {
	fn drop(&mut self) {
		extern "C" { fn cv_Kernel_delete(instance: *mut c_void); }
		unsafe { cv_Kernel_delete(self.as_raw_mut_Kernel()) };
	}
}

unsafe impl Send for Kernel {}

impl core::KernelTraitConst for Kernel {
	#[inline] fn as_raw_Kernel(&self) -> *const c_void { self.as_raw() }
}

impl core::KernelTrait for Kernel {
	#[inline] fn as_raw_mut_Kernel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Kernel {
	// Kernel() /usr/include/opencv2/core/ocl.hpp:459
	#[inline]
	pub fn default() -> core::Kernel {
		let ret = unsafe { sys::cv_ocl_Kernel_Kernel() };
		let ret = unsafe { core::Kernel::opencv_from_extern(ret) };
		ret
	}
	
	// Kernel(const char *, const cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:460
	#[inline]
	pub fn new(kname: &str, prog: &core::Program) -> Result<core::Kernel> {
		extern_container_arg!(kname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_Kernel_const_charX_const_ProgramR(kname.opencv_as_extern(), prog.as_raw_Program(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Kernel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * buildopts: String()
	/// * errmsg: 0
	// Kernel(const char *, const cv::ocl::ProgramSource &, const cv::String &, cv::String *) /usr/include/opencv2/core/ocl.hpp:461
	#[inline]
	pub fn new_1(kname: &str, prog: &core::ProgramSource, buildopts: &str, errmsg: &mut String) -> Result<core::Kernel> {
		extern_container_arg!(kname);
		extern_container_arg!(buildopts);
		string_arg_output_send!(via errmsg_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceR_const_StringR_StringX(kname.opencv_as_extern(), prog.as_raw_ProgramSource(), buildopts.opencv_as_extern(), &mut errmsg_via, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Kernel::opencv_from_extern(ret) };
		string_arg_output_receive!(errmsg_via => errmsg);
		Ok(ret)
	}
	
	// Kernel(const cv::ocl::Kernel &) /usr/include/opencv2/core/ocl.hpp:464
	#[inline]
	pub fn copy(k: &core::Kernel) -> Result<core::Kernel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Kernel_Kernel_const_KernelR(k.as_raw_Kernel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Kernel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Kernel(cv::ocl::Kernel &&) /usr/include/opencv2/core/ocl.hpp:466
	#[inline]
	pub fn copy_mut(k: &mut core::Kernel) -> core::Kernel {
		let ret = unsafe { sys::cv_ocl_Kernel_Kernel_KernelR(k.as_raw_mut_Kernel()) };
		let ret = unsafe { core::Kernel::opencv_from_extern(ret) };
		ret
	}
	
}

impl Default for Kernel {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// KernelArg /usr/include/opencv2/core/ocl.hpp:417
pub trait KernelArgTraitConst {
	fn as_raw_KernelArg(&self) -> *const c_void;

	// flags /usr/include/opencv2/core/ocl.hpp:448
	#[inline]
	fn flags(&self) -> i32 {
		let ret = unsafe { sys::cv_ocl_KernelArg_getPropFlags_const(self.as_raw_KernelArg()) };
		ret
	}
	
	// obj /usr/include/opencv2/core/ocl.hpp:450
	#[inline]
	fn obj(&self) -> *const c_void {
		let ret = unsafe { sys::cv_ocl_KernelArg_getPropObj_const(self.as_raw_KernelArg()) };
		ret
	}
	
	// sz /usr/include/opencv2/core/ocl.hpp:451
	#[inline]
	fn sz(&self) -> size_t {
		let ret = unsafe { sys::cv_ocl_KernelArg_getPropSz_const(self.as_raw_KernelArg()) };
		ret
	}
	
	// wscale /usr/include/opencv2/core/ocl.hpp:452
	#[inline]
	fn wscale(&self) -> i32 {
		let ret = unsafe { sys::cv_ocl_KernelArg_getPropWscale_const(self.as_raw_KernelArg()) };
		ret
	}
	
	// iwscale /usr/include/opencv2/core/ocl.hpp:452
	#[inline]
	fn iwscale(&self) -> i32 {
		let ret = unsafe { sys::cv_ocl_KernelArg_getPropIwscale_const(self.as_raw_KernelArg()) };
		ret
	}
	
}

pub trait KernelArgTrait: core::KernelArgTraitConst {
	fn as_raw_mut_KernelArg(&mut self) -> *mut c_void;

	// flags /usr/include/opencv2/core/ocl.hpp:448
	#[inline]
	fn set_flags(&mut self, val: i32) {
		let ret = unsafe { sys::cv_ocl_KernelArg_setPropFlags_int(self.as_raw_mut_KernelArg(), val) };
		ret
	}
	
	// m /usr/include/opencv2/core/ocl.hpp:449
	#[inline]
	fn m(&mut self) -> core::UMat {
		let ret = unsafe { sys::cv_ocl_KernelArg_getPropM(self.as_raw_mut_KernelArg()) };
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		ret
	}
	
	// m /usr/include/opencv2/core/ocl.hpp:449
	#[inline]
	fn set_m(&mut self, val: &mut core::UMat) {
		let ret = unsafe { sys::cv_ocl_KernelArg_setPropM_UMatX(self.as_raw_mut_KernelArg(), val.as_raw_mut_UMat()) };
		ret
	}
	
	// sz /usr/include/opencv2/core/ocl.hpp:451
	#[inline]
	fn set_sz(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_ocl_KernelArg_setPropSz_size_t(self.as_raw_mut_KernelArg(), val) };
		ret
	}
	
	// wscale /usr/include/opencv2/core/ocl.hpp:452
	#[inline]
	fn set_wscale(&mut self, val: i32) {
		let ret = unsafe { sys::cv_ocl_KernelArg_setPropWscale_int(self.as_raw_mut_KernelArg(), val) };
		ret
	}
	
	// iwscale /usr/include/opencv2/core/ocl.hpp:452
	#[inline]
	fn set_iwscale(&mut self, val: i32) {
		let ret = unsafe { sys::cv_ocl_KernelArg_setPropIwscale_int(self.as_raw_mut_KernelArg(), val) };
		ret
	}
	
}

// KernelArg /usr/include/opencv2/core/ocl.hpp:417
pub struct KernelArg {
	ptr: *mut c_void
}

opencv_type_boxed! { KernelArg }

impl Drop for KernelArg {
	fn drop(&mut self) {
		extern "C" { fn cv_KernelArg_delete(instance: *mut c_void); }
		unsafe { cv_KernelArg_delete(self.as_raw_mut_KernelArg()) };
	}
}

unsafe impl Send for KernelArg {}

impl core::KernelArgTraitConst for KernelArg {
	#[inline] fn as_raw_KernelArg(&self) -> *const c_void { self.as_raw() }
}

impl core::KernelArgTrait for KernelArg {
	#[inline] fn as_raw_mut_KernelArg(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KernelArg {
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	/// * _obj: 0
	/// * _sz: 0
	// KernelArg(int, cv::UMat *, int, int, const void *, size_t) /usr/include/opencv2/core/ocl.hpp:421
	#[inline]
	pub unsafe fn new(_flags: i32, _m: &mut core::UMat, wscale: i32, iwscale: i32, _obj: *const c_void, _sz: size_t) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		{ sys::cv_ocl_KernelArg_KernelArg_int_UMatX_int_int_const_voidX_size_t(_flags, _m.as_raw_mut_UMat(), wscale, iwscale, _obj, _sz, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// KernelArg() /usr/include/opencv2/core/ocl.hpp:422
	#[inline]
	pub fn default() -> core::KernelArg {
		let ret = unsafe { sys::cv_ocl_KernelArg_KernelArg() };
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		ret
	}
	
	// Local(size_t) /usr/include/opencv2/core/ocl.hpp:424
	#[inline]
	pub fn local(local_mem_size: size_t) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_Local_size_t(local_mem_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// PtrWriteOnly(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:426
	#[inline]
	pub fn ptr_write_only(m: &core::UMat) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_PtrWriteOnly_const_UMatR(m.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// PtrReadOnly(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:428
	#[inline]
	pub fn ptr_read_only(m: &core::UMat) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_PtrReadOnly_const_UMatR(m.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// PtrReadWrite(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:430
	#[inline]
	pub fn ptr_read_write(m: &core::UMat) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_PtrReadWrite_const_UMatR(m.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	// ReadWrite(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:432
	#[inline]
	pub fn read_write(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_ReadWrite_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	// ReadWriteNoSize(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:434
	#[inline]
	pub fn read_write_no_size(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_ReadWriteNoSize_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	// ReadOnly(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:436
	#[inline]
	pub fn read_only(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_ReadOnly_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	// WriteOnly(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:438
	#[inline]
	pub fn write_only(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_WriteOnly_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	// ReadOnlyNoSize(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:440
	#[inline]
	pub fn read_only_no_size(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	// WriteOnlyNoSize(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:442
	#[inline]
	pub fn write_only_no_size(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Constant(const cv::Mat &) /usr/include/opencv2/core/ocl.hpp:444
	#[inline]
	pub fn constant(m: &core::Mat) -> Result<core::KernelArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_KernelArg_Constant_const_MatR(m.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::KernelArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for KernelArg {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// OpenCLExecutionContext /usr/include/opencv2/core/ocl.hpp:795
pub trait OpenCLExecutionContextTraitConst {
	fn as_raw_OpenCLExecutionContext(&self) -> *const c_void;

	// getContext() /usr/include/opencv2/core/ocl.hpp:808
	#[inline]
	fn get_context(&self) -> Result<core::Context> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_getContext_const(self.as_raw_OpenCLExecutionContext(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Context::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getDevice() /usr/include/opencv2/core/ocl.hpp:810
	#[inline]
	fn get_device(&self) -> Result<core::Device> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_getDevice_const(self.as_raw_OpenCLExecutionContext(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Device::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getQueue() /usr/include/opencv2/core/ocl.hpp:814
	#[inline]
	fn get_queue(&self) -> Result<core::Queue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_getQueue_const(self.as_raw_OpenCLExecutionContext(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Queue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// useOpenCL() /usr/include/opencv2/core/ocl.hpp:816
	#[inline]
	fn use_opencl(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_useOpenCL_const(self.as_raw_OpenCLExecutionContext(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// bind() /usr/include/opencv2/core/ocl.hpp:836
	#[inline]
	fn bind(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_bind_const(self.as_raw_OpenCLExecutionContext(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// cloneWithNewQueue(const ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:842
	#[inline]
	fn clone_with_new_queue(&self, q: &core::Queue) -> Result<core::OpenCLExecutionContext> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_cloneWithNewQueue_const_const_QueueR(self.as_raw_OpenCLExecutionContext(), q.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// cloneWithNewQueue() /usr/include/opencv2/core/ocl.hpp:844
	#[inline]
	fn clone_with_new_queue_1(&self) -> Result<core::OpenCLExecutionContext> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_cloneWithNewQueue_const(self.as_raw_OpenCLExecutionContext(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:873
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_empty_const(self.as_raw_OpenCLExecutionContext(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait OpenCLExecutionContextTrait: core::OpenCLExecutionContextTraitConst {
	fn as_raw_mut_OpenCLExecutionContext(&mut self) -> *mut c_void;

	// setUseOpenCL(bool) /usr/include/opencv2/core/ocl.hpp:817
	#[inline]
	fn set_use_opencl(&mut self, flag: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_setUseOpenCL_bool(self.as_raw_mut_OpenCLExecutionContext(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/ocl.hpp:874
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_release(self.as_raw_mut_OpenCLExecutionContext(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// OpenCLExecutionContext /usr/include/opencv2/core/ocl.hpp:795
pub struct OpenCLExecutionContext {
	ptr: *mut c_void
}

opencv_type_boxed! { OpenCLExecutionContext }

impl Drop for OpenCLExecutionContext {
	fn drop(&mut self) {
		extern "C" { fn cv_OpenCLExecutionContext_delete(instance: *mut c_void); }
		unsafe { cv_OpenCLExecutionContext_delete(self.as_raw_mut_OpenCLExecutionContext()) };
	}
}

unsafe impl Send for OpenCLExecutionContext {}

impl core::OpenCLExecutionContextTraitConst for OpenCLExecutionContext {
	#[inline] fn as_raw_OpenCLExecutionContext(&self) -> *const c_void { self.as_raw() }
}

impl core::OpenCLExecutionContextTrait for OpenCLExecutionContext {
	#[inline] fn as_raw_mut_OpenCLExecutionContext(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OpenCLExecutionContext {
	// OpenCLExecutionContext() /usr/include/opencv2/core/ocl.hpp:798
	#[inline]
	pub fn default() -> core::OpenCLExecutionContext {
		let ret = unsafe { sys::cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext() };
		let ret = unsafe { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		ret
	}
	
	// OpenCLExecutionContext(const cv::ocl::OpenCLExecutionContext &) /usr/include/opencv2/core/ocl.hpp:801
	#[inline]
	pub fn copy(unnamed: &core::OpenCLExecutionContext) -> core::OpenCLExecutionContext {
		let ret = unsafe { sys::cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext_const_OpenCLExecutionContextR(unnamed.as_raw_OpenCLExecutionContext()) };
		let ret = unsafe { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		ret
	}
	
	// OpenCLExecutionContext(cv::ocl::OpenCLExecutionContext &&) /usr/include/opencv2/core/ocl.hpp:802
	#[inline]
	pub fn copy_mut(unnamed: &mut core::OpenCLExecutionContext) -> core::OpenCLExecutionContext {
		let ret = unsafe { sys::cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext_OpenCLExecutionContextR(unnamed.as_raw_mut_OpenCLExecutionContext()) };
		let ret = unsafe { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		ret
	}
	
	// getCurrent() /usr/include/opencv2/core/ocl.hpp:825
	#[inline]
	pub fn get_current() -> Result<core::OpenCLExecutionContext> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_getCurrent(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getCurrentRef() /usr/include/opencv2/core/ocl.hpp:828
	#[inline]
	pub fn get_current_ref() -> Result<core::OpenCLExecutionContext> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_getCurrentRef(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create(const std::string &, void *, void *, void *) /usr/include/opencv2/core/ocl.hpp:860
	#[inline]
	pub unsafe fn create(platform_name: &str, platform_id: *mut c_void, context: *mut c_void, device_id: *mut c_void) -> Result<core::OpenCLExecutionContext> {
		extern_container_arg!(platform_name);
		return_send!(via ocvrs_return);
		{ sys::cv_ocl_OpenCLExecutionContext_create_const_stringR_voidX_voidX_voidX(platform_name.opencv_as_extern(), platform_id, context, device_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create(const cv::ocl::Context &, const cv::ocl::Device &, const ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:868
	#[inline]
	pub fn create_1(context: &core::Context, device: &core::Device, queue: &core::Queue) -> Result<core::OpenCLExecutionContext> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_create_const_ContextR_const_DeviceR_const_QueueR(context.as_raw_Context(), device.as_raw_Device(), queue.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create(const cv::ocl::Context &, const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:870
	#[inline]
	pub fn create_2(context: &core::Context, device: &core::Device) -> Result<core::OpenCLExecutionContext> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_OpenCLExecutionContext_create_const_ContextR_const_DeviceR(context.as_raw_Context(), device.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::OpenCLExecutionContext::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for OpenCLExecutionContext {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// Platform /usr/include/opencv2/core/ocl.hpp:329
pub trait PlatformTraitConst {
	fn as_raw_Platform(&self) -> *const c_void;

	// ptr() /usr/include/opencv2/core/ocl.hpp:339
	#[inline]
	fn ptr(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Platform_ptr_const(self.as_raw_Platform(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:346
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Platform_empty_const(self.as_raw_Platform(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait PlatformTrait: core::PlatformTraitConst {
	fn as_raw_mut_Platform(&mut self) -> *mut c_void;

}

// Platform /usr/include/opencv2/core/ocl.hpp:329
pub struct Platform {
	ptr: *mut c_void
}

opencv_type_boxed! { Platform }

impl Drop for Platform {
	fn drop(&mut self) {
		extern "C" { fn cv_Platform_delete(instance: *mut c_void); }
		unsafe { cv_Platform_delete(self.as_raw_mut_Platform()) };
	}
}

unsafe impl Send for Platform {}

impl core::PlatformTraitConst for Platform {
	#[inline] fn as_raw_Platform(&self) -> *const c_void { self.as_raw() }
}

impl core::PlatformTrait for Platform {
	#[inline] fn as_raw_mut_Platform(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Platform {
	// Platform() /usr/include/opencv2/core/ocl.hpp:332
	#[inline]
	pub fn default() -> core::Platform {
		let ret = unsafe { sys::cv_ocl_Platform_Platform() };
		let ret = unsafe { core::Platform::opencv_from_extern(ret) };
		ret
	}
	
	// Platform(const cv::ocl::Platform &) /usr/include/opencv2/core/ocl.hpp:334
	#[inline]
	pub fn copy(p: &core::Platform) -> Result<core::Platform> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Platform_Platform_const_PlatformR(p.as_raw_Platform(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Platform::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Platform(cv::ocl::Platform &&) /usr/include/opencv2/core/ocl.hpp:336
	#[inline]
	pub fn copy_mut(p: &mut core::Platform) -> core::Platform {
		let ret = unsafe { sys::cv_ocl_Platform_Platform_PlatformR(p.as_raw_mut_Platform()) };
		let ret = unsafe { core::Platform::opencv_from_extern(ret) };
		ret
	}
	
	// getDefault() /usr/include/opencv2/core/ocl.hpp:342
	#[inline]
	pub fn get_default() -> Result<core::Platform> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Platform_getDefault(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Platform::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for Platform {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// PlatformInfo /usr/include/opencv2/core/ocl.hpp:668
pub trait PlatformInfoTraitConst {
	fn as_raw_PlatformInfo(&self) -> *const c_void;

	// name() /usr/include/opencv2/core/ocl.hpp:683
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_PlatformInfo_name_const(self.as_raw_PlatformInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// vendor() /usr/include/opencv2/core/ocl.hpp:684
	#[inline]
	fn vendor(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_PlatformInfo_vendor_const(self.as_raw_PlatformInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// version() /usr/include/opencv2/core/ocl.hpp:687
	#[inline]
	fn version(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_PlatformInfo_version_const(self.as_raw_PlatformInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// versionMajor() /usr/include/opencv2/core/ocl.hpp:688
	#[inline]
	fn version_major(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_PlatformInfo_versionMajor_const(self.as_raw_PlatformInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// versionMinor() /usr/include/opencv2/core/ocl.hpp:689
	#[inline]
	fn version_minor(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_PlatformInfo_versionMinor_const(self.as_raw_PlatformInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// deviceNumber() /usr/include/opencv2/core/ocl.hpp:691
	#[inline]
	fn device_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_PlatformInfo_deviceNumber_const(self.as_raw_PlatformInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDevice(cv::ocl::Device &, int) /usr/include/opencv2/core/ocl.hpp:692
	#[inline]
	fn get_device(&self, device: &mut core::Device, d: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_PlatformInfo_getDevice_const_DeviceR_int(self.as_raw_PlatformInfo(), device.as_raw_mut_Device(), d, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:695
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_PlatformInfo_empty_const(self.as_raw_PlatformInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait PlatformInfoTrait: core::PlatformInfoTraitConst {
	fn as_raw_mut_PlatformInfo(&mut self) -> *mut c_void;

}

// PlatformInfo /usr/include/opencv2/core/ocl.hpp:668
pub struct PlatformInfo {
	ptr: *mut c_void
}

opencv_type_boxed! { PlatformInfo }

impl Drop for PlatformInfo {
	fn drop(&mut self) {
		extern "C" { fn cv_PlatformInfo_delete(instance: *mut c_void); }
		unsafe { cv_PlatformInfo_delete(self.as_raw_mut_PlatformInfo()) };
	}
}

unsafe impl Send for PlatformInfo {}

impl core::PlatformInfoTraitConst for PlatformInfo {
	#[inline] fn as_raw_PlatformInfo(&self) -> *const c_void { self.as_raw() }
}

impl core::PlatformInfoTrait for PlatformInfo {
	#[inline] fn as_raw_mut_PlatformInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PlatformInfo {
	// PlatformInfo() /usr/include/opencv2/core/ocl.hpp:671
	#[inline]
	pub fn default() -> core::PlatformInfo {
		let ret = unsafe { sys::cv_ocl_PlatformInfo_PlatformInfo() };
		let ret = unsafe { core::PlatformInfo::opencv_from_extern(ret) };
		ret
	}
	
	// PlatformInfo(void *) /usr/include/opencv2/core/ocl.hpp:675
	#[inline]
	pub unsafe fn new(id: *mut c_void) -> Result<core::PlatformInfo> {
		return_send!(via ocvrs_return);
		{ sys::cv_ocl_PlatformInfo_PlatformInfo_voidX(id, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::PlatformInfo::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// PlatformInfo(const cv::ocl::PlatformInfo &) /usr/include/opencv2/core/ocl.hpp:678
	#[inline]
	pub fn copy(i: &core::PlatformInfo) -> Result<core::PlatformInfo> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_PlatformInfo_PlatformInfo_const_PlatformInfoR(i.as_raw_PlatformInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::PlatformInfo::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// PlatformInfo(cv::ocl::PlatformInfo &&) /usr/include/opencv2/core/ocl.hpp:680
	#[inline]
	pub fn copy_mut(i: &mut core::PlatformInfo) -> core::PlatformInfo {
		let ret = unsafe { sys::cv_ocl_PlatformInfo_PlatformInfo_PlatformInfoR(i.as_raw_mut_PlatformInfo()) };
		let ret = unsafe { core::PlatformInfo::opencv_from_extern(ret) };
		ret
	}
	
}

impl Default for PlatformInfo {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// Program /usr/include/opencv2/core/ocl.hpp:549
pub trait ProgramTraitConst {
	fn as_raw_Program(&self) -> *const c_void;

	// ptr() /usr/include/opencv2/core/ocl.hpp:564
	#[inline]
	fn ptr(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_ptr_const(self.as_raw_Program(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getBinary(std::vector<char> &) /usr/include/opencv2/core/ocl.hpp:575
	#[inline]
	fn get_binary(&self, binary: &mut core::Vector<i8>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_getBinary_const_vector_char_R(self.as_raw_Program(), binary.as_raw_mut_VectorOfi8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:579
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_empty_const(self.as_raw_Program(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// write(cv::String &) /usr/include/opencv2/core/ocl.hpp:586
	#[inline]
	fn write(&self, buf: &mut String) -> Result<bool> {
		string_arg_output_send!(via buf_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_write_const_StringR(self.as_raw_Program(), &mut buf_via, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(buf_via => buf);
		Ok(ret)
	}
	
	// source() /usr/include/opencv2/core/ocl.hpp:587
	#[inline]
	fn source(&self) -> Result<core::ProgramSource> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_source_const(self.as_raw_Program(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::ProgramSource::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getPrefix() /usr/include/opencv2/core/ocl.hpp:588
	#[inline]
	fn get_prefix(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_getPrefix_const(self.as_raw_Program(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait ProgramTrait: core::ProgramTraitConst {
	fn as_raw_mut_Program(&mut self) -> *mut c_void;

	// create(const cv::ocl::ProgramSource &, const cv::String &, cv::String &) /usr/include/opencv2/core/ocl.hpp:561
	#[inline]
	fn create(&mut self, src: &core::ProgramSource, buildflags: &str, errmsg: &mut String) -> Result<bool> {
		extern_container_arg!(buildflags);
		string_arg_output_send!(via errmsg_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_create_const_ProgramSourceR_const_StringR_StringR(self.as_raw_mut_Program(), src.as_raw_ProgramSource(), buildflags.opencv_as_extern(), &mut errmsg_via, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(errmsg_via => errmsg);
		Ok(ret)
	}
	
	// read(const cv::String &, const cv::String &) /usr/include/opencv2/core/ocl.hpp:585
	#[inline]
	fn read(&mut self, buf: &str, buildflags: &str) -> Result<bool> {
		extern_container_arg!(buf);
		extern_container_arg!(buildflags);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_read_const_StringR_const_StringR(self.as_raw_mut_Program(), buf.opencv_as_extern(), buildflags.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Program /usr/include/opencv2/core/ocl.hpp:549
pub struct Program {
	ptr: *mut c_void
}

opencv_type_boxed! { Program }

impl Drop for Program {
	fn drop(&mut self) {
		extern "C" { fn cv_Program_delete(instance: *mut c_void); }
		unsafe { cv_Program_delete(self.as_raw_mut_Program()) };
	}
}

unsafe impl Send for Program {}

impl core::ProgramTraitConst for Program {
	#[inline] fn as_raw_Program(&self) -> *const c_void { self.as_raw() }
}

impl core::ProgramTrait for Program {
	#[inline] fn as_raw_mut_Program(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Program {
	// Program() /usr/include/opencv2/core/ocl.hpp:552
	#[inline]
	pub fn default() -> core::Program {
		let ret = unsafe { sys::cv_ocl_Program_Program() };
		let ret = unsafe { core::Program::opencv_from_extern(ret) };
		ret
	}
	
	// Program(const cv::ocl::ProgramSource &, const cv::String &, cv::String &) /usr/include/opencv2/core/ocl.hpp:553
	#[inline]
	pub fn new(src: &core::ProgramSource, buildflags: &str, errmsg: &mut String) -> Result<core::Program> {
		extern_container_arg!(buildflags);
		string_arg_output_send!(via errmsg_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_Program_const_ProgramSourceR_const_StringR_StringR(src.as_raw_ProgramSource(), buildflags.opencv_as_extern(), &mut errmsg_via, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Program::opencv_from_extern(ret) };
		string_arg_output_receive!(errmsg_via => errmsg);
		Ok(ret)
	}
	
	// Program(const cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:555
	#[inline]
	pub fn copy(prog: &core::Program) -> Result<core::Program> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_Program_const_ProgramR(prog.as_raw_Program(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Program::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Program(cv::ocl::Program &&) /usr/include/opencv2/core/ocl.hpp:557
	#[inline]
	pub fn copy_mut(prog: &mut core::Program) -> core::Program {
		let ret = unsafe { sys::cv_ocl_Program_Program_ProgramR(prog.as_raw_mut_Program()) };
		let ret = unsafe { core::Program::opencv_from_extern(ret) };
		ret
	}
	
	// getPrefix(const cv::String &) /usr/include/opencv2/core/ocl.hpp:589
	#[inline]
	pub fn get_prefix_build_flags(buildflags: &str) -> Result<String> {
		extern_container_arg!(buildflags);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Program_getPrefix_const_StringR(buildflags.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for Program {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// ProgramSource /usr/include/opencv2/core/ocl.hpp:594
pub trait ProgramSourceTraitConst {
	fn as_raw_ProgramSource(&self) -> *const c_void;

	// source() /usr/include/opencv2/core/ocl.hpp:609
	#[inline]
	fn source(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_ProgramSource_source_const(self.as_raw_ProgramSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// hash() /usr/include/opencv2/core/ocl.hpp:610
	#[inline]
	fn hash(&self) -> Result<core::ProgramSource_hash_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_ProgramSource_hash_const(self.as_raw_ProgramSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:663
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_ProgramSource_empty_const(self.as_raw_ProgramSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ProgramSourceTrait: core::ProgramSourceTraitConst {
	fn as_raw_mut_ProgramSource(&mut self) -> *mut c_void;

}

// ProgramSource /usr/include/opencv2/core/ocl.hpp:594
pub struct ProgramSource {
	ptr: *mut c_void
}

opencv_type_boxed! { ProgramSource }

impl Drop for ProgramSource {
	fn drop(&mut self) {
		extern "C" { fn cv_ProgramSource_delete(instance: *mut c_void); }
		unsafe { cv_ProgramSource_delete(self.as_raw_mut_ProgramSource()) };
	}
}

unsafe impl Send for ProgramSource {}

impl core::ProgramSourceTraitConst for ProgramSource {
	#[inline] fn as_raw_ProgramSource(&self) -> *const c_void { self.as_raw() }
}

impl core::ProgramSourceTrait for ProgramSource {
	#[inline] fn as_raw_mut_ProgramSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ProgramSource {
	// ProgramSource() /usr/include/opencv2/core/ocl.hpp:599
	#[inline]
	pub fn default() -> core::ProgramSource {
		let ret = unsafe { sys::cv_ocl_ProgramSource_ProgramSource() };
		let ret = unsafe { core::ProgramSource::opencv_from_extern(ret) };
		ret
	}
	
	// ProgramSource(const cv::String &, const cv::String &, const cv::String &, const cv::String &) /usr/include/opencv2/core/ocl.hpp:600
	#[inline]
	pub fn new(module: &str, name: &str, code_str: &str, code_hash: &str) -> Result<core::ProgramSource> {
		extern_container_arg!(module);
		extern_container_arg!(name);
		extern_container_arg!(code_str);
		extern_container_arg!(code_hash);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_ProgramSource_ProgramSource_const_StringR_const_StringR_const_StringR_const_StringR(module.opencv_as_extern(), name.opencv_as_extern(), code_str.opencv_as_extern(), code_hash.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::ProgramSource::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ProgramSource(const cv::String &) /usr/include/opencv2/core/ocl.hpp:601
	#[inline]
	pub fn from_str(prog: &str) -> Result<core::ProgramSource> {
		extern_container_arg!(prog);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_ProgramSource_ProgramSource_const_StringR(prog.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::ProgramSource::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ProgramSource(const cv::ocl::ProgramSource &) /usr/include/opencv2/core/ocl.hpp:604
	#[inline]
	pub fn copy(prog: &core::ProgramSource) -> Result<core::ProgramSource> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_ProgramSource_ProgramSource_const_ProgramSourceR(prog.as_raw_ProgramSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::ProgramSource::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// ProgramSource(cv::ocl::ProgramSource &&) /usr/include/opencv2/core/ocl.hpp:606
	#[inline]
	pub fn copy_mut(prog: &mut core::ProgramSource) -> core::ProgramSource {
		let ret = unsafe { sys::cv_ocl_ProgramSource_ProgramSource_ProgramSourceR(prog.as_raw_mut_ProgramSource()) };
		let ret = unsafe { core::ProgramSource::opencv_from_extern(ret) };
		ret
	}
	
	/// ## C++ default parameters
	/// * build_options: cv::String()
	// fromBinary(const cv::String &, const cv::String &, const unsigned char *, const size_t, const cv::String &) /usr/include/opencv2/core/ocl.hpp:627
	#[inline]
	pub fn from_binary(module: &str, name: &str, binary: &u8, size: size_t, build_options: &str) -> Result<core::ProgramSource> {
		extern_container_arg!(module);
		extern_container_arg!(name);
		extern_container_arg!(build_options);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_ProgramSource_fromBinary_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(module.opencv_as_extern(), name.opencv_as_extern(), binary, size, build_options.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::ProgramSource::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * build_options: cv::String()
	// fromSPIR(const cv::String &, const cv::String &, const unsigned char *, const size_t, const cv::String &) /usr/include/opencv2/core/ocl.hpp:652
	#[inline]
	pub fn from_spir(module: &str, name: &str, binary: &u8, size: size_t, build_options: &str) -> Result<core::ProgramSource> {
		extern_container_arg!(module);
		extern_container_arg!(name);
		extern_container_arg!(build_options);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_ProgramSource_fromSPIR_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(module.opencv_as_extern(), name.opencv_as_extern(), binary, size, build_options.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::ProgramSource::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for ProgramSource {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// Queue /usr/include/opencv2/core/ocl.hpp:390
pub trait QueueTraitConst {
	fn as_raw_Queue(&self) -> *const c_void;

	// ptr() /usr/include/opencv2/core/ocl.hpp:403
	#[inline]
	fn ptr(&self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Queue_ptr_const(self.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getProfilingQueue() /usr/include/opencv2/core/ocl.hpp:407
	#[inline]
	fn get_profiling_queue(&self) -> Result<core::Queue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Queue_getProfilingQueue_const(self.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Queue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:411
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Queue_empty_const(self.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait QueueTrait: core::QueueTraitConst {
	fn as_raw_mut_Queue(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * c: Context()
	/// * d: Device()
	// create(const cv::ocl::Context &, const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:401
	#[inline]
	fn create(&mut self, c: &core::Context, d: &core::Device) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Queue_create_const_ContextR_const_DeviceR(self.as_raw_mut_Queue(), c.as_raw_Context(), d.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// finish() /usr/include/opencv2/core/ocl.hpp:402
	#[inline]
	fn finish(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Queue_finish(self.as_raw_mut_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Queue /usr/include/opencv2/core/ocl.hpp:390
pub struct Queue {
	ptr: *mut c_void
}

opencv_type_boxed! { Queue }

impl Drop for Queue {
	fn drop(&mut self) {
		extern "C" { fn cv_Queue_delete(instance: *mut c_void); }
		unsafe { cv_Queue_delete(self.as_raw_mut_Queue()) };
	}
}

unsafe impl Send for Queue {}

impl core::QueueTraitConst for Queue {
	#[inline] fn as_raw_Queue(&self) -> *const c_void { self.as_raw() }
}

impl core::QueueTrait for Queue {
	#[inline] fn as_raw_mut_Queue(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Queue {
	// Queue() /usr/include/opencv2/core/ocl.hpp:393
	#[inline]
	pub fn default() -> core::Queue {
		let ret = unsafe { sys::cv_ocl_Queue_Queue() };
		let ret = unsafe { core::Queue::opencv_from_extern(ret) };
		ret
	}
	
	/// ## C++ default parameters
	/// * d: Device()
	// Queue(const cv::ocl::Context &, const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:394
	#[inline]
	pub fn new(c: &core::Context, d: &core::Device) -> Result<core::Queue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Queue_Queue_const_ContextR_const_DeviceR(c.as_raw_Context(), d.as_raw_Device(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Queue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Queue(const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:396
	#[inline]
	pub fn copy(q: &core::Queue) -> Result<core::Queue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Queue_Queue_const_QueueR(q.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Queue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Queue(cv::ocl::Queue &&) /usr/include/opencv2/core/ocl.hpp:398
	#[inline]
	pub fn copy_mut(q: &mut core::Queue) -> core::Queue {
		let ret = unsafe { sys::cv_ocl_Queue_Queue_QueueR(q.as_raw_mut_Queue()) };
		let ret = unsafe { core::Queue::opencv_from_extern(ret) };
		ret
	}
	
	// getDefault() /usr/include/opencv2/core/ocl.hpp:404
	#[inline]
	pub fn get_default() -> Result<core::Queue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Queue_getDefault(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Queue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for Queue {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// Timer /usr/include/opencv2/core/ocl.hpp:773
pub trait TimerTraitConst {
	fn as_raw_Timer(&self) -> *const c_void;

	// durationNS() /usr/include/opencv2/core/ocl.hpp:781
	#[inline]
	fn duration_ns(&self) -> Result<u64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Timer_durationNS_const(self.as_raw_Timer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TimerTrait: core::TimerTraitConst {
	fn as_raw_mut_Timer(&mut self) -> *mut c_void;

	// start() /usr/include/opencv2/core/ocl.hpp:778
	#[inline]
	fn start(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Timer_start(self.as_raw_mut_Timer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// stop() /usr/include/opencv2/core/ocl.hpp:779
	#[inline]
	fn stop(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Timer_stop(self.as_raw_mut_Timer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Timer /usr/include/opencv2/core/ocl.hpp:773
pub struct Timer {
	ptr: *mut c_void
}

opencv_type_boxed! { Timer }

impl Drop for Timer {
	fn drop(&mut self) {
		extern "C" { fn cv_Timer_delete(instance: *mut c_void); }
		unsafe { cv_Timer_delete(self.as_raw_mut_Timer()) };
	}
}

unsafe impl Send for Timer {}

impl core::TimerTraitConst for Timer {
	#[inline] fn as_raw_Timer(&self) -> *const c_void { self.as_raw() }
}

impl core::TimerTrait for Timer {
	#[inline] fn as_raw_mut_Timer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Timer {
	// Timer(const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:776
	#[inline]
	pub fn new(q: &core::Queue) -> Result<core::Timer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ocl_Timer_Timer_const_QueueR(q.as_raw_Queue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Timer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Arrays /usr/include/opencv2/core/opengl.hpp:406
pub trait ArraysTraitConst {
	fn as_raw_Arrays(&self) -> *const c_void;

	// bind() /usr/include/opencv2/core/opengl.hpp:460
	#[inline]
	fn bind(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_bind_const(self.as_raw_Arrays(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// size() /usr/include/opencv2/core/opengl.hpp:464
	#[inline]
	fn size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_size_const(self.as_raw_Arrays(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/opengl.hpp:465
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_empty_const(self.as_raw_Arrays(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ArraysTrait: core::ArraysTraitConst {
	fn as_raw_mut_Arrays(&mut self) -> *mut c_void;

	// setVertexArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:416
	#[inline]
	fn set_vertex_array(&mut self, vertex: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(vertex);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_setVertexArray_const__InputArrayR(self.as_raw_mut_Arrays(), vertex.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// resetVertexArray() /usr/include/opencv2/core/opengl.hpp:420
	#[inline]
	fn reset_vertex_array(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_resetVertexArray(self.as_raw_mut_Arrays(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setColorArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:425
	#[inline]
	fn set_color_array(&mut self, color: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(color);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_setColorArray_const__InputArrayR(self.as_raw_mut_Arrays(), color.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// resetColorArray() /usr/include/opencv2/core/opengl.hpp:429
	#[inline]
	fn reset_color_array(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_resetColorArray(self.as_raw_mut_Arrays(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setNormalArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:434
	#[inline]
	fn set_normal_array(&mut self, normal: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(normal);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_setNormalArray_const__InputArrayR(self.as_raw_mut_Arrays(), normal.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// resetNormalArray() /usr/include/opencv2/core/opengl.hpp:438
	#[inline]
	fn reset_normal_array(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_resetNormalArray(self.as_raw_mut_Arrays(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setTexCoordArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:443
	#[inline]
	fn set_tex_coord_array(&mut self, tex_coord: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(tex_coord);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_setTexCoordArray_const__InputArrayR(self.as_raw_mut_Arrays(), tex_coord.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// resetTexCoordArray() /usr/include/opencv2/core/opengl.hpp:447
	#[inline]
	fn reset_tex_coord_array(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_resetTexCoordArray(self.as_raw_mut_Arrays(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/opengl.hpp:451
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_release(self.as_raw_mut_Arrays(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setAutoRelease(bool) /usr/include/opencv2/core/opengl.hpp:456
	#[inline]
	fn set_auto_release(&mut self, flag: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_setAutoRelease_bool(self.as_raw_mut_Arrays(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Arrays /usr/include/opencv2/core/opengl.hpp:406
pub struct Arrays {
	ptr: *mut c_void
}

opencv_type_boxed! { Arrays }

impl Drop for Arrays {
	fn drop(&mut self) {
		extern "C" { fn cv_Arrays_delete(instance: *mut c_void); }
		unsafe { cv_Arrays_delete(self.as_raw_mut_Arrays()) };
	}
}

unsafe impl Send for Arrays {}

impl core::ArraysTraitConst for Arrays {
	#[inline] fn as_raw_Arrays(&self) -> *const c_void { self.as_raw() }
}

impl core::ArraysTrait for Arrays {
	#[inline] fn as_raw_mut_Arrays(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Arrays {
	// Arrays() /usr/include/opencv2/core/opengl.hpp:411
	#[inline]
	pub fn default() -> Result<core::Arrays> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Arrays_Arrays(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Arrays::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Buffer /usr/include/opencv2/core/opengl.hpp:79
pub trait BufferTraitConst {
	fn as_raw_Buffer(&self) -> *const c_void;

	// copyTo(cv::OutputArray) /usr/include/opencv2/core/opengl.hpp:197
	#[inline]
	fn copy_to(&self, arr: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_copyTo_const_const__OutputArrayR(self.as_raw_Buffer(), arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// copyTo(cv::OutputArray, cuda::Stream &) /usr/include/opencv2/core/opengl.hpp:200
	#[inline]
	fn copy_to_1(&self, arr: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_copyTo_const_const__OutputArrayR_StreamR(self.as_raw_Buffer(), arr.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * target: ARRAY_BUFFER
	/// * auto_release: false
	// clone(cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:207
	#[inline]
	fn clone(&self, target: core::Buffer_Target, auto_release: bool) -> Result<core::Buffer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_clone_const_Target_bool(self.as_raw_Buffer(), target, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Buffer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// bind(cv::ogl::Buffer::Target) /usr/include/opencv2/core/opengl.hpp:213
	#[inline]
	fn bind(&self, target: core::Buffer_Target) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_bind_const_Target(self.as_raw_Buffer(), target, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// rows() /usr/include/opencv2/core/opengl.hpp:258
	#[inline]
	fn rows(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_rows_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// cols() /usr/include/opencv2/core/opengl.hpp:259
	#[inline]
	fn cols(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_cols_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// size() /usr/include/opencv2/core/opengl.hpp:260
	#[inline]
	fn size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_size_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/opengl.hpp:261
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_empty_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// type() /usr/include/opencv2/core/opengl.hpp:263
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_type_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// depth() /usr/include/opencv2/core/opengl.hpp:264
	#[inline]
	fn depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_depth_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// channels() /usr/include/opencv2/core/opengl.hpp:265
	#[inline]
	fn channels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_channels_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize() /usr/include/opencv2/core/opengl.hpp:266
	#[inline]
	fn elem_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_elemSize_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize1() /usr/include/opencv2/core/opengl.hpp:267
	#[inline]
	fn elem_size1(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_elemSize1_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// bufId() /usr/include/opencv2/core/opengl.hpp:270
	#[inline]
	fn buf_id(&self) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_bufId_const(self.as_raw_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BufferTrait: core::BufferTraitConst {
	fn as_raw_mut_Buffer(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * target: ARRAY_BUFFER
	/// * auto_release: false
	// create(int, int, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:155
	#[inline]
	fn create(&mut self, arows: i32, acols: i32, atype: i32, target: core::Buffer_Target, auto_release: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_create_int_int_int_Target_bool(self.as_raw_mut_Buffer(), arows, acols, atype, target, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * target: ARRAY_BUFFER
	/// * auto_release: false
	// create(cv::Size, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:163
	#[inline]
	fn create_size(&mut self, asize: core::Size, atype: i32, target: core::Buffer_Target, auto_release: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_create_Size_int_Target_bool(self.as_raw_mut_Buffer(), asize.opencv_as_extern(), atype, target, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/opengl.hpp:169
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_release(self.as_raw_mut_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setAutoRelease(bool) /usr/include/opencv2/core/opengl.hpp:180
	#[inline]
	fn set_auto_release(&mut self, flag: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_setAutoRelease_bool(self.as_raw_mut_Buffer(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * target: ARRAY_BUFFER
	/// * auto_release: false
	// copyFrom(cv::InputArray, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:187
	#[inline]
	fn copy_from(&mut self, arr: &dyn core::ToInputArray, target: core::Buffer_Target, auto_release: bool) -> Result<()> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_copyFrom_const__InputArrayR_Target_bool(self.as_raw_mut_Buffer(), arr.as_raw__InputArray(), target, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * target: ARRAY_BUFFER
	/// * auto_release: false
	// copyFrom(cv::InputArray, cuda::Stream &, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:190
	#[inline]
	fn copy_from_1(&mut self, arr: &dyn core::ToInputArray, stream: &mut core::Stream, target: core::Buffer_Target, auto_release: bool) -> Result<()> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_copyFrom_const__InputArrayR_StreamR_Target_bool(self.as_raw_mut_Buffer(), arr.as_raw__InputArray(), stream.as_raw_mut_Stream(), target, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// mapHost(cv::ogl::Buffer::Access) /usr/include/opencv2/core/opengl.hpp:236
	#[inline]
	fn map_host(&mut self, access: core::Buffer_Access) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_mapHost_Access(self.as_raw_mut_Buffer(), access, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// unmapHost() /usr/include/opencv2/core/opengl.hpp:240
	#[inline]
	fn unmap_host(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_unmapHost(self.as_raw_mut_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// mapDevice() /usr/include/opencv2/core/opengl.hpp:243
	#[inline]
	fn map_device(&mut self) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_mapDevice(self.as_raw_mut_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// unmapDevice() /usr/include/opencv2/core/opengl.hpp:244
	#[inline]
	fn unmap_device(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_unmapDevice(self.as_raw_mut_Buffer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// mapDevice(cuda::Stream &) /usr/include/opencv2/core/opengl.hpp:252
	#[inline]
	fn map_device_1(&mut self, stream: &mut core::Stream) -> Result<core::GpuMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_mapDevice_StreamR(self.as_raw_mut_Buffer(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// unmapDevice(cuda::Stream &) /usr/include/opencv2/core/opengl.hpp:256
	#[inline]
	fn unmap_device_1(&mut self, stream: &mut core::Stream) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_unmapDevice_StreamR(self.as_raw_mut_Buffer(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Buffer /usr/include/opencv2/core/opengl.hpp:79
pub struct Buffer {
	ptr: *mut c_void
}

opencv_type_boxed! { Buffer }

impl Drop for Buffer {
	fn drop(&mut self) {
		extern "C" { fn cv_Buffer_delete(instance: *mut c_void); }
		unsafe { cv_Buffer_delete(self.as_raw_mut_Buffer()) };
	}
}

unsafe impl Send for Buffer {}

impl core::BufferTraitConst for Buffer {
	#[inline] fn as_raw_Buffer(&self) -> *const c_void { self.as_raw() }
}

impl core::BufferTrait for Buffer {
	#[inline] fn as_raw_mut_Buffer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Buffer {
	// Buffer() /usr/include/opencv2/core/opengl.hpp:104
	#[inline]
	pub fn default() -> Result<core::Buffer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_Buffer(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Buffer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * auto_release: false
	// Buffer(int, int, int, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:113
	#[inline]
	pub fn new(arows: i32, acols: i32, atype: i32, abuf_id: u32, auto_release: bool) -> Result<core::Buffer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_Buffer_int_int_int_unsigned_int_bool(arows, acols, atype, abuf_id, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Buffer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * auto_release: false
	// Buffer(cv::Size, int, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:121
	#[inline]
	pub fn new_1(asize: core::Size, atype: i32, abuf_id: u32, auto_release: bool) -> Result<core::Buffer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_Buffer_Size_int_unsigned_int_bool(asize.opencv_as_extern(), atype, abuf_id, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Buffer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * target: ARRAY_BUFFER
	/// * auto_release: false
	// Buffer(int, int, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:130
	#[inline]
	pub fn new_2(arows: i32, acols: i32, atype: i32, target: core::Buffer_Target, auto_release: bool) -> Result<core::Buffer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_Buffer_int_int_int_Target_bool(arows, acols, atype, target, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Buffer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * target: ARRAY_BUFFER
	/// * auto_release: false
	// Buffer(cv::Size, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:138
	#[inline]
	pub fn new_3(asize: core::Size, atype: i32, target: core::Buffer_Target, auto_release: bool) -> Result<core::Buffer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_Buffer_Size_int_Target_bool(asize.opencv_as_extern(), atype, target, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Buffer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * target: ARRAY_BUFFER
	/// * auto_release: false
	// Buffer(cv::InputArray, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:145
	#[inline]
	pub fn new_4(arr: &dyn core::ToInputArray, target: core::Buffer_Target, auto_release: bool) -> Result<core::Buffer> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_Buffer_const__InputArrayR_Target_bool(arr.as_raw__InputArray(), target, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Buffer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// unbind(cv::ogl::Buffer::Target) /usr/include/opencv2/core/opengl.hpp:219
	#[inline]
	pub fn unbind(target: core::Buffer_Target) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Buffer_unbind_Target(target, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Texture2D /usr/include/opencv2/core/opengl.hpp:283
pub trait Texture2DTraitConst {
	fn as_raw_Texture2D(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * ddepth: CV_32F
	/// * auto_release: false
	// copyTo(cv::OutputArray, int, bool) /usr/include/opencv2/core/opengl.hpp:377
	#[inline]
	fn copy_to(&self, arr: &mut dyn core::ToOutputArray, ddepth: i32, auto_release: bool) -> Result<()> {
		output_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_copyTo_const_const__OutputArrayR_int_bool(self.as_raw_Texture2D(), arr.as_raw__OutputArray(), ddepth, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// bind() /usr/include/opencv2/core/opengl.hpp:381
	#[inline]
	fn bind(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_bind_const(self.as_raw_Texture2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// rows() /usr/include/opencv2/core/opengl.hpp:383
	#[inline]
	fn rows(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_rows_const(self.as_raw_Texture2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// cols() /usr/include/opencv2/core/opengl.hpp:384
	#[inline]
	fn cols(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_cols_const(self.as_raw_Texture2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// size() /usr/include/opencv2/core/opengl.hpp:385
	#[inline]
	fn size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_size_const(self.as_raw_Texture2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// empty() /usr/include/opencv2/core/opengl.hpp:386
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_empty_const(self.as_raw_Texture2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// format() /usr/include/opencv2/core/opengl.hpp:388
	#[inline]
	fn format(&self) -> Result<core::Texture2D_Format> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_format_const(self.as_raw_Texture2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// texId() /usr/include/opencv2/core/opengl.hpp:391
	#[inline]
	fn tex_id(&self) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_texId_const(self.as_raw_Texture2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Texture2DTrait: core::Texture2DTraitConst {
	fn as_raw_mut_Texture2D(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * auto_release: false
	// create(int, int, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:337
	#[inline]
	fn create(&mut self, arows: i32, acols: i32, aformat: core::Texture2D_Format, auto_release: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_create_int_int_Format_bool(self.as_raw_mut_Texture2D(), arows, acols, aformat, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * auto_release: false
	// create(cv::Size, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:343
	#[inline]
	fn create_1(&mut self, asize: core::Size, aformat: core::Texture2D_Format, auto_release: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_create_Size_Format_bool(self.as_raw_mut_Texture2D(), asize.opencv_as_extern(), aformat, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/core/opengl.hpp:349
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_release(self.as_raw_mut_Texture2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setAutoRelease(bool) /usr/include/opencv2/core/opengl.hpp:361
	#[inline]
	fn set_auto_release(&mut self, flag: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_setAutoRelease_bool(self.as_raw_mut_Texture2D(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * auto_release: false
	// copyFrom(cv::InputArray, bool) /usr/include/opencv2/core/opengl.hpp:368
	#[inline]
	fn copy_from(&mut self, arr: &dyn core::ToInputArray, auto_release: bool) -> Result<()> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_copyFrom_const__InputArrayR_bool(self.as_raw_mut_Texture2D(), arr.as_raw__InputArray(), auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Texture2D /usr/include/opencv2/core/opengl.hpp:283
pub struct Texture2D {
	ptr: *mut c_void
}

opencv_type_boxed! { Texture2D }

impl Drop for Texture2D {
	fn drop(&mut self) {
		extern "C" { fn cv_Texture2D_delete(instance: *mut c_void); }
		unsafe { cv_Texture2D_delete(self.as_raw_mut_Texture2D()) };
	}
}

unsafe impl Send for Texture2D {}

impl core::Texture2DTraitConst for Texture2D {
	#[inline] fn as_raw_Texture2D(&self) -> *const c_void { self.as_raw() }
}

impl core::Texture2DTrait for Texture2D {
	#[inline] fn as_raw_mut_Texture2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Texture2D {
	// Texture2D() /usr/include/opencv2/core/opengl.hpp:301
	#[inline]
	pub fn default() -> Result<core::Texture2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_Texture2D(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Texture2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * auto_release: false
	// Texture2D(int, int, cv::ogl::Texture2D::Format, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:304
	#[inline]
	pub fn new(arows: i32, acols: i32, aformat: core::Texture2D_Format, atex_id: u32, auto_release: bool) -> Result<core::Texture2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_Texture2D_int_int_Format_unsigned_int_bool(arows, acols, aformat, atex_id, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Texture2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * auto_release: false
	// Texture2D(cv::Size, cv::ogl::Texture2D::Format, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:307
	#[inline]
	pub fn new_1(asize: core::Size, aformat: core::Texture2D_Format, atex_id: u32, auto_release: bool) -> Result<core::Texture2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_Texture2D_Size_Format_unsigned_int_bool(asize.opencv_as_extern(), aformat, atex_id, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Texture2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * auto_release: false
	// Texture2D(int, int, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:315
	#[inline]
	pub fn new_2(arows: i32, acols: i32, aformat: core::Texture2D_Format, auto_release: bool) -> Result<core::Texture2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_Texture2D_int_int_Format_bool(arows, acols, aformat, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Texture2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * auto_release: false
	// Texture2D(cv::Size, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:322
	#[inline]
	pub fn new_3(asize: core::Size, aformat: core::Texture2D_Format, auto_release: bool) -> Result<core::Texture2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_Texture2D_Size_Format_bool(asize.opencv_as_extern(), aformat, auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Texture2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * auto_release: false
	// Texture2D(cv::InputArray, bool) /usr/include/opencv2/core/opengl.hpp:328
	#[inline]
	pub fn new_4(arr: &dyn core::ToInputArray, auto_release: bool) -> Result<core::Texture2D> {
		input_array_arg!(arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ogl_Texture2D_Texture2D_const__InputArrayR_bool(arr.as_raw__InputArray(), auto_release, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Texture2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// LogTag /usr/include/opencv2/core/utils/logtag.hpp:15
pub trait LogTagTraitConst {
	fn as_raw_LogTag(&self) -> *const c_void;

	// name /usr/include/opencv2/core/utils/logtag.hpp:17
	#[inline]
	fn name(&self) -> String {
		let ret = unsafe { sys::cv_utils_logging_LogTag_getPropName_const(self.as_raw_LogTag()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// level /usr/include/opencv2/core/utils/logtag.hpp:18
	#[inline]
	fn level(&self) -> core::LogLevel {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_utils_logging_LogTag_getPropLevel_const(self.as_raw_LogTag(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
}

pub trait LogTagTrait: core::LogTagTraitConst {
	fn as_raw_mut_LogTag(&mut self) -> *mut c_void;

	// level /usr/include/opencv2/core/utils/logtag.hpp:18
	#[inline]
	fn set_level(&mut self, val: core::LogLevel) {
		let ret = unsafe { sys::cv_utils_logging_LogTag_setPropLevel_LogLevel(self.as_raw_mut_LogTag(), val) };
		ret
	}
	
}

// LogTag /usr/include/opencv2/core/utils/logtag.hpp:15
pub struct LogTag {
	ptr: *mut c_void
}

opencv_type_boxed! { LogTag }

impl Drop for LogTag {
	fn drop(&mut self) {
		extern "C" { fn cv_LogTag_delete(instance: *mut c_void); }
		unsafe { cv_LogTag_delete(self.as_raw_mut_LogTag()) };
	}
}

unsafe impl Send for LogTag {}

impl core::LogTagTraitConst for LogTag {
	#[inline] fn as_raw_LogTag(&self) -> *const c_void { self.as_raw() }
}

impl core::LogTagTrait for LogTag {
	#[inline] fn as_raw_mut_LogTag(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LogTag {
	// LogTag(const char *, cv::utils::logging::LogLevel) /usr/include/opencv2/core/utils/logtag.hpp:20
	#[inline]
	pub fn new(_name: &str, _level: core::LogLevel) -> Result<core::LogTag> {
		extern_container_arg!(_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_utils_logging_LogTag_LogTag_const_charX_LogLevel(_name.opencv_as_extern(), _level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::LogTag::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// OriginalClassName /usr/include/opencv2/core/bindings_utils.hpp:227
pub trait OriginalClassNameTraitConst {
	fn as_raw_OriginalClassName(&self) -> *const c_void;

	// getIntParam() /usr/include/opencv2/core/bindings_utils.hpp:247
	#[inline]
	fn get_int_param(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_utils_nested_OriginalClassName_getIntParam_const(self.as_raw_OriginalClassName(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFloatParam() /usr/include/opencv2/core/bindings_utils.hpp:252
	#[inline]
	fn get_float_param(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_utils_nested_OriginalClassName_getFloatParam_const(self.as_raw_OriginalClassName(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait OriginalClassNameTrait: core::OriginalClassNameTraitConst {
	fn as_raw_mut_OriginalClassName(&mut self) -> *mut c_void;

}

// OriginalClassName /usr/include/opencv2/core/bindings_utils.hpp:227
pub struct OriginalClassName {
	ptr: *mut c_void
}

opencv_type_boxed! { OriginalClassName }

impl Drop for OriginalClassName {
	fn drop(&mut self) {
		extern "C" { fn cv_OriginalClassName_delete(instance: *mut c_void); }
		unsafe { cv_OriginalClassName_delete(self.as_raw_mut_OriginalClassName()) };
	}
}

unsafe impl Send for OriginalClassName {}

impl core::OriginalClassNameTraitConst for OriginalClassName {
	#[inline] fn as_raw_OriginalClassName(&self) -> *const c_void { self.as_raw() }
}

impl core::OriginalClassNameTrait for OriginalClassName {
	#[inline] fn as_raw_mut_OriginalClassName(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OriginalClassName {
	/// ## C++ default parameters
	/// * params: OriginalClassName::Params()
	// OriginalClassName(const OriginalClassName::Params &) /usr/include/opencv2/core/bindings_utils.hpp:242
	#[inline]
	pub fn new(params: core::OriginalClassName_Params) -> Result<core::OriginalClassName> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_utils_nested_OriginalClassName_OriginalClassName_const_ParamsR(&params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::OriginalClassName::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// originalName() /usr/include/opencv2/core/bindings_utils.hpp:257
	#[inline]
	pub fn original_name() -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_utils_nested_OriginalClassName_originalName(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * params: OriginalClassName::Params()
	// create(const OriginalClassName::Params &) /usr/include/opencv2/core/bindings_utils.hpp:263
	#[inline]
	pub fn create(params: core::OriginalClassName_Params) -> Result<core::Ptr<core::OriginalClassName>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_utils_nested_OriginalClassName_create_const_ParamsR(&params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<core::OriginalClassName>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Params /usr/include/opencv2/core/bindings_utils.hpp:230
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OriginalClassName_Params {
	pub int_value: i32,
	pub float_value: f32,
}

opencv_type_simple! { core::OriginalClassName_Params }

impl OriginalClassName_Params {
	/// ## C++ default parameters
	/// * int_param: 123
	/// * float_param: 3.5f
	// Params(int, float) /usr/include/opencv2/core/bindings_utils.hpp:235
	#[inline]
	pub fn new(int_param: i32, float_param: f32) -> Result<core::OriginalClassName_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_utils_nested_OriginalClassName_Params_Params_int_float(int_param, float_param, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
