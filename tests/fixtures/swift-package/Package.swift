// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "TextKit",
    products: [.library(name: "TextKit", targets: ["TextKit"])],
    dependencies: [
        .package(url: "https://github.com/apple/swift-algorithms", from: "1.2.0"),
    ],
    targets: [
        .target(name: "TextKit"),
        .testTarget(name: "TextKitTests", dependencies: ["TextKit"]),
    ]
)
