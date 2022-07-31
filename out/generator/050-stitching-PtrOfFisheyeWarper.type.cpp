extern "C" {
	cv::Ptr<cv::FisheyeWarper>* cv_PtrOfFisheyeWarper_new(cv::FisheyeWarper* val) {
		return new cv::Ptr<cv::FisheyeWarper>(val);
	}
	
	void cv_PtrOfFisheyeWarper_delete(cv::Ptr<cv::FisheyeWarper>* instance) {
		delete instance;
	}

	const cv::FisheyeWarper* cv_PtrOfFisheyeWarper_get_inner_ptr(const cv::Ptr<cv::FisheyeWarper>* instance) {
		return instance->get();
	}

	cv::FisheyeWarper* cv_PtrOfFisheyeWarper_get_inner_ptr_mut(cv::Ptr<cv::FisheyeWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfFisheyeWarper_to_PtrOfWarperCreator(cv::Ptr<cv::FisheyeWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

