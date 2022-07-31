extern "C" {
	void cv_PtrOfSelectiveSearchSegmentationStrategyColor_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* instance) {
		delete instance;
	}

	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* instance) {
		return instance->get();
	}

	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrOfSelectiveSearchSegmentationStrategyColor_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* instance) {
		return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}
}

