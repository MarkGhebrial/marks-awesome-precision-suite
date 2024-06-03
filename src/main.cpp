#include <opencv2/opencv.hpp>
#include <iostream>

#include "util.h"

cv::Mat sourceImage;
int threshold = 30;

int cornersOfPaper[4][2] = {{759, 797}, {2865, 406}, {3561, 1912}, {965, 2576}};

// The width and height of your paper target. These can be in any unit, but the units must be the same.
double targetWidth = 11;
double targetHeight = 8.5;

std::vector<cv::Point2f> findCornersOfTarget() {
    std::vector<cv::Point2f> out;

    for (int i = 0; i < 4; i++) {
        out.push_back(cv::Point2f(cornersOfPaper[i][0], cornersOfPaper[i][1]));
    }

    return out;
}

cv::Mat transformImage(cv::Mat image) {
    double aspectRatio = targetWidth / targetHeight;
    std::cout << "ASPECT RATIO" << aspectRatio << std::endl;

    std::vector<cv::Point2f> destinationPoints;
    destinationPoints.push_back(cv::Point2f(0, 0));
    destinationPoints.push_back(cv::Point2f(1500, 0));
    destinationPoints.push_back(cv::Point2f(1500, 1500 / aspectRatio));
    destinationPoints.push_back(cv::Point2f(0, 1500 / aspectRatio));

    cv::Mat transform = cv::getPerspectiveTransform(findCornersOfTarget(), destinationPoints, cv::DECOMP_LU);

    cv::warpPerspective(image, image, transform, cv::Size(1500, 1500/aspectRatio));

    return image;
}

cv::Mat processImage(cv::Mat image) {
    // Convert to grayscale
    cv::cvtColor(image, image, cv::ColorConversionCodes::COLOR_BGR2GRAY);

    // Blur
    cv::GaussianBlur(image, image, cv::Size(cv::Point2i(15, 15)), 0.0);

    cv::threshold(image, image, threshold, 255, cv::ThresholdTypes::THRESH_BINARY_INV);

    //image = transformImage(image);

    std::vector<std::vector<cv::Point>> contours;
    //cv::OutputArray fdsafdss;
    cv::findContours(image, contours, cv::RETR_CCOMP, cv::CHAIN_APPROX_SIMPLE);

    // Covert back to rgb so we can draw the contours in a different color
    cv::cvtColor(image, image, cv::ColorConversionCodes::COLOR_GRAY2BGR);

    for (auto contour : contours) {
        for (int i = 0; i < contour.size() - 1; i++) {
            cv::line(image, contour[i], contour[i+1], cv::Scalar(0, 255, 0), 2);
        }
    }

    return image;
}

void reprocessImage(int, void*) {
    cv::imshow("Display window", processImage(sourceImage));
}

int main(int argc, char** argv) {
    random_function();

    // Check if image path is provided as a command line argument
    if (argc != 2) {
        std::cout << "Usage: ./DisplayImage <Image_Path>" << std::endl;
        return -1;
    }

    // Read the image file
    sourceImage = cv::imread(argv[1]);

    // Scale the image so it fits on my screen 
    //cv::resize(sourceImage, sourceImage, cv::Size(), 0.2, 0.2, cv::INTER_LINEAR);

    // Check for failure
    if (sourceImage.empty()) {
        std::cout << "Could not open or find the image" << std::endl;
        return -1;
    }

    cv::Mat image = processImage(sourceImage);

    // Create a window
    cv::namedWindow("Display window", cv::WINDOW_NORMAL);
    cv::createTrackbar("Threshold", "Display window", &threshold, 255, reprocessImage);
    
    // Create a window for the transformed image
    cv::namedWindow("Transformed image", cv::WINDOW_NORMAL);
    cv::imshow("Transformed image", transformImage(sourceImage));

    // Show our image inside the created window
    cv::imshow("Display window", image);

    // Wait for any keystroke in the window
    cv::waitKey(0);

    return 0;
}