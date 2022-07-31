extern "C" {
	cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* cv_PtrOfBriefDescriptorExtractor_new(cv::xfeatures2d::BriefDescriptorExtractor* val) {
		return new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>(val);
	}
	
	void cv_PtrOfBriefDescriptorExtractor_delete(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::BriefDescriptorExtractor* cv_PtrOfBriefDescriptorExtractor_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::BriefDescriptorExtractor* cv_PtrOfBriefDescriptorExtractor_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfBriefDescriptorExtractor_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

