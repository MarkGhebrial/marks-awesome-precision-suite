#include <opencv2/opencv.hpp>
#include <iostream>
#include <math.h>

void erodeThenDilate(const cv::Mat& inputImage, cv::Mat outputImage, int size) {
    // Circle with diameter of 30
    auto kernel = cv::getStructuringElement(cv::MORPH_ELLIPSE, cv::Size(size, size));

    cv::erode(inputImage, outputImage, kernel);
    cv::dilate(outputImage, outputImage, kernel);
}

double angleBetweenPoints(cv::Point p1, cv::Point corner, cv::Point p3) {
    //cv::Vec<cv::int, 2> v1 = cv::Vec((corner - p1).x, 0);

    cv::Point v1 = p1 - corner;
    cv::Point v2 = p3 - corner;

    double v1length = std::sqrt(v1.x*v1.x + v1.y*v1.y);
    double v2length = std::sqrt(v2.x*v2.x + v2.y*v2.y);

    return std::acos(v1.dot(v2)/(v1length*v2length));
}

void contrast(const cv::Mat& inputImage, cv::Mat outputImage, double alpha, double beta) {
    for (int i = 0; i < inputImage.rows; i++) {
        for (int j = 0; j < inputImage.cols; j++) {
            for( int c = 0; c < 3; c++ ) {
                outputImage.at<cv::Vec3b>(i, j)[c] =
                    cv::saturate_cast<uchar>(alpha*inputImage.at<cv::Vec3b>(i,j)[c] + beta);
            }
        }
    }
}