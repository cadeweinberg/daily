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

set(DAILY_WARNINGS
    -Wall
    -Wextra
    -Wconversion
)

find_package(PkgConfig REQUIRED)

pkg_check_modules(GTK REQUIRED IMPORTED_TARGET gtk4>=4.20)

# build main project

add_library(daily_data)

target_sources(daily_data PUBLIC
    ${DAILY_INCLUDE_DIR}/data/sqlite3.h
)

target_sources(daily_data PRIVATE
    ${DAILY_SOURCE_DIR}/data/sqlite3.c
)

add_executable(daily)

target_link_libraries(daily PRIVATE 
    PkgConfig::GTK
    daily_data
)

target_sources(daily PRIVATE
    ${DAILY_SOURCE_DIR}/app/main.c
)