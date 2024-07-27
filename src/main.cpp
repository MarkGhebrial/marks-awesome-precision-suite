#include <opencv2/opencv.hpp>
#include <iostream>
#include <math.h>

#include "util.h"

using std::vector;
typedef vector<cv::Point> Contour;
typedef vector<cv::Point2f> Contour2f;

using std::cout, std::endl;

cv::Mat sourceImage;
int threshold = 159;

int cornersOfPaper[4][2] = {{759, 797}, {2865, 406}, {3561, 1912}, {965, 2576}};

// The width and height of your paper target. These can be in any unit, but the units must be the same.
double targetWidth = 11;
double targetHeight = 8.5;

vector<cv::Point2f> findCornersOfTarget(vector<Contour> contours) {
    vector<Contour> quadrilaterals;

    // Find all the contours with four corners
    for (auto contour : contours) {
        Contour simplifiedContour;
        double epsilon = 0.02 * cv::arcLength(contour, true);
        cv::approxPolyDP(contour, simplifiedContour, epsilon, true); // Simplify the countour

        if (simplifiedContour.size() == 4) {
            quadrilaterals.push_back(simplifiedContour);
        }

        // vector<double> angles;
        // for (int i = 0; i < simplifiedContour.size(); i++) {

        //     double angle = angleBetweenPoints(simplifiedContour[(i-1) % simplifiedContour.size()], simplifiedContour[i], simplifiedContour[(i+1) % simplifiedContour.size()]);
        //     std::cout << "Degrees for contour " << i << ": " << angle * 180.0 / 3.14159265358979323846 << std::endl;

        //     cv::line(image, simplifiedContour[i], simplifiedContour[(i+1) % simplifiedContour.size()], cv::Scalar(0, 255, 0), 5);
        // }
    }

    // Find the contour with the biggest area
    Contour biggestContour;
    double areaOfBiggestContour = 0;
    for (auto contour : quadrilaterals) {
        double area = cv::contourArea(contour);
        if (area > areaOfBiggestContour) {
            areaOfBiggestContour = area;
            biggestContour = contour;
        }
    }

    Contour2f out;
    for (auto point : biggestContour) {
        out.push_back(point);
    }

    return out;
}

cv::Mat transformImage(cv::Mat image, Contour2f corners) {
    double aspectRatio = targetWidth / targetHeight;
    std::cout << "ASPECT RATIO" << aspectRatio << std::endl;

    vector<cv::Point2f> destinationPoints;
    destinationPoints.push_back(cv::Point2f(0, 0));
    destinationPoints.push_back(cv::Point2f(1500, 0));
    destinationPoints.push_back(cv::Point2f(1500, 1500 / aspectRatio));
    destinationPoints.push_back(cv::Point2f(0, 1500 / aspectRatio));

    // Make sure the corners are sorted in the right spatial order
    Contour2f sortedCorners(4);
    cv::Point2f center;
    float sumx = 0, sumy = 0;
    for (cv::Point2f point : corners) {
        sumx += point.x;
        sumy += point.y;
    }
    center.x = sumx / corners.size();
    center.y = sumy / corners.size();
    for (cv::Point2f point : corners) {
        if (point.x <= center.x && point.y <= center.y) {
            sortedCorners[0] = point;
        } else if (point.x >= center.x && point.y <= center.y) {
            sortedCorners[1] = point;
        } else if (point.x >= center.x && point.y >= center.y) {
            sortedCorners[2] = point;
        } else {
            sortedCorners[3] = point;
        }
    }

    cv::Mat transform = cv::getPerspectiveTransform(sortedCorners, destinationPoints, cv::DECOMP_LU);

    cv::warpPerspective(image, image, transform, cv::Size(1500, 1500/aspectRatio));

    return image;
}

struct processImageResults {
    cv::Mat image;
    Contour2f corners;
};

processImageResults processImage(cv::Mat image) {
    // Convert to grayscale
    cv::cvtColor(image, image, cv::ColorConversionCodes::COLOR_BGR2GRAY);

    // Blur
    cv::GaussianBlur(image, image, cv::Size(cv::Point2i(15, 15)), 0.0);

    cv::threshold(image, image, threshold, 255, cv::ThresholdTypes::THRESH_BINARY);
    erodeThenDilate(image, image);

    //image = transformImage(image);

    vector<Contour> contours;
    cv::findContours(image, contours, cv::RETR_CCOMP, cv::CHAIN_APPROX_SIMPLE);

    Contour2f corners = findCornersOfTarget(contours);

    // Covert back to rgb so we can draw the contours in a different color
    cv::cvtColor(image, image, cv::ColorConversionCodes::COLOR_GRAY2BGR);

    // Draw all the contours onto the image
    for (auto contour : contours) {
        for (int i = 0; i < contour.size(); i++) {
            cv::line(image, contour[i], contour[(i+1) % contour.size()], cv::Scalar(0, 255, 0), 5);
        }
    }

    cout << corners.size() << endl;

    // Draw the borders of the target onto the image
    for (auto contour : corners) {
        for (int i = 0; i < corners.size(); i++) {
            cv::line(image, corners[i], corners[(i+1) % corners.size()], cv::Scalar(255, 255, 0), 50);
        }
    }

    return {image, corners};
}

void updateImage(int, void*) {
    processImageResults results = processImage(sourceImage);
    cv::imshow("Display window", results.image);

    cv::imshow("Transformed image", transformImage(sourceImage, results.corners));
}

int main(int argc, char** argv) {
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

    processImageResults results = processImage(sourceImage);

    // Create a window
    cv::namedWindow("Display window", cv::WINDOW_NORMAL);
    cv::createTrackbar("Threshold", "Display window", &threshold, 255, updateImage);
    
    // Create a window for the transformed image
    cv::namedWindow("Transformed image", cv::WINDOW_NORMAL);
    cv::imshow("Transformed image", transformImage(sourceImage, results.corners));

    // Show our image inside the created window
    cv::imshow("Display window", results.image);

    // Wait for any keystroke in the window
    cv::waitKey(0);

    return 0;
}