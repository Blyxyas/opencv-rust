extern "C" {
	void cv_PtrOfSelectiveSearchSegmentationStrategyFill_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
		delete instance;
	}

	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
		return instance->get();
	}

	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrOfSelectiveSearchSegmentationStrategyFill_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
		return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}
}

