cmake_minimum_required(VERSION 3.4.1)

set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -u ANativeActivity_onCreate")

find_library(log-lib log)

add_library(native-lib SHARED native-lib.cpp)
add_library(rust-lib STATIC IMPORTED)

if (ANDROID_ABI STREQUAL x86)
    set(RUST_ARCH i686-linux-android)
elseif (ANDROID_ABI STREQUAL armeabi-v7a)
    set(RUST_ARCH armv7-linux-androideabi)
elseif (ANDROID_ABI STREQUAL arm64-v8a)
    set(RUST_ARCH aarch64-linux-android)
elseif (ANDROID_ABI STREQUAL x86_64)
    set(RUST_ARCH x86_64-linux-android)
else ()
    message(FATAL "Unknown architecture")
endif ()

set_property(TARGET rust-lib PROPERTY IMPORTED_LOCATION_DEBUG ${CMAKE_CURRENT_LIST_DIR}/app/target/${RUST_ARCH}/debug/libapp.a)
set_property(TARGET rust-lib PROPERTY IMPORTED_LOCATION_RELEASE ${CMAKE_CURRENT_LIST_DIR}/app/target/${RUST_ARCH}/release/libapp.a)

target_link_libraries(native-lib android rust-lib ${log-lib})
