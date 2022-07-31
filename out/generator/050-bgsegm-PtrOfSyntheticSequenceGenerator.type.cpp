extern "C" {
	cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* cv_PtrOfSyntheticSequenceGenerator_new(cv::bgsegm::SyntheticSequenceGenerator* val) {
		return new cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>(val);
	}
	
	void cv_PtrOfSyntheticSequenceGenerator_delete(cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
		delete instance;
	}

	const cv::bgsegm::SyntheticSequenceGenerator* cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr(const cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
		return instance->get();
	}

	cv::bgsegm::SyntheticSequenceGenerator* cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
		return instance->get();
	}
}

