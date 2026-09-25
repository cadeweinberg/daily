# SPDX-License-Identifier: GPL-3.0-only
cmake_minimum_required(VERSION 3.16)

project(daily
    VERSION 0.0.1.0
    DESCRIPTION "A daemon to ask you questions each day"
    LANGUAGES C
    HOMEPAGE_URL "github.com/cadeweinberg/daily"
)

set(DAILY_BASE_DIR ${PROJECT_SOURCE_DIR})
set(DAILY_SOURCE_DIR ${DAILY_BASE_DIR}/source)
set(DAILY_INCLUDE_DIR ${DAILY_BASE_DIR}/include)
set(DAILY_BUILD_DIR ${PROJECT_BINARY_DIR})

set(DAILY_WINDOW_DEFAULT_TITLE "Daily")
set(DAILY_WINDOW_DEFAULT_HEIGHT 720)
set(DAILY_WINDOW_DEFAULT_WIDTH  1024)

set(DAILY_WARNINGS
    -Wall
    -Wextra
    -Wconversion
)

find_package(PkgConfig REQUIRED)

pkg_check_modules(GTK REQUIRED IMPORTED_TARGET gtk4>=4.20)

# build main project

configure_file(
    ${DAILY_INCLUDE_DIR}/common/config.h.in
    ${DAILY_INCLUDE_DIR}/common/config.h
)

add_library(daily_common INTERFACE)

target_sources(daily_common PUBLIC
    ${DAILY_INCLUDE_DIR}/common/config.h
)

add_library(daily_data)

target_sources(daily_data PUBLIC
    ${DAILY_INCLUDE_DIR}/data/sqlite3.h
)

target_sources(daily_data PRIVATE
    ${DAILY_SOURCE_DIR}/data/sqlite3.c
)

add_executable(daily)

target_include_directories(daily PRIVATE
    ${DAILY_INCLUDE_DIR}
)

target_link_libraries(daily PRIVATE 
    PkgConfig::GTK
    daily_data
    daily_common
)

target_sources(daily PUBLIC
    ${DAILY_INCLUDE_DIR}/app/application.h
)

target_sources(daily PRIVATE
    ${DAILY_SOURCE_DIR}/app/application.c
    ${DAILY_SOURCE_DIR}/app/main.c
)

add_executable(daily_notify)

target_sources(daily_notify PRIVATE
    ${DAILY_SOURCE_DIR}/notify/main.c
)

target_link_libraries(daily_notify PRIVATE
    daily_data
)