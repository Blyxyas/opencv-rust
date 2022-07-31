extern "C" {
	void cv_VectorOfElliptic_KeyPoint_delete(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
		delete instance;
	}

	std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* cv_VectorOfElliptic_KeyPoint_new() {
		return new std::vector<cv::xfeatures2d::Elliptic_KeyPoint>();
	}

	size_t cv_VectorOfElliptic_KeyPoint_len(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
		return instance->size();
	}

	bool cv_VectorOfElliptic_KeyPoint_is_empty(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfElliptic_KeyPoint_capacity(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfElliptic_KeyPoint_resize(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfElliptic_KeyPoint_shrink_to_fit(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfElliptic_KeyPoint_reserve(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfElliptic_KeyPoint_remove(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfElliptic_KeyPoint_swap(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfElliptic_KeyPoint_clear(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
		instance->clear();
	}

	void cv_VectorOfElliptic_KeyPoint_push(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, cv::xfeatures2d::Elliptic_KeyPoint* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfElliptic_KeyPoint_insert(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index, cv::xfeatures2d::Elliptic_KeyPoint* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfElliptic_KeyPoint_get(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index, cv::xfeatures2d::Elliptic_KeyPoint** ocvrs_return) {
		*ocvrs_return = new cv::xfeatures2d::Elliptic_KeyPoint((*instance)[index]);
	}

	void cv_VectorOfElliptic_KeyPoint_set(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index, cv::xfeatures2d::Elliptic_KeyPoint* val) {
		(*instance)[index] = *val;
	}

}


